//! Text Shape - Text content rendering for slides

use crate::dml::color::RGBColor;

/// Text shape for rendering text on slides
#[derive(Clone, Debug)]
pub struct TextShape {
    /// Shape ID
    id: u32,
    /// Shape name
    name: String,
    /// Text content
    text: String,
    /// Position X (EMU)
    x: i32,
    /// Position Y (EMU)
    y: i32,
    /// Width (EMU)
    width: u32,
    /// Height (EMU)
    height: u32,
}

impl TextShape {
    /// Create a new text shape
    pub fn new(id: u32, name: String, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            id,
            name,
            text: String::new(),
            x,
            y,
            width,
            height,
        }
    }

    /// Get shape ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get shape name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get text content
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set text content
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    /// Get position X
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Get position Y
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Get width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set position
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// Set size
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Generate XML for text shape
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        
        // Shape element
        xml.push_str(&format!(
            r#"<p:sp><p:nvSpPr><p:cNvPr id="{}" name="{}"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr/></p:nvSpPr>"#,
            self.id, self.name
        ));
        
        // Shape properties (position and size)
        xml.push_str(&format!(
            r#"<p:spPr><a:xfrm><a:off x="{}" y="{}"/><a:ext cx="{}" cy="{}"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom></p:spPr>"#,
            self.x, self.y, self.width, self.height
        ));
        
        // Text body
        xml.push_str("<p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:t>");
        xml.push_str(&self.text);
        xml.push_str("</a:t></a:r></a:p></p:txBody>");
        xml.push_str("</p:sp>");
        
        xml
    }
}

/// Text shape manager for managing multiple text shapes
#[derive(Clone, Debug)]
pub struct TextShapeManager {
    /// Text shapes
    shapes: Vec<TextShape>,
    /// Next shape ID
    next_id: u32,
}

impl TextShapeManager {
    /// Create a new text shape manager
    pub fn new() -> Self {
        Self {
            shapes: vec![],
            next_id: 2, // ID 1 is reserved for group shape
        }
    }

    /// Add a text shape
    pub fn add_shape(&mut self, shape: TextShape) -> usize {
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    /// Create and add a text shape
    pub fn create_shape(&mut self, name: String, x: i32, y: i32, width: u32, height: u32) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.add_shape(TextShape::new(id, name, x, y, width, height))
    }

    /// Get shape by index
    pub fn get(&self, index: usize) -> Option<&TextShape> {
        self.shapes.get(index)
    }

    /// Get mutable shape by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut TextShape> {
        self.shapes.get_mut(index)
    }

    /// Get all shapes
    pub fn all(&self) -> &[TextShape] {
        &self.shapes
    }

    /// Get shape count
    pub fn len(&self) -> usize {
        self.shapes.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    /// Generate XML for all shapes
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        for shape in &self.shapes {
            xml.push_str(&shape.to_xml());
        }
        xml
    }
}

impl Default for TextShapeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_shape_creation() {
        let shape = TextShape::new(2, "Title".to_string(), 100, 200, 1000, 500);
        assert_eq!(shape.id(), 2);
        assert_eq!(shape.name(), "Title");
        assert_eq!(shape.x(), 100);
        assert_eq!(shape.y(), 200);
        assert_eq!(shape.width(), 1000);
        assert_eq!(shape.height(), 500);
    }

    #[test]
    fn test_text_shape_position() {
        let mut shape = TextShape::new(2, "Title".to_string(), 100, 200, 1000, 500);
        shape.set_position(300, 400);
        assert_eq!(shape.x(), 300);
        assert_eq!(shape.y(), 400);
    }

    #[test]
    fn test_text_shape_size() {
        let mut shape = TextShape::new(2, "Title".to_string(), 100, 200, 1000, 500);
        shape.set_size(2000, 1000);
        assert_eq!(shape.width(), 2000);
        assert_eq!(shape.height(), 1000);
    }

    #[test]
    fn test_text_shape_text() {
        let mut shape = TextShape::new(2, "Title".to_string(), 100, 200, 1000, 500);
        shape.set_text("Hello World".to_string());
        assert_eq!(shape.text(), "Hello World");
    }

    #[test]
    fn test_text_shape_to_xml() {
        let shape = TextShape::new(2, "Title".to_string(), 100, 200, 1000, 500);
        let xml = shape.to_xml();
        assert!(xml.contains(r#"<p:sp>"#));
        assert!(xml.contains(r#"id="2""#));
        assert!(xml.contains(r#"name="Title""#));
        assert!(xml.contains(r#"<p:txBody>"#));
    }

    #[test]
    fn test_text_shape_manager_creation() {
        let manager = TextShapeManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_text_shape_manager_add() {
        let mut manager = TextShapeManager::new();
        let shape = TextShape::new(2, "Title".to_string(), 100, 200, 1000, 500);
        manager.add_shape(shape);
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_text_shape_manager_create() {
        let mut manager = TextShapeManager::new();
        manager.create_shape("Title".to_string(), 100, 200, 1000, 500);
        manager.create_shape("Subtitle".to_string(), 100, 700, 1000, 300);
        assert_eq!(manager.len(), 2);
    }

    #[test]
    fn test_text_shape_manager_get() {
        let mut manager = TextShapeManager::new();
        manager.create_shape("Title".to_string(), 100, 200, 1000, 500);
        let shape = manager.get(0);
        assert!(shape.is_some());
        assert_eq!(shape.unwrap().name(), "Title");
    }

    #[test]
    fn test_text_shape_manager_get_mut() {
        let mut manager = TextShapeManager::new();
        manager.create_shape("Title".to_string(), 100, 200, 1000, 500);
        if let Some(shape) = manager.get_mut(0) {
            shape.set_text("Updated".to_string());
        }
        assert_eq!(manager.get(0).unwrap().text(), "Updated");
    }

    #[test]
    fn test_text_shape_manager_to_xml() {
        let mut manager = TextShapeManager::new();
        manager.create_shape("Title".to_string(), 100, 200, 1000, 500);
        let xml = manager.to_xml();
        assert!(xml.contains(r#"<p:sp>"#));
        assert!(xml.contains(r#"name="Title""#));
    }

    #[test]
    fn test_text_shape_manager_default() {
        let manager = TextShapeManager::default();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_text_shape_manager_id_increment() {
        let mut manager = TextShapeManager::new();
        manager.create_shape("Shape1".to_string(), 0, 0, 100, 100);
        manager.create_shape("Shape2".to_string(), 0, 0, 100, 100);
        manager.create_shape("Shape3".to_string(), 0, 0, 100, 100);
        
        assert_eq!(manager.get(0).unwrap().id(), 2);
        assert_eq!(manager.get(1).unwrap().id(), 3);
        assert_eq!(manager.get(2).unwrap().id(), 4);
    }
}
