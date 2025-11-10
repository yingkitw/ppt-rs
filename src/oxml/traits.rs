//! XML traits for type-safe OOXML element handling
//!
//! Provides traits for compile-time element metadata and serialization/deserialization.

use crate::error::Result;

/// Element type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenXmlElementType {
    /// Leaf element (plain text or CDATA)
    Leaf,
    /// Internal node element
    Node,
    /// Root element of a part
    Root,
}

/// Marker trait for leaf elements (plain text)
pub trait OpenXmlLeafElement {}

/// Marker trait for node elements (internal XML elements)
pub trait OpenXmlNodeElement {}

/// Marker trait for root elements
pub trait OpenXmlRootElement {}

/// Static information about an OpenXML element
///
/// Provides compile-time metadata about XML elements.
pub trait OpenXmlElementInfo: Sized {
    /// Get the XML tag name
    fn tag_name() -> &'static str;

    /// Get the element type
    fn element_type() -> OpenXmlElementType {
        OpenXmlElementType::Node
    }

    /// Check if this is a leaf text element
    fn is_leaf_text_element() -> bool {
        matches!(Self::element_type(), OpenXmlElementType::Leaf)
    }

    /// Check if this is a root element
    fn is_root_element() -> bool {
        matches!(Self::element_type(), OpenXmlElementType::Root)
    }

    /// Check if the element has a tag name
    fn have_tag_name() -> bool {
        !matches!(Self::element_type(), OpenXmlElementType::Leaf)
    }

    /// Check if the element can have namespace declarations
    fn can_have_namespace_declarations() -> bool {
        !matches!(Self::element_type(), OpenXmlElementType::Leaf)
    }

    /// Check if the element can have attributes
    fn can_have_attributes() -> bool {
        !matches!(Self::element_type(), OpenXmlElementType::Leaf)
    }

    /// Get the namespace URI for this element
    fn namespace_uri() -> Option<&'static str> {
        None
    }

    /// Get the namespace prefix for this element
    fn namespace_prefix() -> Option<&'static str> {
        None
    }
}

/// Custom XML serialization for OOXML elements
pub trait OpenXmlSerialize {
    /// Serialize to XML string
    fn to_xml(&self) -> Result<String>;

    /// Serialize to XML bytes
    fn to_xml_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.to_xml()?.into_bytes())
    }
}

/// Custom XML deserialization for OOXML elements
pub trait OpenXmlDeserialize: Sized {
    /// Deserialize from XML string
    fn from_xml(xml: &str) -> Result<Self>;

    /// Deserialize from XML bytes
    fn from_xml_bytes(xml: &[u8]) -> Result<Self> {
        let xml_str = String::from_utf8(xml.to_vec())
            .map_err(|e| crate::error::PptError::ValueError(format!("Invalid UTF-8: {}", e)))?;
        Self::from_xml(&xml_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestLeafElement;
    impl OpenXmlLeafElement for TestLeafElement {}
    impl OpenXmlElementInfo for TestLeafElement {
        fn tag_name() -> &'static str {
            "text"
        }

        fn element_type() -> OpenXmlElementType {
            OpenXmlElementType::Leaf
        }
    }

    struct TestNodeElement;
    impl OpenXmlNodeElement for TestNodeElement {}
    impl OpenXmlElementInfo for TestNodeElement {
        fn tag_name() -> &'static str {
            "node"
        }

        fn element_type() -> OpenXmlElementType {
            OpenXmlElementType::Node
        }
    }

    struct TestRootElement;
    impl OpenXmlRootElement for TestRootElement {}
    impl OpenXmlElementInfo for TestRootElement {
        fn tag_name() -> &'static str {
            "root"
        }

        fn element_type() -> OpenXmlElementType {
            OpenXmlElementType::Root
        }
    }

    #[test]
    fn test_leaf_element() {
        assert_eq!(TestLeafElement::tag_name(), "text");
        assert_eq!(TestLeafElement::element_type(), OpenXmlElementType::Leaf);
        assert!(TestLeafElement::is_leaf_text_element());
        assert!(!TestLeafElement::have_tag_name());
        assert!(!TestLeafElement::can_have_attributes());
    }

    #[test]
    fn test_node_element() {
        assert_eq!(TestNodeElement::tag_name(), "node");
        assert_eq!(TestNodeElement::element_type(), OpenXmlElementType::Node);
        assert!(!TestNodeElement::is_leaf_text_element());
        assert!(TestNodeElement::have_tag_name());
        assert!(TestNodeElement::can_have_attributes());
    }

    #[test]
    fn test_root_element() {
        assert_eq!(TestRootElement::tag_name(), "root");
        assert_eq!(TestRootElement::element_type(), OpenXmlElementType::Root);
        assert!(TestRootElement::is_root_element());
        assert!(TestRootElement::have_tag_name());
        assert!(TestRootElement::can_have_namespace_declarations());
    }
}
