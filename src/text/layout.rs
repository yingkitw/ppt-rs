//! Text layout functionality

use crate::enums::text::TextAlign;

/// Paragraph in a text frame
pub struct Paragraph {
    text: String,
    alignment: TextAlign,
    level: u32,
}

impl Paragraph {
    /// Create a new paragraph
    pub fn new() -> Self {
        Self {
            text: String::new(),
            alignment: TextAlign::Left,
            level: 0,
        }
    }

    /// Set the text content
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// Get the text content
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Clear the paragraph text
    pub fn clear(&mut self) {
        self.text.clear();
    }

    /// Get the alignment
    pub fn alignment(&self) -> TextAlign {
        self.alignment
    }

    /// Set the alignment
    pub fn set_alignment(&mut self, alignment: TextAlign) {
        self.alignment = alignment;
    }

    /// Get the paragraph level (for indentation)
    pub fn level(&self) -> u32 {
        self.level
    }

    /// Set the paragraph level
    pub fn set_level(&mut self, level: u32) {
        self.level = level;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::text::TextAlign;

    #[test]
    fn test_paragraph_new() {
        let para = Paragraph::new();
        assert_eq!(para.text(), "");
        assert_eq!(para.alignment(), TextAlign::Left);
        assert_eq!(para.level(), 0);
    }

    #[test]
    fn test_paragraph_set_text() {
        let mut para = Paragraph::new();
        para.set_text("Hello");
        assert_eq!(para.text(), "Hello");
    }

    #[test]
    fn test_paragraph_alignment() {
        let mut para = Paragraph::new();
        para.set_alignment(TextAlign::Center);
        assert_eq!(para.alignment(), TextAlign::Center);
        para.set_alignment(TextAlign::Right);
        assert_eq!(para.alignment(), TextAlign::Right);
    }

    #[test]
    fn test_paragraph_level() {
        let mut para = Paragraph::new();
        para.set_level(2);
        assert_eq!(para.level(), 2);
    }

    #[test]
    fn test_paragraph_clear() {
        let mut para = Paragraph::new();
        para.set_text("Hello");
        para.clear();
        assert_eq!(para.text(), "");
    }
}
