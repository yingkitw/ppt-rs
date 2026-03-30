//! Slide layout part
//!
//! Represents a slide layout template (ppt/slideLayouts/slideLayoutN.xml).

use super::base::{ContentType, Part, PartType};
use crate::exc::PptxError;

/// Slide layout types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutType {
    Title,
    TitleAndContent,
    SectionHeader,
    TwoContent,
    Comparison,
    TitleOnly,
    Blank,
    ContentWithCaption,
    PictureWithCaption,
    TitleAndVerticalText,
    VerticalTitleAndText,
    Custom,
}

impl LayoutType {
    /// Get the layout name
    pub fn name(&self) -> &'static str {
        match self {
            LayoutType::Title => "Title Slide",
            LayoutType::TitleAndContent => "Title and Content",
            LayoutType::SectionHeader => "Section Header",
            LayoutType::TwoContent => "Two Content",
            LayoutType::Comparison => "Comparison",
            LayoutType::TitleOnly => "Title Only",
            LayoutType::Blank => "Blank",
            LayoutType::ContentWithCaption => "Content with Caption",
            LayoutType::PictureWithCaption => "Picture with Caption",
            LayoutType::TitleAndVerticalText => "Title and Vertical Text",
            LayoutType::VerticalTitleAndText => "Vertical Title and Text",
            LayoutType::Custom => "Custom",
        }
    }

    /// Get the layout type attribute value
    pub fn type_value(&self) -> &'static str {
        match self {
            LayoutType::Title => "title",
            LayoutType::TitleAndContent => "obj",
            LayoutType::SectionHeader => "secHead",
            LayoutType::TwoContent => "twoObj",
            LayoutType::Comparison => "twoTxTwoObj",
            LayoutType::TitleOnly => "titleOnly",
            LayoutType::Blank => "blank",
            LayoutType::ContentWithCaption => "objTx",
            LayoutType::PictureWithCaption => "picTx",
            LayoutType::TitleAndVerticalText => "vertTx",
            LayoutType::VerticalTitleAndText => "vertTitleAndTx",
            LayoutType::Custom => "cust",
        }
    }
}

/// Slide layout part (ppt/slideLayouts/slideLayoutN.xml)
#[derive(Debug, Clone)]
pub struct SlideLayoutPart {
    path: String,
    layout_number: usize,
    layout_type: LayoutType,
    name: String,
    master_rel_id: String,
    xml_content: Option<String>,
}

impl SlideLayoutPart {
    /// Create a new slide layout part
    pub fn new(layout_number: usize, layout_type: LayoutType) -> Self {
        SlideLayoutPart {
            path: format!("ppt/slideLayouts/slideLayout{}.xml", layout_number),
            layout_number,
            layout_type,
            name: layout_type.name().to_string(),
            master_rel_id: "rId1".to_string(),
            xml_content: None,
        }
    }

    /// Get layout number
    pub fn layout_number(&self) -> usize {
        self.layout_number
    }

    /// Get layout type
    pub fn layout_type(&self) -> LayoutType {
        self.layout_type
    }

    /// Get layout name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set custom name
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Set master relationship ID
    pub fn set_master_rel_id(&mut self, rel_id: impl Into<String>) {
        self.master_rel_id = rel_id.into();
    }

    /// Get relative path for relationships
    pub fn rel_target(&self) -> String {
        format!("slideLayouts/slideLayout{}.xml", self.layout_number)
    }

    fn generate_xml(&self) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="{}" preserve="1">
  <p:cSld name="{}">
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
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sldLayout>"#,
            self.layout_type.type_value(),
            self.name
        )
    }
}

impl Part for SlideLayoutPart {
    fn path(&self) -> &str {
        &self.path
    }

    fn part_type(&self) -> PartType {
        PartType::SlideLayout
    }

    fn content_type(&self) -> ContentType {
        ContentType::SlideLayout
    }

    fn to_xml(&self) -> Result<String, PptxError> {
        if let Some(ref xml) = self.xml_content {
            return Ok(xml.clone());
        }
        Ok(self.generate_xml())
    }

    fn from_xml(xml: &str) -> Result<Self, PptxError> {
        Ok(SlideLayoutPart {
            path: "ppt/slideLayouts/slideLayout1.xml".to_string(),
            layout_number: 1,
            layout_type: LayoutType::TitleAndContent,
            name: "Layout".to_string(),
            master_rel_id: "rId1".to_string(),
            xml_content: Some(xml.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_layout_new() {
        let layout = SlideLayoutPart::new(1, LayoutType::Title);
        assert_eq!(layout.layout_number(), 1);
        assert_eq!(layout.layout_type(), LayoutType::Title);
        assert_eq!(layout.path(), "ppt/slideLayouts/slideLayout1.xml");
    }

    #[test]
    fn test_layout_type_name() {
        assert_eq!(LayoutType::Title.name(), "Title Slide");
        assert_eq!(LayoutType::Blank.name(), "Blank");
        assert_eq!(LayoutType::TwoContent.name(), "Two Content");
    }

    #[test]
    fn test_layout_type_value() {
        assert_eq!(LayoutType::Title.type_value(), "title");
        assert_eq!(LayoutType::Blank.type_value(), "blank");
        assert_eq!(LayoutType::TitleAndContent.type_value(), "obj");
    }

    #[test]
    fn test_slide_layout_to_xml() {
        let layout = SlideLayoutPart::new(1, LayoutType::Title);
        let xml = layout.to_xml().unwrap();
        assert!(xml.contains("p:sldLayout"));
        assert!(xml.contains("type=\"title\""));
    }

    #[test]
    fn test_slide_layout_rel_target() {
        let layout = SlideLayoutPart::new(3, LayoutType::Blank);
        assert_eq!(layout.rel_target(), "slideLayouts/slideLayout3.xml");
    }
}
