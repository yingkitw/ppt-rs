//! Slide ID Management - Manages slide IDs and relationships

/// Slide ID - represents a slide in the presentation
#[derive(Clone, Debug)]
pub struct SlideId {
    /// Unique slide ID (starting from 256)
    id: u32,
    /// Relationship ID (rId)
    rel_id: String,
}

impl SlideId {
    /// Create a new slide ID
    pub fn new(id: u32, rel_id: String) -> Self {
        Self { id, rel_id }
    }

    /// Get the slide ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get the relationship ID
    pub fn rel_id(&self) -> &str {
        &self.rel_id
    }

    /// Generate XML for sldId element
    pub fn to_xml(&self) -> String {
        format!(r#"    <p:sldId id="{}" r:id="{}"/>"#, self.id, self.rel_id)
    }
}

/// Slide ID Manager - manages all slide IDs in a presentation
#[derive(Clone, Debug)]
pub struct SlideIdManager {
    /// List of slide IDs
    slide_ids: Vec<SlideId>,
    /// Next available slide ID
    next_id: u32,
}

impl SlideIdManager {
    /// Create a new slide ID manager
    pub fn new() -> Self {
        Self {
            slide_ids: vec![],
            next_id: 256,
        }
    }

    /// Add a new slide ID
    pub fn add_slide(&mut self, rel_id: String) -> SlideId {
        let slide_id = SlideId::new(self.next_id, rel_id);
        self.slide_ids.push(slide_id.clone());
        self.next_id += 1;
        slide_id
    }

    /// Get all slide IDs
    pub fn all(&self) -> &[SlideId] {
        &self.slide_ids
    }

    /// Get number of slides
    pub fn len(&self) -> usize {
        self.slide_ids.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.slide_ids.is_empty()
    }

    /// Generate XML for sldIdLst
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("  <p:sldIdLst>\n");
        for slide_id in &self.slide_ids {
            xml.push_str(&slide_id.to_xml());
            xml.push('\n');
        }
        xml.push_str("  </p:sldIdLst>");
        xml
    }
}

impl Default for SlideIdManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_id_creation() {
        let slide_id = SlideId::new(256, "rId7".to_string());
        assert_eq!(slide_id.id(), 256);
        assert_eq!(slide_id.rel_id(), "rId7");
    }

    #[test]
    fn test_slide_id_to_xml() {
        let slide_id = SlideId::new(256, "rId7".to_string());
        let xml = slide_id.to_xml();
        assert!(xml.contains(r#"<p:sldId id="256" r:id="rId7"/>"#));
    }

    #[test]
    fn test_slide_id_manager_creation() {
        let manager = SlideIdManager::new();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_slide_id_manager_add_slide() {
        let mut manager = SlideIdManager::new();
        
        let slide1 = manager.add_slide("rId7".to_string());
        assert_eq!(slide1.id(), 256);
        assert_eq!(manager.len(), 1);
        
        let slide2 = manager.add_slide("rId8".to_string());
        assert_eq!(slide2.id(), 257);
        assert_eq!(manager.len(), 2);
    }

    #[test]
    fn test_slide_id_manager_to_xml() {
        let mut manager = SlideIdManager::new();
        manager.add_slide("rId7".to_string());
        manager.add_slide("rId8".to_string());
        
        let xml = manager.to_xml();
        assert!(xml.contains(r#"<p:sldIdLst>"#));
        assert!(xml.contains(r#"<p:sldId id="256" r:id="rId7"/>"#));
        assert!(xml.contains(r#"<p:sldId id="257" r:id="rId8"/>"#));
        assert!(xml.contains(r#"</p:sldIdLst>"#));
    }

    #[test]
    fn test_slide_id_manager_all() {
        let mut manager = SlideIdManager::new();
        manager.add_slide("rId7".to_string());
        manager.add_slide("rId8".to_string());
        
        let all = manager.all();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].id(), 256);
        assert_eq!(all[1].id(), 257);
    }

    #[test]
    fn test_slide_id_manager_default() {
        let manager = SlideIdManager::default();
        assert_eq!(manager.len(), 0);
    }
}
