//! Text formatting options

/// Text formatting options
#[derive(Clone, Debug, Default)]
pub struct TextFormat {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub color: Option<String>,      // RGB hex color (e.g., "FF0000" for red)
    pub font_size: Option<u32>,     // in points
    pub font_family: Option<String>, // Font family name (e.g., "Arial")
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
        self.color = Some(hex_color.trim_start_matches('#').to_uppercase());
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

        if let Some(size) = self.font_size {
            attrs.push_str(&format!(" sz=\"{}\"", size * 100));
        }

        attrs
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
}
