//! XML builder utilities for OOXML elements
//!
//! Provides utilities for building XML elements with proper namespace declarations
//! and attribute handling.

use crate::opc::Namespaces;
use std::collections::HashMap;

/// XML element builder for OOXML elements
#[derive(Debug, Clone)]
pub struct XmlBuilder {
    tag_name: String,
    namespaces: Namespaces,
    attributes: HashMap<String, String>,
    children: Vec<String>,
    text_content: Option<String>,
}

impl XmlBuilder {
    /// Create a new XML element builder
    pub fn new(tag_name: &str) -> Self {
        Self {
            tag_name: tag_name.to_string(),
            namespaces: Namespaces::new(),
            attributes: HashMap::new(),
            children: Vec::new(),
            text_content: None,
        }
    }

    /// Create a new XML element builder with standard namespaces
    pub fn with_standard_namespaces(tag_name: &str) -> Self {
        Self {
            tag_name: tag_name.to_string(),
            namespaces: Namespaces::with_standard(),
            attributes: HashMap::new(),
            children: Vec::new(),
            text_content: None,
        }
    }

    /// Add a namespace
    pub fn add_namespace(mut self, prefix: String, uri: String) -> Self {
        self.namespaces.add(prefix, uri);
        self
    }

    /// Add an attribute
    pub fn add_attribute(mut self, name: String, value: String) -> Self {
        self.attributes.insert(name, value);
        self
    }

    /// Add multiple attributes
    pub fn add_attributes(mut self, attrs: HashMap<String, String>) -> Self {
        self.attributes.extend(attrs);
        self
    }

    /// Add a child element
    pub fn add_child(mut self, child: String) -> Self {
        self.children.push(child);
        self
    }

    /// Add multiple children
    pub fn add_children(mut self, children: Vec<String>) -> Self {
        self.children.extend(children);
        self
    }

    /// Set text content
    pub fn set_text(mut self, text: String) -> Self {
        self.text_content = Some(text);
        self
    }

    /// Build the XML element as a string
    pub fn build(&self) -> String {
        let mut xml = String::new();

        // Opening tag
        xml.push('<');
        xml.push_str(&self.tag_name);

        // Add namespace declarations
        for (prefix, uri) in self.namespaces.iter() {
            xml.push_str(&format!(r#" xmlns:{}="{}""#, prefix, uri));
        }

        // Add attributes
        for (name, value) in &self.attributes {
            xml.push_str(&format!(r#" {}="{}""#, name, value));
        }

        // Check if self-closing
        if self.children.is_empty() && self.text_content.is_none() {
            xml.push_str("/>");
            return xml;
        }

        xml.push('>');

        // Add text content
        if let Some(ref text) = self.text_content {
            xml.push_str(text);
        }

        // Add children
        for child in &self.children {
            xml.push_str(child);
        }

        // Closing tag
        xml.push_str("</");
        xml.push_str(&self.tag_name);
        xml.push('>');

        xml
    }

    /// Build with XML declaration
    pub fn build_with_declaration(&self) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>{}"#,
            self.build()
        )
    }
}

/// Helper function to generate slide XML with proper namespaces
pub fn generate_slide_xml(namespaces: &Namespaces) -> String {
    let mut builder = XmlBuilder::new("p:sld");

    // Add namespaces
    if let Some(p_uri) = namespaces.get("p") {
        builder = builder.add_namespace("p".to_string(), p_uri.to_string());
    }
    if let Some(a_uri) = namespaces.get("a") {
        builder = builder.add_namespace("a".to_string(), a_uri.to_string());
    }
    if let Some(r_uri) = namespaces.get("r") {
        builder = builder.add_namespace("r".to_string(), r_uri.to_string());
    }

    // Build the XML
    builder.build_with_declaration()
}

/// Helper function to generate presentation XML with proper namespaces
pub fn generate_presentation_xml(namespaces: &Namespaces) -> String {
    let mut builder = XmlBuilder::new("p:presentation");

    // Add namespaces
    if let Some(a_uri) = namespaces.get("a") {
        builder = builder.add_namespace("a".to_string(), a_uri.to_string());
    }
    if let Some(r_uri) = namespaces.get("r") {
        builder = builder.add_namespace("r".to_string(), r_uri.to_string());
    }
    if let Some(p_uri) = namespaces.get("p") {
        builder = builder.add_namespace("p".to_string(), p_uri.to_string());
    }

    // Add attributes
    let mut attrs = HashMap::new();
    attrs.insert("saveSubsetFonts".to_string(), "1".to_string());
    attrs.insert("autoCompressPictures".to_string(), "0".to_string());

    builder = builder.add_attributes(attrs);

    // Build the XML
    builder.build_with_declaration()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_builder_simple() {
        let builder = XmlBuilder::new("element");
        let xml = builder.build();
        assert_eq!(xml, "<element/>");
    }

    #[test]
    fn test_xml_builder_with_text() {
        let builder = XmlBuilder::new("element").set_text("content".to_string());
        let xml = builder.build();
        assert_eq!(xml, "<element>content</element>");
    }

    #[test]
    fn test_xml_builder_with_attributes() {
        let builder = XmlBuilder::new("element")
            .add_attribute("id".to_string(), "1".to_string())
            .add_attribute("name".to_string(), "test".to_string());
        let xml = builder.build();
        assert!(xml.contains(r#"id="1""#));
        assert!(xml.contains(r#"name="test""#));
    }

    #[test]
    fn test_xml_builder_with_namespaces() {
        let builder = XmlBuilder::new("p:element")
            .add_namespace("p".to_string(), "http://example.com/p".to_string())
            .add_namespace("a".to_string(), "http://example.com/a".to_string());
        let xml = builder.build();
        assert!(xml.contains(r#"xmlns:p="http://example.com/p""#));
        assert!(xml.contains(r#"xmlns:a="http://example.com/a""#));
    }

    #[test]
    fn test_xml_builder_with_declaration() {
        let builder = XmlBuilder::new("element");
        let xml = builder.build_with_declaration();
        assert!(xml.starts_with(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#));
        assert!(xml.contains("<element/>"));
    }

    #[test]
    fn test_generate_slide_xml() {
        let ns = Namespaces::with_standard();
        let xml = generate_slide_xml(&ns);
        assert!(xml.contains(r#"<?xml version="1.0""#));
        assert!(xml.contains("<p:sld"));
        assert!(xml.contains(r#"xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main""#));
        assert!(xml.contains(r#"xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main""#));
    }

    #[test]
    fn test_generate_presentation_xml() {
        let ns = Namespaces::with_standard();
        let xml = generate_presentation_xml(&ns);
        assert!(xml.contains(r#"<?xml version="1.0""#));
        assert!(xml.contains("<p:presentation"));
        assert!(xml.contains(r#"saveSubsetFonts="1""#));
        assert!(xml.contains(r#"autoCompressPictures="0""#));
    }
}
