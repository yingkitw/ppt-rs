//! XML element base classes and utilities for parsing Office XML
//!
//! Provides XML parsing using xml-rs and a DOM-like structure for OXML elements.

use crate::exc::PptxError;
use std::collections::HashMap;
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

/// Represents an XML element with attributes and children
#[derive(Debug, Clone)]
pub struct XmlElement {
    /// Element tag name (with namespace prefix if present)
    pub tag: String,
    /// Local name without namespace prefix
    pub local_name: String,
    /// Namespace URI
    pub namespace: Option<String>,
    /// Element attributes
    pub attributes: HashMap<String, String>,
    /// Child elements
    pub children: Vec<XmlElement>,
    /// Text content
    pub text: String,
}

impl XmlElement {
    /// Create a new XML element
    pub fn new(tag: &str) -> Self {
        let local_name = tag.split(':').last().unwrap_or(tag).to_string();
        XmlElement {
            tag: tag.to_string(),
            local_name,
            namespace: None,
            attributes: HashMap::new(),
            children: Vec::new(),
            text: String::new(),
        }
    }

    /// Create element with namespace
    pub fn with_namespace(tag: &str, namespace: &str) -> Self {
        let local_name = tag.split(':').last().unwrap_or(tag).to_string();
        XmlElement {
            tag: tag.to_string(),
            local_name,
            namespace: Some(namespace.to_string()),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: String::new(),
        }
    }

    /// Get attribute value
    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    /// Set attribute
    pub fn set_attr(&mut self, name: &str, value: &str) {
        self.attributes.insert(name.to_string(), value.to_string());
    }

    /// Add child element
    pub fn add_child(&mut self, child: XmlElement) {
        self.children.push(child);
    }

    /// Find first child by local name
    pub fn find(&self, local_name: &str) -> Option<&XmlElement> {
        self.children.iter().find(|c| c.local_name == local_name)
    }

    /// Find all children by local name
    pub fn find_all(&self, local_name: &str) -> Vec<&XmlElement> {
        self.children
            .iter()
            .filter(|c| c.local_name == local_name)
            .collect()
    }

    /// Find first descendant by local name (recursive)
    pub fn find_descendant(&self, local_name: &str) -> Option<&XmlElement> {
        for child in &self.children {
            if child.local_name == local_name {
                return Some(child);
            }
            if let Some(found) = child.find_descendant(local_name) {
                return Some(found);
            }
        }
        None
    }

    /// Find all descendants by local name (recursive)
    pub fn find_all_descendants(&self, local_name: &str) -> Vec<&XmlElement> {
        let mut results = Vec::new();
        self.collect_descendants(local_name, &mut results);
        results
    }

    fn collect_descendants<'a>(&'a self, local_name: &str, results: &mut Vec<&'a XmlElement>) {
        for child in &self.children {
            if child.local_name == local_name {
                results.push(child);
            }
            child.collect_descendants(local_name, results);
        }
    }

    /// Get all text content recursively
    pub fn text_content(&self) -> String {
        let mut result = self.text.clone();
        for child in &self.children {
            result.push_str(&child.text_content());
        }
        result
    }

    /// Check if element has specific local name
    pub fn is(&self, local_name: &str) -> bool {
        self.local_name == local_name
    }
}

/// XML Parser for Office XML documents
pub struct XmlParser;

impl XmlParser {
    /// Parse XML from a string
    pub fn parse_str(xml: &str) -> Result<XmlElement, PptxError> {
        Self::parse(xml.as_bytes())
    }

    /// Parse XML from a reader
    pub fn parse<R: Read>(reader: R) -> Result<XmlElement, PptxError> {
        let parser = EventReader::new(reader);
        let mut stack: Vec<XmlElement> = Vec::new();
        let mut root: Option<XmlElement> = None;

        for event in parser {
            match event {
                Ok(XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => {
                    let tag = if let Some(ref prefix) = name.prefix {
                        format!("{}:{}", prefix, name.local_name)
                    } else {
                        name.local_name.clone()
                    };

                    let mut element = XmlElement::new(&tag);
                    element.namespace = namespace
                        .get(&name.prefix.clone().unwrap_or_default())
                        .map(|s| s.to_string());

                    // Add attributes
                    for attr in attributes {
                        let attr_name = if let Some(ref prefix) = attr.name.prefix {
                            format!("{}:{}", prefix, attr.name.local_name)
                        } else {
                            attr.name.local_name
                        };
                        element.set_attr(&attr_name, &attr.value);
                    }

                    stack.push(element);
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    if let Some(element) = stack.pop() {
                        if let Some(parent) = stack.last_mut() {
                            parent.add_child(element);
                        } else {
                            root = Some(element);
                        }
                    }
                }
                Ok(XmlEvent::Characters(text)) => {
                    if let Some(current) = stack.last_mut() {
                        current.text.push_str(&text);
                    }
                }
                Ok(XmlEvent::CData(text)) => {
                    if let Some(current) = stack.last_mut() {
                        current.text.push_str(&text);
                    }
                }
                Err(e) => {
                    return Err(PptxError::XmlParse(e.to_string()));
                }
                _ => {}
            }
        }

        root.ok_or_else(|| PptxError::XmlParse("Empty XML document".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_xml() {
        let xml = r#"<?xml version="1.0"?><root><child attr="value">text</child></root>"#;
        let result = XmlParser::parse_str(xml);
        assert!(result.is_ok());

        let root = result.unwrap();
        assert_eq!(root.local_name, "root");
        assert_eq!(root.children.len(), 1);

        let child = &root.children[0];
        assert_eq!(child.local_name, "child");
        assert_eq!(child.attr("attr"), Some("value"));
        assert_eq!(child.text, "text");
    }

    #[test]
    fn test_parse_namespaced_xml() {
        let xml = r#"<?xml version="1.0"?>
        <p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
            <p:cSld>
                <p:spTree/>
            </p:cSld>
        </p:sld>"#;

        let result = XmlParser::parse_str(xml);
        assert!(result.is_ok());

        let root = result.unwrap();
        assert_eq!(root.local_name, "sld");
        assert!(root.find("cSld").is_some());
    }

    #[test]
    fn test_find_descendants() {
        let xml = r#"<?xml version="1.0"?>
        <root>
            <level1>
                <target>found1</target>
            </level1>
            <level1>
                <level2>
                    <target>found2</target>
                </level2>
            </level1>
        </root>"#;

        let root = XmlParser::parse_str(xml).unwrap();
        let targets = root.find_all_descendants("target");
        assert_eq!(targets.len(), 2);
        assert_eq!(targets[0].text, "found1");
        assert_eq!(targets[1].text, "found2");
    }

    #[test]
    fn test_text_content() {
        let xml = r#"<?xml version="1.0"?><p>Hello <b>World</b></p>"#;
        let root = XmlParser::parse_str(xml).unwrap();
        assert_eq!(root.text_content(), "Hello World");
    }
}
