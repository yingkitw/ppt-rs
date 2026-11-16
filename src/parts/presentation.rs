//! Presentation part
//!
//! Represents the main presentation.xml part.

use super::base::{ContentType, Part, PartType};
use super::relationships::{RelationshipType, Relationships};
use crate::exc::PptxError;
use crate::oxml::XmlParser;

/// Slide reference in presentation
#[derive(Debug, Clone)]
pub struct SlideRef {
    pub id: u32,
    pub r_id: String,
}

/// Presentation part (presentation.xml)
#[derive(Debug, Clone)]
pub struct PresentationPart {
    path: String,
    slide_refs: Vec<SlideRef>,
    slide_master_r_id: Option<String>,
    theme_r_id: Option<String>,
    slide_width: u32,
    slide_height: u32,
}

impl PresentationPart {
    /// Create a new presentation part
    pub fn new() -> Self {
        PresentationPart {
            path: "ppt/presentation.xml".to_string(),
            slide_refs: Vec::new(),
            slide_master_r_id: None,
            theme_r_id: None,
            slide_width: 9144000,  // 10 inches
            slide_height: 6858000, // 7.5 inches
        }
    }

    /// Set slide master relationship ID
    pub fn set_slide_master(&mut self, r_id: &str) {
        self.slide_master_r_id = Some(r_id.to_string());
    }

    /// Set theme relationship ID
    pub fn set_theme(&mut self, r_id: &str) {
        self.theme_r_id = Some(r_id.to_string());
    }

    /// Add a slide reference
    pub fn add_slide(&mut self, r_id: &str) -> u32 {
        let id = 256 + self.slide_refs.len() as u32 + 1;
        self.slide_refs.push(SlideRef {
            id,
            r_id: r_id.to_string(),
        });
        id
    }

    /// Get slide count
    pub fn slide_count(&self) -> usize {
        self.slide_refs.len()
    }

    /// Get slide references
    pub fn slides(&self) -> &[SlideRef] {
        &self.slide_refs
    }

    /// Set slide dimensions
    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.slide_width = width;
        self.slide_height = height;
    }

    /// Create default relationships for presentation
    pub fn create_relationships(&self) -> Relationships {
        let mut rels = Relationships::new();

        // Add slide master
        if let Some(ref r_id) = self.slide_master_r_id {
            rels.add_with_id(
                r_id,
                RelationshipType::SlideMaster,
                "slideMasters/slideMaster1.xml",
            );
        } else {
            rels.add(
                RelationshipType::SlideMaster,
                "slideMasters/slideMaster1.xml",
            );
        }

        // Add theme
        if let Some(ref r_id) = self.theme_r_id {
            rels.add_with_id(r_id, RelationshipType::Theme, "theme/theme1.xml");
        } else {
            rels.add(RelationshipType::Theme, "theme/theme1.xml");
        }

        // Add slides
        for (i, slide_ref) in self.slide_refs.iter().enumerate() {
            rels.add_with_id(
                &slide_ref.r_id,
                RelationshipType::Slide,
                &format!("slides/slide{}.xml", i + 1),
            );
        }

        rels
    }
}

impl Default for PresentationPart {
    fn default() -> Self {
        Self::new()
    }
}

impl Part for PresentationPart {
    fn path(&self) -> &str {
        &self.path
    }

    fn part_type(&self) -> PartType {
        PartType::Presentation
    }

    fn content_type(&self) -> ContentType {
        ContentType::Presentation
    }

    fn to_xml(&self) -> Result<String, PptxError> {
        let mut slide_id_list = String::new();
        for slide_ref in &self.slide_refs {
            slide_id_list.push_str(&format!(
                "\n<p:sldId id=\"{}\" r:id=\"{}\"/>",
                slide_ref.id, slide_ref.r_id
            ));
        }

        let master_r_id = self.slide_master_r_id.as_deref().unwrap_or("rId1");

        let xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" saveSubsetFonts="1">
<p:sldMasterIdLst>
<p:sldMasterId id="2147483648" r:id="{}"/>
</p:sldMasterIdLst>
<p:sldIdLst>{}
</p:sldIdLst>
<p:sldSz cx="{}" cy="{}" type="screen4x3"/>
<p:notesSz cx="{}" cy="{}"/>
</p:presentation>"#,
            master_r_id,
            slide_id_list,
            self.slide_width,
            self.slide_height,
            self.slide_height,
            self.slide_width,
        );

        Ok(xml)
    }

    fn from_xml(xml: &str) -> Result<Self, PptxError> {
        let root = XmlParser::parse_str(xml)?;
        let mut part = PresentationPart::new();

        // Parse slide references
        for sld_id in root.find_all_descendants("sldId") {
            if let (Some(id), Some(r_id)) = (sld_id.attr("id"), sld_id.attr("r:id")) {
                if let Ok(id_num) = id.parse::<u32>() {
                    part.slide_refs.push(SlideRef {
                        id: id_num,
                        r_id: r_id.to_string(),
                    });
                }
            }
        }

        // Parse slide master reference
        if let Some(master_id) = root.find_descendant("sldMasterId") {
            part.slide_master_r_id = master_id.attr("r:id").map(|s| s.to_string());
        }

        // Parse slide size
        if let Some(sld_sz) = root.find_descendant("sldSz") {
            if let Some(cx) = sld_sz.attr("cx").and_then(|s| s.parse().ok()) {
                part.slide_width = cx;
            }
            if let Some(cy) = sld_sz.attr("cy").and_then(|s| s.parse().ok()) {
                part.slide_height = cy;
            }
        }

        Ok(part)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentation_part_new() {
        let part = PresentationPart::new();
        assert_eq!(part.slide_count(), 0);
        assert_eq!(part.path(), "ppt/presentation.xml");
    }

    #[test]
    fn test_add_slides() {
        let mut part = PresentationPart::new();
        part.add_slide("rId3");
        part.add_slide("rId4");

        assert_eq!(part.slide_count(), 2);
        assert_eq!(part.slides()[0].r_id, "rId3");
    }

    #[test]
    fn test_to_xml() {
        let mut part = PresentationPart::new();
        part.set_slide_master("rId1");
        part.add_slide("rId3");

        let xml = part.to_xml().unwrap();
        assert!(xml.contains("p:presentation"));
        assert!(xml.contains("p:sldId"));
        assert!(xml.contains("rId3"));
    }

    #[test]
    fn test_from_xml() {
        let xml = r#"<?xml version="1.0"?>
        <p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" 
                        xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
            <p:sldMasterIdLst>
                <p:sldMasterId id="2147483648" r:id="rId1"/>
            </p:sldMasterIdLst>
            <p:sldIdLst>
                <p:sldId id="257" r:id="rId3"/>
                <p:sldId id="258" r:id="rId4"/>
            </p:sldIdLst>
            <p:sldSz cx="9144000" cy="6858000"/>
        </p:presentation>"#;

        let part = PresentationPart::from_xml(xml).unwrap();
        assert_eq!(part.slide_count(), 2);
        assert_eq!(part.slides()[0].id, 257);
    }
}
