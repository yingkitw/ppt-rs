//! Presentation Relationship Management - Manages all presentation relationships


/// Presentation Relationship Manager
#[derive(Clone, Debug)]
pub struct PresentationRelationshipManager {
    /// Core relationships (master, printer settings, properties, theme, table styles)
    core_rels: Vec<(String, String, String)>, // (rId, type, target)
    /// Slide relationships
    slide_rels: Vec<(String, String, String)>,
}

impl PresentationRelationshipManager {
    /// Create a new presentation relationship manager
    pub fn new() -> Self {
        let core_rels = vec![
            ("rId1".to_string(), "slideMaster".to_string(), "slideMasters/slideMaster1.xml".to_string()),
            ("rId2".to_string(), "printerSettings".to_string(), "printerSettings/printerSettings1.bin".to_string()),
            ("rId3".to_string(), "presProps".to_string(), "presProps.xml".to_string()),
            ("rId4".to_string(), "viewProps".to_string(), "viewProps.xml".to_string()),
            ("rId5".to_string(), "theme".to_string(), "theme/theme1.xml".to_string()),
            ("rId6".to_string(), "tableStyles".to_string(), "tableStyles.xml".to_string()),
        ];

        Self {
            core_rels,
            slide_rels: vec![],
        }
    }

    /// Add a slide relationship
    pub fn add_slide_rel(&mut self, slide_index: usize) {
        let rid = format!("rId{}", 7 + slide_index - 1);
        let rel_type = "slide".to_string();
        let target = format!("slides/slide{}.xml", slide_index);
        self.slide_rels.push((rid, rel_type, target));
    }

    /// Get all relationships
    pub fn all_rels(&self) -> Vec<(String, String, String)> {
        let mut all = self.core_rels.clone();
        all.extend(self.slide_rels.clone());
        all
    }

    /// Generate presentation relationships XML
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
        xml.push('\n');
        xml.push_str(r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">"#);
        xml.push('\n');

        for (rid, rel_type, target) in self.all_rels() {
            let full_type = match rel_type.as_str() {
                "slideMaster" => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster",
                "printerSettings" => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
                "presProps" => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps",
                "viewProps" => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps",
                "theme" => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
                "tableStyles" => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles",
                "slide" => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
                _ => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/unknown",
            };
            xml.push_str(&format!(r#"  <Relationship Id="{}" Type="{}" Target="{}"/>"#, rid, full_type, target));
            xml.push('\n');
        }

        xml.push_str(r#"</Relationships>"#);
        xml
    }
}

impl Default for PresentationRelationshipManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentation_relationship_manager_creation() {
        let manager = PresentationRelationshipManager::new();
        let rels = manager.all_rels();
        assert_eq!(rels.len(), 6); // 6 core relationships
    }

    #[test]
    fn test_add_slide_rel() {
        let mut manager = PresentationRelationshipManager::new();
        manager.add_slide_rel(1);
        manager.add_slide_rel(2);
        
        let rels = manager.all_rels();
        assert_eq!(rels.len(), 8); // 6 core + 2 slides
    }

    #[test]
    fn test_to_xml() {
        let mut manager = PresentationRelationshipManager::new();
        manager.add_slide_rel(1);
        
        let xml = manager.to_xml();
        assert!(xml.contains(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#));
        assert!(xml.contains(r#"<Relationships"#));
        assert!(xml.contains(r#"slideMaster"#));
        assert!(xml.contains(r#"slides/slide1.xml"#));
        assert!(xml.contains(r#"</Relationships>"#));
    }

    #[test]
    fn test_relationship_ids() {
        let mut manager = PresentationRelationshipManager::new();
        manager.add_slide_rel(1);
        manager.add_slide_rel(2);
        manager.add_slide_rel(3);
        
        let rels = manager.all_rels();
        // Check that slide relationships have correct rIds (rId7, rId8, rId9)
        let slide_rels: Vec<_> = rels.iter().filter(|(_, t, _)| t == "slide").collect();
        assert_eq!(slide_rels.len(), 3);
        assert_eq!(slide_rels[0].0, "rId7");
        assert_eq!(slide_rels[1].0, "rId8");
        assert_eq!(slide_rels[2].0, "rId9");
    }

    #[test]
    fn test_default() {
        let manager = PresentationRelationshipManager::default();
        let rels = manager.all_rels();
        assert_eq!(rels.len(), 6);
    }
}
