//! Package - main OPC package handler

use crate::error::{PptError, Result};
use crate::opc::constants::RELATIONSHIP_TYPE;
use crate::opc::part::Part;
use crate::opc::packuri::PackURI;
use crate::opc::relationships::Relationships;
use crate::opc::serialized::PackageReader;
use std::collections::HashMap;
use std::io::{Read, Seek, Write};

/// OPC Package - represents a .pptx file
pub struct Package {
    parts: HashMap<PackURI, Box<dyn Part>>,
    relationships: Relationships,
}

impl Package {
    /// Create a new empty package
    pub fn new() -> Self {
        Self {
            parts: HashMap::new(),
            relationships: Relationships::new(),
        }
    }

    /// Open a package from a reader
    pub fn open<R: Read + Seek>(reader: R) -> Result<Self> {
        let _pkg_reader = PackageReader::new(reader)?;
        
        // TODO: Load parts and relationships from the reader
        // This is a simplified version - full implementation would:
        // 1. Parse [Content_Types].xml to get content types
        // 2. Parse _rels/.rels to get package relationships
        // 3. Load all parts
        // 4. Parse each part's relationships
        
        Ok(Self {
            parts: HashMap::new(),
            relationships: Relationships::new(),
        })
    }

    /// Save the package to a writer
    pub fn save<W: Write + Seek>(&self, _writer: W) -> Result<()> {
        // TODO: Implement proper saving
        // The challenge is that we need to convert &Box<dyn Part> to Box<dyn Part>
        // For now, we'll need a different approach - perhaps using a PartFactory
        // or requiring parts to implement Clone
        Err(PptError::NotImplemented("Package saving - need to handle Part trait objects".to_string()))
    }

    /// Get the main document part
    pub fn main_document_part(&self) -> Result<&dyn Part> {
        self.relationships
            .part_with_reltype(RELATIONSHIP_TYPE::OFFICE_DOCUMENT, &self.parts)
    }

    /// Add a part to the package
    pub fn add_part(&mut self, part: Box<dyn Part>) {
        let uri = part.uri().clone();
        self.parts.insert(uri, part);
    }

    /// Get a part by URI
    pub fn get_part(&self, uri: &PackURI) -> Option<&dyn Part> {
        self.parts.get(uri).map(|p| p.as_ref())
    }

    /// Get relationships
    pub fn relationships(&self) -> &Relationships {
        &self.relationships
    }

    /// Get mutable relationships
    pub fn relationships_mut(&mut self) -> &mut Relationships {
        &mut self.relationships
    }

    /// Iterate over all parts
    pub fn iter_parts(&self) -> impl Iterator<Item = &Box<dyn Part>> {
        self.parts.values()
    }
}


