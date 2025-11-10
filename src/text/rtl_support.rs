//! Right-to-Left (RTL) Text Support
//!
//! This module provides comprehensive RTL text support for presentations including:
//! - RTL language detection (Arabic, Hebrew, Persian, Urdu, etc.)
//! - Text direction management
//! - Paragraph alignment for RTL
//! - Bidirectional text handling

use crate::error::Result;

/// RTL language types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RTLLanguage {
    /// Arabic
    Arabic,
    /// Hebrew
    Hebrew,
    /// Persian/Farsi
    Persian,
    /// Urdu
    Urdu,
    /// Pashto
    Pashto,
    /// Kurdish
    Kurdish,
    /// Uyghur
    Uyghur,
    /// Dhivehi
    Dhivehi,
}

impl RTLLanguage {
    /// Get language code
    pub fn code(&self) -> &str {
        match self {
            RTLLanguage::Arabic => "ar",
            RTLLanguage::Hebrew => "he",
            RTLLanguage::Persian => "fa",
            RTLLanguage::Urdu => "ur",
            RTLLanguage::Pashto => "ps",
            RTLLanguage::Kurdish => "ku",
            RTLLanguage::Uyghur => "ug",
            RTLLanguage::Dhivehi => "dv",
        }
    }

    /// Get language name
    pub fn name(&self) -> &str {
        match self {
            RTLLanguage::Arabic => "Arabic",
            RTLLanguage::Hebrew => "Hebrew",
            RTLLanguage::Persian => "Persian",
            RTLLanguage::Urdu => "Urdu",
            RTLLanguage::Pashto => "Pashto",
            RTLLanguage::Kurdish => "Kurdish",
            RTLLanguage::Uyghur => "Uyghur",
            RTLLanguage::Dhivehi => "Dhivehi",
        }
    }

    /// Detect RTL language from text
    pub fn detect(text: &str) -> Option<RTLLanguage> {
        // Check for Arabic Unicode range (0x0600-0x06FF)
        if text.chars().any(|c| (c as u32) >= 0x0600 && (c as u32) <= 0x06FF) {
            return Some(RTLLanguage::Arabic);
        }

        // Check for Hebrew Unicode range (0x0590-0x05FF)
        if text.chars().any(|c| (c as u32) >= 0x0590 && (c as u32) <= 0x05FF) {
            return Some(RTLLanguage::Hebrew);
        }

        None
    }
}

/// Text direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDirection {
    /// Left-to-right (LTR)
    LeftToRight,
    /// Right-to-left (RTL)
    RightToLeft,
}

impl TextDirection {
    /// Get XML representation
    pub fn to_xml_str(&self) -> &str {
        match self {
            TextDirection::LeftToRight => "ltr",
            TextDirection::RightToLeft => "rtl",
        }
    }

    /// Check if RTL
    pub fn is_rtl(&self) -> bool {
        matches!(self, TextDirection::RightToLeft)
    }
}

/// RTL text configuration
#[derive(Debug, Clone)]
pub struct RTLTextConfig {
    /// Text direction
    direction: TextDirection,
    /// Language
    language: Option<RTLLanguage>,
    /// Enable bidirectional text handling
    bidirectional: bool,
    /// Text content
    text: String,
}

impl RTLTextConfig {
    /// Create a new RTL text configuration
    pub fn new(text: impl Into<String>) -> Self {
        let text_str = text.into();
        let language = RTLLanguage::detect(&text_str);
        let direction = if language.is_some() {
            TextDirection::RightToLeft
        } else {
            TextDirection::LeftToRight
        };

        Self {
            direction,
            language,
            bidirectional: language.is_some(),
            text: text_str,
        }
    }

    /// Create with explicit direction
    pub fn with_direction(text: impl Into<String>, direction: TextDirection) -> Self {
        Self {
            direction,
            language: None,
            bidirectional: direction.is_rtl(),
            text: text.into(),
        }
    }

    /// Create with language
    pub fn with_language(text: impl Into<String>, language: RTLLanguage) -> Self {
        Self {
            direction: TextDirection::RightToLeft,
            language: Some(language),
            bidirectional: true,
            text: text.into(),
        }
    }

    /// Get text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get direction
    pub fn direction(&self) -> TextDirection {
        self.direction
    }

    /// Get language
    pub fn language(&self) -> Option<RTLLanguage> {
        self.language
    }

    /// Set direction
    pub fn set_direction(mut self, direction: TextDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set language
    pub fn set_language(mut self, language: RTLLanguage) -> Self {
        self.language = Some(language);
        self.direction = TextDirection::RightToLeft;
        self
    }

    /// Enable bidirectional text
    pub fn enable_bidirectional(mut self) -> Self {
        self.bidirectional = true;
        self
    }

    /// Disable bidirectional text
    pub fn disable_bidirectional(mut self) -> Self {
        self.bidirectional = false;
        self
    }

    /// Check if bidirectional
    pub fn is_bidirectional(&self) -> bool {
        self.bidirectional
    }

    /// Generate XML for RTL text properties
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<a:rPr");

        if self.direction.is_rtl() {
            xml.push_str(" rtl=\"1\"");
        }

        if let Some(lang) = self.language {
            xml.push_str(&format!(" lang=\"{}\"", lang.code()));
        }

        if self.bidirectional {
            xml.push_str(" bidi=\"1\"");
        }

        xml.push_str("/>");
        xml
    }
}

/// RTL paragraph configuration
#[derive(Debug, Clone)]
pub struct RTLParagraph {
    /// Paragraph text
    text: String,
    /// Text direction
    direction: TextDirection,
    /// Language
    language: Option<RTLLanguage>,
    /// Paragraph alignment (for RTL: right=0, center=1, left=2)
    alignment: ParagraphAlignment,
}

/// Paragraph alignment for RTL
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParagraphAlignment {
    /// Right alignment (default for RTL)
    Right,
    /// Center alignment
    Center,
    /// Left alignment
    Left,
    /// Justified alignment
    Justified,
}

impl ParagraphAlignment {
    /// Get XML representation
    pub fn to_xml_str(&self) -> &str {
        match self {
            ParagraphAlignment::Right => "r",
            ParagraphAlignment::Center => "ctr",
            ParagraphAlignment::Left => "l",
            ParagraphAlignment::Justified => "just",
        }
    }
}

impl RTLParagraph {
    /// Create a new RTL paragraph
    pub fn new(text: impl Into<String>) -> Self {
        let text_str = text.into();
        let language = RTLLanguage::detect(&text_str);
        let direction = if language.is_some() {
            TextDirection::RightToLeft
        } else {
            TextDirection::LeftToRight
        };

        let alignment = if direction.is_rtl() {
            ParagraphAlignment::Right
        } else {
            ParagraphAlignment::Left
        };

        Self {
            text: text_str,
            direction,
            language,
            alignment,
        }
    }

    /// Get text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get direction
    pub fn direction(&self) -> TextDirection {
        self.direction
    }

    /// Get language
    pub fn language(&self) -> Option<RTLLanguage> {
        self.language
    }

    /// Get alignment
    pub fn alignment(&self) -> ParagraphAlignment {
        self.alignment
    }

    /// Set alignment
    pub fn set_alignment(mut self, alignment: ParagraphAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set language
    pub fn set_language(mut self, language: RTLLanguage) -> Self {
        self.language = Some(language);
        self.direction = TextDirection::RightToLeft;
        self
    }

    /// Generate XML for RTL paragraph
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<a:p");

        if self.direction.is_rtl() {
            xml.push_str(" rtl=\"1\"");
        }

        xml.push_str(&format!(" algn=\"{}\"", self.alignment.to_xml_str()));

        if let Some(lang) = self.language {
            xml.push_str(&format!(" lang=\"{}\"", lang.code()));
        }

        xml.push_str("/>");
        xml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtl_language_arabic() {
        let lang = RTLLanguage::Arabic;
        assert_eq!(lang.code(), "ar");
        assert_eq!(lang.name(), "Arabic");
    }

    #[test]
    fn test_rtl_language_hebrew() {
        let lang = RTLLanguage::Hebrew;
        assert_eq!(lang.code(), "he");
        assert_eq!(lang.name(), "Hebrew");
    }

    #[test]
    fn test_rtl_language_persian() {
        let lang = RTLLanguage::Persian;
        assert_eq!(lang.code(), "fa");
        assert_eq!(lang.name(), "Persian");
    }

    #[test]
    fn test_text_direction_rtl() {
        let dir = TextDirection::RightToLeft;
        assert!(dir.is_rtl());
        assert_eq!(dir.to_xml_str(), "rtl");
    }

    #[test]
    fn test_text_direction_ltr() {
        let dir = TextDirection::LeftToRight;
        assert!(!dir.is_rtl());
        assert_eq!(dir.to_xml_str(), "ltr");
    }

    #[test]
    fn test_rtl_text_config_new() {
        let config = RTLTextConfig::new("Hello");
        assert_eq!(config.text(), "Hello");
        assert_eq!(config.direction(), TextDirection::LeftToRight);
    }

    #[test]
    fn test_rtl_text_config_with_direction() {
        let config = RTLTextConfig::with_direction("Test", TextDirection::RightToLeft);
        assert_eq!(config.direction(), TextDirection::RightToLeft);
    }

    #[test]
    fn test_rtl_text_config_with_language() {
        let config = RTLTextConfig::with_language("Test", RTLLanguage::Arabic);
        assert_eq!(config.language(), Some(RTLLanguage::Arabic));
        assert_eq!(config.direction(), TextDirection::RightToLeft);
    }

    #[test]
    fn test_rtl_text_config_bidirectional() {
        let config = RTLTextConfig::new("Test").enable_bidirectional();
        assert!(config.is_bidirectional());
    }

    #[test]
    fn test_rtl_text_config_to_xml() {
        let config = RTLTextConfig::with_language("Test", RTLLanguage::Arabic);
        let xml = config.to_xml();
        assert!(xml.contains("rtl=\"1\""));
        assert!(xml.contains("lang=\"ar\""));
    }

    #[test]
    fn test_paragraph_alignment_right() {
        let align = ParagraphAlignment::Right;
        assert_eq!(align.to_xml_str(), "r");
    }

    #[test]
    fn test_paragraph_alignment_center() {
        let align = ParagraphAlignment::Center;
        assert_eq!(align.to_xml_str(), "ctr");
    }

    #[test]
    fn test_rtl_paragraph_new() {
        let para = RTLParagraph::new("Test");
        assert_eq!(para.text(), "Test");
    }

    #[test]
    fn test_rtl_paragraph_with_language() {
        let para = RTLParagraph::new("Test").set_language(RTLLanguage::Hebrew);
        assert_eq!(para.language(), Some(RTLLanguage::Hebrew));
    }

    #[test]
    fn test_rtl_paragraph_alignment() {
        let para = RTLParagraph::new("Test")
            .set_alignment(ParagraphAlignment::Center);
        assert_eq!(para.alignment(), ParagraphAlignment::Center);
    }

    #[test]
    fn test_rtl_paragraph_to_xml() {
        let para = RTLParagraph::new("Test")
            .set_language(RTLLanguage::Arabic);
        let xml = para.to_xml();
        assert!(xml.contains("rtl=\"1\""));
        assert!(xml.contains("lang=\"ar\""));
    }
}
