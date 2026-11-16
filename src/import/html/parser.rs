//! Builds [`SlideContent`] from tokenized HTML events.

use super::style::{InlineStyle, is_font_style_italic, is_font_weight_bold};
use super::tokenizer::HtmlEvent;
use crate::generator::slide_content::{BulletPoint, BulletTextFormat};
use crate::generator::{CodeBlock, SlideContent};
use super::HtmlParseOptions;

// ---------------------------------------------------------------------------
// Slide builder from events
// ---------------------------------------------------------------------------

/// Tags whose content should be entirely skipped
const SKIP_TAGS: &[&str] = &[
    "script", "style", "noscript", "nav", "form", "svg", "canvas", "iframe", "title",
];

/// Tags that are void (self-closing) and should not push/pop the style stack
const VOID_TAGS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

pub(crate) struct HtmlSlideParser {
    options: HtmlParseOptions,
    slides: Vec<SlideContent>,
    current_slide: Option<SlideContent>,
    text_buffer: String,
    tag_stack: Vec<String>,
    style_stack: Vec<InlineStyle>,
    in_list: bool,
    in_table: bool,
    in_code: bool,
    in_blockquote: bool,
    italic: bool,
    list_items: Vec<(String, Option<BulletTextFormat>)>,
    table_rows: Vec<Vec<String>>,
    current_row: Vec<String>,
    current_cell: String,
    code_content: String,
    blockquote_text: String,
    presentation_title: Option<String>,
    current_href: Option<String>,
}

impl HtmlSlideParser {
    pub(crate) fn new(options: HtmlParseOptions) -> Self {
        Self {
            options,
            slides: Vec::new(),
            current_slide: None,
            text_buffer: String::new(),
            tag_stack: Vec::new(),
            style_stack: Vec::new(),
            in_list: false,
            in_table: false,
            in_code: false,
            in_blockquote: false,
            italic: false,
            list_items: Vec::new(),
            table_rows: Vec::new(),
            current_row: Vec::new(),
            current_cell: String::new(),
            code_content: String::new(),
            blockquote_text: String::new(),
            presentation_title: None,
            current_href: None,
        }
    }

    /// Return the current active style (top of the style stack)
    fn active_style(&self) -> Option<&InlineStyle> {
        self.style_stack.last()
    }

    pub(crate) fn parse(&mut self, events: &[HtmlEvent]) -> Result<Vec<SlideContent>, String> {
        for event in events {
            match event {
                HtmlEvent::OpenTag { name, attrs } => {
                    self.tag_stack.push(name.clone());
                    self.handle_open_tag(name, attrs);
                }
                HtmlEvent::CloseTag(name) => {
                    self.handle_close_tag(name);
                    self.tag_stack.pop();
                }
                HtmlEvent::Text(text) => {
                    self.handle_text(text);
                }
            }
        }

        self.finalize_current_slide();

        if self.slides.is_empty() {
            return Err("No slide content found in HTML".to_string());
        }

        // Trim to max_slides
        if self.slides.len() > self.options.max_slides {
            self.slides.truncate(self.options.max_slides);
        }

        Ok(std::mem::take(&mut self.slides))
    }

    fn is_inside_skip_tag(&self) -> bool {
        self.tag_stack
            .iter()
            .any(|t| SKIP_TAGS.contains(&t.as_str()))
    }

    fn handle_open_tag(&mut self, name: &str, attrs: &[(String, String)]) {
        if self.is_inside_skip_tag() {
            return;
        }

        // Push style for this element (inherits from parent). Skip void tags.
        if !VOID_TAGS.contains(&name) {
            let parent = self.style_stack.last().cloned().unwrap_or_default();
            let style = if let Some(style_attr) = attrs.iter().find(|(k, _)| k == "style") {
                parent.merge(&InlineStyle::parse(&style_attr.1))
            } else {
                parent
            };
            self.style_stack.push(style);
        }

        match name {
            "h1" => {
                self.flush_text_buffer();
                self.finalize_current_slide();
            }
            "h2" | "h3" | "h4" | "h5" | "h6" => {
                self.flush_text_buffer();
            }
            "p" | "div" | "article" | "section" | "main" | "li" => {}
            "pre" => {
                self.in_code = true;
                self.code_content.clear();
            }
            "table" => {
                self.in_table = true;
                self.table_rows.clear();
            }
            "blockquote" => {
                self.in_blockquote = true;
                self.blockquote_text.clear();
            }
            "ul" | "ol" => {
                self.in_list = true;
                self.list_items.clear();
            }
            "strong" | "b" => {
                self.text_buffer.push_str("**");
            }
            "em" | "i" => {
                self.text_buffer.push('*');
                self.italic = true;
            }
            "title" => {}
            "img" => {
                if self.options.include_images {
                    let alt = attrs
                        .iter()
                        .find(|(k, _)| k == "alt")
                        .map(|(_, v)| v.as_str())
                        .unwrap_or("");
                    let src = attrs
                        .iter()
                        .find(|(k, _)| k == "src")
                        .map(|(_, v)| v.as_str())
                        .unwrap_or("");

                    if !src.is_empty() {
                        // Try to download and embed the actual image
                        if let Some(image) = self.load_image(src, alt) {
                            if let Some(ref mut slide) = self.current_slide {
                                slide.images.push(image);
                            } else {
                                let mut slide = SlideContent::new("Image");
                                slide.images.push(image);
                                self.current_slide = Some(slide);
                            }
                        } else {
                            // Fallback to placeholder if image loading fails
                            let label = if alt.is_empty() { src } else { alt };
                            self.add_paragraph(&format!("[Image: {}]", label));
                        }
                    }
                }
            }
            "a" => {
                // Handle hyperlinks - just mark that we're inside a link
                // The actual link text will be handled in text processing
                if let Some(href) = attrs
                    .iter()
                    .find(|(k, _)| k == "href")
                    .map(|(_, v)| v.as_str())
                {
                    // Store the current href for later use
                    self.current_href = Some(href.to_string());
                }
            }
            "br" => {
                self.text_buffer.push('\n');
            }
            "hr" => {
                self.flush_text_buffer();
                self.finalize_current_slide();
            }
            _ => {}
        }
    }

    fn handle_close_tag(&mut self, name: &str) {
        if self.is_inside_skip_tag() {
            return;
        }

        match name {
            "h1" => {
                let title = std::mem::take(&mut self.text_buffer).trim().to_string();
                if self.presentation_title.is_none() && !title.is_empty() {
                    self.presentation_title = Some(title.clone());
                }
                let slide_title = if title.is_empty() {
                    "Slide".to_string()
                } else {
                    title
                };
                let mut slide = SlideContent::new(&slide_title);
                // Apply title-level styles from active style
                if let Some(s) = self.active_style() {
                    if let Some(ref c) = s.color {
                        slide = slide.title_color(c);
                    }
                    if let Some(sz) = s.font_size {
                        slide = slide.title_size(sz);
                    }
                    if let Some(ref fw) = s.font_weight {
                        if is_font_weight_bold(fw) {
                            slide = slide.title_bold(true);
                        }
                    }
                    if let Some(ref fs) = s.font_style {
                        if is_font_style_italic(fs) {
                            slide = slide.title_italic(true);
                        }
                    }
                    if let Some(ref td) = s.text_decoration {
                        if td.contains("underline") {
                            slide = slide.title_underline(true);
                        }
                    }
                }
                self.current_slide = Some(slide);
            }
            "h2" | "h3" | "h4" | "h5" | "h6" => {
                let text = std::mem::take(&mut self.text_buffer).trim().to_string();
                if !text.is_empty() {
                    self.add_formatted_text(&format!("**{}**", text));
                }
            }
            "p" => {
                let text = std::mem::take(&mut self.text_buffer).trim().to_string();
                if !text.is_empty() {
                    self.add_paragraph(&text);
                }
            }
            "div" | "article" | "section" | "main" => {
                let text = std::mem::take(&mut self.text_buffer).trim().to_string();
                if !text.is_empty() {
                    self.add_paragraph(&text);
                }
            }
            "li" => {
                let item = std::mem::take(&mut self.text_buffer).trim().to_string();
                if !item.is_empty() {
                    let item_style = self.active_style().and_then(|s| s.to_bullet_format());
                    self.list_items.push((item, item_style));
                }
            }
            "ul" | "ol" => {
                self.flush_list_items();
                self.in_list = false;
            }
            "pre" => {
                self.in_code = false;
                self.flush_code_block();
            }
            "table" => {
                self.in_table = false;
                self.flush_table();
            }
            "blockquote" => {
                self.in_blockquote = false;
                self.flush_blockquote();
            }
            "th" | "td" => {
                let cell = std::mem::take(&mut self.current_cell).trim().to_string();
                self.current_row.push(cell);
            }
            "tr" => {
                if !self.current_row.is_empty() {
                    self.table_rows.push(std::mem::take(&mut self.current_row));
                    self.current_row = Vec::new();
                }
            }
            "strong" | "b" => {
                self.text_buffer.push_str("**");
            }
            "em" | "i" => {
                self.text_buffer.push('*');
                self.italic = false;
            }
            "a" => {
                // When closing an anchor tag, we have the link text in the buffer
                // and the href in current_href. For now, we'll just clear the href
                // since PowerPoint hyperlink support requires more complex XML handling
                self.current_href = None;
            }
            _ => {}
        }

        // Pop style stack for non-void tags (mirrors push in handle_open_tag)
        if !VOID_TAGS.contains(&name) {
            self.style_stack.pop();
        }
    }

    fn handle_text(&mut self, text: &str) {
        if self.is_inside_skip_tag() {
            return;
        }

        if self.in_code {
            self.code_content.push_str(text);
        } else if self.in_table {
            self.current_cell.push_str(text);
        } else if self.in_blockquote {
            self.blockquote_text.push_str(text);
        } else {
            self.text_buffer.push_str(text);
        }
    }

    fn add_formatted_text(&mut self, text: &str) {
        let fmt = self.active_style().and_then(|s| s.to_bullet_format());
        if let Some(ref mut slide) = self.current_slide {
            let mut bp = BulletPoint::new(text).with_style(slide.bullet_style);
            if let Some(ref f) = fmt {
                bp = bp.with_format(f.clone());
            }
            slide.content.push(text.to_string());
            slide.bullets.push(bp);
        } else {
            let mut slide = SlideContent::new("Slide");
            let mut bp = BulletPoint::new(text).with_style(slide.bullet_style);
            if let Some(ref f) = fmt {
                bp = bp.with_format(f.clone());
            }
            slide.content.push(text.to_string());
            slide.bullets.push(bp);
            self.current_slide = Some(slide);
        }
    }

    fn add_paragraph(&mut self, text: &str) {
        let fmt = self.active_style().and_then(|s| s.to_bullet_format());
        if let Some(ref mut slide) = self.current_slide {
            if slide.content.len() < self.options.max_bullets {
                let mut bp = BulletPoint::new(text).with_style(slide.bullet_style);
                if let Some(ref f) = fmt {
                    bp = bp.with_format(f.clone());
                }
                slide.content.push(text.to_string());
                slide.bullets.push(bp);
            }
        } else {
            let title = self
                .presentation_title
                .clone()
                .unwrap_or_else(|| "Overview".to_string());
            let mut slide = SlideContent::new(&title);
            let mut bp = BulletPoint::new(text).with_style(slide.bullet_style);
            if let Some(ref f) = fmt {
                bp = bp.with_format(f.clone());
            }
            slide.content.push(text.to_string());
            slide.bullets.push(bp);
            self.current_slide = Some(slide);
        }
    }

    fn flush_text_buffer(&mut self) {
        let text = std::mem::take(&mut self.text_buffer);
        let trimmed = text.trim().to_string();
        if !trimmed.is_empty() {
            self.add_paragraph(&trimmed);
        }
    }

    fn flush_list_items(&mut self) {
        let items = std::mem::take(&mut self.list_items);
        if items.is_empty() {
            return;
        }

        if let Some(ref mut slide) = self.current_slide {
            for (item, item_style) in items {
                if slide.content.len() < self.options.max_bullets {
                    let mut bp = BulletPoint::new(&item).with_style(slide.bullet_style);
                    if let Some(ref f) = item_style {
                        bp = bp.with_format(f.clone());
                    }
                    slide.content.push(item);
                    slide.bullets.push(bp);
                }
            }
        } else {
            let title = self
                .presentation_title
                .clone()
                .unwrap_or_else(|| "Key Points".to_string());
            let mut slide = SlideContent::new(&title);
            for (item, item_style) in items {
                if slide.content.len() < self.options.max_bullets {
                    let mut bp = BulletPoint::new(&item).with_style(slide.bullet_style);
                    if let Some(ref f) = item_style {
                        bp = bp.with_format(f.clone());
                    }
                    slide.content.push(item);
                    slide.bullets.push(bp);
                }
            }
            self.current_slide = Some(slide);
        }
    }

    fn flush_table(&mut self) {
        if !self.options.include_tables || self.table_rows.is_empty() {
            return;
        }

        let rows = std::mem::take(&mut self.table_rows);
        let table = crate::generator::table::table_from_string_rows(rows, true);

        if let Some(ref mut slide) = self.current_slide {
            slide.table = Some(table);
            slide.has_table = true;
        } else {
            let mut slide = SlideContent::new("Data Table");
            slide.table = Some(table);
            slide.has_table = true;
            self.current_slide = Some(slide);
        }
    }

    fn flush_code_block(&mut self) {
        if !self.options.include_code || self.code_content.is_empty() {
            return;
        }

        let code = std::mem::take(&mut self.code_content);
        let code_block = CodeBlock::new(code.trim(), "text");

        if let Some(ref mut slide) = self.current_slide {
            slide.code_blocks.push(code_block);
        } else {
            let mut slide = SlideContent::new("Code");
            slide.code_blocks.push(code_block);
            self.current_slide = Some(slide);
        }
    }

    fn flush_blockquote(&mut self) {
        let text = std::mem::take(&mut self.blockquote_text).trim().to_string();
        if text.is_empty() {
            return;
        }

        if let Some(ref mut slide) = self.current_slide {
            slide.notes = Some(text);
        }
    }

    fn finalize_current_slide(&mut self) {
        self.flush_text_buffer();
        self.flush_list_items();
        if let Some(slide) = self.current_slide.take() {
            self.slides.push(slide);
        }
    }

    /// Load an image from URL or local file path
    fn load_image(&self, src: &str, _alt: &str) -> Option<crate::generator::Image> {
        use crate::generator::ImageBuilder;
        use std::path::Path;

        // Check if it's a URL
        if src.starts_with("http://") || src.starts_with("https://") {
            // Try to download the image
            #[cfg(feature = "web2ppt")]
            {
                if let Ok(bytes) = self.download_image(src) {
                    let img = ImageBuilder::auto(bytes)
                        .at(2000000, 2000000)
                        .size(5000000, 3000000)
                        .build();
                    return Some(img);
                }
            }
            None
        } else {
            // Try to load from local file path
            let path = Path::new(src);
            if path.exists() {
                if let Ok(bytes) = std::fs::read(path) {
                    let img = ImageBuilder::auto(bytes)
                        .at(2000000, 2000000)
                        .size(5000000, 3000000)
                        .build();
                    return Some(img);
                }
            }
            None
        }
    }

    /// Download an image from a URL (requires web2ppt feature)
    #[cfg(feature = "web2ppt")]
    fn download_image(&self, url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use reqwest::blocking::Client;
        use std::time::Duration;

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
            .build()?;

        let response = client.get(url).send()?;
        if response.status().is_success() {
            Ok(response.bytes()?.to_vec())
        } else {
            Err(format!("Failed to download image: {}", response.status()).into())
        }
    }
}
