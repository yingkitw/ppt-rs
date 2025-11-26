//! XML namespace handling

use std::collections::HashMap;

/// Represents an XML namespace
#[derive(Debug, Clone)]
pub struct Namespace {
    prefix: String,
    uri: String,
}

impl Namespace {
    /// Create a new Namespace
    pub fn new(prefix: &str, uri: &str) -> Self {
        Namespace {
            prefix: prefix.to_string(),
            uri: uri.to_string(),
        }
    }

    /// Get the prefix
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Get the URI
    pub fn uri(&self) -> &str {
        &self.uri
    }
}

/// Namespace registry
pub struct NamespaceRegistry {
    namespaces: HashMap<String, String>,
}

impl NamespaceRegistry {
    /// Create a new NamespaceRegistry
    pub fn new() -> Self {
        let mut namespaces = HashMap::new();
        
        // Register standard namespaces
        namespaces.insert("p".to_string(), "http://schemas.openxmlformats.org/presentationml/2006/main".to_string());
        namespaces.insert("a".to_string(), "http://schemas.openxmlformats.org/drawingml/2006/main".to_string());
        namespaces.insert("r".to_string(), "http://schemas.openxmlformats.org/officeDocument/2006/relationships".to_string());
        namespaces.insert("rel".to_string(), "http://schemas.openxmlformats.org/package/2006/relationships".to_string());
        namespaces.insert("c".to_string(), "http://schemas.openxmlformats.org/drawingml/2006/chart".to_string());
        
        NamespaceRegistry { namespaces }
    }

    /// Register a namespace
    pub fn register(&mut self, prefix: &str, uri: &str) {
        self.namespaces.insert(prefix.to_string(), uri.to_string());
    }

    /// Get a namespace URI by prefix
    pub fn get(&self, prefix: &str) -> Option<&str> {
        self.namespaces.get(prefix).map(|s| s.as_str())
    }

    /// Get all namespaces
    pub fn all(&self) -> &HashMap<String, String> {
        &self.namespaces
    }
}

impl Default for NamespaceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Standard namespace constants
pub const PML: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
pub const DML: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
pub const RELATIONSHIPS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
pub const PACKAGE_RELATIONSHIPS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
pub const CHART: &str = "http://schemas.openxmlformats.org/drawingml/2006/chart";
pub const CORE_PROPERTIES: &str = "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
