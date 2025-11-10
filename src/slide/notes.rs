//! Notes slide support
//!
//! Notes slides contain speaker notes for each slide.

use crate::error::Result;

/// Notes text frame
#[derive(Clone)]
pub struct NotesTextFrame {
    text: String,
}

impl std::fmt::Debug for NotesTextFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotesTextFrame")
            .field("text", &self.text)
            .finish()
    }
}

impl NotesTextFrame {
    /// Create a new notes text frame
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    /// Get the text content
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the text content
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl Default for NotesTextFrame {
    fn default() -> Self {
        Self::new()
    }
}

/// Notes slide
#[derive(Clone)]
pub struct NotesSlide {
    notes_text_frame: NotesTextFrame,
}

impl std::fmt::Debug for NotesSlide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotesSlide")
            .field("notes_text_frame", &self.notes_text_frame)
            .finish()
    }
}

impl NotesSlide {
    /// Create a new notes slide
    pub fn new() -> Self {
        Self {
            notes_text_frame: NotesTextFrame::new(),
        }
    }

    /// Get the notes text frame
    pub fn notes_text_frame(&self) -> &NotesTextFrame {
        &self.notes_text_frame
    }

    /// Get mutable reference to the notes text frame
    pub fn notes_text_frame_mut(&mut self) -> &mut NotesTextFrame {
        &mut self.notes_text_frame
    }

    /// Get the notes text content
    pub fn text(&self) -> &str {
        self.notes_text_frame.text()
    }

    /// Set the notes text content
    pub fn set_text(&mut self, text: String) {
        self.notes_text_frame.set_text(text);
    }

    /// Generate notes slide XML
    pub fn to_xml(&self) -> Result<String> {
        let text = self.notes_text_frame.text();
        let escaped_text = escape_xml(&text);
        
        let xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:notes xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
         xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
         xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Notes"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p>
            <a:r>
              <a:rPr lang="en-US" dirty="0"/>
              <a:t>{}</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:notes>"#,
            escaped_text
        );
        
        Ok(xml)
    }
}

impl Default for NotesSlide {
    fn default() -> Self {
        Self::new()
    }
}

/// Escape XML special characters
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notes_text_frame() {
        let mut frame = NotesTextFrame::new();
        frame.set_text("Test notes".to_string());
        assert_eq!(frame.text(), "Test notes");
    }

    #[test]
    fn test_notes_slide() {
        let mut slide = NotesSlide::new();
        slide.set_text("Speaker notes".to_string());
        assert_eq!(slide.text(), "Speaker notes");
    }

    #[test]
    fn test_notes_slide_xml() {
        let mut slide = NotesSlide::new();
        slide.set_text("Test".to_string());
        let xml = slide.to_xml().unwrap();
        assert!(xml.contains("Test"));
        assert!(xml.contains("p:notes"));
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("a&b"), "a&amp;b");
        assert_eq!(escape_xml("a<b"), "a&lt;b");
        assert_eq!(escape_xml("a>b"), "a&gt;b");
        assert_eq!(escape_xml("a\"b"), "a&quot;b");
        assert_eq!(escape_xml("a'b"), "a&apos;b");
    }
}
