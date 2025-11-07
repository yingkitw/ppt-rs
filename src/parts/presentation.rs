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
    pub fn with_xml(uri: PackURI, _xml_content: String) -> Result<Self> {
        let base = BasePart::new(CONTENT_TYPE::PML_PRESENTATION_MAIN, uri)?;
        // TODO: Parse and store XML element
        Ok(Self { base })
    }

    /// Get the core properties part
    pub fn core_properties(&self) -> Result<()> {
        // TODO: Implement
        Ok(())
    }

    /// Add a slide to the presentation
    pub fn add_slide(&mut self, _slide_layout_part: &dyn Part) -> Result<String> {
        // TODO: Implement slide addition
        // This should:
        // 1. Create a new SlidePart
        // 2. Add relationship to it
        // 3. Return the rId
        Ok("rId1".to_string())
    }

    /// Get the next available slide partname
    pub fn next_slide_partname(&self) -> Result<PackURI> {
        // TODO: Count existing slides and return next partname
        PackURI::new("/ppt/slides/slide1.xml")
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
        // TODO: Serialize XML element to bytes
        self.base.blob()
    }

    fn to_xml(&self) -> Result<String> {
        // TODO: Serialize to XML
        self.base.to_xml()
    }

    fn from_xml<R: std::io::Read>(_reader: R) -> Result<Self> {
        // TODO: Parse XML and create PresentationPart
        Self::new()
    }
}

