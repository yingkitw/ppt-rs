//! Relationship handling

use crate::error::{PptError, Result};
use crate::opc::part::Part;
use crate::opc::packuri::PackURI;
use std::collections::HashMap;

/// Relationship between parts
pub struct Relationship {
    pub r_id: String,
    pub rel_type: String,
    pub target: String,
    pub is_external: bool,
    pub target_part: Option<Box<dyn Part>>,
}

/// Collection of relationships
pub struct Relationships {
    relationships: HashMap<String, Relationship>,
    #[allow(dead_code)]
    base_uri: String,
}

impl Relationships {
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
            base_uri: "/".to_string(),
        }
    }

    pub fn with_base_uri(base_uri: String) -> Self {
        Self {
            relationships: HashMap::new(),
            base_uri,
        }
    }

    pub fn add(&mut self, r_id: String, rel_type: String, target: String, is_external: bool) {
        let r_id_clone = r_id.clone();
        self.relationships.insert(
            r_id,
            Relationship {
                r_id: r_id_clone,
                rel_type,
                target,
                is_external,
                target_part: None,
            },
        );
    }

    pub fn get(&self, r_id: &str) -> Option<&Relationship> {
        self.relationships.get(r_id)
    }

    pub fn get_mut(&mut self, r_id: &str) -> Option<&mut Relationship> {
        self.relationships.get_mut(r_id)
    }

    pub fn contains(&self, r_id: &str) -> bool {
        self.relationships.contains_key(r_id)
    }

    pub fn len(&self) -> usize {
        self.relationships.len()
    }

    pub fn is_empty(&self) -> bool {
        self.relationships.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Relationship)> {
        self.relationships.iter()
    }

    pub fn get_or_add(&mut self, rel_type: String, target: String, is_external: bool) -> String {
        // Check if relationship already exists
        for (r_id, rel) in &self.relationships {
            if rel.rel_type == rel_type && rel.target == target && rel.is_external == is_external {
                return r_id.clone();
            }
        }
        // Add new relationship
        let r_id = self.next_r_id();
        self.add(r_id.clone(), rel_type, target, is_external);
        r_id
    }

    pub fn get_or_add_ext_rel(&mut self, rel_type: String, target_ref: String) -> String {
        self.get_or_add(rel_type, target_ref, true)
    }

    pub fn next_r_id(&self) -> String {
        let mut n = self.relationships.len() + 1;
        loop {
            let r_id = format!("rId{}", n);
            if !self.relationships.contains_key(&r_id) {
                return r_id;
            }
            n += 1;
        }
    }

    pub fn remove(&mut self, r_id: &str) {
        self.relationships.remove(r_id);
    }

    pub fn part_with_reltype<'a>(&self, rel_type: &str, parts: &'a HashMap<PackURI, Box<dyn Part>>) -> Result<&'a dyn Part> {
        let mut matches: Vec<&Relationship> = self
            .relationships
            .values()
            .filter(|r| r.rel_type == rel_type && !r.is_external)
            .collect();

        match matches.len() {
            0 => Err(PptError::ValueError(format!(
                "no relationship of type '{}' in collection",
                rel_type
            ))),
            1 => {
                let target_ref = &matches[0].target;
                for (uri, part) in parts {
                    if uri.as_str().ends_with(target_ref) || target_ref.ends_with(uri.as_str()) {
                        return Ok(part.as_ref());
                    }
                }
                Err(PptError::PartNotFound(format!("part for relationship target '{}' not found", target_ref)))
            },
            _ => Err(PptError::ValueError(format!(
                "multiple relationships of type '{}' in collection",
                rel_type
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationships_new() {
        let rels = Relationships::new();
        assert!(rels.is_empty());
        assert_eq!(rels.len(), 0);
    }

    #[test]
    fn test_relationships_add() {
        let mut rels = Relationships::new();
        rels.add(
            "rId1".to_string(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument".to_string(),
            "/ppt/presentation.xml".to_string(),
            false,
        );
        
        assert_eq!(rels.len(), 1);
        assert!(!rels.is_empty());
        assert!(rels.contains("rId1"));
    }

    #[test]
    fn test_relationships_get() {
        let mut rels = Relationships::new();
        rels.add(
            "rId1".to_string(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument".to_string(),
            "/ppt/presentation.xml".to_string(),
            false,
        );
        
        let rel = rels.get("rId1").unwrap();
        assert_eq!(rel.r_id, "rId1");
        assert_eq!(rel.target, "/ppt/presentation.xml");
        assert!(!rel.is_external);
    }

    #[test]
    fn test_relationships_next_r_id() {
        let mut rels = Relationships::new();
        assert_eq!(rels.next_r_id(), "rId1");
        
        rels.add(
            "rId1".to_string(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument".to_string(),
            "/ppt/presentation.xml".to_string(),
            false,
        );
        
        assert_eq!(rels.next_r_id(), "rId2");
    }

    #[test]
    fn test_relationships_remove() {
        let mut rels = Relationships::new();
        rels.add(
            "rId1".to_string(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument".to_string(),
            "/ppt/presentation.xml".to_string(),
            false,
        );
        
        assert_eq!(rels.len(), 1);
        rels.remove("rId1");
        assert_eq!(rels.len(), 0);
        assert!(rels.is_empty());
    }

    #[test]
    fn test_relationships_get_or_add() {
        let mut rels = Relationships::new();
        let r_id1 = rels.get_or_add(
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument".to_string(),
            "/ppt/presentation.xml".to_string(),
            false,
        );
        assert_eq!(r_id1, "rId1");
        
        // Getting the same relationship should return the same r_id
        let r_id2 = rels.get_or_add(
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument".to_string(),
            "/ppt/presentation.xml".to_string(),
            false,
        );
        assert_eq!(r_id1, r_id2);
        assert_eq!(rels.len(), 1);
    }
}
