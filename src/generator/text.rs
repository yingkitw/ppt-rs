//! Text formatting support for PPTX generation
//!
//! Provides structures for rich text formatting in PPTX:
//! - `TextFrame` - Container for text content
//! - `Paragraph` - A paragraph with alignment and spacing
//! - `Run` - A run of text with consistent formatting
//! - `TextFormat` - Formatting options (bold, italic, etc.)

/// Text alignment options
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

impl TextAlign {
    /// Get the OOXML alignment value
    pub fn to_xml(&self) -> &'static str {
        match self {
            TextAlign::Left => "l",
            TextAlign::Center => "ctr",
            TextAlign::Right => "r",
            TextAlign::Justify => "just",
        }
    }
}

/// Vertical text anchor
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum TextAnchor {
    #[default]
    Top,
    Middle,
    Bottom,
}

impl TextAnchor {
    /// Get the OOXML anchor value
    pub fn to_xml(&self) -> &'static str {
        match self {
            TextAnchor::Top => "t",
            TextAnchor::Middle => "ctr",
            TextAnchor::Bottom => "b",
        }
    }
}

/// A run of text with consistent formatting
#[derive(Clone, Debug)]
pub struct Run {
    pub text: String,
    pub format: TextFormat,
}

impl Run {
    /// Create a new text run
    pub fn new(text: &str) -> Self {
        Run {
            text: text.to_string(),
            format: TextFormat::default(),
        }
    }

    /// Apply formatting
    pub fn with_format(mut self, format: TextFormat) -> Self {
        self.format = format;
        self
    }

    /// Set bold
    pub fn bold(mut self) -> Self {
        self.format.bold = true;
        self
    }

    /// Set italic
    pub fn italic(mut self) -> Self {
        self.format.italic = true;
        self
    }

    /// Set underline
    pub fn underline(mut self) -> Self {
        self.format.underline = true;
        self
    }

    /// Set color
    pub fn color(mut self, hex: &str) -> Self {
        self.format.color = Some(hex.trim_start_matches('#').to_uppercase());
        self
    }

    /// Set font size
    pub fn size(mut self, points: u32) -> Self {
        self.format.font_size = Some(points);
        self
    }

    /// Set font family
    pub fn font(mut self, family: &str) -> Self {
        self.format.font_family = Some(family.to_string());
        self
    }

    /// Generate XML for this run
    pub fn to_xml(&self) -> String {
        let size = self.format.font_size.unwrap_or(18) * 100;
        let bold = if self.format.bold { "1" } else { "0" };
        let italic = if self.format.italic { "1" } else { "0" };
        let underline = if self.format.underline { " u=\"sng\"" } else { "" };
        
        let color_xml = self.format.color.as_ref()
            .map(|c| format!(r#"<a:solidFill><a:srgbClr val="{}"/></a:solidFill>"#, c))
            .unwrap_or_default();
        
        let font_xml = self.format.font_family.as_ref()
            .map(|f| format!(r#"<a:latin typeface="{}"/>"#, escape_xml(f)))
            .unwrap_or_default();

        format!(
            r#"<a:r><a:rPr lang="en-US" sz="{}" b="{}" i="{}"{} dirty="0">{}{}</a:rPr><a:t>{}</a:t></a:r>"#,
            size, bold, italic, underline, color_xml, font_xml, escape_xml(&self.text)
        )
    }
}

/// A paragraph containing one or more runs
#[derive(Clone, Debug)]
pub struct Paragraph {
    pub runs: Vec<Run>,
    pub align: TextAlign,
    pub level: u32,
    pub bullet: bool,
    pub spacing_before: Option<u32>,
    pub spacing_after: Option<u32>,
    pub line_spacing: Option<u32>,
}

impl Paragraph {
    /// Create a new empty paragraph
    pub fn new() -> Self {
        Paragraph {
            runs: Vec::new(),
            align: TextAlign::Left,
            level: 0,
            bullet: false,
            spacing_before: None,
            spacing_after: None,
            line_spacing: None,
        }
    }

    /// Create a paragraph with text
    pub fn with_text(text: &str) -> Self {
        let mut p = Self::new();
        p.runs.push(Run::new(text));
        p
    }

    /// Add a run
    pub fn add_run(mut self, run: Run) -> Self {
        self.runs.push(run);
        self
    }

    /// Add plain text
    pub fn add_text(mut self, text: &str) -> Self {
        self.runs.push(Run::new(text));
        self
    }

    /// Set alignment
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Set as bullet point
    pub fn bullet(mut self) -> Self {
        self.bullet = true;
        self
    }

    /// Set indent level (0-8)
    pub fn level(mut self, level: u32) -> Self {
        self.level = level.min(8);
        self
    }

    /// Set spacing before (in points * 100)
    pub fn spacing_before(mut self, points: u32) -> Self {
        self.spacing_before = Some(points * 100);
        self
    }

    /// Set spacing after (in points * 100)
    pub fn spacing_after(mut self, points: u32) -> Self {
        self.spacing_after = Some(points * 100);
        self
    }

    /// Generate XML for this paragraph
    pub fn to_xml(&self) -> String {
        let mut xml = String::from("<a:p>");
        
        // Paragraph properties
        let mut ppr = format!(r#"<a:pPr algn="{}" lvl="{}""#, self.align.to_xml(), self.level);
        
        if self.spacing_before.is_some() || self.spacing_after.is_some() || self.line_spacing.is_some() {
            ppr.push('>');
            if let Some(before) = self.spacing_before {
                ppr.push_str(&format!(r#"<a:spcBef><a:spcPts val="{}"/></a:spcBef>"#, before));
            }
            if let Some(after) = self.spacing_after {
                ppr.push_str(&format!(r#"<a:spcAft><a:spcPts val="{}"/></a:spcAft>"#, after));
            }
            if self.bullet {
                ppr.push_str("<a:buChar char=\"•\"/>");
            }
            ppr.push_str("</a:pPr>");
        } else if self.bullet {
            ppr.push_str("><a:buChar char=\"•\"/></a:pPr>");
        } else {
            ppr.push_str("/>");
        }
        
        xml.push_str(&ppr);
        
        // Runs
        for run in &self.runs {
            xml.push_str(&run.to_xml());
        }
        
        xml.push_str("</a:p>");
        xml
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Self::new()
    }
}

/// A text frame containing paragraphs
#[derive(Clone, Debug)]
pub struct TextFrame {
    pub paragraphs: Vec<Paragraph>,
    pub anchor: TextAnchor,
    pub wrap: bool,
    pub margin_left: u32,
    pub margin_right: u32,
    pub margin_top: u32,
    pub margin_bottom: u32,
}

impl TextFrame {
    /// Create a new empty text frame
    pub fn new() -> Self {
        TextFrame {
            paragraphs: Vec::new(),
            anchor: TextAnchor::Top,
            wrap: true,
            margin_left: 91440,   // 0.1 inch
            margin_right: 91440,
            margin_top: 45720,    // 0.05 inch
            margin_bottom: 45720,
        }
    }

    /// Create with a single paragraph
    pub fn with_text(text: &str) -> Self {
        let mut tf = Self::new();
        tf.paragraphs.push(Paragraph::with_text(text));
        tf
    }

    /// Add a paragraph
    pub fn add_paragraph(mut self, para: Paragraph) -> Self {
        self.paragraphs.push(para);
        self
    }

    /// Add plain text as a paragraph
    pub fn add_text(mut self, text: &str) -> Self {
        self.paragraphs.push(Paragraph::with_text(text));
        self
    }

    /// Set vertical anchor
    pub fn anchor(mut self, anchor: TextAnchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Set margins (in EMU)
    pub fn margins(mut self, left: u32, right: u32, top: u32, bottom: u32) -> Self {
        self.margin_left = left;
        self.margin_right = right;
        self.margin_top = top;
        self.margin_bottom = bottom;
        self
    }

    /// Generate XML for this text frame
    pub fn to_xml(&self) -> String {
        let wrap = if self.wrap { "square" } else { "none" };
        
        let mut xml = format!(
            r#"<p:txBody><a:bodyPr wrap="{}" lIns="{}" rIns="{}" tIns="{}" bIns="{}" anchor="{}"/><a:lstStyle/>"#,
            wrap, self.margin_left, self.margin_right, self.margin_top, self.margin_bottom, self.anchor.to_xml()
        );
        
        for para in &self.paragraphs {
            xml.push_str(&para.to_xml());
        }
        
        // Add empty paragraph if none
        if self.paragraphs.is_empty() {
            xml.push_str("<a:p/>");
        }
        
        xml.push_str("</p:txBody>");
        xml
    }
}

impl Default for TextFrame {
    fn default() -> Self {
        Self::new()
    }
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Text formatting options
#[derive(Clone, Debug)]
pub struct TextFormat {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub color: Option<String>,      // RGB hex color (e.g., "FF0000" for red)
    pub font_size: Option<u32>,     // in points
    pub font_family: Option<String>, // Font family name (e.g., "Arial")
}

impl Default for TextFormat {
    fn default() -> Self {
        TextFormat {
            bold: false,
            italic: false,
            underline: false,
            color: None,
            font_size: None,
            font_family: None,
        }
    }
}

impl TextFormat {
    /// Create a new text format with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set bold formatting
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set italic formatting
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Set underline formatting
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Set text color (RGB hex format)
    pub fn color(mut self, hex_color: &str) -> Self {
        self.color = Some(hex_color.to_uppercase());
        self
    }

    /// Set font size in points
    pub fn font_size(mut self, size: u32) -> Self {
        self.font_size = Some(size);
        self
    }

    /// Set font family
    pub fn font_family(mut self, family: &str) -> Self {
        self.font_family = Some(family.to_string());
        self
    }
}

/// Formatted text with styling
#[derive(Clone, Debug)]
pub struct FormattedText {
    pub text: String,
    pub format: TextFormat,
}

impl FormattedText {
    /// Create new formatted text
    pub fn new(text: &str) -> Self {
        FormattedText {
            text: text.to_string(),
            format: TextFormat::default(),
        }
    }

    /// Apply formatting
    pub fn with_format(mut self, format: TextFormat) -> Self {
        self.format = format;
        self
    }

    /// Builder method for bold
    pub fn bold(mut self) -> Self {
        self.format = self.format.bold();
        self
    }

    /// Builder method for italic
    pub fn italic(mut self) -> Self {
        self.format = self.format.italic();
        self
    }

    /// Builder method for underline
    pub fn underline(mut self) -> Self {
        self.format = self.format.underline();
        self
    }

    /// Builder method for color
    pub fn color(mut self, hex_color: &str) -> Self {
        self.format = self.format.color(hex_color);
        self
    }

    /// Builder method for font size
    pub fn font_size(mut self, size: u32) -> Self {
        self.format = self.format.font_size(size);
        self
    }
}

/// Generate XML attributes for text formatting
pub fn format_to_xml_attrs(format: &TextFormat) -> String {
    let mut attrs = String::new();

    if format.bold {
        attrs.push_str(" b=\"1\"");
    }

    if format.italic {
        attrs.push_str(" i=\"1\"");
    }

    if format.underline {
        attrs.push_str(" u=\"sng\"");
    }

    if let Some(size) = format.font_size {
        attrs.push_str(&format!(" sz=\"{}\"", size * 100)); // Convert points to hundredths
    }

    attrs
}

/// Generate XML color element
pub fn color_to_xml(hex_color: &str) -> String {
    let clean_color = hex_color.trim_start_matches('#').to_uppercase();
    format!("<a:solidFill><a:srgbClr val=\"{}\"/></a:solidFill>", clean_color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_format_builder() {
        let format = TextFormat::new()
            .bold()
            .italic()
            .color("FF0000")
            .font_size(24);

        assert!(format.bold);
        assert!(format.italic);
        assert_eq!(format.color, Some("FF0000".to_string()));
        assert_eq!(format.font_size, Some(24));
    }

    #[test]
    fn test_formatted_text_builder() {
        let text = FormattedText::new("Hello")
            .bold()
            .italic()
            .color("0000FF");

        assert_eq!(text.text, "Hello");
        assert!(text.format.bold);
        assert!(text.format.italic);
        assert_eq!(text.format.color, Some("0000FF".to_string()));
    }

    #[test]
    fn test_format_to_xml_attrs() {
        let format = TextFormat::new().bold().italic().font_size(24);
        let attrs = format_to_xml_attrs(&format);
        assert!(attrs.contains("b=\"1\""));
        assert!(attrs.contains("i=\"1\""));
        assert!(attrs.contains("sz=\"2400\""));
    }

    #[test]
    fn test_color_to_xml() {
        let xml = color_to_xml("FF0000");
        assert!(xml.contains("FF0000"));
        assert!(xml.contains("srgbClr"));
    }

    #[test]
    fn test_run_to_xml() {
        let run = Run::new("Hello").bold().color("FF0000").size(24);
        let xml = run.to_xml();
        
        assert!(xml.contains("Hello"));
        assert!(xml.contains("b=\"1\""));
        assert!(xml.contains("FF0000"));
        assert!(xml.contains("sz=\"2400\""));
    }

    #[test]
    fn test_paragraph_to_xml() {
        let para = Paragraph::new()
            .add_run(Run::new("Bold text").bold())
            .add_run(Run::new(" normal text"))
            .align(TextAlign::Center);
        
        let xml = para.to_xml();
        
        assert!(xml.contains("<a:p>"));
        assert!(xml.contains("algn=\"ctr\""));
        assert!(xml.contains("Bold text"));
        assert!(xml.contains("normal text"));
    }

    #[test]
    fn test_paragraph_with_bullet() {
        let para = Paragraph::with_text("Bullet item").bullet();
        let xml = para.to_xml();
        
        assert!(xml.contains("buChar"));
    }

    #[test]
    fn test_text_frame_to_xml() {
        let tf = TextFrame::new()
            .add_paragraph(Paragraph::with_text("Title").align(TextAlign::Center))
            .add_paragraph(Paragraph::with_text("Content"))
            .anchor(TextAnchor::Middle);
        
        let xml = tf.to_xml();
        
        assert!(xml.contains("<p:txBody>"));
        assert!(xml.contains("anchor=\"ctr\""));
        assert!(xml.contains("Title"));
        assert!(xml.contains("Content"));
    }

    #[test]
    fn test_text_align() {
        assert_eq!(TextAlign::Left.to_xml(), "l");
        assert_eq!(TextAlign::Center.to_xml(), "ctr");
        assert_eq!(TextAlign::Right.to_xml(), "r");
        assert_eq!(TextAlign::Justify.to_xml(), "just");
    }

    #[test]
    fn test_text_anchor() {
        assert_eq!(TextAnchor::Top.to_xml(), "t");
        assert_eq!(TextAnchor::Middle.to_xml(), "ctr");
        assert_eq!(TextAnchor::Bottom.to_xml(), "b");
    }

    #[test]
    fn test_font_family() {
        let run = Run::new("Arial text").font("Arial");
        let xml = run.to_xml();
        
        assert!(xml.contains("typeface=\"Arial\""));
    }
}
