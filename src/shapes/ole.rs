//! OLE Object Embedding - Embedded object support

use crate::error::Result;

/// OLE object type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OleObjectType {
    /// Excel worksheet
    ExcelWorksheet,
    /// Word document
    WordDocument,
    /// PowerPoint presentation
    PowerPointPresentation,
    /// PDF document
    PdfDocument,
    /// Generic object
    Generic,
}

impl OleObjectType {
    /// Get MIME type
    pub fn mime_type(&self) -> &str {
        match self {
            OleObjectType::ExcelWorksheet => "application/vnd.ms-excel",
            OleObjectType::WordDocument => "application/msword",
            OleObjectType::PowerPointPresentation => "application/vnd.ms-powerpoint",
            OleObjectType::PdfDocument => "application/pdf",
            OleObjectType::Generic => "application/octet-stream",
        }
    }

    /// Get file extension
    pub fn extension(&self) -> &str {
        match self {
            OleObjectType::ExcelWorksheet => "xls",
            OleObjectType::WordDocument => "doc",
            OleObjectType::PowerPointPresentation => "ppt",
            OleObjectType::PdfDocument => "pdf",
            OleObjectType::Generic => "bin",
        }
    }
}

/// OLE Object
#[derive(Clone, Debug)]
pub struct OleObject {
    /// Object ID
    id: u32,
    /// Object type
    object_type: OleObjectType,
    /// Object name
    name: String,
    /// Object data (binary)
    data: Vec<u8>,
    /// Relationship ID
    rel_id: Option<String>,
    /// Display as icon
    display_as_icon: bool,
}

impl OleObject {
    /// Create a new OLE object
    pub fn new(id: u32, object_type: OleObjectType, name: String, data: Vec<u8>) -> Self {
        Self {
            id,
            object_type,
            name,
            data,
            rel_id: None,
            display_as_icon: true,
        }
    }

    /// Set relationship ID
    pub fn set_rel_id(&mut self, rel_id: String) {
        self.rel_id = Some(rel_id);
    }

    /// Get relationship ID
    pub fn rel_id(&self) -> Option<&str> {
        self.rel_id.as_deref()
    }

    /// Set display as icon
    pub fn set_display_as_icon(&mut self, display_as_icon: bool) {
        self.display_as_icon = display_as_icon;
    }

    /// Is display as icon
    pub fn display_as_icon(&self) -> bool {
        self.display_as_icon
    }

    /// Get object ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get object type
    pub fn object_type(&self) -> &OleObjectType {
        &self.object_type
    }

    /// Get object name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get object data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get data size
    pub fn data_size(&self) -> usize {
        self.data.len()
    }

    /// Generate XML for OLE object
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(&format!(
            r#"<p:oleObj spPr="" progId="Excel.Sheet.8" name="{}">"#,
            self.name
        ));
        xml.push('\n');

        if let Some(rid) = &self.rel_id {
            xml.push_str(&format!(r#"  <p:oleObjData r:id="{}"/>"#, rid));
        }

        xml.push_str(r#"</p:oleObj>"#);
        xml
    }
}

/// OLE Object Manager
#[derive(Clone, Debug)]
pub struct OleObjectManager {
    /// OLE objects
    objects: Vec<OleObject>,
}

impl OleObjectManager {
    /// Create a new OLE object manager
    pub fn new() -> Self {
        Self {
            objects: vec![],
        }
    }

    /// Add an OLE object
    pub fn add_object(&mut self, object: OleObject) -> usize {
        self.objects.push(object);
        self.objects.len() - 1
    }

    /// Create and add a new OLE object
    pub fn create_object(
        &mut self,
        object_type: OleObjectType,
        name: String,
        data: Vec<u8>,
    ) -> usize {
        let id = self.objects.len() as u32;
        self.add_object(OleObject::new(id, object_type, name, data))
    }

    /// Get OLE object by index
    pub fn get(&self, index: usize) -> Option<&OleObject> {
        self.objects.get(index)
    }

    /// Get mutable OLE object by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut OleObject> {
        self.objects.get_mut(index)
    }

    /// Get all OLE objects
    pub fn all(&self) -> &[OleObject] {
        &self.objects
    }

    /// Get number of OLE objects
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Get total data size
    pub fn total_data_size(&self) -> usize {
        self.objects.iter().map(|o| o.data_size()).sum()
    }
}

impl Default for OleObjectManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ole_object_type_mime() {
        assert_eq!(OleObjectType::ExcelWorksheet.mime_type(), "application/vnd.ms-excel");
        assert_eq!(OleObjectType::WordDocument.mime_type(), "application/msword");
        assert_eq!(OleObjectType::PdfDocument.mime_type(), "application/pdf");
    }

    #[test]
    fn test_ole_object_type_extension() {
        assert_eq!(OleObjectType::ExcelWorksheet.extension(), "xls");
        assert_eq!(OleObjectType::WordDocument.extension(), "doc");
        assert_eq!(OleObjectType::PdfDocument.extension(), "pdf");
    }

    #[test]
    fn test_ole_object_creation() {
        let data = vec![1, 2, 3, 4, 5];
        let obj = OleObject::new(
            1,
            OleObjectType::ExcelWorksheet,
            "Sheet1".to_string(),
            data.clone(),
        );

        assert_eq!(obj.id(), 1);
        assert_eq!(obj.name(), "Sheet1");
        assert_eq!(obj.data_size(), 5);
        assert!(obj.display_as_icon());
    }

    #[test]
    fn test_ole_object_rel_id() {
        let mut obj = OleObject::new(
            1,
            OleObjectType::ExcelWorksheet,
            "Sheet1".to_string(),
            vec![],
        );

        assert!(obj.rel_id().is_none());
        obj.set_rel_id("rId1".to_string());
        assert_eq!(obj.rel_id(), Some("rId1"));
    }

    #[test]
    fn test_ole_object_display_as_icon() {
        let mut obj = OleObject::new(
            1,
            OleObjectType::ExcelWorksheet,
            "Sheet1".to_string(),
            vec![],
        );

        assert!(obj.display_as_icon());
        obj.set_display_as_icon(false);
        assert!(!obj.display_as_icon());
    }

    #[test]
    fn test_ole_object_to_xml() {
        let mut obj = OleObject::new(
            1,
            OleObjectType::ExcelWorksheet,
            "Sheet1".to_string(),
            vec![],
        );
        obj.set_rel_id("rId1".to_string());

        let xml = obj.to_xml();
        assert!(xml.contains(r#"<p:oleObj"#));
        assert!(xml.contains(r#"name="Sheet1""#));
        assert!(xml.contains(r#"r:id="rId1""#));
        assert!(xml.contains(r#"</p:oleObj>"#));
    }

    #[test]
    fn test_ole_object_manager_creation() {
        let manager = OleObjectManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_ole_object_manager_add() {
        let mut manager = OleObjectManager::new();
        let obj = OleObject::new(
            0,
            OleObjectType::ExcelWorksheet,
            "Sheet1".to_string(),
            vec![1, 2, 3],
        );
        manager.add_object(obj);

        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_ole_object_manager_create() {
        let mut manager = OleObjectManager::new();
        manager.create_object(
            OleObjectType::ExcelWorksheet,
            "Sheet1".to_string(),
            vec![1, 2, 3],
        );

        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_ole_object_manager_get() {
        let mut manager = OleObjectManager::new();
        manager.create_object(
            OleObjectType::ExcelWorksheet,
            "Sheet1".to_string(),
            vec![1, 2, 3],
        );

        let obj = manager.get(0);
        assert!(obj.is_some());
        assert_eq!(obj.unwrap().name(), "Sheet1");
    }

    #[test]
    fn test_ole_object_manager_get_mut() {
        let mut manager = OleObjectManager::new();
        manager.create_object(
            OleObjectType::ExcelWorksheet,
            "Sheet1".to_string(),
            vec![1, 2, 3],
        );

        if let Some(obj) = manager.get_mut(0) {
            obj.set_rel_id("rId1".to_string());
        }

        assert_eq!(manager.get(0).unwrap().rel_id(), Some("rId1"));
    }

    #[test]
    fn test_ole_object_manager_total_data_size() {
        let mut manager = OleObjectManager::new();
        manager.create_object(
            OleObjectType::ExcelWorksheet,
            "Sheet1".to_string(),
            vec![1, 2, 3],
        );
        manager.create_object(
            OleObjectType::WordDocument,
            "Doc1".to_string(),
            vec![1, 2, 3, 4, 5],
        );

        assert_eq!(manager.total_data_size(), 8);
    }

    #[test]
    fn test_ole_object_manager_default() {
        let manager = OleObjectManager::default();
        assert!(manager.is_empty());
    }
}
