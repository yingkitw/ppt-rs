//! Font functionality

/// Underline style for text
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnderlineStyle {
    None,
    Single,
    Double,
    Wavy,
    DottedSingle,
    DottedDouble,
    DashedSingle,
    DashedDouble,
    DashDotSingle,
    DashDotDouble,
}

/// Font properties
pub struct Font {
    name: String,
    size: u32,      // in points
    bold: bool,
    italic: bool,
    underline: UnderlineStyle,
    color: Option<String>, // RGB color as hex string
    character_spacing: Option<i32>, // in EMU (English Metric Units)
    transparency: Option<u32>, // 0-100 percentage
    subscript: bool,
    superscript: bool,
    strikethrough: bool,
}

impl Font {
    /// Create a new font with default properties
    pub fn new() -> Self {
        Self {
            name: "Calibri".to_string(),
            size: 18,
            bold: false,
            italic: false,
            underline: UnderlineStyle::None,
            color: None,
            character_spacing: None,
            transparency: None,
            subscript: false,
            superscript: false,
            strikethrough: false,
        }
    }

    /// Create a new font with name and size
    pub fn with_name_size(name: String, size: u32) -> Self {
        Self {
            name,
            size,
            bold: false,
            italic: false,
            underline: UnderlineStyle::None,
            color: None,
            character_spacing: None,
            transparency: None,
            subscript: false,
            superscript: false,
            strikethrough: false,
        }
    }

    /// Get the font name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the font name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get the font size in points
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Set the font size in points
    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    /// Check if font is bold
    pub fn is_bold(&self) -> bool {
        self.bold
    }

    /// Set bold
    pub fn set_bold(&mut self, bold: bool) {
        self.bold = bold;
    }

    /// Check if font is italic
    pub fn is_italic(&self) -> bool {
        self.italic
    }

    /// Set italic
    pub fn set_italic(&mut self, italic: bool) {
        self.italic = italic;
    }

    /// Get underline style
    pub fn underline_style(&self) -> UnderlineStyle {
        self.underline
    }

    /// Set underline style
    pub fn set_underline_style(&mut self, style: UnderlineStyle) {
        self.underline = style;
    }

    /// Check if font is underlined
    pub fn is_underline(&self) -> bool {
        self.underline != UnderlineStyle::None
    }

    /// Set underline (simple on/off)
    pub fn set_underline(&mut self, underline: bool) {
        self.underline = if underline { UnderlineStyle::Single } else { UnderlineStyle::None };
    }

    /// Get the font color (RGB hex string)
    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    /// Set the font color (RGB hex string, e.g., "FF0000" for red)
    pub fn set_color(&mut self, color: String) {
        self.color = Some(color);
    }

    /// Get character spacing in EMU
    pub fn character_spacing(&self) -> Option<i32> {
        self.character_spacing
    }

    /// Set character spacing in EMU (English Metric Units)
    /// Positive values increase spacing, negative values decrease spacing
    pub fn set_character_spacing(&mut self, spacing: i32) {
        self.character_spacing = Some(spacing);
    }

    /// Get transparency (0-100 percentage)
    pub fn transparency(&self) -> Option<u32> {
        self.transparency
    }

    /// Set transparency (0-100 percentage)
    pub fn set_transparency(&mut self, transparency: u32) {
        if transparency <= 100 {
            self.transparency = Some(transparency);
        }
    }

    /// Check if text is subscript
    pub fn is_subscript(&self) -> bool {
        self.subscript
    }

    /// Set subscript
    pub fn set_subscript(&mut self, subscript: bool) {
        self.subscript = subscript;
        if subscript {
            self.superscript = false; // Can't be both
        }
    }

    /// Check if text is superscript
    pub fn is_superscript(&self) -> bool {
        self.superscript
    }

    /// Set superscript
    pub fn set_superscript(&mut self, superscript: bool) {
        self.superscript = superscript;
        if superscript {
            self.subscript = false; // Can't be both
        }
    }

    /// Check if text has strikethrough
    pub fn is_strikethrough(&self) -> bool {
        self.strikethrough
    }

    /// Set strikethrough
    pub fn set_strikethrough(&mut self, strikethrough: bool) {
        self.strikethrough = strikethrough;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_new() {
        let font = Font::new();
        assert_eq!(font.name(), "Calibri");
        assert_eq!(font.size(), 18);
        assert!(!font.is_bold());
        assert!(!font.is_italic());
        assert!(!font.is_underline());
        assert_eq!(font.underline_style(), UnderlineStyle::None);
    }

    #[test]
    fn test_font_with_name_size() {
        let font = Font::with_name_size("Arial".to_string(), 24);
        assert_eq!(font.name(), "Arial");
        assert_eq!(font.size(), 24);
    }

    #[test]
    fn test_font_properties() {
        let mut font = Font::new();
        font.set_name("Times New Roman".to_string());
        font.set_size(12);
        font.set_bold(true);
        font.set_italic(true);
        font.set_underline(true);
        font.set_color("FF0000".to_string());
        
        assert_eq!(font.name(), "Times New Roman");
        assert_eq!(font.size(), 12);
        assert!(font.is_bold());
        assert!(font.is_italic());
        assert!(font.is_underline());
        assert_eq!(font.color(), Some("FF0000"));
    }

    #[test]
    fn test_underline_styles() {
        let mut font = Font::new();
        
        font.set_underline_style(UnderlineStyle::Single);
        assert_eq!(font.underline_style(), UnderlineStyle::Single);
        
        font.set_underline_style(UnderlineStyle::Double);
        assert_eq!(font.underline_style(), UnderlineStyle::Double);
        
        font.set_underline_style(UnderlineStyle::Wavy);
        assert_eq!(font.underline_style(), UnderlineStyle::Wavy);
    }

    #[test]
    fn test_character_spacing() {
        let mut font = Font::new();
        assert_eq!(font.character_spacing(), None);
        
        font.set_character_spacing(100);
        assert_eq!(font.character_spacing(), Some(100));
        
        font.set_character_spacing(-50);
        assert_eq!(font.character_spacing(), Some(-50));
    }

    #[test]
    fn test_transparency() {
        let mut font = Font::new();
        assert_eq!(font.transparency(), None);
        
        font.set_transparency(50);
        assert_eq!(font.transparency(), Some(50));
        
        font.set_transparency(0);
        assert_eq!(font.transparency(), Some(0));
        
        font.set_transparency(100);
        assert_eq!(font.transparency(), Some(100));
    }

    #[test]
    fn test_subscript_superscript() {
        let mut font = Font::new();
        assert!(!font.is_subscript());
        assert!(!font.is_superscript());
        
        font.set_subscript(true);
        assert!(font.is_subscript());
        assert!(!font.is_superscript());
        
        font.set_superscript(true);
        assert!(!font.is_subscript());
        assert!(font.is_superscript());
    }

    #[test]
    fn test_strikethrough() {
        let mut font = Font::new();
        assert!(!font.is_strikethrough());
        
        font.set_strikethrough(true);
        assert!(font.is_strikethrough());
    }
}
