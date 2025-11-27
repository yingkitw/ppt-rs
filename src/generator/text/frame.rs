//! TextFrame - container for paragraphs

use super::paragraph::Paragraph;
use super::TextAnchor;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::text::TextAlign;

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
}
