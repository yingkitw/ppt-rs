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
        namespaces.insert(
            "p".to_string(),
            "http://schemas.openxmlformats.org/presentationml/2006/main".to_string(),
        );
        namespaces.insert(
            "a".to_string(),
            "http://schemas.openxmlformats.org/drawingml/2006/main".to_string(),
        );
        namespaces.insert(
            "r".to_string(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships".to_string(),
        );
        namespaces.insert(
            "rel".to_string(),
            "http://schemas.openxmlformats.org/package/2006/relationships".to_string(),
        );
        namespaces.insert(
            "c".to_string(),
            "http://schemas.openxmlformats.org/drawingml/2006/chart".to_string(),
        );

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
pub const RELATIONSHIPS: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
pub const PACKAGE_RELATIONSHIPS: &str =
    "http://schemas.openxmlformats.org/package/2006/relationships";
pub const CHART: &str = "http://schemas.openxmlformats.org/drawingml/2006/chart";
pub const CORE_PROPERTIES: &str =
    "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_creation() {
        let ns = Namespace::new("p", PML);
        assert_eq!(ns.prefix(), "p");
        assert_eq!(ns.uri(), PML);
    }

    #[test]
    fn test_namespace_clone() {
        let ns1 = Namespace::new("a", DML);
        let ns2 = ns1.clone();
        assert_eq!(ns1.prefix(), ns2.prefix());
        assert_eq!(ns1.uri(), ns2.uri());
    }

    #[test]
    fn test_registry_new() {
        let registry = NamespaceRegistry::new();
        assert!(registry.get("p").is_some());
        assert!(registry.get("a").is_some());
        assert!(registry.get("r").is_some());
        assert!(registry.get("c").is_some());
    }

    #[test]
    fn test_registry_default() {
        let registry = NamespaceRegistry::default();
        assert_eq!(registry.get("p"), Some(PML));
        assert_eq!(registry.get("a"), Some(DML));
    }

    #[test]
    fn test_registry_get() {
        let registry = NamespaceRegistry::new();
        assert_eq!(registry.get("p"), Some(PML));
        assert_eq!(registry.get("a"), Some(DML));
        assert_eq!(registry.get("r"), Some(RELATIONSHIPS));
        assert_eq!(registry.get("c"), Some(CHART));
        assert_eq!(registry.get("unknown"), None);
    }

    #[test]
    fn test_registry_register() {
        let mut registry = NamespaceRegistry::new();
        registry.register("custom", "http://example.com/custom");
        assert_eq!(registry.get("custom"), Some("http://example.com/custom"));
    }

    #[test]
    fn test_registry_all() {
        let registry = NamespaceRegistry::new();
        let all = registry.all();
        assert!(all.len() >= 5);
        assert!(all.contains_key("p"));
        assert!(all.contains_key("a"));
    }

    #[test]
    fn test_namespace_constants() {
        assert!(PML.contains("presentationml"));
        assert!(DML.contains("drawingml"));
        assert!(RELATIONSHIPS.contains("relationships"));
        assert!(CHART.contains("chart"));
        assert!(CORE_PROPERTIES.contains("core-properties"));
    }

    #[test]
    fn test_registry_override() {
        let mut registry = NamespaceRegistry::new();
        let original = registry.get("p").unwrap().to_string();
        registry.register("p", "http://custom.com/pml");
        assert_eq!(registry.get("p"), Some("http://custom.com/pml"));
        assert_ne!(registry.get("p"), Some(original.as_str()));
    }
}
