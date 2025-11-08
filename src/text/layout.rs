//! Text layout functionality

use crate::enums::text::TextAlign;
use crate::text::run::Run;

/// Paragraph in a text frame
pub struct Paragraph {
    text: String,
    alignment: TextAlign,
    level: u32,
    runs: Vec<Run>,
}

impl Paragraph {
    /// Create a new paragraph
    pub fn new() -> Self {
        Self {
            text: String::new(),
            alignment: TextAlign::Left,
            level: 0,
            runs: Vec::new(),
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

    /// Add a text run to this paragraph
    pub fn add_run(&mut self, text: &str) -> &mut Run {
        let run = Run::new(text);
        self.runs.push(run);
        self.runs.last_mut().unwrap()
    }

    /// Get all runs in this paragraph
    pub fn runs(&self) -> &[Run] {
        &self.runs
    }

    /// Get mutable runs
    pub fn runs_mut(&mut self) -> &mut [Run] {
        &mut self.runs
    }

    /// Get a specific run by index
    pub fn run(&self, index: usize) -> Option<&Run> {
        self.runs.get(index)
    }

    /// Get a mutable run by index
    pub fn run_mut(&mut self, index: usize) -> Option<&mut Run> {
        self.runs.get_mut(index)
    }

    /// Clear all runs
    pub fn clear_runs(&mut self) {
        self.runs.clear();
    }

    /// Get the number of runs
    pub fn run_count(&self) -> usize {
        self.runs.len()
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

    #[test]
    fn test_paragraph_add_run() {
        let mut para = Paragraph::new();
        para.add_run("Hello");
        para.add_run("World");
        
        assert_eq!(para.run_count(), 2);
        assert_eq!(para.run(0).unwrap().text(), "Hello");
        assert_eq!(para.run(1).unwrap().text(), "World");
    }

    #[test]
    fn test_paragraph_run_with_hyperlink() {
        use crate::shapes::hyperlink::Hyperlink;
        
        let mut para = Paragraph::new();
        let run = para.add_run("Click here");
        
        let hyperlink = Hyperlink::with_address("https://example.com".to_string());
        run.add_hyperlink(hyperlink);
        
        assert!(para.run(0).unwrap().has_hyperlink());
        assert_eq!(para.run(0).unwrap().hyperlink_address(), Some("https://example.com"));
    }

    #[test]
    fn test_paragraph_clear_runs() {
        let mut para = Paragraph::new();
        para.add_run("Hello");
        para.add_run("World");
        
        assert_eq!(para.run_count(), 2);
        para.clear_runs();
        assert_eq!(para.run_count(), 0);
    }
}
