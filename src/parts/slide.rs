//! Slide-related parts

use crate::error::Result;
use crate::opc::constants::CONTENT_TYPE;
use crate::opc::part::{BasePart, Part};
use crate::opc::packuri::PackURI;
use crate::opc::relationships::Relationships;

/// Base slide part - common functionality for all slide types
pub struct BaseSlidePart {
    base: BasePart,
}

impl BaseSlidePart {
    pub fn new(content_type: &str, uri: PackURI) -> Result<Self> {
        let base = BasePart::new(content_type, uri)?;
        Ok(Self { base })
    }

    pub fn with_xml(content_type: &str, uri: PackURI, xml_content: String) -> Result<Self> {
        let mut base = BasePart::new(content_type, uri)?;
        // Store XML content as blob
        base.set_blob(xml_content.as_bytes().to_vec());
        Ok(Self { base })
    }
}

impl Part for BaseSlidePart {
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
        let base_blob = self.base.blob()?;
        if !base_blob.is_empty() {
            return Ok(base_blob);
        }
        
        // Generate minimal valid slide XML if empty
        let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
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
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>"#;
        Ok(xml.as_bytes().to_vec())
    }

    fn to_xml(&self) -> Result<String> {
        let blob = self.blob()?;
        String::from_utf8(blob)
            .map_err(|e| crate::error::PptError::ValueError(format!("Invalid UTF-8 in XML: {}", e)))
    }

    fn from_xml<R: std::io::Read>(mut reader: R) -> Result<Self> {
        use std::io::Read;
        let mut content = String::new();
        reader.read_to_string(&mut content)
            .map_err(|e| crate::error::PptError::ValueError(format!("Failed to read XML: {}", e)))?;
        
        // Use default URI and content type - caller should set proper values
        let uri = PackURI::new("/ppt/slides/slide1.xml")?;
        let content_type = CONTENT_TYPE::PML_SLIDE;
        Self::with_xml(content_type, uri, content)
    }
}

/// Slide part
pub struct SlidePart {
    base: BaseSlidePart,
}

impl SlidePart {
    /// Create a new slide part
    pub fn new(partname: PackURI, _slide_layout_part: &dyn Part) -> Result<Self> {
        let base = BaseSlidePart::new(CONTENT_TYPE::PML_SLIDE, partname)?;
        Ok(Self { base })
    }
    
    /// Create a slide part from BaseSlidePart
    pub fn from_base(base: BaseSlidePart) -> Self {
        Self { base }
    }
    
    /// Update the slide part XML
    pub fn update_xml(&mut self, xml: String) -> Result<()> {
        let uri = Part::uri(self).clone();
        self.base = BaseSlidePart::with_xml(CONTENT_TYPE::PML_SLIDE, uri, xml)?;
        Ok(())
    }

    /// Get the slide ID
    pub fn slide_id(&self) -> Result<u32> {
        use crate::opc::part::Part;
        // Parse presentation.xml to find slide ID for this slide
        // This is a simplified implementation - in full version would need access to presentation part
        // For now, extract from URI (e.g., /ppt/slides/slide1.xml -> 1)
        let uri_str = Part::uri(self).as_str();
        if let Some(slide_num_str) = uri_str.split("slide").nth(1).and_then(|s| s.split('.').next()) {
            if let Ok(id) = slide_num_str.parse::<u32>() {
                // Slide IDs start at 256, so add 255 to slide number
                return Ok(255 + id);
            }
        }
        Ok(256) // Default to first slide ID
    }

    /// Check if this slide has a notes slide
    pub fn has_notes_slide(&self) -> bool {
        use crate::opc::constants::RELATIONSHIP_TYPE;
        use crate::opc::part::Part;
        // Check for notes slide relationship
        let rels = Part::relationships(self);
        rels.iter().any(|(_, r)| r.rel_type == RELATIONSHIP_TYPE::NOTES_SLIDE)
    }
}

impl Part for SlidePart {
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
        self.base.blob()
    }

    fn to_xml(&self) -> Result<String> {
        self.base.to_xml()
    }

    fn from_xml<R: std::io::Read>(reader: R) -> Result<Self> {
        let base = BaseSlidePart::from_xml(reader)?;
        Ok(Self { base })
    }
}

/// Slide layout part
pub struct SlideLayoutPart {
    base: BaseSlidePart,
}

impl SlideLayoutPart {
    /// Create a new slide layout part
    pub fn new(partname: PackURI) -> Result<Self> {
        let base = BaseSlidePart::new(CONTENT_TYPE::PML_SLIDE_LAYOUT, partname)?;
        Ok(Self { base })
    }
}

impl Part for SlideLayoutPart {
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
        self.base.blob()
    }

    fn to_xml(&self) -> Result<String> {
        self.base.to_xml()
    }

    fn from_xml<R: std::io::Read>(reader: R) -> Result<Self> {
        let base = BaseSlidePart::from_xml(reader)?;
        Ok(Self { base })
    }
}

/// Slide master part
pub struct SlideMasterPart {
    base: BaseSlidePart,
}

impl SlideMasterPart {
    /// Create a new slide master part
    pub fn new(partname: PackURI) -> Result<Self> {
        let base = BaseSlidePart::new(CONTENT_TYPE::PML_SLIDE_MASTER, partname)?;
        Ok(Self { base })
    }
}

impl Part for SlideMasterPart {
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
        self.base.blob()
    }

    fn to_xml(&self) -> Result<String> {
        self.base.to_xml()
    }

    fn from_xml<R: std::io::Read>(reader: R) -> Result<Self> {
        let base = BaseSlidePart::from_xml(reader)?;
        Ok(Self { base })
    }
}

/// Notes master part
pub struct NotesMasterPart {
    base: BaseSlidePart,
}

impl NotesMasterPart {
    /// Create a new notes master part
    pub fn new(partname: PackURI) -> Result<Self> {
        let base = BaseSlidePart::new(CONTENT_TYPE::PML_NOTES_MASTER, partname)?;
        Ok(Self { base })
    }

    /// Create a default notes master part
    pub fn create_default() -> Result<Self> {
        Self::new(PackURI::new("/ppt/notesMasters/notesMaster1.xml")?)
    }
}

impl Part for NotesMasterPart {
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
        self.base.blob()
    }

    fn to_xml(&self) -> Result<String> {
        self.base.to_xml()
    }

    fn from_xml<R: std::io::Read>(reader: R) -> Result<Self> {
        let base = BaseSlidePart::from_xml(reader)?;
        Ok(Self { base })
    }
}

/// Notes slide part
pub struct NotesSlidePart {
    base: BaseSlidePart,
}

impl NotesSlidePart {
    /// Create a new notes slide part
    pub fn new(partname: PackURI, _slide_part: &dyn Part) -> Result<Self> {
        let base = BaseSlidePart::new(CONTENT_TYPE::PML_NOTES_SLIDE, partname)?;
        Ok(Self { base })
    }
}

impl Part for NotesSlidePart {
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
        self.base.blob()
    }

    fn to_xml(&self) -> Result<String> {
        self.base.to_xml()
    }

    fn from_xml<R: std::io::Read>(reader: R) -> Result<Self> {
        let base = BaseSlidePart::from_xml(reader)?;
        Ok(Self { base })
    }
}

