//! Part - base type for all package parts

use crate::error::Result;
use crate::opc::packuri::PackURI;
use crate::opc::relationships::Relationships;
use std::io::Read;

/// Base trait for all package parts
pub trait Part: Send + Sync {
    /// Get the content type of this part
    fn content_type(&self) -> &str;

    /// Get the URI of this part
    fn uri(&self) -> &PackURI;

    /// Get relationships for this part
    fn relationships(&self) -> &Relationships;

    /// Get mutable relationships for this part
    fn relationships_mut(&mut self) -> &mut Relationships;

    /// Get the blob (binary content) of this part
    fn blob(&self) -> Result<Vec<u8>>;

    /// Serialize this part to XML
    fn to_xml(&self) -> Result<String>;

    /// Deserialize this part from XML
    fn from_xml<R: Read>(reader: R) -> Result<Self>
    where
        Self: Sized;
}

/// Base part implementation
pub struct BasePart {
    content_type: String,
    uri: PackURI,
    relationships: Relationships,
    blob_data: Option<Vec<u8>>,
}

impl BasePart {
    pub fn new(content_type: &str, uri: PackURI) -> Result<Self> {
        Ok(Self {
            content_type: content_type.to_string(),
            uri,
            relationships: Relationships::new(),
            blob_data: None,
        })
    }

    pub fn with_blob(content_type: &str, uri: PackURI, blob: Vec<u8>) -> Result<Self> {
        Ok(Self {
            content_type: content_type.to_string(),
            uri,
            relationships: Relationships::new(),
            blob_data: Some(blob),
        })
    }

    pub fn set_blob(&mut self, blob: Vec<u8>) {
        self.blob_data = Some(blob);
    }
}

impl Part for BasePart {
    fn content_type(&self) -> &str {
        &self.content_type
    }

    fn uri(&self) -> &PackURI {
        &self.uri
    }

    fn relationships(&self) -> &Relationships {
        &self.relationships
    }

    fn relationships_mut(&mut self) -> &mut Relationships {
        &mut self.relationships
    }

    fn blob(&self) -> Result<Vec<u8>> {
        Ok(self.blob_data.clone().unwrap_or_default())
    }

    fn to_xml(&self) -> Result<String> {
        // For binary parts, return empty XML
        Ok(String::new())
    }

    fn from_xml<R: Read>(_reader: R) -> Result<Self> {
        Err(crate::error::PptError::NotImplemented(
            "BasePart::from_xml not implemented".to_string(),
        ))
    }
}

