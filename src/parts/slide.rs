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

    pub fn with_xml(content_type: &str, uri: PackURI, _xml_content: String) -> Result<Self> {
        let base = BasePart::new(content_type, uri)?;
        // TODO: Parse and store XML element
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
        self.base.blob()
    }

    fn to_xml(&self) -> Result<String> {
        self.base.to_xml()
    }

    fn from_xml<R: std::io::Read>(_reader: R) -> Result<Self> {
        // TODO: Parse XML
        Err(crate::error::PptError::NotImplemented(
            "BaseSlidePart::from_xml".to_string(),
        ))
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

    /// Get the slide ID
    pub fn slide_id(&self) -> Result<u32> {
        // TODO: Get slide ID from presentation part
        Ok(1)
    }

    /// Check if this slide has a notes slide
    pub fn has_notes_slide(&self) -> bool {
        // TODO: Check for notes slide relationship
        false
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

