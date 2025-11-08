//! Presentation part - the main document part

use crate::error::Result;
use crate::opc::constants::CONTENT_TYPE;
use crate::opc::part::{BasePart, Part};
use crate::opc::packuri::PackURI;
use crate::opc::relationships::Relationships;

/// Presentation part - the main document part
pub struct PresentationPart {
    base: BasePart,
    /// Slide ID manager for tracking slides
    slide_id_manager: crate::slide::SlideIdManager,
}

impl PresentationPart {
    /// Create a new presentation part
    pub fn new() -> Result<Self> {
        let uri = PackURI::new("/ppt/presentation.xml")?;
        let base = BasePart::new(CONTENT_TYPE::PML_PRESENTATION_MAIN, uri)?;
        Ok(Self {
            base,
            slide_id_manager: crate::slide::SlideIdManager::new(),
        })
    }

    /// Create a new presentation part with XML content
    pub fn with_xml(uri: PackURI, xml_content: String) -> Result<Self> {
        let mut base = BasePart::new(CONTENT_TYPE::PML_PRESENTATION_MAIN, uri)?;
        // Store XML content as blob
        base.set_blob(xml_content.as_bytes().to_vec());
        Ok(Self {
            base,
            slide_id_manager: crate::slide::SlideIdManager::new(),
        })
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

    /// Get the slide ID manager
    pub fn slide_id_manager(&self) -> &crate::slide::SlideIdManager {
        &self.slide_id_manager
    }

    /// Get mutable slide ID manager
    pub fn slide_id_manager_mut(&mut self) -> &mut crate::slide::SlideIdManager {
        &mut self.slide_id_manager
    }

    /// Generate presentation.xml with slide IDs
    pub fn generate_presentation_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
        xml.push('\n');
        xml.push_str(r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main""#);
        xml.push('\n');
        xml.push_str(r#"                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships""#);
        xml.push('\n');
        xml.push_str(r#"                xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main""#);
        xml.push('\n');
        xml.push_str(r#"                saveSubsetFonts="1" autoCompressPictures="0">"#);
        xml.push('\n');
        
        // Slide master ID list
        xml.push_str(r#"  <p:sldMasterIdLst>"#);
        xml.push('\n');
        xml.push_str(r#"    <p:sldMasterId id="2147483648" r:id="rId1"/>"#);
        xml.push('\n');
        xml.push_str(r#"  </p:sldMasterIdLst>"#);
        xml.push('\n');
        
        // Slide ID list
        xml.push_str(&self.slide_id_manager.to_xml());
        xml.push('\n');
        
        // Slide size
        xml.push_str(r#"  <p:sldSz cx="9144000" cy="6858000" type="screen4x3"/>"#);
        xml.push('\n');
        xml.push_str(r#"  <p:notesSz cx="6858000" cy="9144000"/>"#);
        xml.push('\n');
        
        // Default text style
        xml.push_str(r#"  <p:defaultTextStyle>"#);
        xml.push('\n');
        xml.push_str(r#"    <a:defPPr><a:defRPr lang="en-US"/></a:defPPr>"#);
        xml.push('\n');
        
        // Text levels (1-9)
        for level in 1..=9 {
            let margin = (level - 1) * 457200;
            let size = if level == 1 { 1800 } else { 1800 };
            xml.push_str(&format!(
                r#"    <a:lvl{}pPr marL="{}" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1"><a:defRPr sz="{}" kern="1200"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill><a:latin typeface="+mn-lt"/><a:ea typeface="+mn-ea"/><a:cs typeface="+mn-cs"/></a:defRPr></a:lvl{}pPr>"#,
                level, margin, size, level
            ));
            xml.push('\n');
        }
        
        xml.push_str(r#"  </p:defaultTextStyle>"#);
        xml.push('\n');
        xml.push_str(r#"</p:presentation>"#);
        
        xml
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
        
        // Generate minimal valid presentation.xml with slide master reference
        let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
                saveSubsetFonts="1" autoCompressPictures="0">
  <p:sldMasterIdLst>
    <p:sldMasterId id="2147483648" r:id="rId1"/>
  </p:sldMasterIdLst>
  <p:sldIdLst/>
  <p:sldSz cx="9144000" cy="6858000" type="screen4x3"/>
  <p:notesSz cx="6858000" cy="9144000"/>
  <p:defaultTextStyle>
    <a:defPPr>
      <a:defRPr lang="en-US"/>
    </a:defPPr>
    <a:lvl1pPr marL="0" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
      <a:defRPr sz="1800" kern="1200">
        <a:solidFill>
          <a:schemeClr val="tx1"/>
        </a:solidFill>
        <a:latin typeface="+mn-lt"/>
        <a:ea typeface="+mn-ea"/>
        <a:cs typeface="+mn-cs"/>
      </a:defRPr>
    </a:lvl1pPr>
    <a:lvl2pPr marL="457200" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
      <a:defRPr sz="1800" kern="1200">
        <a:solidFill>
          <a:schemeClr val="tx1"/>
        </a:solidFill>
        <a:latin typeface="+mn-lt"/>
        <a:ea typeface="+mn-ea"/>
        <a:cs typeface="+mn-cs"/>
      </a:defRPr>
    </a:lvl2pPr>
    <a:lvl3pPr marL="914400" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
      <a:defRPr sz="1800" kern="1200">
        <a:solidFill>
          <a:schemeClr val="tx1"/>
        </a:solidFill>
        <a:latin typeface="+mn-lt"/>
        <a:ea typeface="+mn-ea"/>
        <a:cs typeface="+mn-cs"/>
      </a:defRPr>
    </a:lvl3pPr>
    <a:lvl4pPr marL="1371600" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
      <a:defRPr sz="1800" kern="1200">
        <a:solidFill>
          <a:schemeClr val="tx1"/>
        </a:solidFill>
        <a:latin typeface="+mn-lt"/>
        <a:ea typeface="+mn-ea"/>
        <a:cs typeface="+mn-cs"/>
      </a:defRPr>
    </a:lvl4pPr>
    <a:lvl5pPr marL="1828800" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
      <a:defRPr sz="1800" kern="1200">
        <a:solidFill>
          <a:schemeClr val="tx1"/>
        </a:solidFill>
        <a:latin typeface="+mn-lt"/>
        <a:ea typeface="+mn-ea"/>
        <a:cs typeface="+mn-cs"/>
      </a:defRPr>
    </a:lvl5pPr>
    <a:lvl6pPr marL="2286000" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
      <a:defRPr sz="1800" kern="1200">
        <a:solidFill>
          <a:schemeClr val="tx1"/>
        </a:solidFill>
        <a:latin typeface="+mn-lt"/>
        <a:ea typeface="+mn-ea"/>
        <a:cs typeface="+mn-cs"/>
      </a:defRPr>
    </a:lvl6pPr>
    <a:lvl7pPr marL="2743200" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
      <a:defRPr sz="1800" kern="1200">
        <a:solidFill>
          <a:schemeClr val="tx1"/>
        </a:solidFill>
        <a:latin typeface="+mn-lt"/>
        <a:ea typeface="+mn-ea"/>
        <a:cs typeface="+mn-cs"/>
      </a:defRPr>
    </a:lvl7pPr>
    <a:lvl8pPr marL="3200400" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
      <a:defRPr sz="1800" kern="1200">
        <a:solidFill>
          <a:schemeClr val="tx1"/>
        </a:solidFill>
        <a:latin typeface="+mn-lt"/>
        <a:ea typeface="+mn-ea"/>
        <a:cs typeface="+mn-cs"/>
      </a:defRPr>
    </a:lvl8pPr>
    <a:lvl9pPr marL="3657600" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
      <a:defRPr sz="1800" kern="1200">
        <a:solidFill>
          <a:schemeClr val="tx1"/>
        </a:solidFill>
        <a:latin typeface="+mn-lt"/>
        <a:ea typeface="+mn-ea"/>
        <a:cs typeface="+mn-cs"/>
      </a:defRPr>
    </a:lvl9pPr>
  </p:defaultTextStyle>
  <p:notesMasterIdLst/>
  <p:handoutMasterIdLst/>
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

