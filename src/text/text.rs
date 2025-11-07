//! Text frame functionality

use crate::text::layout::Paragraph;

/// Text frame - container for text in a shape
pub struct TextFrame {
    paragraphs: Vec<Paragraph>,
    text: String,
    word_wrap: bool,
    margin_left: u32,   // in EMU
    margin_right: u32,  // in EMU
    margin_top: u32,    // in EMU
    margin_bottom: u32, // in EMU
}

impl TextFrame {
    /// Create a new text frame
    pub fn new() -> Self {
        Self {
            paragraphs: vec![Paragraph::new()],
            text: String::new(),
            word_wrap: true,
            margin_left: 0,
            margin_right: 0,
            margin_top: 0,
            margin_bottom: 0,
        }
    }

    /// Set the text content
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        // TODO: Parse text into paragraphs
        self.paragraphs = vec![Paragraph::new()];
        if let Some(p) = self.paragraphs.first_mut() {
            p.set_text(text);
        }
    }

    /// Get the text content
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Clear all text
    pub fn clear(&mut self) {
        self.text.clear();
        self.paragraphs = vec![Paragraph::new()];
    }

    /// Add a paragraph
    pub fn add_paragraph(&mut self) -> &mut Paragraph {
        let para = Paragraph::new();
        self.paragraphs.push(para);
        self.paragraphs.last_mut().unwrap()
    }

    /// Get paragraphs
    pub fn paragraphs(&self) -> &[Paragraph] {
        &self.paragraphs
    }

    /// Get mutable paragraphs
    pub fn paragraphs_mut(&mut self) -> &mut [Paragraph] {
        &mut self.paragraphs
    }

    /// Get word wrap setting
    pub fn word_wrap(&self) -> bool {
        self.word_wrap
    }

    /// Set word wrap
    pub fn set_word_wrap(&mut self, wrap: bool) {
        self.word_wrap = wrap;
    }

    /// Get margin left in EMU
    pub fn margin_left(&self) -> u32 {
        self.margin_left
    }

    /// Set margin left in EMU
    pub fn set_margin_left(&mut self, margin: u32) {
        self.margin_left = margin;
    }

    /// Get margin right in EMU
    pub fn margin_right(&self) -> u32 {
        self.margin_right
    }

    /// Set margin right in EMU
    pub fn set_margin_right(&mut self, margin: u32) {
        self.margin_right = margin;
    }

    /// Get margin top in EMU
    pub fn margin_top(&self) -> u32 {
        self.margin_top
    }

    /// Set margin top in EMU
    pub fn set_margin_top(&mut self, margin: u32) {
        self.margin_top = margin;
    }

    /// Get margin bottom in EMU
    pub fn margin_bottom(&self) -> u32 {
        self.margin_bottom
    }

    /// Set margin bottom in EMU
    pub fn set_margin_bottom(&mut self, margin: u32) {
        self.margin_bottom = margin;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_frame_new() {
        let tf = TextFrame::new();
        assert_eq!(tf.text(), "");
        assert!(tf.word_wrap());
        assert_eq!(tf.paragraphs().len(), 1);
    }

    #[test]
    fn test_text_frame_set_text() {
        let mut tf = TextFrame::new();
        tf.set_text("Hello, World!");
        assert_eq!(tf.text(), "Hello, World!");
    }

    #[test]
    fn test_text_frame_clear() {
        let mut tf = TextFrame::new();
        tf.set_text("Hello");
        tf.clear();
        assert_eq!(tf.text(), "");
        assert_eq!(tf.paragraphs().len(), 1);
    }

    #[test]
    fn test_text_frame_add_paragraph() {
        let mut tf = TextFrame::new();
        let para = tf.add_paragraph();
        para.set_text("New paragraph");
        assert_eq!(tf.paragraphs().len(), 2);
    }

    #[test]
    fn test_text_frame_margins() {
        let mut tf = TextFrame::new();
        tf.set_margin_left(914400); // 0.1 inch
        tf.set_margin_right(914400);
        tf.set_margin_top(457200);
        tf.set_margin_bottom(457200);
        
        assert_eq!(tf.margin_left(), 914400);
        assert_eq!(tf.margin_right(), 914400);
        assert_eq!(tf.margin_top(), 457200);
        assert_eq!(tf.margin_bottom(), 457200);
    }
}
