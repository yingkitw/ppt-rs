//! Notes slide part
//!
//! Represents speaker notes for a slide (ppt/notesSlides/notesSlideN.xml).

use super::base::{Part, PartType, ContentType};
use crate::exc::PptxError;
use crate::core::escape_xml;
use crate::util::format_lang_attributes;

/// Notes slide part (ppt/notesSlides/notesSlideN.xml)
#[derive(Debug, Clone)]
pub struct NotesSlidePart {
    path: String,
    notes_number: usize,
    slide_rel_id: String,
    notes_text: String,
    xml_content: Option<String>,
}

impl NotesSlidePart {
    /// Create a new notes slide part
    pub fn new(notes_number: usize) -> Self {
        NotesSlidePart {
            path: format!("ppt/notesSlides/notesSlide{}.xml", notes_number),
            notes_number,
            slide_rel_id: "rId1".to_string(),
            notes_text: String::new(),
            xml_content: None,
        }
    }

    /// Create with notes text
    pub fn with_text(notes_number: usize, text: impl Into<String>) -> Self {
        let mut part = Self::new(notes_number);
        part.notes_text = text.into();
        part
    }

    /// Get notes number
    pub fn notes_number(&self) -> usize {
        self.notes_number
    }

    /// Get notes text
    pub fn notes_text(&self) -> &str {
        &self.notes_text
    }

    /// Set notes text
    pub fn set_notes_text(&mut self, text: impl Into<String>) {
        self.notes_text = text.into();
        self.xml_content = None;
    }

    /// Set slide relationship ID
    pub fn set_slide_rel_id(&mut self, rel_id: impl Into<String>) {
        self.slide_rel_id = rel_id.into();
    }

    /// Get relative path for relationships
    pub fn rel_target(&self) -> String {
        format!("../notesSlides/notesSlide{}.xml", self.notes_number)
    }

    fn generate_xml(&self) -> String {
        let lang_attrs = format_lang_attributes();
        let paragraphs: String = if self.notes_text.is_empty() {
            format!("<a:p><a:endParaRPr {}/></a:p>", lang_attrs)
        } else {
            self.notes_text
                .lines()
                .map(|line| {
                    format!(
                        "<a:p><a:r><a:rPr {} dirty=\"0\"><a:t>{}</a:t></a:r></a:p>",
                        lang_attrs,
                        escape_xml(line)
                    )
                })
                .collect::<Vec<_>>()
                .join("\n              ")
        };

        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:notes xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
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
          <p:cNvPr id="2" name="Slide Image Placeholder 1"/>
          <p:cNvSpPr><a:spLocks noGrp="1" noRot="1" noChangeAspect="1"/></p:cNvSpPr>
          <p:nvPr><p:ph type="sldImg"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr/>
      </p:sp>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="3" name="Notes Placeholder 2"/>
          <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
          <p:nvPr><p:ph type="body" idx="1"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          {}
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr>
</p:notes>"#,
            paragraphs
        )
    }
}

impl Part for NotesSlidePart {
    fn path(&self) -> &str {
        &self.path
    }

    fn part_type(&self) -> PartType {
        PartType::Slide // Notes are associated with slides
    }

    fn content_type(&self) -> ContentType {
        ContentType::Xml // Notes have their own content type
    }

    fn to_xml(&self) -> Result<String, PptxError> {
        if let Some(ref xml) = self.xml_content {
            return Ok(xml.clone());
        }
        Ok(self.generate_xml())
    }

    fn from_xml(xml: &str) -> Result<Self, PptxError> {
        Ok(NotesSlidePart {
            path: "ppt/notesSlides/notesSlide1.xml".to_string(),
            notes_number: 1,
            slide_rel_id: "rId1".to_string(),
            notes_text: String::new(),
            xml_content: Some(xml.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notes_slide_new() {
        let notes = NotesSlidePart::new(1);
        assert_eq!(notes.notes_number(), 1);
        assert_eq!(notes.path(), "ppt/notesSlides/notesSlide1.xml");
    }

    #[test]
    fn test_notes_slide_with_text() {
        let notes = NotesSlidePart::with_text(1, "Speaker notes here");
        assert_eq!(notes.notes_text(), "Speaker notes here");
    }

    #[test]
    fn test_notes_slide_set_text() {
        let mut notes = NotesSlidePart::new(1);
        notes.set_notes_text("Updated notes");
        assert_eq!(notes.notes_text(), "Updated notes");
    }

    #[test]
    fn test_notes_slide_to_xml() {
        let notes = NotesSlidePart::with_text(1, "Test notes");
        let xml = notes.to_xml().unwrap();
        assert!(xml.contains("p:notes"));
        assert!(xml.contains("Test notes"));
    }

    #[test]
    fn test_notes_slide_multiline() {
        let notes = NotesSlidePart::with_text(1, "Line 1\nLine 2\nLine 3");
        let xml = notes.to_xml().unwrap();
        assert!(xml.contains("Line 1"));
        assert!(xml.contains("Line 2"));
        assert!(xml.contains("Line 3"));
    }

    #[test]
    fn test_notes_slide_rel_target() {
        let notes = NotesSlidePart::new(3);
        assert_eq!(notes.rel_target(), "../notesSlides/notesSlide3.xml");
    }
}
