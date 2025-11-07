//! Presentation part - the main document part

use crate::error::Result;
use crate::opc::constants::CONTENT_TYPE;
use crate::opc::part::{BasePart, Part};
use crate::opc::packuri::PackURI;
use crate::opc::relationships::Relationships;

/// Presentation part - the main document part
pub struct PresentationPart {
    base: BasePart,
}

impl PresentationPart {
    /// Create a new presentation part
    pub fn new() -> Result<Self> {
        let uri = PackURI::new("/ppt/presentation.xml")?;
        let base = BasePart::new(CONTENT_TYPE::PML_PRESENTATION_MAIN, uri)?;
        Ok(Self { base })
    }

    /// Create a new presentation part with XML content
    pub fn with_xml(uri: PackURI, xml_content: String) -> Result<Self> {
        let mut base = BasePart::new(CONTENT_TYPE::PML_PRESENTATION_MAIN, uri)?;
        // Store XML content as blob
        base.set_blob(xml_content.as_bytes().to_vec());
        Ok(Self { base })
    }

    /// Get the core properties part
    pub fn core_properties(&self) -> Result<crate::parts::coreprops::CorePropertiesPart> {
        use crate::opc::constants::RELATIONSHIP_TYPE;
        use crate::opc::part::Part;
        
        // Look for core properties relationship
        let rels = self.relationships();
        if let Some(rel) = rels.iter().find(|(_, r)| r.rel_type == RELATIONSHIP_TYPE::CORE_PROPERTIES) {
            let target = &rel.1.target;
            let partname = if target.starts_with('/') {
                PackURI::new(target)?
            } else {
                PackURI::new(&format!("/{}", target))?
            };
            
            // Create a basic CorePropertiesPart
            // In full implementation, would load from package
            crate::parts::coreprops::CorePropertiesPart::new(partname)
        } else {
            // Return default core properties
            crate::parts::coreprops::CorePropertiesPart::new(PackURI::new("/docProps/core.xml")?)
        }
    }

    /// Add a slide to the presentation
    pub fn add_slide(&mut self, slide_layout_part: &dyn Part) -> Result<String> {
        use crate::opc::constants::RELATIONSHIP_TYPE;
        use crate::opc::part::Part;
        use crate::parts::slide::SlidePart;
        
        // 1. Get next slide partname
        let slide_partname = self.next_slide_partname()?;
        
        // 2. Create a new SlidePart
        let slide_part = SlidePart::new(slide_partname.clone(), slide_layout_part)?;
        
        // 3. Add relationship to the slide part
        let r_id = self.relationships_mut().next_r_id();
        self.relationships_mut().add(
            r_id.clone(),
            RELATIONSHIP_TYPE::SLIDE.to_string(),
            slide_partname.membername().to_string(),
            false,
        );
        
        // 4. Update presentation.xml to add sldId entry
        let mut xml = Part::to_xml(self)?;
        
        // Find next slide ID (starting at 256)
        let next_id = {
            let mut max_id = 255u32;
            let re = regex::Regex::new(r#"<p:sldId\s+[^>]*id="(\d+)""#)
                .map_err(|e| crate::error::PptError::ValueError(format!("Invalid regex: {}", e)))?;
            for cap in re.captures_iter(&xml) {
                if let Ok(id) = cap[1].parse::<u32>() {
                    if id > max_id {
                        max_id = id;
                    }
                }
            }
            max_id + 1
        };
        
        // Add sldId entry to sldIdLst
        let sld_id_entry = format!(r#"<p:sldId id="{}" r:id="{}"/>"#, next_id, r_id);
        if xml.contains("<p:sldIdLst/>") {
            xml = xml.replace("<p:sldIdLst/>", &format!("<p:sldIdLst>\n    {}\n  </p:sldIdLst>", sld_id_entry));
        } else if xml.contains("<p:sldIdLst>") {
            // Insert before closing tag
            if let Some(pos) = xml.find("</p:sldIdLst>") {
                xml.insert_str(pos, &format!("    {}\n  ", sld_id_entry));
            }
        } else {
            // Add sldIdLst if it doesn't exist
            xml = xml.replace("<p:sldSz", &format!("<p:sldIdLst>\n    {}\n  </p:sldIdLst>\n  <p:sldSz", sld_id_entry));
        }
        
        // Store updated XML while preserving relationships
        let uri = Part::uri(self).clone();
        let old_rels = self.relationships_mut().clone();
        let mut new_part = Self::with_xml(uri, xml)?;
        // Copy relationships to the new part
        for (r_id, rel) in old_rels.iter() {
            new_part.relationships_mut().add(
                r_id.clone(),
                rel.rel_type.clone(),
                rel.target.clone(),
                rel.is_external,
            );
        }
        *self = new_part;
        
        Ok(r_id)
    }

    /// Get the next available slide partname
    pub fn next_slide_partname(&self) -> Result<PackURI> {
        use crate::opc::part::Part;
        // Count existing slides and return next partname
        let slide_count = if let Ok(blob) = Part::blob(self) {
            if let Ok(xml) = String::from_utf8(blob) {
                // Count <p:sldId> tags (not <p:sldIdLst>)
                let pattern = "<p:sldId ";
                xml.matches(pattern).count()
            } else {
                0
            }
        } else {
            0
        };
        
        PackURI::new(&format!("/ppt/slides/slide{}.xml", slide_count + 1))
    }
}

impl Part for PresentationPart {
    fn content_type(&self) -> &str {
        self.base.content_type()
    }

    fn uri(&self) -> &PackURI {
        self.base.uri()
    }

    fn relationships(&self) -> &Relationships {
        self.base.relationships()
    }

    fn relationships_mut(&mut self) -> &mut Relationships {
        self.base.relationships_mut()
    }

    fn blob(&self) -> Result<Vec<u8>> {
        // Use base blob if available, otherwise generate default XML
        let base_blob = self.base.blob()?;
        if !base_blob.is_empty() {
            return Ok(base_blob);
        }
        
        // Generate minimal valid presentation.xml
        let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:sldIdLst/>
  <p:sldSz cx="9144000" cy="6858000"/>
  <p:notesSz cx="6858000" cy="9144000"/>
</p:presentation>"#;
        Ok(xml.as_bytes().to_vec())
    }

    fn to_xml(&self) -> Result<String> {
        // Return the XML blob as a string
        let blob = self.blob()?;
        String::from_utf8(blob)
            .map_err(|e| crate::error::PptError::ValueError(format!("Invalid UTF-8 in XML: {}", e)))
    }

    fn from_xml<R: std::io::Read>(mut reader: R) -> Result<Self> {
        use std::io::Read;
        let mut content = String::new();
        reader.read_to_string(&mut content)
            .map_err(|e| crate::error::PptError::ValueError(format!("Failed to read XML: {}", e)))?;
        
        // Parse URI from XML or use default
        let uri = PackURI::new("/ppt/presentation.xml")?;
        Self::with_xml(uri, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentation_part_new() {
        let part = PresentationPart::new();
        assert!(part.is_ok());
        let part = part.unwrap();
        assert_eq!(Part::uri(&part).as_str(), "/ppt/presentation.xml");
        assert_eq!(Part::content_type(&part), CONTENT_TYPE::PML_PRESENTATION_MAIN);
    }

    #[test]
    fn test_presentation_part_blob() {
        let part = PresentationPart::new().unwrap();
        let blob = part.blob();
        assert!(blob.is_ok());
        
        let blob = blob.unwrap();
        assert!(!blob.is_empty());
        
        // Verify it's valid XML
        let xml = String::from_utf8(blob).unwrap();
        assert!(xml.contains("<?xml"));
        assert!(xml.contains("presentation"));
        assert!(xml.contains("sldIdLst"));
        assert!(xml.contains("sldSz"));
        assert!(xml.contains("notesSz"));
    }

    #[test]
    fn test_presentation_part_relationships() {
        let part = PresentationPart::new().unwrap();
        let rels = part.relationships();
        assert!(rels.is_empty());
        
        let mut part = PresentationPart::new().unwrap();
        let rels_mut = part.relationships_mut();
        assert_eq!(rels_mut.len(), 0);
    }
}

