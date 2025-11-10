//! Namespace management for OOXML documents
//!
//! Centralizes namespace definitions and provides utilities for namespace handling.

use std::collections::HashMap;

/// Namespace manager for OOXML documents
#[derive(Debug, Clone)]
pub struct Namespaces {
    namespaces: HashMap<String, String>,
}

impl Namespaces {
    /// Create a new namespace manager
    pub fn new() -> Self {
        Self {
            namespaces: HashMap::new(),
        }
    }

    /// Create a namespace manager with standard OOXML namespaces
    pub fn with_standard() -> Self {
        let mut ns = Self::new();
        ns.add_standard_namespaces();
        ns
    }

    /// Add a namespace
    pub fn add(&mut self, prefix: String, uri: String) {
        self.namespaces.insert(prefix, uri);
    }

    /// Get a namespace URI by prefix
    pub fn get(&self, prefix: &str) -> Option<&str> {
        self.namespaces.get(prefix).map(|s| s.as_str())
    }

    /// Check if a namespace exists
    pub fn contains(&self, prefix: &str) -> bool {
        self.namespaces.contains_key(prefix)
    }

    /// Get all namespaces
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.namespaces.iter()
    }

    /// Add all standard OOXML namespaces
    fn add_standard_namespaces(&mut self) {
        self.add("p".to_string(), PRESENTATION_ML.to_string());
        self.add("a".to_string(), DRAWING_ML.to_string());
        self.add("r".to_string(), OFFICE_DOCUMENT.to_string());
        self.add("rel".to_string(), PACKAGE_RELATIONSHIPS.to_string());
        self.add("ct".to_string(), CONTENT_TYPES.to_string());
    }
}

impl Default for Namespaces {
    fn default() -> Self {
        Self::new()
    }
}

// Standard OOXML namespace URIs
/// PresentationML namespace
pub const PRESENTATION_ML: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";

/// DrawingML namespace
pub const DRAWING_ML: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";

/// Office Document namespace (relationships)
pub const OFFICE_DOCUMENT: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

/// Package relationships namespace
pub const PACKAGE_RELATIONSHIPS: &str =
    "http://schemas.openxmlformats.org/package/2006/relationships";

/// Content types namespace
pub const CONTENT_TYPES: &str = "http://schemas.openxmlformats.org/package/2006/content-types";

/// SpreadsheetML namespace
pub const SPREADSHEET_ML: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";

/// WordprocessingML namespace
pub const WORDPROCESSING_ML: &str =
    "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_new() {
        let ns = Namespaces::new();
        assert_eq!(ns.get("p"), None);
    }

    #[test]
    fn test_namespace_add() {
        let mut ns = Namespaces::new();
        ns.add("p".to_string(), PRESENTATION_ML.to_string());
        assert_eq!(ns.get("p"), Some(PRESENTATION_ML));
    }

    #[test]
    fn test_namespace_with_standard() {
        let ns = Namespaces::with_standard();
        assert_eq!(ns.get("p"), Some(PRESENTATION_ML));
        assert_eq!(ns.get("a"), Some(DRAWING_ML));
        assert_eq!(ns.get("r"), Some(OFFICE_DOCUMENT));
    }

    #[test]
    fn test_namespace_contains() {
        let mut ns = Namespaces::new();
        ns.add("p".to_string(), PRESENTATION_ML.to_string());
        assert!(ns.contains("p"));
        assert!(!ns.contains("a"));
    }

    #[test]
    fn test_namespace_iter() {
        let mut ns = Namespaces::new();
        ns.add("p".to_string(), PRESENTATION_ML.to_string());
        ns.add("a".to_string(), DRAWING_ML.to_string());
        assert_eq!(ns.iter().count(), 2);
    }
}
