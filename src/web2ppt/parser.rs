//! HTML parser for Web2PPT

use super::{Web2PptError, Result, Web2PptConfig};
use scraper::{Html, Selector, ElementRef};

/// Type of content block
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentType {
    /// Main title (h1)
    Title,
    /// Section heading (h2-h6)
    Heading(u8),
    /// Paragraph text
    Paragraph,
    /// List item
    ListItem,
    /// Code block
    Code,
    /// Image with URL and alt text
    Image { src: String, alt: String },
    /// Table data
    Table(Vec<Vec<String>>),
    /// Blockquote
    Quote,
    /// Link with URL
    Link { text: String, href: String },
}

/// A block of content extracted from the page
#[derive(Clone, Debug)]
pub struct ContentBlock {
    /// Type of content
    pub content_type: ContentType,
    /// Text content
    pub text: String,
    /// Nesting level (for lists)
    pub level: u8,
}

impl ContentBlock {
    /// Create a new content block
    pub fn new(content_type: ContentType, text: &str) -> Self {
        ContentBlock {
            content_type,
            text: text.trim().to_string(),
            level: 0,
        }
    }

    /// Create with level
    pub fn with_level(mut self, level: u8) -> Self {
        self.level = level;
        self
    }

    /// Check if this is a heading
    pub fn is_heading(&self) -> bool {
        matches!(self.content_type, ContentType::Title | ContentType::Heading(_))
    }

    /// Get heading level (1 for title, 2-6 for headings)
    pub fn heading_level(&self) -> Option<u8> {
        match self.content_type {
            ContentType::Title => Some(1),
            ContentType::Heading(level) => Some(level),
            _ => None,
        }
    }
}

/// Extracted web content
#[derive(Clone, Debug)]
pub struct WebContent {
    /// Page title
    pub title: String,
    /// Page URL
    pub url: String,
    /// Meta description
    pub description: Option<String>,
    /// Content blocks
    pub blocks: Vec<ContentBlock>,
    /// Images found
    pub images: Vec<(String, String)>, // (src, alt)
}

impl WebContent {
    /// Create empty web content
    pub fn new(url: &str) -> Self {
        WebContent {
            title: String::new(),
            url: url.to_string(),
            description: None,
            blocks: Vec::new(),
            images: Vec::new(),
        }
    }

    /// Check if content is empty
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Get all headings
    pub fn headings(&self) -> Vec<&ContentBlock> {
        self.blocks.iter().filter(|b| b.is_heading()).collect()
    }

    /// Get content grouped by headings
    pub fn grouped_by_headings(&self) -> Vec<(&ContentBlock, Vec<&ContentBlock>)> {
        let mut groups: Vec<(&ContentBlock, Vec<&ContentBlock>)> = Vec::new();
        let mut current_heading: Option<&ContentBlock> = None;
        let mut current_content: Vec<&ContentBlock> = Vec::new();

        for block in &self.blocks {
            if block.is_heading() {
                // Save previous group
                if let Some(heading) = current_heading {
                    groups.push((heading, current_content));
                    current_content = Vec::new();
                }
                current_heading = Some(block);
            } else {
                current_content.push(block);
            }
        }

        // Save last group
        if let Some(heading) = current_heading {
            groups.push((heading, current_content));
        }

        groups
    }
}

/// HTML parser for extracting content
pub struct WebParser {
    config: Web2PptConfig,
}

impl WebParser {
    /// Create a new parser with default config
    pub fn new() -> Self {
        Self::with_config(Web2PptConfig::default())
    }

    /// Create a new parser with custom config
    pub fn with_config(config: Web2PptConfig) -> Self {
        WebParser { config }
    }

    /// Parse HTML content
    pub fn parse(&self, html: &str, url: &str) -> Result<WebContent> {
        let document = Html::parse_document(html);
        let mut content = WebContent::new(url);

        // Extract title
        content.title = self.extract_title(&document);

        // Extract meta description
        content.description = self.extract_meta_description(&document);

        // Extract main content
        self.extract_content(&document, &mut content)?;

        if content.is_empty() {
            return Err(Web2PptError::NoContent);
        }

        Ok(content)
    }

    /// Extract page title
    fn extract_title(&self, document: &Html) -> String {
        // Try <title> tag first
        if let Ok(selector) = Selector::parse("title") {
            if let Some(element) = document.select(&selector).next() {
                let title = element.text().collect::<String>().trim().to_string();
                if !title.is_empty() {
                    return title;
                }
            }
        }

        // Try h1
        if let Ok(selector) = Selector::parse("h1") {
            if let Some(element) = document.select(&selector).next() {
                let title = element.text().collect::<String>().trim().to_string();
                if !title.is_empty() {
                    return title;
                }
            }
        }

        // Try og:title
        if let Ok(selector) = Selector::parse("meta[property='og:title']") {
            if let Some(element) = document.select(&selector).next() {
                if let Some(content) = element.value().attr("content") {
                    return content.trim().to_string();
                }
            }
        }

        "Untitled".to_string()
    }

    /// Extract meta description
    fn extract_meta_description(&self, document: &Html) -> Option<String> {
        // Try meta description
        if let Ok(selector) = Selector::parse("meta[name='description']") {
            if let Some(element) = document.select(&selector).next() {
                if let Some(content) = element.value().attr("content") {
                    let desc = content.trim().to_string();
                    if !desc.is_empty() {
                        return Some(desc);
                    }
                }
            }
        }

        // Try og:description
        if let Ok(selector) = Selector::parse("meta[property='og:description']") {
            if let Some(element) = document.select(&selector).next() {
                if let Some(content) = element.value().attr("content") {
                    let desc = content.trim().to_string();
                    if !desc.is_empty() {
                        return Some(desc);
                    }
                }
            }
        }

        None
    }

    /// Extract main content from document - preserving document order
    fn extract_content(&self, document: &Html, content: &mut WebContent) -> Result<()> {
        // Try to find main content area
        let main_selectors = [
            "main article",
            "article",
            "main",
            "[role='main']",
            ".content",
            ".post-content",
            ".article-content",
            ".entry-content",
            ".markdown-body",
            ".prose",
            "#content",
            "#main",
            "#article",
            ".article",
            "body",
        ];

        let mut main_element: Option<ElementRef> = None;

        for selector_str in &main_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    // Check if this element has meaningful content
                    let text_len: usize = element.text().collect::<String>().len();
                    if text_len > 100 {
                        main_element = Some(element);
                        break;
                    }
                }
            }
        }

        let main = main_element.ok_or(Web2PptError::NoContent)?;

        // Extract content in document order by walking the DOM
        self.walk_element(&main, content, 0);

        Ok(())
    }

    /// Walk element tree and extract content in order
    fn walk_element(&self, element: &ElementRef, content: &mut WebContent, depth: u8) {
        // Skip script, style, nav, footer, aside, header elements
        let tag_name = element.value().name();
        let skip_tags = ["script", "style", "noscript", "svg", "form", "button", "input", "select", "textarea", "iframe"];
        if skip_tags.contains(&tag_name) {
            return;
        }

        // Check for class/id names that indicate non-content (but be less aggressive)
        if let Some(class) = element.value().attr("class") {
            let class_lower = class.to_lowercase();
            // Only skip if clearly navigation/ads
            let skip_classes = ["advertisement", "ad-container", "social-share", "comment-section"];
            if skip_classes.iter().any(|c| class_lower.contains(c)) {
                return;
            }
        }

        match tag_name {
            "h1" => {
                let text = self.clean_text(element);
                if !text.is_empty() && text.len() < 300 {
                    content.blocks.push(ContentBlock::new(ContentType::Title, &text));
                }
            }
            "h2" | "h3" | "h4" | "h5" | "h6" => {
                let text = self.clean_text(element);
                if !text.is_empty() && text.len() < 300 {
                    let level = tag_name.chars().last().unwrap().to_digit(10).unwrap() as u8;
                    content.blocks.push(ContentBlock::new(ContentType::Heading(level), &text));
                }
            }
            "p" => {
                let text = self.clean_text(element);
                // Accept paragraphs with at least 10 chars
                if text.len() >= 10 {
                    content.blocks.push(ContentBlock::new(ContentType::Paragraph, &text));
                }
            }
            "li" => {
                let text = self.clean_text(element);
                if !text.is_empty() && text.len() < 500 {
                    content.blocks.push(ContentBlock::new(ContentType::ListItem, &text).with_level(depth));
                }
            }
            "blockquote" => {
                let text = self.clean_text(element);
                if !text.is_empty() {
                    content.blocks.push(ContentBlock::new(ContentType::Quote, &text));
                }
            }
            "pre" | "code" => {
                if self.config.include_code {
                    let text = element.text().collect::<String>();
                    let text = text.trim();
                    if !text.is_empty() && text.len() <= 1000 {
                        content.blocks.push(ContentBlock::new(ContentType::Code, text));
                    }
                }
                return; // Don't recurse into code blocks
            }
            "img" => {
                if self.config.include_images {
                    if let Some(src) = element.value().attr("src") {
                        let alt = element.value().attr("alt").unwrap_or("").to_string();
                        if !src.starts_with("data:") && !alt.is_empty() {
                            content.images.push((src.to_string(), alt.clone()));
                            content.blocks.push(ContentBlock::new(
                                ContentType::Image { src: src.to_string(), alt },
                                ""
                            ));
                        }
                    }
                }
            }
            "table" => {
                if self.config.include_tables {
                    self.extract_table(element, content);
                }
                return; // Don't recurse into tables
            }
            "a" => {
                // Extract important links
                if self.config.extract_links {
                    if let Some(href) = element.value().attr("href") {
                        let text = self.clean_text(element);
                        if !text.is_empty() && text.len() > 5 && href.starts_with("http") {
                            // Only add standalone links, not inline ones
                            // This is handled by not recursing for links with substantial text
                        }
                    }
                }
            }
            _ => {}
        }

        // Always recurse into children (except for leaf elements we've already processed)
        let no_recurse_tags = ["p", "li", "pre", "code", "img", "table", "blockquote", "h1", "h2", "h3", "h4", "h5", "h6"];
        if !no_recurse_tags.contains(&tag_name) {
            for child in element.children() {
                if let Some(child_elem) = ElementRef::wrap(child) {
                    self.walk_element(&child_elem, content, depth + 1);
                }
            }
        }
    }

    /// Clean and normalize text
    fn clean_text(&self, element: &ElementRef) -> String {
        let text: String = element.text().collect();
        // Normalize whitespace
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        text.trim().to_string()
    }

    /// Extract table content
    fn extract_table(&self, element: &ElementRef, content: &mut WebContent) {
        let mut rows: Vec<Vec<String>> = Vec::new();

        if let Ok(row_selector) = Selector::parse("tr") {
            for row in element.select(&row_selector) {
                let mut cells: Vec<String> = Vec::new();

                if let Ok(cell_selector) = Selector::parse("th, td") {
                    for cell in row.select(&cell_selector) {
                        let text = self.clean_text(&cell);
                        cells.push(text);
                    }
                }

                if !cells.is_empty() {
                    rows.push(cells);
                }
            }
        }

        if !rows.is_empty() && rows.len() <= 30 {
            content.blocks.push(ContentBlock::new(
                ContentType::Table(rows),
                ""
            ));
        }
    }
}

impl Default for WebParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_html() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head><title>Test Page</title></head>
            <body>
                <h1>Main Title</h1>
                <p>This is a paragraph with enough text to be included.</p>
                <h2>Section 1</h2>
                <ul>
                    <li>Item 1</li>
                    <li>Item 2</li>
                </ul>
            </body>
            </html>
        "#;

        let parser = WebParser::new();
        let content = parser.parse(html, "https://example.com").unwrap();

        assert_eq!(content.title, "Test Page");
        assert!(!content.blocks.is_empty());
    }

    #[test]
    fn test_content_block() {
        let block = ContentBlock::new(ContentType::Heading(2), "Test Heading");
        assert!(block.is_heading());
        assert_eq!(block.heading_level(), Some(2));
    }

    #[test]
    fn test_grouped_by_headings() {
        let mut content = WebContent::new("https://example.com");
        content.blocks.push(ContentBlock::new(ContentType::Title, "Title"));
        content.blocks.push(ContentBlock::new(ContentType::Paragraph, "Intro text"));
        content.blocks.push(ContentBlock::new(ContentType::Heading(2), "Section 1"));
        content.blocks.push(ContentBlock::new(ContentType::Paragraph, "Section 1 text"));

        let groups = content.grouped_by_headings();
        assert_eq!(groups.len(), 2);
    }
}
