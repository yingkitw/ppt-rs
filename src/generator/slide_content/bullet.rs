//! Bullet point types and formatting

use crate::core::ToXml;

/// Bullet style for lists
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum BulletStyle {
    /// Standard bullet point (•)
    #[default]
    Bullet,
    /// Numbered list (1, 2, 3...)
    Number,
    /// Lettered list (a, b, c...)
    LetterLower,
    /// Lettered list uppercase (A, B, C...)
    LetterUpper,
    /// Roman numerals lowercase (i, ii, iii...)
    RomanLower,
    /// Roman numerals uppercase (I, II, III...)
    RomanUpper,
    /// Custom bullet character
    Custom(char),
    /// No bullet
    None,
}

impl BulletStyle {
    /// Get the OOXML bullet type attribute
    pub fn to_xml(&self) -> String {
        match self {
            BulletStyle::Bullet => r#"<a:buChar char="•"/>"#.to_string(),
            BulletStyle::Number => r#"<a:buAutoNum type="arabicPeriod"/>"#.to_string(),
            BulletStyle::LetterLower => r#"<a:buAutoNum type="alphaLcPeriod"/>"#.to_string(),
            BulletStyle::LetterUpper => r#"<a:buAutoNum type="alphaUcPeriod"/>"#.to_string(),
            BulletStyle::RomanLower => r#"<a:buAutoNum type="romanLcPeriod"/>"#.to_string(),
            BulletStyle::RomanUpper => r#"<a:buAutoNum type="romanUcPeriod"/>"#.to_string(),
            BulletStyle::Custom(ch) => format!(r#"<a:buChar char="{}"/>"#, ch),
            BulletStyle::None => r#"<a:buNone/>"#.to_string(),
        }
    }
    
    /// Get the OOXML indent level XML
    pub fn indent_xml(&self, level: u32) -> String {
        let indent = 457200 + (level * 457200); // 0.5 inch base + 0.5 inch per level
        let margin_left = level * 457200;
        format!(r#"indent="-{}" marL="{}""#, indent, margin_left + indent)
    }
}

impl ToXml for BulletStyle {
    fn to_xml(&self) -> String {
        BulletStyle::to_xml(self)
    }
}

/// Text formatting for bullet points
#[derive(Clone, Debug, Default)]
pub struct BulletTextFormat {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub subscript: bool,
    pub superscript: bool,
    pub color: Option<String>,
    pub highlight: Option<String>,
    pub font_size: Option<u32>,
    pub font_family: Option<String>,
}

impl BulletTextFormat {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
    
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }
    
    pub fn subscript(mut self) -> Self {
        self.subscript = true;
        self.superscript = false;
        self
    }
    
    pub fn superscript(mut self) -> Self {
        self.superscript = true;
        self.subscript = false;
        self
    }
    
    pub fn color(mut self, hex: &str) -> Self {
        self.color = Some(hex.trim_start_matches('#').to_uppercase());
        self
    }
    
    pub fn highlight(mut self, hex: &str) -> Self {
        self.highlight = Some(hex.trim_start_matches('#').to_uppercase());
        self
    }
    
    pub fn font_size(mut self, size: u32) -> Self {
        self.font_size = Some(size);
        self
    }
    
    pub fn font_family(mut self, family: &str) -> Self {
        self.font_family = Some(family.to_string());
        self
    }
}

/// A bullet point with optional style and formatting
#[derive(Clone, Debug)]
pub struct BulletPoint {
    pub text: String,
    pub level: u32,
    pub style: BulletStyle,
    pub format: Option<BulletTextFormat>,
}

impl BulletPoint {
    pub fn new(text: &str) -> Self {
        BulletPoint {
            text: text.to_string(),
            level: 0,
            style: BulletStyle::Bullet,
            format: None,
        }
    }
    
    pub fn with_level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }
    
    pub fn with_style(mut self, style: BulletStyle) -> Self {
        self.style = style;
        self
    }
    
    pub fn with_format(mut self, format: BulletTextFormat) -> Self {
        self.format = Some(format);
        self
    }
    
    pub fn bold(mut self) -> Self {
        self.format = Some(self.format.unwrap_or_default().bold());
        self
    }
    
    pub fn italic(mut self) -> Self {
        self.format = Some(self.format.unwrap_or_default().italic());
        self
    }
    
    pub fn strikethrough(mut self) -> Self {
        self.format = Some(self.format.unwrap_or_default().strikethrough());
        self
    }
    
    pub fn subscript(mut self) -> Self {
        self.format = Some(self.format.unwrap_or_default().subscript());
        self
    }
    
    pub fn superscript(mut self) -> Self {
        self.format = Some(self.format.unwrap_or_default().superscript());
        self
    }
    
    pub fn highlight(mut self, color: &str) -> Self {
        self.format = Some(self.format.unwrap_or_default().highlight(color));
        self
    }
    
    pub fn color(mut self, hex: &str) -> Self {
        self.format = Some(self.format.unwrap_or_default().color(hex));
        self
    }
    
    pub fn font_size(mut self, size: u32) -> Self {
        self.format = Some(self.format.unwrap_or_default().font_size(size));
        self
    }
}

