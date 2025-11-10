//! Generic part container for managing parts and relationships
//!
//! Provides a reusable container for managing collections of parts
//! with relationship tracking and type-based queries.

use crate::error::Result;
use crate::opc::part::Part;
use crate::opc::packuri::PackURI;
use crate::opc::relationships::Relationships;
use linked_hash_map::LinkedHashMap;

/// Generic container for managing parts
pub struct PartContainer {
    parts: LinkedHashMap<PackURI, Box<dyn Part>>,
    relationships: Relationships,
}

impl PartContainer {
    /// Create a new empty part container
    pub fn new() -> Self {
        Self {
            parts: LinkedHashMap::new(),
            relationships: Relationships::new(),
        }
    }

    /// Add a part to the container
    pub fn add_part(&mut self, uri: PackURI, part: Box<dyn Part>) -> Result<()> {
        self.parts.insert(uri, part);
        Ok(())
    }

    /// Get a part by URI
    pub fn get_part(&self, uri: &PackURI) -> Option<&dyn Part> {
        self.parts.get(uri).map(|p| p.as_ref())
    }

    /// Get a mutable reference to a part by URI
    pub fn get_part_mut(&mut self, uri: &PackURI) -> Option<&mut Box<dyn Part>> {
        self.parts.get_mut(uri)
    }

    /// Remove a part from the container
    pub fn remove_part(&mut self, uri: &PackURI) -> Option<Box<dyn Part>> {
        self.parts.remove(uri)
    }

    /// Get all parts
    pub fn parts(&self) -> impl Iterator<Item = (&PackURI, &dyn Part)> {
        self.parts.iter().map(|(uri, part)| (uri, part.as_ref()))
    }

    /// Get all parts (mutable)
    pub fn parts_mut(&mut self) -> impl Iterator<Item = (&PackURI, &mut Box<dyn Part>)> {
        self.parts.iter_mut()
    }

    /// Get parts by relationship type
    pub fn get_parts_by_relationship_type(&self, rel_type: &str) -> Vec<&dyn Part> {
        self.relationships
            .get_by_type(rel_type)
            .iter()
            .filter_map(|rel| {
                // Try to find part by target
                for (uri, part) in &self.parts {
                    if uri.as_str().ends_with(&rel.target) || rel.target.ends_with(uri.as_str()) {
                        return Some(part.as_ref());
                    }
                }
                None
            })
            .collect()
    }

    /// Get the number of parts
    pub fn len(&self) -> usize {
        self.parts.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    /// Get relationships
    pub fn relationships(&self) -> &Relationships {
        &self.relationships
    }

    /// Get mutable relationships
    pub fn relationships_mut(&mut self) -> &mut Relationships {
        &mut self.relationships
    }

    /// Create a relationship between parts
    pub fn create_relationship(
        &mut self,
        rel_type: String,
        target: String,
        external: bool,
    ) -> String {
        self.relationships.get_or_add(rel_type, target, external)
    }

    /// Get relationship by ID
    pub fn get_relationship(&self, r_id: &str) -> Option<&crate::opc::relationships::Relationship> {
        self.relationships.get(r_id)
    }

    /// Remove relationship by ID
    pub fn remove_relationship(&mut self, r_id: &str) {
        self.relationships.remove(r_id);
    }

    /// Clear all parts and relationships
    pub fn clear(&mut self) {
        self.parts.clear();
        *self.relationships_mut() = Relationships::new();
    }
}

impl Default for PartContainer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock part for testing
    struct MockPart {
        uri: PackURI,
        content_type: String,
        relationships: crate::opc::relationships::Relationships,
    }

    impl Part for MockPart {
        fn content_type(&self) -> &str {
            &self.content_type
        }

        fn uri(&self) -> &PackURI {
            &self.uri
        }

        fn relationships(&self) -> &crate::opc::relationships::Relationships {
            &self.relationships
        }

        fn relationships_mut(&mut self) -> &mut crate::opc::relationships::Relationships {
            &mut self.relationships
        }

        fn blob(&self) -> Result<Vec<u8>> {
            Ok(Vec::new())
        }

        fn to_xml(&self) -> Result<String> {
            Ok(String::new())
        }

        fn from_xml<R: std::io::Read>(_reader: R) -> Result<Self> {
            Err(crate::error::PptError::NotImplemented("MockPart::from_xml".to_string()))
        }
    }

    #[test]
    fn test_part_container_new() {
        let container = PartContainer::new();
        assert!(container.is_empty());
        assert_eq!(container.len(), 0);
    }

    #[test]
    fn test_part_container_add_part() -> Result<()> {
        let mut container = PartContainer::new();
        let uri = PackURI::new("/ppt/presentation.xml")?;
        let part = Box::new(MockPart {
            uri: uri.clone(),
            content_type: "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml".to_string(),
            relationships: crate::opc::relationships::Relationships::new(),
        });

        container.add_part(uri.clone(), part)?;
        assert_eq!(container.len(), 1);
        assert!(container.get_part(&uri).is_some());
        Ok(())
    }

    #[test]
    fn test_part_container_remove_part() -> Result<()> {
        let mut container = PartContainer::new();
        let uri = PackURI::new("/ppt/presentation.xml")?;
        let part = Box::new(MockPart {
            uri: uri.clone(),
            content_type: "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml".to_string(),
            relationships: crate::opc::relationships::Relationships::new(),
        });

        container.add_part(uri.clone(), part)?;
        assert_eq!(container.len(), 1);

        let removed = container.remove_part(&uri);
        assert!(removed.is_some());
        assert_eq!(container.len(), 0);
        Ok(())
    }

    #[test]
    fn test_part_container_relationships() -> Result<()> {
        let mut container = PartContainer::new();
        
        let r_id = container.create_relationship(
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide".to_string(),
            "/ppt/slides/slide1.xml".to_string(),
            false,
        );

        assert_eq!(r_id, "rId1");
        assert!(container.get_relationship(&r_id).is_some());
        Ok(())
    }

    #[test]
    fn test_part_container_clear() -> Result<()> {
        let mut container = PartContainer::new();
        let uri = PackURI::new("/ppt/presentation.xml")?;
        let part = Box::new(MockPart {
            uri: uri.clone(),
            content_type: "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml".to_string(),
            relationships: crate::opc::relationships::Relationships::new(),
        });

        container.add_part(uri, part)?;
        container.create_relationship(
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide".to_string(),
            "/ppt/slides/slide1.xml".to_string(),
            false,
        );

        assert_eq!(container.len(), 1);
        container.clear();
        assert_eq!(container.len(), 0);
        Ok(())
    }

    #[test]
    fn test_part_container_default() {
        let container = PartContainer::default();
        assert!(container.is_empty());
    }
}
