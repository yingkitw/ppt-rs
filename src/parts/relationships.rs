//! Relationships handling for package parts
//!
//! Manages relationships between parts in a PPTX package.

use crate::exc::PptxError;
use crate::oxml::XmlParser;
use crate::core::ToXml;

/// Relationship types
#[derive(Debug, Clone, PartialEq)]
pub enum RelationshipType {
    OfficeDocument,
    Slide,
    SlideLayout,
    SlideMaster,
    Theme,
    Image,
    Chart,
    CoreProperties,
    ExtendedProperties,
    Custom(String),
}

impl RelationshipType {
    /// Get the relationship type URI
    pub fn uri(&self) -> &str {
        match self {
            RelationshipType::OfficeDocument => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
            RelationshipType::Slide => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
            RelationshipType::SlideLayout => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout",
            RelationshipType::SlideMaster => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster",
            RelationshipType::Theme => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
            RelationshipType::Image => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
            RelationshipType::Chart => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
            RelationshipType::CoreProperties => "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
            RelationshipType::ExtendedProperties => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
            RelationshipType::Custom(uri) => uri,
        }
    }

    /// Parse from URI string
    pub fn from_uri(uri: &str) -> Self {
        if uri.contains("/slide") && !uri.contains("Layout") && !uri.contains("Master") {
            RelationshipType::Slide
        } else if uri.contains("/slideLayout") {
            RelationshipType::SlideLayout
        } else if uri.contains("/slideMaster") {
            RelationshipType::SlideMaster
        } else if uri.contains("/theme") {
            RelationshipType::Theme
        } else if uri.contains("/image") {
            RelationshipType::Image
        } else if uri.contains("/chart") {
            RelationshipType::Chart
        } else if uri.contains("/officeDocument") {
            RelationshipType::OfficeDocument
        } else if uri.contains("core-properties") {
            RelationshipType::CoreProperties
        } else if uri.contains("extended-properties") {
            RelationshipType::ExtendedProperties
        } else {
            RelationshipType::Custom(uri.to_string())
        }
    }
}

/// A single relationship
#[derive(Debug, Clone)]
pub struct Relationship {
    pub id: String,
    pub rel_type: RelationshipType,
    pub target: String,
}

impl Relationship {
    /// Create a new relationship
    pub fn new(id: &str, rel_type: RelationshipType, target: &str) -> Self {
        Relationship {
            id: id.to_string(),
            rel_type,
            target: target.to_string(),
        }
    }

    /// Generate XML for this relationship
    pub fn to_xml(&self) -> String {
        format!(
            r#"<Relationship Id="{}" Type="{}" Target="{}"/>"#,
            self.id, self.rel_type.uri(), self.target
        )
    }
}

impl ToXml for Relationship {
    fn to_xml(&self) -> String {
        Relationship::to_xml(self)
    }
}

/// Collection of relationships
#[derive(Debug, Clone, Default)]
pub struct Relationships {
    relationships: Vec<Relationship>,
    next_id: u32,
}

impl Relationships {
    /// Create new empty relationships
    pub fn new() -> Self {
        Relationships {
            relationships: Vec::new(),
            next_id: 1,
        }
    }

    /// Add a relationship and return its ID
    pub fn add(&mut self, rel_type: RelationshipType, target: &str) -> String {
        let id = format!("rId{}", self.next_id);
        self.next_id += 1;
        self.relationships.push(Relationship::new(&id, rel_type, target));
        id
    }

    /// Add a relationship with specific ID
    pub fn add_with_id(&mut self, id: &str, rel_type: RelationshipType, target: &str) {
        self.relationships.push(Relationship::new(id, rel_type, target));
        // Update next_id if needed
        if let Some(num) = id.strip_prefix("rId").and_then(|s| s.parse::<u32>().ok()) {
            if num >= self.next_id {
                self.next_id = num + 1;
            }
        }
    }

    /// Get relationship by ID
    pub fn get(&self, id: &str) -> Option<&Relationship> {
        self.relationships.iter().find(|r| r.id == id)
    }

    /// Get all relationships of a type
    pub fn get_by_type(&self, rel_type: &RelationshipType) -> Vec<&Relationship> {
        self.relationships.iter().filter(|r| &r.rel_type == rel_type).collect()
    }

    /// Get all relationships
    pub fn all(&self) -> &[Relationship] {
        &self.relationships
    }

    /// Get count
    pub fn len(&self) -> usize {
        self.relationships.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.relationships.is_empty()
    }

    /// Generate XML for all relationships
    pub fn to_xml(&self) -> String {
        let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">"#);
        
        for rel in &self.relationships {
            xml.push_str("\n    ");
            xml.push_str(&rel.to_xml());
        }
        
        xml.push_str("\n</Relationships>");
        xml
    }

}

impl ToXml for Relationships {
    fn to_xml(&self) -> String {
        Relationships::to_xml(self)
    }
}

impl Relationships {
    /// Parse relationships from XML
    pub fn from_xml(xml: &str) -> Result<Self, PptxError> {
        let root = XmlParser::parse_str(xml)?;
        let mut rels = Relationships::new();

        for rel_elem in root.find_all("Relationship") {
            if let (Some(id), Some(rel_type), Some(target)) = (
                rel_elem.attr("Id"),
                rel_elem.attr("Type"),
                rel_elem.attr("Target"),
            ) {
                rels.add_with_id(id, RelationshipType::from_uri(rel_type), target);
            }
        }

        Ok(rels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_type_uri() {
        assert!(RelationshipType::Slide.uri().contains("/slide"));
        assert!(RelationshipType::Image.uri().contains("/image"));
    }

    #[test]
    fn test_relationship_type_from_uri() {
        let slide = RelationshipType::from_uri("http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide");
        assert_eq!(slide, RelationshipType::Slide);
    }

    #[test]
    fn test_relationships_add() {
        let mut rels = Relationships::new();
        let id1 = rels.add(RelationshipType::Slide, "slides/slide1.xml");
        let id2 = rels.add(RelationshipType::Slide, "slides/slide2.xml");
        
        assert_eq!(id1, "rId1");
        assert_eq!(id2, "rId2");
        assert_eq!(rels.len(), 2);
    }

    #[test]
    fn test_relationships_to_xml() {
        let mut rels = Relationships::new();
        rels.add(RelationshipType::SlideMaster, "slideMasters/slideMaster1.xml");
        rels.add(RelationshipType::Theme, "theme/theme1.xml");
        
        let xml = rels.to_xml();
        assert!(xml.contains("rId1"));
        assert!(xml.contains("slideMaster"));
        assert!(xml.contains("theme"));
    }

    #[test]
    fn test_relationships_from_xml() {
        let xml = r#"<?xml version="1.0"?>
        <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
            <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/>
            <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>
        </Relationships>"#;
        
        let rels = Relationships::from_xml(xml).unwrap();
        assert_eq!(rels.len(), 2);
        assert!(rels.get("rId1").is_some());
    }
}
