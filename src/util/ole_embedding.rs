//! OLE Embedding Support
//!
//! This module provides OLE (Object Linking and Embedding) support for presentations:
//! - Embed external objects (Excel, Word, etc.)
//! - Object type management
//! - Object relationships and references
//! - Embedded object metadata

use crate::error::Result;
use std::collections::HashMap;

/// OLE object type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OLEObjectType {
    /// Excel worksheet
    ExcelWorksheet,
    /// Word document
    WordDocument,
    /// PowerPoint presentation
    PowerPointPresentation,
    /// PDF document
    PDFDocument,
    /// Generic object
    Generic,
}

impl OLEObjectType {
    /// Get the MIME type
    pub fn mime_type(&self) -> &str {
        match self {
            OLEObjectType::ExcelWorksheet => "application/vnd.ms-excel",
            OLEObjectType::WordDocument => "application/msword",
            OLEObjectType::PowerPointPresentation => "application/vnd.ms-powerpoint",
            OLEObjectType::PDFDocument => "application/pdf",
            OLEObjectType::Generic => "application/octet-stream",
        }
    }

    /// Get the file extension
    pub fn extension(&self) -> &str {
        match self {
            OLEObjectType::ExcelWorksheet => "xlsx",
            OLEObjectType::WordDocument => "docx",
            OLEObjectType::PowerPointPresentation => "pptx",
            OLEObjectType::PDFDocument => "pdf",
            OLEObjectType::Generic => "bin",
        }
    }

    /// Get the object name
    pub fn name(&self) -> &str {
        match self {
            OLEObjectType::ExcelWorksheet => "Excel Worksheet",
            OLEObjectType::WordDocument => "Word Document",
            OLEObjectType::PowerPointPresentation => "PowerPoint Presentation",
            OLEObjectType::PDFDocument => "PDF Document",
            OLEObjectType::Generic => "Generic Object",
        }
    }
}

/// OLE embedded object
#[derive(Debug, Clone)]
pub struct OLEEmbeddedObject {
    /// Object ID
    id: String,
    /// Object type
    object_type: OLEObjectType,
    /// Object data (binary)
    data: Vec<u8>,
    /// Object name
    name: String,
    /// Object description
    description: Option<String>,
    /// Metadata
    metadata: HashMap<String, String>,
}

impl OLEEmbeddedObject {
    /// Create a new OLE embedded object
    pub fn new(id: impl Into<String>, object_type: OLEObjectType, data: Vec<u8>) -> Self {
        Self {
            id: id.into(),
            object_type,
            data,
            name: object_type.name().to_string(),
            description: None,
            metadata: HashMap::new(),
        }
    }

    /// Get object ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get object type
    pub fn object_type(&self) -> OLEObjectType {
        self.object_type
    }

    /// Get object data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get object data size
    pub fn data_size(&self) -> usize {
        self.data.len()
    }

    /// Get object name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set object name
    pub fn set_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Get object description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set object description
    pub fn set_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add metadata
    pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }

    /// Get all metadata
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Generate XML for OLE object
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<p:oleObj");
        xml.push_str(&format!(" r:id=\"{}\"", self.id));
        xml.push_str(&format!(" name=\"{}\"", self.name));
        xml.push_str(&format!(" type=\"{}\"", self.object_type.mime_type()));
        xml.push_str("/>");
        xml
    }
}

/// OLE object manager
#[derive(Debug, Clone)]
pub struct OLEObjectManager {
    /// Embedded objects
    objects: HashMap<String, OLEEmbeddedObject>,
    /// Object counter
    counter: u32,
}

impl Default for OLEObjectManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OLEObjectManager {
    /// Create a new OLE object manager
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            counter: 1,
        }
    }

    /// Add an embedded object
    pub fn add_object(&mut self, object: OLEEmbeddedObject) -> String {
        let id = object.id().to_string();
        self.objects.insert(id.clone(), object);
        id
    }

    /// Create and add a new embedded object
    pub fn create_object(
        &mut self,
        object_type: OLEObjectType,
        data: Vec<u8>,
    ) -> String {
        let id = format!("ole{}", self.counter);
        self.counter += 1;
        let object = OLEEmbeddedObject::new(&id, object_type, data);
        self.add_object(object)
    }

    /// Get an embedded object
    pub fn get_object(&self, id: &str) -> Option<&OLEEmbeddedObject> {
        self.objects.get(id)
    }

    /// Get mutable embedded object
    pub fn get_object_mut(&mut self, id: &str) -> Option<&mut OLEEmbeddedObject> {
        self.objects.get_mut(id)
    }

    /// Get all objects
    pub fn all_objects(&self) -> Vec<&OLEEmbeddedObject> {
        self.objects.values().collect()
    }

    /// Get number of objects
    pub fn count(&self) -> usize {
        self.objects.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Remove an object
    pub fn remove_object(&mut self, id: &str) -> Option<OLEEmbeddedObject> {
        self.objects.remove(id)
    }

    /// Clear all objects
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// Generate XML for all objects
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<p:oleObjList>");
        for obj in self.objects.values() {
            xml.push_str(&obj.to_xml());
        }
        xml.push_str("</p:oleObjList>");
        xml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ole_object_type_excel() {
        let ot = OLEObjectType::ExcelWorksheet;
        assert_eq!(ot.mime_type(), "application/vnd.ms-excel");
        assert_eq!(ot.extension(), "xlsx");
        assert_eq!(ot.name(), "Excel Worksheet");
    }

    #[test]
    fn test_ole_object_type_word() {
        let ot = OLEObjectType::WordDocument;
        assert_eq!(ot.mime_type(), "application/msword");
        assert_eq!(ot.extension(), "docx");
    }

    #[test]
    fn test_ole_object_type_pdf() {
        let ot = OLEObjectType::PDFDocument;
        assert_eq!(ot.mime_type(), "application/pdf");
        assert_eq!(ot.extension(), "pdf");
    }

    #[test]
    fn test_ole_embedded_object_new() {
        let data = vec![1, 2, 3, 4, 5];
        let obj = OLEEmbeddedObject::new("ole1", OLEObjectType::ExcelWorksheet, data.clone());
        assert_eq!(obj.id(), "ole1");
        assert_eq!(obj.object_type(), OLEObjectType::ExcelWorksheet);
        assert_eq!(obj.data_size(), 5);
    }

    #[test]
    fn test_ole_embedded_object_name() {
        let obj = OLEEmbeddedObject::new("ole1", OLEObjectType::WordDocument, vec![])
            .set_name("My Document");
        assert_eq!(obj.name(), "My Document");
    }

    #[test]
    fn test_ole_embedded_object_description() {
        let obj = OLEEmbeddedObject::new("ole1", OLEObjectType::ExcelWorksheet, vec![])
            .set_description("Financial Data");
        assert_eq!(obj.description(), Some("Financial Data"));
    }

    #[test]
    fn test_ole_embedded_object_metadata() {
        let obj = OLEEmbeddedObject::new("ole1", OLEObjectType::PDFDocument, vec![])
            .add_metadata("author", "John Doe")
            .add_metadata("version", "1.0");
        assert_eq!(obj.get_metadata("author"), Some("John Doe"));
        assert_eq!(obj.get_metadata("version"), Some("1.0"));
    }

    #[test]
    fn test_ole_embedded_object_to_xml() {
        let obj = OLEEmbeddedObject::new("ole1", OLEObjectType::ExcelWorksheet, vec![]);
        let xml = obj.to_xml();
        assert!(xml.contains("p:oleObj"));
        assert!(xml.contains("r:id=\"ole1\""));
    }

    #[test]
    fn test_ole_object_manager_new() {
        let manager = OLEObjectManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_ole_object_manager_add_object() {
        let mut manager = OLEObjectManager::new();
        let obj = OLEEmbeddedObject::new("ole1", OLEObjectType::ExcelWorksheet, vec![]);
        manager.add_object(obj);
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_ole_object_manager_create_object() {
        let mut manager = OLEObjectManager::new();
        let id = manager.create_object(OLEObjectType::WordDocument, vec![1, 2, 3]);
        assert_eq!(manager.count(), 1);
        assert!(manager.get_object(&id).is_some());
    }

    #[test]
    fn test_ole_object_manager_remove_object() {
        let mut manager = OLEObjectManager::new();
        let id = manager.create_object(OLEObjectType::ExcelWorksheet, vec![]);
        manager.remove_object(&id);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_ole_object_manager_to_xml() {
        let mut manager = OLEObjectManager::new();
        manager.create_object(OLEObjectType::ExcelWorksheet, vec![]);
        let xml = manager.to_xml();
        assert!(xml.contains("<p:oleObjList>"));
        assert!(xml.contains("</p:oleObjList>"));
    }
}
