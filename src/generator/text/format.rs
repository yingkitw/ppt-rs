//! Text formatting options

/// Text formatting options
#[derive(Clone, Debug, Default)]
pub struct TextFormat {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub color: Option<String>,      // RGB hex color (e.g., "FF0000" for red)
    pub highlight: Option<String>,  // Highlight/background color
    pub font_size: Option<u32>,     // in points
    pub font_family: Option<String>, // Font family name (e.g., "Arial")
    pub subscript: bool,
    pub superscript: bool,
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
    
    /// Set strikethrough formatting
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    /// Set text color (RGB hex format)
    pub fn color(mut self, hex_color: &str) -> Self {
        self.color = Some(hex_color.trim_start_matches('#').to_uppercase());
        self
    }
    
    /// Set highlight/background color (RGB hex format)
    pub fn highlight(mut self, hex_color: &str) -> Self {
        self.highlight = Some(hex_color.trim_start_matches('#').to_uppercase());
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
    
    /// Set subscript formatting
    pub fn subscript(mut self) -> Self {
        self.subscript = true;
        self.superscript = false; // Can't be both
        self
    }
    
    /// Set superscript formatting
    pub fn superscript(mut self) -> Self {
        self.superscript = true;
        self.subscript = false; // Can't be both
        self
    }

    /// Generate XML attributes for text formatting
    pub fn to_xml_attrs(&self) -> String {
        let mut attrs = String::new();

        if self.bold {
            attrs.push_str(" b=\"1\"");
        }

        if self.italic {
            attrs.push_str(" i=\"1\"");
        }

        if self.underline {
            attrs.push_str(" u=\"sng\"");
        }
        
        if self.strikethrough {
            attrs.push_str(" strike=\"sngStrike\"");
        }
        
        if self.subscript {
            attrs.push_str(" baseline=\"-25000\""); // 25% below baseline
        } else if self.superscript {
            attrs.push_str(" baseline=\"30000\""); // 30% above baseline
        }

        if let Some(size) = self.font_size {
            attrs.push_str(&format!(" sz=\"{}\"", size * 100));
        }

        attrs
    }
    
    /// Generate highlight element if set
    pub fn to_highlight_xml(&self) -> String {
        if let Some(ref color) = self.highlight {
            format!(r#"<a:highlight><a:srgbClr val="{}"/></a:highlight>"#, color)
        } else {
            String::new()
        }
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
    
    /// Builder method for strikethrough
    pub fn strikethrough(mut self) -> Self {
        self.format = self.format.strikethrough();
        self
    }

    /// Builder method for color
    pub fn color(mut self, hex_color: &str) -> Self {
        self.format = self.format.color(hex_color);
        self
    }
    
    /// Builder method for highlight
    pub fn highlight(mut self, hex_color: &str) -> Self {
        self.format = self.format.highlight(hex_color);
        self
    }

    /// Builder method for font size
    pub fn font_size(mut self, size: u32) -> Self {
        self.format = self.format.font_size(size);
        self
    }
    
    /// Builder method for subscript
    pub fn subscript(mut self) -> Self {
        self.format = self.format.subscript();
        self
    }
    
    /// Builder method for superscript
    pub fn superscript(mut self) -> Self {
        self.format = self.format.superscript();
        self
    }
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
        let attrs = format.to_xml_attrs();
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
    fn test_strikethrough() {
        let format = TextFormat::new().strikethrough();
        let attrs = format.to_xml_attrs();
        assert!(attrs.contains("strike=\"sngStrike\""));
    }
    
    #[test]
    fn test_highlight() {
        let format = TextFormat::new().highlight("FFFF00");
        let xml = format.to_highlight_xml();
        assert!(xml.contains("highlight"));
        assert!(xml.contains("FFFF00"));
    }
    
    #[test]
    fn test_subscript_superscript() {
        let sub = TextFormat::new().subscript();
        let attrs = sub.to_xml_attrs();
        assert!(attrs.contains("baseline=\"-25000\""));
        
        let sup = TextFormat::new().superscript();
        let attrs = sup.to_xml_attrs();
        assert!(attrs.contains("baseline=\"30000\""));
    }
    
    #[test]
    fn test_formatted_text_strikethrough() {
        let text = FormattedText::new("Deleted")
            .strikethrough();
        assert!(text.format.strikethrough);
    }
    
    #[test]
    fn test_formatted_text_highlight() {
        let text = FormattedText::new("Important")
            .highlight("FFFF00");
        assert_eq!(text.format.highlight, Some("FFFF00".to_string()));
    }
    
    #[test]
    fn test_formatted_text_subscript_superscript() {
        let sub = FormattedText::new("2").subscript();
        assert!(sub.format.subscript);
        
        let sup = FormattedText::new("2").superscript();
        assert!(sup.format.superscript);
    }
}
