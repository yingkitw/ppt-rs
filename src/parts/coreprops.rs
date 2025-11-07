//! Core properties part - Dublin Core metadata

use crate::error::Result;
use crate::opc::constants::CONTENT_TYPE;
use crate::opc::part::{BasePart, Part};
use crate::opc::packuri::PackURI;
use crate::opc::relationships::Relationships;

/// Core properties part - contains Dublin Core metadata
pub struct CorePropertiesPart {
    base: BasePart,
    title: Option<String>,
    subject: Option<String>,
    creator: Option<String>,
    keywords: Option<String>,
    description: Option<String>,
    last_modified_by: Option<String>,
    revision: Option<u32>,
    created: Option<String>,
    modified: Option<String>,
}

impl CorePropertiesPart {
    /// Create a new core properties part
    pub fn new(partname: PackURI) -> Result<Self> {
        let base = BasePart::new(CONTENT_TYPE::OPC_CORE_PROPERTIES, partname)?;
        Ok(Self {
            base,
            title: None,
            subject: None,
            creator: None,
            keywords: None,
            description: None,
            last_modified_by: None,
            revision: Some(1),
            created: None,
            modified: None,
        })
    }

    /// Set the title
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    /// Get the title
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Set the creator
    pub fn set_creator(&mut self, creator: String) {
        self.creator = Some(creator);
    }

    /// Get the creator
    pub fn creator(&self) -> Option<&str> {
        self.creator.as_deref()
    }

    /// Set the subject
    pub fn set_subject(&mut self, subject: String) {
        self.subject = Some(subject);
    }

    /// Get the subject
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }
}

impl Part for CorePropertiesPart {
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
        // TODO: Serialize core properties to XML
        self.base.blob()
    }

    fn to_xml(&self) -> Result<String> {
        // TODO: Generate core properties XML
        Ok(String::new())
    }

    fn from_xml<R: std::io::Read>(_reader: R) -> Result<Self> {
        // TODO: Parse core properties XML
        Self::new(PackURI::new("/docProps/core.xml")?)
    }
}

