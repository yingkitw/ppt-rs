//! HTML to PowerPoint conversion
//!
//! Parses HTML content and converts it into PowerPoint slide structures.
//! No external dependencies required - uses a lightweight state-machine parser.
//!
//! # This is the Basic HTML Parser
//!
//! This parser is designed for simple, fast HTML-to-PowerPoint conversion
//! with minimal dependencies. For advanced web scraping and content extraction,
//! see the `web2ppt::parser` module (requires scraper crate).
//!
//! ## When to use this parser:
//! - Converting simple HTML strings
//! - Processing well-structured HTML files
//! - When you want zero external dependencies
//! - For embedded applications
//!
//! ## When to use web2ppt::parser instead:
//! - Processing live web pages with navigation/ads
//! - When you need intelligent content extraction
//! - For automatic content cleaning and detection
//! - When you need to handle complex web page structures
//!
//! # Supported HTML elements
//!
//! - `<h1>` → New slide title
//! - `<h2>` through `<h6>` → Bold section headers on current slide
//! - `<p>` → Bullet points / paragraphs
//! - `<ul>/<ol>` with `<li>` → List items
//! - `<table>` with `<tr>/<th>/<td>` → Table objects (styled header row)
//! - `<pre>/<code>` → Code blocks
//! - `<img>` → Image placeholders (with alt text)
//! - `<blockquote>` → Speaker notes
//! - `<strong>/<b>` → Bold text (via markdown-style `**`)
//! - `<em>/<i>` → Italic text (via markdown-style `*`)
//! - `<a href="...">` → Hyperlink text
//! - `<hr>` → Slide break
//! - `<br>` → Line break within text
//! - `<title>` → Presentation title (falls back to first `<h1>`)

use crate::generator::{CodeBlock, SlideContent};
use crate::generator::slide_content::{BulletPoint, BulletStyle, BulletTextFormat};

/// Options for HTML parsing
#[derive(Clone, Debug)]
pub struct HtmlParseOptions {
    /// Maximum slides to generate
    pub max_slides: usize,
    /// Maximum bullet points per slide
    pub max_bullets: usize,
    /// Include code blocks
    pub include_code: bool,
    /// Include tables
    pub include_tables: bool,
    /// Include image placeholders
    pub include_images: bool,
}

impl Default for HtmlParseOptions {
    fn default() -> Self {
        Self {
            max_slides: 50,
            max_bullets: 10,
            include_code: true,
            include_tables: true,
            include_images: true,
        }
    }
}

impl HtmlParseOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn max_slides(mut self, n: usize) -> Self {
        self.max_slides = n;
        self
    }

    pub fn max_bullets(mut self, n: usize) -> Self {
        self.max_bullets = n;
        self
    }

    pub fn include_code(mut self, include: bool) -> Self {
        self.include_code = include;
        self
    }

    pub fn include_tables(mut self, include: bool) -> Self {
        self.include_tables = include;
        self
    }

    pub fn include_images(mut self, include: bool) -> Self {
        self.include_images = include;
        self
    }
}

/// Parse HTML content into slides with default options
pub fn parse_html(html: &str) -> Result<Vec<SlideContent>, String> {
    Html2Ppt::with_options(HtmlParseOptions::default()).parse(html)
}

/// Parse HTML content into slides with custom options
pub fn parse_html_with_options(html: &str, options: HtmlParseOptions) -> Result<Vec<SlideContent>, String> {
    Html2Ppt::with_options(options).parse(html)
}

// ---------------------------------------------------------------------------
// Struct-based HTML parser (no lifetimes trickery)
// ---------------------------------------------------------------------------

/// A simple tag-based HTML event
#[derive(Debug)]
enum HtmlEvent {
    OpenTag { name: String, attrs: Vec<(String, String)> },
    CloseTag(String),
    Text(String),
}

/// Decode common HTML entities
fn decode_entities(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'&' {
            if let Some(end) = s[i..].find(';') {
                let entity = &s[i + 1..i + end];
                let ch = match entity {
                    "amp" => Some('&'),
                    "lt" => Some('<'),
                    "gt" => Some('>'),
                    "quot" => Some('"'),
                    "apos" | "#39" | "#x27" => Some('\''),
                    "nbsp" => Some('\u{00a0}'),
                    "#x2018" => Some('\u{2018}'),
                    "#x2019" => Some('\u{2019}'),
                    "#x201c" => Some('\u{201c}'),
                    "#x201d" => Some('\u{201d}'),
                    "#x2014" => Some('\u{2014}'),
                    "#x2013" => Some('\u{2013}'),
                    _ => {
                        if let Some(hex) = entity.strip_prefix("#x") {
                            u32::from_str_radix(hex, 16).ok().and_then(char::from_u32)
                        } else if let Some(num) = entity.strip_prefix('#') {
                            num.parse::<u32>().ok().and_then(char::from_u32)
                        } else {
                            None
                        }
                    }
                };
                if let Some(c) = ch {
                    out.push(c);
                    i = i + end + 1;
                    continue;
                }
            }
        }
        // Preserve full Unicode characters (not just ASCII bytes)
        let c = s[i..].chars().next().unwrap();
        out.push(c);
        i += c.len_utf8();
    }
    out
}

// ---------------------------------------------------------------------------
// Inline CSS style parsing
// ---------------------------------------------------------------------------

/// Convert a named CSS color to its 6-digit hex representation
fn css_named_color(name: &str) -> Option<&'static str> {
    match name {
        "red" => Some("FF0000"),
        "blue" => Some("0000FF"),
        "green" => Some("008000"),
        "yellow" => Some("FFFF00"),
        "white" => Some("FFFFFF"),
        "black" => Some("000000"),
        "gray" | "grey" => Some("808080"),
        "silver" => Some("C0C0C0"),
        "maroon" => Some("800000"),
        "purple" => Some("800080"),
        "fuchsia" => Some("FF00FF"),
        "lime" => Some("00FF00"),
        "olive" => Some("808000"),
        "navy" => Some("000080"),
        "teal" => Some("008080"),
        "aqua" => Some("00FFFF"),
        "orange" => Some("FFA500"),
        "pink" => Some("FFC0CB"),
        "coral" => Some("FF7F50"),
        "tomato" => Some("FF6347"),
        "darkred" => Some("8B0000"),
        "darkblue" => Some("00008B"),
        "darkgreen" => Some("006400"),
        "darkgray" | "darkgrey" => Some("A9A9A9"),
        "lightgray" | "lightgrey" => Some("D3D3D3"),
        "darkorange" => Some("FF8C00"),
        "brown" => Some("A52A2A"),
        "crimson" => Some("DC143C"),
        "gold" => Some("FFD700"),
        "goldenrod" => Some("DAA520"),
        "indigo" => Some("4B0082"),
        "salmon" => Some("FA8072"),
        "chocolate" => Some("D2691E"),
        "steelblue" => Some("4682B4"),
        "violet" => Some("EE82EE"),
        "orchid" => Some("DA70D6"),
        "plum" => Some("DDA0DD"),
        "wheat" => Some("F5DEB3"),
        "deeppink" => Some("FF1493"),
        "hotpink" => Some("FF69B4"),
        "royalblue" => Some("4169E1"),
        "skyblue" => Some("87CEEB"),
        "seagreen" => Some("2E8B57"),
        "forestgreen" => Some("228B22"),
        _ => None,
    }
}

/// Parse a CSS color value to a 6-digit hex string (without #)
fn parse_css_color(value: &str) -> Option<String> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix('#') {
        let hex = match hex.len() {
            3 => hex.chars().map(|c| format!("{c}{c}")).collect::<String>(),
            6 => hex.to_string(),
            8 => hex[..6].to_string(), // ignore alpha
            _ => return None,
        };
        Some(hex.to_uppercase())
    } else if let Some(named) = css_named_color(value) {
        Some(named.to_string())
    } else if let Some(rgb) = value.strip_prefix("rgba(").or_else(|| value.strip_prefix("rgb(")) {
        if let Some(end) = rgb.rfind(')') {
            let parts: Vec<&str> = rgb[..end].split(',').collect();
            if parts.len() >= 3 {
                let r = parts[0].trim().parse::<u8>().ok()?;
                let g = parts[1].trim().parse::<u8>().ok()?;
                let b = parts[2].trim().parse::<u8>().ok()?;
                return Some(format!("{:02X}{:02X}{:02X}", r, g, b));
            }
        }
        None
    } else {
        None
    }
}

/// Parse a CSS font-size value to points
fn parse_font_size(value: &str) -> Option<u32> {
    let value = value.trim();
    if let Some(px) = value.strip_suffix("px") {
        let px = px.trim().parse::<f64>().ok()?;
        Some((px / 1.333).round() as u32)
    } else if let Some(pt) = value.strip_suffix("pt") {
        let pt = pt.trim().parse::<f64>().ok()?;
        Some(pt.round() as u32)
    } else {
        value.parse::<u32>().ok()
    }
}

/// Check if a CSS font-weight value represents bold
fn is_font_weight_bold(value: &str) -> bool {
    matches!(value.trim().to_lowercase().as_str(), "bold" | "bolder" | "700" | "800" | "900")
}

/// Check if a CSS font-style represents italic
fn is_font_style_italic(value: &str) -> bool {
    matches!(value.trim().to_lowercase().as_str(), "italic" | "oblique")
}

/// Parsed inline CSS style declarations
#[derive(Clone, Debug, Default)]
struct InlineStyle {
    color: Option<String>,
    background_color: Option<String>,
    font_size: Option<u32>,
    font_weight: Option<String>,
    font_style: Option<String>,
    text_decoration: Option<String>,
    font_family: Option<String>,
    text_align: Option<String>,
    margin_top: Option<String>,
    margin_bottom: Option<String>,
    margin_left: Option<String>,
    margin_right: Option<String>,
    padding: Option<String>,
    border: Option<String>,
    line_height: Option<String>,
    letter_spacing: Option<String>,
}

impl InlineStyle {
    fn parse(style_str: &str) -> Self {
        let mut style = InlineStyle::default();
        for decl in style_str.split(';') {
            let decl = decl.trim();
            if decl.is_empty() {
                continue;
            }
            if let Some(eq) = decl.find(':') {
                let prop = decl[..eq].trim().to_lowercase();
                let value = decl[eq + 1..].trim();
                match prop.as_str() {
                    "color" => style.color = parse_css_color(value),
                    "background-color" => style.background_color = parse_css_color(value),
                    "font-size" => style.font_size = parse_font_size(value),
                    "font-weight" => style.font_weight = Some(value.to_string()),
                    "font-style" => style.font_style = Some(value.to_string()),
                    "text-decoration" => style.text_decoration = Some(value.to_string()),
                    "font-family" => {
                        style.font_family = Some(value.trim_matches('"').trim_matches('\'').to_string());
                    }
                    "text-align" => style.text_align = Some(value.to_string()),
                    "margin-top" => style.margin_top = Some(value.to_string()),
                    "margin-bottom" => style.margin_bottom = Some(value.to_string()),
                    "margin-left" => style.margin_left = Some(value.to_string()),
                    "margin-right" => style.margin_right = Some(value.to_string()),
                    "padding" => style.padding = Some(value.to_string()),
                    "border" => style.border = Some(value.to_string()),
                    "line-height" => style.line_height = Some(value.to_string()),
                    "letter-spacing" => style.letter_spacing = Some(value.to_string()),
                    _ => {}
                }
            }
        }
        style
    }

    /// Merge another style on top, with the other's non-None values taking precedence
    fn merge(&self, other: &InlineStyle) -> InlineStyle {
        InlineStyle {
            color: other.color.clone().or_else(|| self.color.clone()),
            background_color: other.background_color.clone().or_else(|| self.background_color.clone()),
            font_size: other.font_size.or(self.font_size),
            font_weight: other.font_weight.clone().or_else(|| self.font_weight.clone()),
            font_style: other.font_style.clone().or_else(|| self.font_style.clone()),
            text_decoration: other.text_decoration.clone().or_else(|| self.text_decoration.clone()),
            font_family: other.font_family.clone().or_else(|| self.font_family.clone()),
            text_align: other.text_align.clone().or_else(|| self.text_align.clone()),
            margin_top: other.margin_top.clone().or_else(|| self.margin_top.clone()),
            margin_bottom: other.margin_bottom.clone().or_else(|| self.margin_bottom.clone()),
            margin_left: other.margin_left.clone().or_else(|| self.margin_left.clone()),
            margin_right: other.margin_right.clone().or_else(|| self.margin_right.clone()),
            padding: other.padding.clone().or_else(|| self.padding.clone()),
            border: other.border.clone().or_else(|| self.border.clone()),
            line_height: other.line_height.clone().or_else(|| self.line_height.clone()),
            letter_spacing: other.letter_spacing.clone().or_else(|| self.letter_spacing.clone()),
        }
    }

    /// Returns true if no style properties are set
    fn is_empty(&self) -> bool {
        self.color.is_none()
            && self.background_color.is_none()
            && self.font_size.is_none()
            && self.font_weight.is_none()
            && self.font_style.is_none()
            && self.text_decoration.is_none()
            && self.font_family.is_none()
            && self.text_align.is_none()
            && self.margin_top.is_none()
            && self.margin_bottom.is_none()
            && self.margin_left.is_none()
            && self.margin_right.is_none()
            && self.padding.is_none()
            && self.border.is_none()
            && self.line_height.is_none()
            && self.letter_spacing.is_none()
    }

    /// Convert to a BulletTextFormat for PPTX output. Returns None if no relevant properties set.
    fn to_bullet_format(&self) -> Option<BulletTextFormat> {
        if self.is_empty() {
            return None;
        }
        let mut fmt = BulletTextFormat::new();
        if let Some(ref c) = self.color {
            fmt = fmt.color(c);
        }
        if let Some(ref bg) = self.background_color {
            fmt = fmt.highlight(bg);
        }
        if let Some(sz) = self.font_size {
            fmt = fmt.font_size(sz);
        }
        if let Some(ref fw) = self.font_weight {
            if is_font_weight_bold(fw) {
                fmt = fmt.bold();
            }
        }
        if let Some(ref fs) = self.font_style {
            if is_font_style_italic(fs) {
                fmt = fmt.italic();
            }
        }
        if let Some(ref td) = self.text_decoration {
            if td.contains("underline") {
                fmt = fmt.underline();
            }
            if td.contains("line-through") {
                fmt = fmt.strikethrough();
            }
        }
        if let Some(ref ff) = self.font_family {
            fmt = fmt.font_family(ff);
        }
        Some(fmt)
    }
}

/// Tags that are void (self-closing) and should not push/pop the style stack
const VOID_TAGS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input",
    "link", "meta", "param", "source", "track", "wbr",
];

/// Walk through HTML and produce events
fn tokenize_html(html: &str) -> Vec<HtmlEvent> {
    let mut events = Vec::new();
    let chars: Vec<char> = html.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if chars[i] == '<' {
            i += 1;
            if i >= len {
                break;
            }

            // Comment <!-- ... -->
            if i + 3 <= len && chars[i] == '!' && i + 1 < len && chars[i + 1] == '-' && i + 2 < len && chars[i + 2] == '-' {
                // skip to -->
                i += 3;
                while i + 2 < len && !(chars[i] == '-' && chars[i + 1] == '-' && chars[i + 2] == '>') {
                    i += 1;
                }
                i += 3; // skip -->
                continue;
            }

            // Doctype/other <!...>
            if chars[i] == '!' {
                while i < len && chars[i] != '>' {
                    i += 1;
                }
                i += 1;
                continue;
            }

            // Closing tag </...>
            if chars[i] == '/' {
                i += 1;
                // skip whitespace
                while i < len && (chars[i] == ' ' || chars[i] == '\t' || chars[i] == '\n' || chars[i] == '\r') {
                    i += 1;
                }
                let mut name = String::new();
                while i < len && chars[i] != '>' {
                    if chars[i].is_alphanumeric() || chars[i] == '-' || chars[i] == ':' || chars[i] == '_' || chars[i] == '.' {
                        name.push(chars[i]);
                    }
                    i += 1;
                }
                if i < len {
                    i += 1; // skip '>'
                }
                if !name.is_empty() {
                    events.push(HtmlEvent::CloseTag(name.to_lowercase()));
                }
                continue;
            }

            // Opening or self-closing tag
            // Skip whitespace before tag name
            while i < len && (chars[i] == ' ' || chars[i] == '\t' || chars[i] == '\n' || chars[i] == '\r') {
                i += 1;
            }
            let mut name = String::new();
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '-' || chars[i] == ':' || chars[i] == '_' || chars[i] == '.') {
                name.push(chars[i]);
                i += 1;
            }
            let tag_name = name.to_lowercase();

            // Parse attributes
            let mut attrs: Vec<(String, String)> = Vec::new();
            let mut self_closing = false;

            while i < len && chars[i] != '>' {
                // Skip whitespace
                while i < len && (chars[i] == ' ' || chars[i] == '\t' || chars[i] == '\n' || chars[i] == '\r') {
                    i += 1;
                }
                if i >= len || chars[i] == '>' {
                    break;
                }
                if chars[i] == '/' {
                    self_closing = true;
                    i += 1;
                    continue;
                }

                // Read attribute name
                let mut attr_name = String::new();
                while i < len && chars[i] != '=' && chars[i] != '>' && chars[i] != ' ' && chars[i] != '\t' && chars[i] != '\n' && chars[i] != '\r' && chars[i] != '/' {
                    attr_name.push(chars[i]);
                    i += 1;
                }

                // Skip whitespace around =
                while i < len && (chars[i] == ' ' || chars[i] == '\t' || chars[i] == '\n' || chars[i] == '\r') {
                    i += 1;
                }

                let mut attr_value = String::new();
                if i < len && chars[i] == '=' {
                    i += 1;
                    // Skip whitespace after =
                    while i < len && (chars[i] == ' ' || chars[i] == '\t' || chars[i] == '\n' || chars[i] == '\r') {
                        i += 1;
                    }
                    if i < len && (chars[i] == '"' || chars[i] == '\'') {
                        let quote = chars[i];
                        i += 1;
                        while i < len && chars[i] != quote {
                            attr_value.push(chars[i]);
                            i += 1;
                        }
                        if i < len {
                            i += 1; // skip closing quote
                        }
                    } else {
                        // unquoted value
                        while i < len && chars[i] != '>' && chars[i] != ' ' && chars[i] != '\t' && chars[i] != '\n' && chars[i] != '\r' && chars[i] != '/' {
                            attr_value.push(chars[i]);
                            i += 1;
                        }
                    }
                }

                attrs.push((attr_name.to_lowercase(), decode_entities(&attr_value)));
            }

            if i < len {
                i += 1; // skip '>'
            }

            if !tag_name.is_empty() {
                let void_tags = [
                    "area", "base", "br", "col", "embed", "hr", "img", "input",
                    "link", "meta", "param", "source", "track", "wbr",
                ];
                let is_void = void_tags.contains(&tag_name.as_str());

                if self_closing || is_void {
                    events.push(HtmlEvent::OpenTag { name: tag_name, attrs });
                } else {
                    events.push(HtmlEvent::OpenTag { name: tag_name, attrs });
                }
            }
        } else {
            // Text content
            let mut text = String::new();
            while i < len && chars[i] != '<' {
                text.push(chars[i]);
                i += 1;
            }
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                events.push(HtmlEvent::Text(decode_entities(&text)));
            }
        }
    }

    events
}

// ---------------------------------------------------------------------------
// Slide builder from events
// ---------------------------------------------------------------------------

/// Tags whose content should be entirely skipped
const SKIP_TAGS: &[&str] = &[
    "script", "style", "noscript", "nav", "form", "svg", "canvas", "iframe",
    "title",
];

struct HtmlSlideParser {
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
    fn new(options: HtmlParseOptions) -> Self {
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

    /// Create a BulletPoint with the current active style applied
    #[allow(dead_code)]
    fn make_bullet(&self, text: &str, bullet_style: BulletStyle) -> BulletPoint {
        let mut bp = BulletPoint::new(text).with_style(bullet_style);
        if let Some(ref s) = self.active_style() {
            if let Some(fmt) = s.to_bullet_format() {
                bp = bp.with_format(fmt);
            }
        }
        bp
    }

    fn parse(&mut self, events: &[HtmlEvent]) -> Result<Vec<SlideContent>, String> {
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
        self.tag_stack.iter().any(|t| SKIP_TAGS.contains(&t.as_str()))
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
                    let alt = attrs.iter().find(|(k, _)| k == "alt").map(|(_, v)| v.as_str()).unwrap_or("");
                    let src = attrs.iter().find(|(k, _)| k == "src").map(|(_, v)| v.as_str()).unwrap_or("");

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
                if let Some(href) = attrs.iter().find(|(k, _)| k == "href").map(|(_, v)| v.as_str()) {
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
                let slide_title = if title.is_empty() { "Slide".to_string() } else { title };
                let mut slide = SlideContent::new(&slide_title);
                // Apply title-level styles from active style
                if let Some(ref s) = self.active_style() {
                    if let Some(ref c) = s.color { slide = slide.title_color(c); }
                    if let Some(sz) = s.font_size { slide = slide.title_size(sz); }
                    if let Some(ref fw) = s.font_weight { if is_font_weight_bold(fw) { slide = slide.title_bold(true); } }
                    if let Some(ref fs) = s.font_style { if is_font_style_italic(fs) { slide = slide.title_italic(true); } }
                    if let Some(ref td) = s.text_decoration { if td.contains("underline") { slide = slide.title_underline(true); } }
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
        } else if self.in_list {
            self.text_buffer.push_str(text);
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
            let title = self.presentation_title.clone().unwrap_or_else(|| "Overview".to_string());
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
            let title = self.presentation_title.clone().unwrap_or_else(|| "Key Points".to_string());
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

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// HTML to PowerPoint converter
pub struct Html2Ppt {
    options: HtmlParseOptions,
}

impl Html2Ppt {
    pub fn new() -> Self {
        Self::with_options(HtmlParseOptions::default())
    }

    pub fn with_options(options: HtmlParseOptions) -> Self {
        Self { options }
    }

    /// Parse an HTML string into slide content
    pub fn parse(&self, html: &str) -> Result<Vec<SlideContent>, String> {
        let events = tokenize_html(html);
        HtmlSlideParser::new(self.options.clone()).parse(&events)
    }

    /// Parse HTML from a file path
    pub fn parse_file(&self, path: &str) -> Result<Vec<SlideContent>, String> {
        let html = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read HTML file: {e}"))?;
        self.parse(&html)
    }
}

impl Default for Html2Ppt {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let events = tokenize_html("<h1>Hello</h1>");
        assert_eq!(events.len(), 3);
        match &events[0] {
            HtmlEvent::OpenTag { name, .. } => assert_eq!(name, "h1"),
            _ => panic!("expected OpenTag"),
        }
        match &events[1] {
            HtmlEvent::Text(t) => assert_eq!(t.trim(), "Hello"),
            _ => panic!("expected Text"),
        }
        match &events[2] {
            HtmlEvent::CloseTag(n) => assert_eq!(n, "h1"),
            _ => panic!("expected CloseTag"),
        }
    }

    #[test]
    fn test_simple_headings() {
        let html = "<h1>First Slide</h1><p>Some content</p><h1>Second Slide</h1>";
        let slides = parse_html(html).unwrap();
        assert_eq!(slides.len(), 2);
        assert_eq!(slides[0].title, "First Slide");
        assert_eq!(slides[1].title, "Second Slide");
        assert_eq!(slides[1].content.len(), 0);
    }

    #[test]
    fn test_table() {
        let html = r#"
            <html><body>
                <h1>Data</h1>
                <table>
                    <tr><th>Name</th><th>Value</th></tr>
                    <tr><td>A</td><td>1</td></tr>
                    <tr><td>B</td><td>2</td></tr>
                </table>
            </body></html>
        "#;
        let slides = parse_html(html).unwrap();
        assert!(slides[0].table.is_some());
    }

    #[test]
    fn test_code_block() {
        let html = r#"
            <html><body>
                <h1>Code Example</h1>
                <pre><code>fn main() { println!("hello"); }</code></pre>
            </body></html>
        "#;
        let slides = parse_html(html).unwrap();
        assert!(!slides[0].code_blocks.is_empty());
        assert!(slides[0].code_blocks[0].code.contains("fn main()"));
    }

    #[test]
    fn test_blockquote_notes() {
        let html = r#"
            <html><body>
                <h1>Slide</h1>
                <p>Content</p>
                <blockquote>Speaker note here</blockquote>
            </body></html>
        "#;
        let slides = parse_html(html).unwrap();
        assert_eq!(slides[0].notes, Some("Speaker note here".to_string()));
    }

    #[test]
    fn test_hr_slide_break() {
        let html = "<h1>Slide 1</h1><p>Content</p><hr><h1>Slide 2</h1><p>More content</p>";
        let slides = parse_html(html).unwrap();
        assert_eq!(slides.len(), 2);
    }

    #[test]
    fn test_entity_decoding() {
        let html = r#"
            <html><body>
                <h1>Test</h1>
                <p>AT&amp;T &lt;test&gt; &quot;quote&quot;</p>
            </body></html>
        "#;
        let slides = parse_html(html).unwrap();
        assert!(slides[0].content[0].contains("AT&T"));
        assert!(slides[0].content[0].contains("<test>"));
    }

    #[test]
    fn test_img_placeholder() {
        let html = r#"
            <html><body>
                <h1>Images</h1>
                <img src="photo.jpg" alt="A photo">
            </body></html>
        "#;
        let slides = parse_html(html).unwrap();
        assert!(slides[0].content.iter().any(|c| c.contains("[Image: A photo]")));
    }

    #[test]
    fn test_skip_script_style() {
        let html = r#"
            <html><body>
                <h1>Real Content</h1>
                <p>Visible text</p>
                <script>var x = "should not appear";</script>
                <style>.hidden { color: red; }</style>
            </body></html>
        "#;
        let slides = parse_html(html).unwrap();
        assert_eq!(slides.len(), 1);
        assert_eq!(slides[0].content.len(), 1);
        assert!(slides[0].content[0].contains("Visible"));
    }

    #[test]
    fn test_no_h1_fallback() {
        let html = r#"<html><body><p>Just a paragraph.</p></body></html>"#;
        let slides = parse_html(html).unwrap();
        assert_eq!(slides.len(), 1);
    }

    #[test]
    fn test_empty_input() {
        let result = parse_html("<html><body></body></html>");
        assert!(result.is_err());
    }

    #[test]
    fn test_br_tag() {
        let html = r#"<html><body><h1>Title</h1><p>Line 1<br>Line 2</p></body></html>"#;
        let slides = parse_html(html).unwrap();
        assert!(!slides[0].content.is_empty());
    }

    #[test]
    fn test_bold_italic() {
        let html = r#"
            <html><body>
                <h1>Formatting</h1>
                <p><strong>Bold</strong> and <em>italic</em> text</p>
            </body></html>
        "#;
        let slides = parse_html(html).unwrap();
        let c = &slides[0].content[0];
        assert!(c.contains("**Bold**"));
    }

    #[test]
    fn test_complex_nested() {
        let html = r#"
            <html><body>
                <h1>Welcome</h1>
                <p>Introduction paragraph.</p>
                <h2>Section A</h2>
                <ul>
                    <li>First item</li>
                    <li>Second item</li>
                </ul>
                <h1>Details</h1>
                <table><tr><th>Col1</th><th>Col2</th></tr>
                       <tr><td>A</td><td>B</td></tr></table>
                <pre><code>let x = 1;</code></pre>
            </body></html>
        "#;
        let slides = parse_html(html).unwrap();
        assert_eq!(slides.len(), 2);
        assert_eq!(slides[0].title, "Welcome");
        assert!(!slides[1].code_blocks.is_empty());
        assert!(slides[1].table.is_some());
    }

    #[test]
    fn test_html2ppt_options() {
        let options = HtmlParseOptions::new()
            .max_slides(3)
            .max_bullets(5)
            .include_images(false);
        assert_eq!(options.max_slides, 3);
        assert_eq!(options.max_bullets, 5);
        assert!(!options.include_images);
    }

    #[test]
    fn test_html2ppt_struct() {
        let converter = Html2Ppt::new();
        let html = "<h1>Test</h1><p>Content</p>";
        let slides = converter.parse(html).unwrap();
        assert_eq!(slides.len(), 1);
    }

    #[test]
    fn test_nested_elements() {
        let html = r#"
            <div><div><div><div><div>
                <h1>Deep Nesting</h1>
                <p>Still works</p>
            </div></div></div></div></div>
        "#;
        let slides = parse_html(html).unwrap();
        assert_eq!(slides[0].title, "Deep Nesting");
    }

    #[test]
    fn test_link_with_href() {
        let html = r#"
            <html><body>
                <h1>Links</h1>
                <p>Visit <a href="https://example.com">Example</a> website</p>
            </body></html>
        "#;
        let slides = parse_html(html).unwrap();
        assert!(slides[0].content[0].contains("Example"));
    }

    #[test]
    fn test_attrs_with_single_quotes() {
        let events = tokenize_html(r#"<img src='pic.jpg' alt='hello'>"#);
        assert_eq!(events.len(), 1);
        match &events[0] {
            HtmlEvent::OpenTag { name, attrs } => {
                assert_eq!(name, "img");
                assert_eq!(attrs.iter().find(|(k,_)| k == "src").map(|(_,v)| v.as_str()), Some("pic.jpg"));
                assert_eq!(attrs.iter().find(|(k,_)| k == "alt").map(|(_,v)| v.as_str()), Some("hello"));
            }
            _ => panic!("expected OpenTag"),
        }
    }

    #[test]
    fn test_tokenizer_complex() {
        let events = tokenize_html(r#"<div class="main"><h1 id="title">Hello</h1></div>"#);
        assert_eq!(events.len(), 5);
        match &events[0] {
            HtmlEvent::OpenTag { name, attrs } => {
                assert_eq!(name, "div");
                assert_eq!(attrs[0].0, "class");
                assert_eq!(attrs[0].1, "main");
            }
            _ => panic!("expected OpenTag div"),
        }
    }

    #[test]
    fn test_self_closing_void_tags() {
        let events = tokenize_html(r#"<br><hr><img src="x.jpg">"#);
        assert_eq!(events.len(), 3);
        for event in &events {
            match event {
                HtmlEvent::OpenTag { name, .. } => {
                    assert!(["br", "hr", "img"].contains(&name.as_str()));
                }
                _ => panic!("expected OpenTag for void elements"),
            }
        }
    }

    #[test]
    fn test_comments_skipped() {
        let events = tokenize_html(r#"<h1>A</h1><!-- comment --><p>B</p>"#);
        // Events: OpenTag(h1), Text(A), CloseTag(h1), OpenTag(p), Text(B), CloseTag(p)
        assert_eq!(events.len(), 6);
        match &events[3] {
            HtmlEvent::OpenTag { name, .. } => assert_eq!(name, "p"),
            _ => panic!("expected p"),
        }
    }

    #[test]
    fn test_doctype_skipped() {
        let events = tokenize_html("<!DOCTYPE html><h1>Title</h1>");
        assert_eq!(events.len(), 3);
        match &events[0] {
            HtmlEvent::OpenTag { name, .. } => assert_eq!(name, "h1"),
            _ => panic!("expected h1"),
        }
    }

    #[test]
    fn test_multiple_attributes() {
        let events = tokenize_html(r#"<a href="https://x.com" class="link" id="main">text</a>"#);
        assert_eq!(events.len(), 3);
        match &events[0] {
            HtmlEvent::OpenTag { name, attrs } => {
                assert_eq!(name, "a");
                assert_eq!(attrs.len(), 3);
            }
            _ => panic!("expected OpenTag"),
        }
    }

    // ========================================================================
    // CSS Style Parsing Tests
    // ========================================================================

    #[test]
    fn test_parse_css_color_hex() {
        assert_eq!(parse_css_color("#ff0000"), Some("FF0000".to_string()));
        assert_eq!(parse_css_color("#FF0000"), Some("FF0000".to_string()));
        assert_eq!(parse_css_color("#f00"), Some("FF0000".to_string()));
        assert_eq!(parse_css_color("#abc"), Some("AABBCC".to_string()));
    }

    #[test]
    fn test_parse_css_color_named() {
        assert_eq!(parse_css_color("red"), Some("FF0000".to_string()));
        assert_eq!(parse_css_color("blue"), Some("0000FF".to_string()));
        assert_eq!(parse_css_color("green"), Some("008000".to_string()));
        assert_eq!(parse_css_color("white"), Some("FFFFFF".to_string()));
        assert_eq!(parse_css_color("black"), Some("000000".to_string()));
    }

    #[test]
    fn test_parse_css_color_rgb() {
        assert_eq!(parse_css_color("rgb(255,0,0)"), Some("FF0000".to_string()));
        assert_eq!(parse_css_color("rgb(0, 128, 0)"), Some("008000".to_string()));
        assert_eq!(parse_css_color("rgba(0, 0, 255, 0.5)"), Some("0000FF".to_string()));
    }

    #[test]
    fn test_parse_css_color_invalid() {
        assert_eq!(parse_css_color("notacolor"), None);
        assert_eq!(parse_css_color("transparent"), None);
        assert_eq!(parse_css_color("#ggggg"), None);
    }

    #[test]
    fn test_parse_font_size() {
        assert_eq!(parse_font_size("20px"), Some(15)); // 20/1.333 ≈ 15
        assert_eq!(parse_font_size("16px"), Some(12));
        assert_eq!(parse_font_size("18pt"), Some(18));
        assert_eq!(parse_font_size("12pt"), Some(12));
        assert_eq!(parse_font_size("44"), Some(44));
    }

    #[test]
    fn test_is_font_weight_bold() {
        assert!(is_font_weight_bold("bold"));
        assert!(is_font_weight_bold("700"));
        assert!(is_font_weight_bold("800"));
        assert!(is_font_weight_bold("900"));
        assert!(is_font_weight_bold("bolder"));
        assert!(!is_font_weight_bold("normal"));
        assert!(!is_font_weight_bold("400"));
        assert!(!is_font_weight_bold("100"));
    }

    #[test]
    fn test_is_font_style_italic() {
        assert!(is_font_style_italic("italic"));
        assert!(is_font_style_italic("oblique"));
        assert!(!is_font_style_italic("normal"));
    }

    #[test]
    fn test_inline_style_parse_single() {
        let s = InlineStyle::parse("color: red");
        assert_eq!(s.color, Some("FF0000".to_string()));
        assert_eq!(s.background_color, None);
    }

    #[test]
    fn test_inline_style_parse_multiple() {
        let s = InlineStyle::parse("color: #0000FF; font-size: 20px; font-weight: bold");
        assert_eq!(s.color, Some("0000FF".to_string()));
        assert_eq!(s.font_size, Some(15));
        assert_eq!(s.font_weight, Some("bold".to_string()));
    }

    #[test]
    fn test_inline_style_parse_background() {
        let s = InlineStyle::parse("background-color: yellow");
        assert_eq!(s.background_color, Some("FFFF00".to_string()));
    }

    #[test]
    fn test_inline_style_parse_text_decoration() {
        let s = InlineStyle::parse("text-decoration: underline");
        assert_eq!(s.text_decoration, Some("underline".to_string()));
        let s = InlineStyle::parse("text-decoration: line-through");
        assert_eq!(s.text_decoration, Some("line-through".to_string()));
    }

    #[test]
    fn test_inline_style_parse_font_family() {
        let s = InlineStyle::parse("font-family: Arial");
        assert_eq!(s.font_family, Some("Arial".to_string()));
        let s = InlineStyle::parse("font-family: 'Times New Roman'");
        assert_eq!(s.font_family, Some("Times New Roman".to_string()));
    }

    #[test]
    fn test_inline_style_merge_child_overrides() {
        let parent = InlineStyle {
            color: Some("FF0000".to_string()),
            font_size: Some(20),
            ..Default::default()
        };
        let child = InlineStyle {
            color: Some("0000FF".to_string()),
            ..Default::default()
        };
        let merged = parent.merge(&child);
        assert_eq!(merged.color, Some("0000FF".to_string())); // child overrides
        assert_eq!(merged.font_size, Some(20)); // parent preserved
    }

    #[test]
    fn test_inline_style_merge_empty_child() {
        let parent = InlineStyle {
            color: Some("FF0000".to_string()),
            ..Default::default()
        };
        let child = InlineStyle::default();
        let merged = parent.merge(&child);
        assert_eq!(merged.color, Some("FF0000".to_string())); // parent preserved
    }

    #[test]
    fn test_inline_style_merge_no_parent() {
        let parent = InlineStyle::default();
        let child = InlineStyle::parse("color: red; font-size: 18pt");
        let merged = parent.merge(&child);
        assert_eq!(merged.color, Some("FF0000".to_string()));
        assert_eq!(merged.font_size, Some(18));
    }

    // ========================================================================
    // Style Propagation Tests (from HTML attributes to BulletFormat)
    // ========================================================================

    #[test]
    fn test_paragraph_inline_color() {
        let html = r#"<h1>Test</h1><p style="color:red">Red text</p>"#;
        let slides = parse_html(html).unwrap();
        assert_eq!(slides[0].bullets.len(), 1);
        let fmt = slides[0].bullets[0].format.as_ref().expect("Should have format");
        assert_eq!(fmt.color, Some("FF0000".to_string()));
    }

    #[test]
    fn test_paragraph_inline_font_size() {
        let html = r#"<h1>Test</h1><p style="font-size:20px">Bigger text</p>"#;
        let slides = parse_html(html).unwrap();
        let fmt = slides[0].bullets[0].format.as_ref().expect("Should have format");
        assert_eq!(fmt.font_size, Some(15));
    }

    #[test]
    fn test_paragraph_inline_bold() {
        let html = r#"<h1>Test</h1><p style="font-weight:bold">Bold paragraph</p>"#;
        let slides = parse_html(html).unwrap();
        let fmt = slides[0].bullets[0].format.as_ref().expect("Should have format");
        assert!(fmt.bold);
    }

    #[test]
    fn test_paragraph_inline_italic() {
        let html = r#"<h1>Test</h1><p style="font-style:italic">Italic paragraph</p>"#;
        let slides = parse_html(html).unwrap();
        let fmt = slides[0].bullets[0].format.as_ref().expect("Should have format");
        assert!(fmt.italic);
    }

    #[test]
    fn test_paragraph_inline_underline() {
        let html = r#"<h1>Test</h1><p style="text-decoration:underline">Underlined</p>"#;
        let slides = parse_html(html).unwrap();
        let fmt = slides[0].bullets[0].format.as_ref().expect("Should have format");
        assert!(fmt.underline);
    }

    #[test]
    fn test_paragraph_inline_multiple_styles() {
        let html = r#"<h1>Test</h1><p style="color:blue; font-size:18pt; font-weight:bold">Styled</p>"#;
        let slides = parse_html(html).unwrap();
        let fmt = slides[0].bullets[0].format.as_ref().expect("Should have format");
        assert_eq!(fmt.color, Some("0000FF".to_string()));
        assert_eq!(fmt.font_size, Some(18));
        assert!(fmt.bold);
    }

    #[test]
    fn test_paragraph_no_style_no_format() {
        let html = "<h1>Test</h1><p>Plain text</p>";
        let slides = parse_html(html).unwrap();
        assert!(slides[0].bullets[0].format.is_none());
    }

    #[test]
    fn test_h1_inline_color() {
        let html = r#"<h1 style="color:green">Green Title</h1>"#;
        let slides = parse_html(html).unwrap();
        assert_eq!(slides[0].title_color, Some("008000".to_string()));
    }

    #[test]
    fn test_h1_inline_font_size() {
        let html = r#"<h1 style="font-size:36pt">Big Title</h1>"#;
        let slides = parse_html(html).unwrap();
        assert_eq!(slides[0].title_size, Some(36));
    }

    #[test]
    fn test_h1_inline_bold_true() {
        let html = r#"<h1 style="font-weight:bold">Bold Title</h1>"#;
        let slides = parse_html(html).unwrap();
        assert!(slides[0].title_bold); // True because css bold=True; default is also true
    }

    #[test]
    fn test_h1_inline_italic() {
        let html = r#"<h1 style="font-style:italic">Italic Title</h1>"#;
        let slides = parse_html(html).unwrap();
        assert!(slides[0].title_italic);
    }

    #[test]
    fn test_h1_underline_from_style() {
        let html = r#"<h1 style="text-decoration:underline">Underlined Title</h1>"#;
        let slides = parse_html(html).unwrap();
        assert!(slides[0].title_underline);
    }

    #[test]
    fn test_list_item_with_inline_style() {
        let html = r#"<h1>List</h1><ul><li style="color:red">Red item</li><li>Normal item</li></ul>"#;
        let slides = parse_html(html).unwrap();
        let fmt0 = slides[0].bullets[0].format.as_ref().expect("First item should have format");
        assert_eq!(fmt0.color, Some("FF0000".to_string()));
        assert!(slides[0].bullets[1].format.is_none()); // second item has no style
    }

    #[test]
    fn test_nested_style_inheritance() {
        let html = r#"<div style="color:red"><p>Red text</p><p style="color:blue">Blue text</p></div>"#;
        let slides = parse_html(html).unwrap();
        // Both paragraphs ended up as bullets on the same slide (if first h1 existed or auto-title)
        assert_eq!(slides[0].bullets.len(), 2);
        let fmt0 = slides[0].bullets[0].format.as_ref().expect("First should have format");
        assert_eq!(fmt0.color, Some("FF0000".to_string())); // inherits red from div
        let fmt1 = slides[0].bullets[1].format.as_ref().expect("Second should have format");
        assert_eq!(fmt1.color, Some("0000FF".to_string())); // overrides to blue
    }

    #[test]
    fn test_style_on_container_div() {
        let html = r#"<h1>Styled Container</h1><div style="color:purple"><p>Purple paragraph</p></div>"#;
        let slides = parse_html(html).unwrap();
        let fmt = slides[0].bullets[0].format.as_ref().expect("Should have format");
        assert_eq!(fmt.color, Some("800080".to_string()));
    }

    #[test]
    fn test_void_tag_br_does_not_affect_style() {
        let html = r#"<h1>Test</h1><p style="color:red">First<br style="color:blue">Second</p>"#;
        let slides = parse_html(html).unwrap();
        // The <br> should not push/pop style, so the paragraph should have color:red
        let fmt = slides[0].bullets[0].format.as_ref().expect("Should have format");
        assert_eq!(fmt.color, Some("FF0000".to_string()));
    }

    #[test]
    fn test_style_content_size_default() {
        let html = "<h1>Test</h1><p>Default size</p>";
        let slides = parse_html(html).unwrap();
        assert_eq!(slides[0].content_size, Some(28));
    }

    #[test]
    fn test_background_color_as_highlight() {
        let html = r#"<h1>Test</h1><p style="background-color:yellow">Highlighted</p>"#;
        let slides = parse_html(html).unwrap();
        let fmt = slides[0].bullets[0].format.as_ref().expect("Should have format");
        assert_eq!(fmt.highlight, Some("FFFF00".to_string()));
    }
}
