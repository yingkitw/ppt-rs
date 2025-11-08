//! Picture Shape - Image content rendering for slides

/// Picture shape for rendering images on slides
#[derive(Clone, Debug)]
pub struct PictureShape {
    /// Shape ID
    id: u32,
    /// Shape name
    name: String,
    /// Image path/reference
    image_path: String,
    /// Position X (EMU)
    x: i32,
    /// Position Y (EMU)
    y: i32,
    /// Width (EMU)
    width: u32,
    /// Height (EMU)
    height: u32,
    /// Relationship ID
    rel_id: String,
}

impl PictureShape {
    /// Create a new picture shape
    pub fn new(id: u32, name: String, image_path: String, x: i32, y: i32, width: u32, height: u32, rel_id: String) -> Self {
        Self {
            id,
            name,
            image_path,
            x,
            y,
            width,
            height,
            rel_id,
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

    /// Get image path
    pub fn image_path(&self) -> &str {
        &self.image_path
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

    /// Get relationship ID
    pub fn rel_id(&self) -> &str {
        &self.rel_id
    }

    /// Generate XML for picture shape
    pub fn to_xml(&self) -> String {
        format!(
            r#"<p:pic><p:nvPicPr><p:cNvPr id="{}" name="{}"/><p:cNvPicPr/><p:nvPr/></p:nvPicPr><p:blipFill><a:blip r:embed="{}"/><a:stretch><a:fillRect/></a:stretch></p:blipFill><p:spPr><a:xfrm><a:off x="{}" y="{}"/><a:ext cx="{}" cy="{}"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom></p:spPr></p:pic>"#,
            self.id, self.name, self.rel_id, self.x, self.y, self.width, self.height
        )
    }
}

/// Picture shape manager
#[derive(Clone, Debug)]
pub struct PictureShapeManager {
    shapes: Vec<PictureShape>,
    next_id: u32,
}

impl PictureShapeManager {
    /// Create a new picture shape manager
    pub fn new() -> Self {
        Self {
            shapes: vec![],
            next_id: 2,
        }
    }

    /// Add a picture shape
    pub fn add_shape(&mut self, shape: PictureShape) -> usize {
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    /// Create and add a picture shape
    pub fn create_shape(&mut self, name: String, image_path: String, x: i32, y: i32, width: u32, height: u32, rel_id: String) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.add_shape(PictureShape::new(id, name, image_path, x, y, width, height, rel_id))
    }

    /// Get shape by index
    pub fn get(&self, index: usize) -> Option<&PictureShape> {
        self.shapes.get(index)
    }

    /// Get mutable shape by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut PictureShape> {
        self.shapes.get_mut(index)
    }

    /// Get all shapes
    pub fn all(&self) -> &[PictureShape] {
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
        self.shapes.iter().map(|s| s.to_xml()).collect()
    }
}

impl Default for PictureShapeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_picture_shape_creation() {
        let shape = PictureShape::new(2, "Image".to_string(), "image.png".to_string(), 100, 200, 1000, 800, "rId2".to_string());
        assert_eq!(shape.id(), 2);
        assert_eq!(shape.name(), "Image");
        assert_eq!(shape.image_path(), "image.png");
        assert_eq!(shape.rel_id(), "rId2");
    }

    #[test]
    fn test_picture_shape_to_xml() {
        let shape = PictureShape::new(2, "Image".to_string(), "image.png".to_string(), 100, 200, 1000, 800, "rId2".to_string());
        let xml = shape.to_xml();
        assert!(xml.contains(r#"<p:pic>"#));
        assert!(xml.contains(r#"id="2""#));
        assert!(xml.contains(r#"r:embed="rId2""#));
    }

    #[test]
    fn test_picture_shape_manager() {
        let mut manager = PictureShapeManager::new();
        manager.create_shape("Image".to_string(), "image.png".to_string(), 100, 200, 1000, 800, "rId2".to_string());
        assert_eq!(manager.len(), 1);
        assert_eq!(manager.get(0).unwrap().name(), "Image");
    }
}
