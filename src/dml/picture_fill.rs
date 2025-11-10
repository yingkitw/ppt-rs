//! Picture Fill - Picture fill support for shapes


/// Picture Fill - represents a picture fill for shapes
#[derive(Clone, Debug)]
pub struct PictureFill {
    /// Image path/URI
    image_path: String,
    /// Image relationship ID
    rel_id: Option<String>,
    /// Stretch fill (true) or tile fill (false)
    stretch: bool,
}

impl PictureFill {
    /// Create a new picture fill
    pub fn new(image_path: String) -> Self {
        Self {
            image_path,
            rel_id: None,
            stretch: true,
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

    /// Get image path
    pub fn image_path(&self) -> &str {
        &self.image_path
    }

    /// Set stretch fill
    pub fn set_stretch(&mut self, stretch: bool) {
        self.stretch = stretch;
    }

    /// Is stretch fill
    pub fn is_stretch(&self) -> bool {
        self.stretch
    }

    /// Generate XML for picture fill
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(r#"<a:blipFill>"#);
        xml.push('\n');
        
        // Blip element with relationship ID
        if let Some(rid) = &self.rel_id {
            xml.push_str(&format!(r#"  <a:blip r:embed="{}"/>"#, rid));
        } else {
            xml.push_str(r#"  <a:blip/>"#);
        }
        xml.push('\n');
        
        // Stretch or tile
        if self.stretch {
            xml.push_str(r#"  <a:stretch><a:fillRect/></a:stretch>"#);
        } else {
            xml.push_str(r#"  <a:tile/>"#);
        }
        xml.push('\n');
        
        xml.push_str(r#"</a:blipFill>"#);
        xml
    }
}

/// Picture Fill Manager
#[derive(Clone, Debug)]
pub struct PictureFillManager {
    /// Picture fills
    fills: Vec<PictureFill>,
}

impl PictureFillManager {
    /// Create a new picture fill manager
    pub fn new() -> Self {
        Self {
            fills: vec![],
        }
    }

    /// Add a picture fill
    pub fn add_fill(&mut self, fill: PictureFill) -> usize {
        self.fills.push(fill);
        self.fills.len() - 1
    }

    /// Get picture fill by index
    pub fn get(&self, index: usize) -> Option<&PictureFill> {
        self.fills.get(index)
    }

    /// Get mutable picture fill by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut PictureFill> {
        self.fills.get_mut(index)
    }

    /// Get all picture fills
    pub fn all(&self) -> &[PictureFill] {
        &self.fills
    }

    /// Get number of picture fills
    pub fn len(&self) -> usize {
        self.fills.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.fills.is_empty()
    }
}

impl Default for PictureFillManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_picture_fill_creation() {
        let fill = PictureFill::new("image.png".to_string());
        assert_eq!(fill.image_path(), "image.png");
        assert!(fill.rel_id().is_none());
        assert!(fill.is_stretch());
    }

    #[test]
    fn test_picture_fill_with_rel_id() {
        let mut fill = PictureFill::new("image.png".to_string());
        fill.set_rel_id("rId1".to_string());
        
        assert_eq!(fill.rel_id(), Some("rId1"));
    }

    #[test]
    fn test_picture_fill_stretch() {
        let mut fill = PictureFill::new("image.png".to_string());
        assert!(fill.is_stretch());
        
        fill.set_stretch(false);
        assert!(!fill.is_stretch());
    }

    #[test]
    fn test_picture_fill_to_xml_stretch() {
        let mut fill = PictureFill::new("image.png".to_string());
        fill.set_rel_id("rId1".to_string());
        
        let xml = fill.to_xml();
        assert!(xml.contains(r#"<a:blipFill>"#));
        assert!(xml.contains(r#"r:embed="rId1""#));
        assert!(xml.contains(r#"<a:stretch>"#));
        assert!(xml.contains(r#"</a:blipFill>"#));
    }

    #[test]
    fn test_picture_fill_to_xml_tile() {
        let mut fill = PictureFill::new("image.png".to_string());
        fill.set_rel_id("rId1".to_string());
        fill.set_stretch(false);
        
        let xml = fill.to_xml();
        assert!(xml.contains(r#"<a:tile/>"#));
        assert!(!xml.contains(r#"<a:stretch>"#));
    }

    #[test]
    fn test_picture_fill_manager_creation() {
        let manager = PictureFillManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_picture_fill_manager_add() {
        let mut manager = PictureFillManager::new();
        let fill1 = PictureFill::new("image1.png".to_string());
        let fill2 = PictureFill::new("image2.png".to_string());
        
        manager.add_fill(fill1);
        manager.add_fill(fill2);
        
        assert_eq!(manager.len(), 2);
    }

    #[test]
    fn test_picture_fill_manager_get() {
        let mut manager = PictureFillManager::new();
        let fill = PictureFill::new("image.png".to_string());
        manager.add_fill(fill);
        
        let retrieved = manager.get(0);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().image_path(), "image.png");
    }

    #[test]
    fn test_picture_fill_manager_get_mut() {
        let mut manager = PictureFillManager::new();
        let fill = PictureFill::new("image.png".to_string());
        manager.add_fill(fill);
        
        let retrieved = manager.get_mut(0);
        assert!(retrieved.is_some());
        retrieved.unwrap().set_rel_id("rId1".to_string());
        
        assert_eq!(manager.get(0).unwrap().rel_id(), Some("rId1"));
    }

    #[test]
    fn test_picture_fill_manager_default() {
        let manager = PictureFillManager::default();
        assert!(manager.is_empty());
    }
}
