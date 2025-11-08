//! Content Types Management - Manages [Content_Types].xml generation

use std::collections::HashMap;

/// Content Types Manager
#[derive(Clone, Debug)]
pub struct ContentTypesManager {
    /// Default content types (extension -> content type)
    defaults: HashMap<String, String>,
    /// Override content types (part name -> content type)
    overrides: HashMap<String, String>,
}

impl ContentTypesManager {
    /// Create a new content types manager with default entries
    pub fn new() -> Self {
        let mut defaults = HashMap::new();
        defaults.insert("bin".to_string(), "application/vnd.openxmlformats-officedocument.presentationml.printerSettings".to_string());
        defaults.insert("jpeg".to_string(), "image/jpeg".to_string());
        defaults.insert("rels".to_string(), "application/vnd.openxmlformats-package.relationships+xml".to_string());
        defaults.insert("xml".to_string(), "application/xml".to_string());

        let mut overrides = HashMap::new();
        // Core overrides
        overrides.insert("/docProps/app.xml".to_string(), "application/vnd.openxmlformats-officedocument.extended-properties+xml".to_string());
        overrides.insert("/docProps/core.xml".to_string(), "application/vnd.openxmlformats-package.core-properties+xml".to_string());
        overrides.insert("/ppt/presentation.xml".to_string(), "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml".to_string());
        overrides.insert("/ppt/presProps.xml".to_string(), "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml".to_string());
        overrides.insert("/ppt/viewProps.xml".to_string(), "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml".to_string());
        overrides.insert("/ppt/tableStyles.xml".to_string(), "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml".to_string());
        overrides.insert("/ppt/theme/theme1.xml".to_string(), "application/vnd.openxmlformats-officedocument.theme+xml".to_string());
        
        // Master and layouts
        overrides.insert("/ppt/slideMasters/slideMaster1.xml".to_string(), "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml".to_string());
        for i in 1..=11 {
            overrides.insert(format!("/ppt/slideLayouts/slideLayout{}.xml", i), "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml".to_string());
        }

        Self { defaults, overrides }
    }

    /// Add a slide override
    pub fn add_slide(&mut self, slide_index: usize) {
        let part_name = format!("/ppt/slides/slide{}.xml", slide_index);
        let content_type = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml".to_string();
        self.overrides.insert(part_name, content_type);
    }

    /// Add an image override
    pub fn add_image(&mut self, image_path: &str, content_type: &str) {
        self.overrides.insert(image_path.to_string(), content_type.to_string());
    }

    /// Generate [Content_Types].xml
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
        xml.push('\n');
        xml.push_str(r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">"#);
        xml.push('\n');

        // Add default entries
        for (ext, content_type) in &self.defaults {
            xml.push_str(&format!(r#"  <Default Extension="{}" ContentType="{}"/>"#, ext, content_type));
            xml.push('\n');
        }

        // Add override entries
        for (part_name, content_type) in &self.overrides {
            xml.push_str(&format!(r#"  <Override PartName="{}" ContentType="{}"/>"#, part_name, content_type));
            xml.push('\n');
        }

        xml.push_str(r#"</Types>"#);
        xml
    }
}

impl Default for ContentTypesManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_types_manager_creation() {
        let manager = ContentTypesManager::new();
        assert_eq!(manager.defaults.len(), 4);
        assert!(manager.defaults.contains_key("bin"));
        assert!(manager.defaults.contains_key("jpeg"));
        assert!(manager.defaults.contains_key("rels"));
        assert!(manager.defaults.contains_key("xml"));
    }

    #[test]
    fn test_add_slide() {
        let mut manager = ContentTypesManager::new();
        manager.add_slide(1);
        manager.add_slide(2);
        
        assert!(manager.overrides.contains_key("/ppt/slides/slide1.xml"));
        assert!(manager.overrides.contains_key("/ppt/slides/slide2.xml"));
    }

    #[test]
    fn test_add_image() {
        let mut manager = ContentTypesManager::new();
        manager.add_image("/ppt/media/image1.png", "image/png");
        
        assert!(manager.overrides.contains_key("/ppt/media/image1.png"));
    }

    #[test]
    fn test_to_xml() {
        let mut manager = ContentTypesManager::new();
        manager.add_slide(1);
        
        let xml = manager.to_xml();
        assert!(xml.contains(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#));
        assert!(xml.contains(r#"<Types"#));
        assert!(xml.contains(r#"<Default Extension="bin""#));
        assert!(xml.contains(r#"<Override PartName="/ppt/slides/slide1.xml""#));
        assert!(xml.contains(r#"</Types>"#));
    }

    #[test]
    fn test_all_layouts_in_overrides() {
        let manager = ContentTypesManager::new();
        for i in 1..=11 {
            let key = format!("/ppt/slideLayouts/slideLayout{}.xml", i);
            assert!(manager.overrides.contains_key(&key));
        }
    }

    #[test]
    fn test_default() {
        let manager = ContentTypesManager::default();
        assert_eq!(manager.defaults.len(), 4);
    }
}
