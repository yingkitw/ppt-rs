//! Slide part
//!
//! Represents a slide.xml part.

use super::base::{Part, PartType, ContentType};
use super::relationships::{Relationships, RelationshipType};
use crate::exc::PptxError;
use crate::generator::SlideContent;
use crate::generator::slide_xml::create_slide_xml_with_content;
use crate::oxml::{SlideParser, ParsedSlide};

/// Slide part (ppt/slides/slideN.xml)
#[derive(Debug, Clone)]
pub struct SlidePart {
    path: String,
    slide_number: usize,
    content: Option<SlideContent>,
    parsed: Option<ParsedSlide>,
    xml_content: Option<String>,
}

impl SlidePart {
    /// Create a new slide part
    pub fn new(slide_number: usize) -> Self {
        SlidePart {
            path: format!("ppt/slides/slide{}.xml", slide_number),
            slide_number,
            content: None,
            parsed: None,
            xml_content: None,
        }
    }

    /// Create from SlideContent
    pub fn from_content(slide_number: usize, content: SlideContent) -> Self {
        SlidePart {
            path: format!("ppt/slides/slide{}.xml", slide_number),
            slide_number,
            content: Some(content),
            parsed: None,
            xml_content: None,
        }
    }

    /// Get slide number
    pub fn slide_number(&self) -> usize {
        self.slide_number
    }

    /// Get content if available
    pub fn content(&self) -> Option<&SlideContent> {
        self.content.as_ref()
    }

    /// Set content
    pub fn set_content(&mut self, content: SlideContent) {
        self.content = Some(content);
        self.xml_content = None; // Clear cached XML
    }

    /// Get parsed slide if available
    pub fn parsed(&self) -> Option<&ParsedSlide> {
        self.parsed.as_ref()
    }

    /// Get title from parsed content
    pub fn title(&self) -> Option<&str> {
        self.parsed.as_ref().and_then(|p| p.title.as_deref())
    }

    /// Get body text from parsed content
    pub fn body_text(&self) -> Vec<&str> {
        self.parsed.as_ref()
            .map(|p| p.body_text.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get relationships path
    pub fn rels_path(&self) -> String {
        format!("ppt/slides/_rels/slide{}.xml.rels", self.slide_number)
    }

    /// Create default relationships for slide
    pub fn create_relationships(&self) -> Relationships {
        let mut rels = Relationships::new();
        rels.add(RelationshipType::SlideLayout, "../slideLayouts/slideLayout1.xml");
        rels
    }

    /// Generate relationships XML
    pub fn rels_xml(&self) -> String {
        self.create_relationships().to_xml()
    }
}

impl Part for SlidePart {
    fn path(&self) -> &str {
        &self.path
    }

    fn part_type(&self) -> PartType {
        PartType::Slide
    }

    fn content_type(&self) -> ContentType {
        ContentType::Slide
    }

    fn to_xml(&self) -> Result<String, PptxError> {
        // Return cached XML if available
        if let Some(ref xml) = self.xml_content {
            return Ok(xml.clone());
        }

        // Generate from content if available
        if let Some(ref content) = self.content {
            let xml = create_slide_xml_with_content(self.slide_number, content, &[]);
            return Ok(xml);
        }

        // Return minimal slide XML
        Ok(format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr/>
</p:spTree>
</p:cSld>
</p:sld>"#
        ))
    }

    fn from_xml(xml: &str) -> Result<Self, PptxError> {
        let parsed = SlideParser::parse(xml)?;
        
        // Try to determine slide number from parsed content
        // Default to 1 if unknown
        let slide_number = 1;
        
        Ok(SlidePart {
            path: format!("ppt/slides/slide{}.xml", slide_number),
            slide_number,
            content: None,
            parsed: Some(parsed),
            xml_content: Some(xml.to_string()),
        })
    }
}

/// Parse slide from XML with known slide number
pub fn parse_slide(xml: &str, slide_number: usize) -> Result<SlidePart, PptxError> {
    let parsed = SlideParser::parse(xml)?;
    
    Ok(SlidePart {
        path: format!("ppt/slides/slide{}.xml", slide_number),
        slide_number,
        content: None,
        parsed: Some(parsed),
        xml_content: Some(xml.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_part_new() {
        let part = SlidePart::new(1);
        assert_eq!(part.slide_number(), 1);
        assert_eq!(part.path(), "ppt/slides/slide1.xml");
    }

    #[test]
    fn test_slide_part_from_content() {
        let content = SlideContent::new("Test Title")
            .add_bullet("Bullet 1");
        let part = SlidePart::from_content(2, content);
        
        assert_eq!(part.slide_number(), 2);
        assert!(part.content().is_some());
    }

    #[test]
    fn test_slide_part_to_xml() {
        let content = SlideContent::new("Test")
            .add_bullet("Point");
        let part = SlidePart::from_content(1, content);
        
        let xml = part.to_xml().unwrap();
        assert!(xml.contains("p:sld"));
        assert!(xml.contains("Test"));
    }

    #[test]
    fn test_slide_part_rels() {
        let part = SlidePart::new(1);
        let rels_xml = part.rels_xml();
        
        assert!(rels_xml.contains("slideLayout"));
        assert!(rels_xml.contains("rId1"));
    }
}
