//! XML element base classes and utilities

use std::rc::Rc;
use std::cell::RefCell;

/// Base class for Office XML elements
pub struct BaseOxmlElement {
    // XML element representation
}

impl BaseOxmlElement {
    /// Create a new BaseOxmlElement
    pub fn new() -> Self {
        BaseOxmlElement {}
    }
}

impl Default for BaseOxmlElement {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents an XML element with child elements
pub struct OxmlElement {
    tag: String,
    attributes: std::collections::HashMap<String, String>,
    children: Vec<Rc<RefCell<OxmlElement>>>,
}

impl OxmlElement {
    /// Create a new OxmlElement
    pub fn new(tag: &str) -> Self {
        OxmlElement {
            tag: tag.to_string(),
            attributes: std::collections::HashMap::new(),
            children: Vec::new(),
        }
    }

    /// Get the tag name
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// Set an attribute
    pub fn set_attribute(&mut self, name: &str, value: &str) {
        self.attributes.insert(name.to_string(), value.to_string());
    }

    /// Get an attribute
    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    /// Add a child element
    pub fn add_child(&mut self, child: Rc<RefCell<OxmlElement>>) {
        self.children.push(child);
    }

    /// Get children
    pub fn children(&self) -> &[Rc<RefCell<OxmlElement>>] {
        &self.children
    }
}
