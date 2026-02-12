//! Content types part
//!
//! Represents [Content_Types].xml which defines MIME types for all parts.

use super::base::{ContentType, Part, PartType};
use crate::exc::PptxError;
use crate::oxml::XmlParser;

/// Default content type mapping (by extension)
#[derive(Debug, Clone)]
pub struct DefaultType {
    pub extension: String,
    pub content_type: String,
}

impl DefaultType {
    pub fn new(extension: impl Into<String>, content_type: impl Into<String>) -> Self {
        DefaultType {
            extension: extension.into(),
            content_type: content_type.into(),
        }
    }
}

/// Override content type mapping (by part name)
#[derive(Debug, Clone)]
pub struct OverrideType {
    pub part_name: String,
    pub content_type: String,
}

impl OverrideType {
    pub fn new(part_name: impl Into<String>, content_type: impl Into<String>) -> Self {
        OverrideType {
            part_name: part_name.into(),
            content_type: content_type.into(),
        }
    }
}

/// Content types part ([Content_Types].xml)
#[derive(Debug, Clone)]
pub struct ContentTypesPart {
    defaults: Vec<DefaultType>,
    overrides: Vec<OverrideType>,
}

impl ContentTypesPart {
    /// Create a new content types part with standard defaults
    pub fn new() -> Self {
        ContentTypesPart {
            defaults: vec![
                DefaultType::new(
                    "rels",
                    "application/vnd.openxmlformats-package.relationships+xml",
                ),
                DefaultType::new("xml", "application/xml"),
                DefaultType::new("jpeg", "image/jpeg"),
                DefaultType::new("jpg", "image/jpeg"),
                DefaultType::new("png", "image/png"),
                DefaultType::new("gif", "image/gif"),
                DefaultType::new("bmp", "image/bmp"),
                DefaultType::new("tiff", "image/tiff"),
                DefaultType::new("svg", "image/svg+xml"),
                DefaultType::new("mp4", "video/mp4"),
                DefaultType::new("mp3", "audio/mpeg"),
                DefaultType::new("wav", "audio/wav"),
            ],
            overrides: vec![],
        }
    }

    /// Add a default type
    pub fn add_default(&mut self, extension: impl Into<String>, content_type: impl Into<String>) {
        self.defaults
            .push(DefaultType::new(extension, content_type));
    }

    /// Add an override type
    pub fn add_override(&mut self, part_name: impl Into<String>, content_type: impl Into<String>) {
        self.overrides
            .push(OverrideType::new(part_name, content_type));
    }

    /// Add presentation part
    pub fn add_presentation(&mut self) {
        self.add_override(
            "/ppt/presentation.xml",
            "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml",
        );
    }

    /// Add slide part
    pub fn add_slide(&mut self, slide_number: usize) {
        self.add_override(
            format!("/ppt/slides/slide{}.xml", slide_number),
            "application/vnd.openxmlformats-officedocument.presentationml.slide+xml",
        );
    }

    /// Add slide layout part
    pub fn add_slide_layout(&mut self, layout_number: usize) {
        self.add_override(
            format!("/ppt/slideLayouts/slideLayout{}.xml", layout_number),
            "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml",
        );
    }

    /// Add slide master part
    pub fn add_slide_master(&mut self, master_number: usize) {
        self.add_override(
            format!("/ppt/slideMasters/slideMaster{}.xml", master_number),
            "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml",
        );
    }

    /// Add theme part
    pub fn add_theme(&mut self, theme_number: usize) {
        self.add_override(
            format!("/ppt/theme/theme{}.xml", theme_number),
            "application/vnd.openxmlformats-officedocument.theme+xml",
        );
    }

    /// Add notes slide part
    pub fn add_notes_slide(&mut self, notes_number: usize) {
        self.add_override(
            format!("/ppt/notesSlides/notesSlide{}.xml", notes_number),
            "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml",
        );
    }

    /// Add chart part
    pub fn add_chart(&mut self, chart_number: usize) {
        self.add_override(
            format!("/ppt/charts/chart{}.xml", chart_number),
            "application/vnd.openxmlformats-officedocument.drawingml.chart+xml",
        );
    }

    /// Add core properties
    pub fn add_core_properties(&mut self) {
        self.add_override(
            "/docProps/core.xml",
            "application/vnd.openxmlformats-package.core-properties+xml",
        );
    }

    /// Add app properties
    pub fn add_app_properties(&mut self) {
        self.add_override(
            "/docProps/app.xml",
            "application/vnd.openxmlformats-officedocument.extended-properties+xml",
        );
    }

    /// Get content type for a given part path
    pub fn get_content_type(&self, part_path: &str) -> Option<&str> {
        // Normalize path (add leading slash if missing)
        let normalized = if part_path.starts_with('/') {
            part_path.to_string()
        } else {
            format!("/{}", part_path)
        };

        // First check overrides (exact match)
        for override_type in &self.overrides {
            if override_type.part_name == normalized {
                return Some(&override_type.content_type);
            }
        }

        // Then check defaults (by extension)
        if let Some(ext) = part_path.rsplit('.').next() {
            let ext_lower = ext.to_lowercase();
            for default_type in &self.defaults {
                if default_type.extension.to_lowercase() == ext_lower {
                    return Some(&default_type.content_type);
                }
            }
        }

        None
    }

    /// Get all defaults
    pub fn defaults(&self) -> &[DefaultType] {
        &self.defaults
    }

    /// Get all overrides
    pub fn overrides(&self) -> &[OverrideType] {
        &self.overrides
    }

    fn generate_xml(&self) -> String {
        let defaults_xml: String = self
            .defaults
            .iter()
            .map(|d| {
                format!(
                    r#"<Default Extension="{}" ContentType="{}"/>"#,
                    d.extension, d.content_type
                )
            })
            .collect::<Vec<_>>()
            .join("\n  ");

        let overrides_xml: String = self
            .overrides
            .iter()
            .map(|o| {
                format!(
                    r#"<Override PartName="{}" ContentType="{}"/>"#,
                    o.part_name, o.content_type
                )
            })
            .collect::<Vec<_>>()
            .join("\n  ");

        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  {}
  {}
</Types>"#,
            defaults_xml, overrides_xml
        )
    }
}

impl Default for ContentTypesPart {
    fn default() -> Self {
        Self::new()
    }
}

impl Part for ContentTypesPart {
    fn path(&self) -> &str {
        "[Content_Types].xml"
    }

    fn part_type(&self) -> PartType {
        PartType::Relationships
    }

    fn content_type(&self) -> ContentType {
        ContentType::Xml
    }

    fn to_xml(&self) -> Result<String, PptxError> {
        Ok(self.generate_xml())
    }

    fn from_xml(xml: &str) -> Result<Self, PptxError> {
        let root = XmlParser::parse_str(xml)?;
        let mut ct = ContentTypesPart::default();

        // Parse Default elements
        for default in root.find_all("Default") {
            if let (Some(ext), Some(content_type)) =
                (default.attr("Extension"), default.attr("ContentType"))
            {
                ct.defaults.push(DefaultType::new(ext, content_type));
            }
        }

        // Parse Override elements
        for override_elem in root.find_all("Override") {
            if let (Some(part_name), Some(content_type)) = (
                override_elem.attr("PartName"),
                override_elem.attr("ContentType"),
            ) {
                ct.overrides
                    .push(OverrideType::new(part_name, content_type));
            }
        }

        Ok(ct)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_types_new() {
        let ct = ContentTypesPart::new();
        assert!(!ct.defaults.is_empty());
    }

    #[test]
    fn test_content_types_add_slide() {
        let mut ct = ContentTypesPart::new();
        ct.add_slide(1);
        ct.add_slide(2);
        assert_eq!(ct.overrides.len(), 2);
    }

    #[test]
    fn test_content_types_to_xml() {
        let mut ct = ContentTypesPart::new();
        ct.add_presentation();
        ct.add_slide(1);
        let xml = ct.to_xml().unwrap();
        assert!(xml.contains("<Types"));
        assert!(xml.contains("Default Extension"));
        assert!(xml.contains("Override PartName"));
    }

    #[test]
    fn test_content_types_path() {
        let ct = ContentTypesPart::new();
        assert_eq!(ct.path(), "[Content_Types].xml");
    }

    #[test]
    fn test_content_types_add_all() {
        let mut ct = ContentTypesPart::new();
        ct.add_presentation();
        ct.add_slide(1);
        ct.add_slide_layout(1);
        ct.add_slide_master(1);
        ct.add_theme(1);
        ct.add_core_properties();
        ct.add_app_properties();
        assert_eq!(ct.overrides.len(), 7);
    }

    #[test]
    fn test_content_types_from_xml() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
            <Default Extension="png" ContentType="image/png"/>
            <Default Extension="jpeg" ContentType="image/jpeg"/>
            <Default Extension="xml" ContentType="application/xml"/>
            <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
            <Override PartName="/ppt/slides/slide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
        </Types>"#;

        let ct = ContentTypesPart::from_xml(xml).unwrap();
        // from_xml creates fresh instance, not using new() defaults
        assert!(ct.defaults().len() >= 3);
        assert!(ct.overrides().len() >= 2);

        // Verify specific values were parsed
        assert!(ct.get_content_type("test.png").is_some());
        assert!(ct.get_content_type("ppt/presentation.xml").is_some());
    }

    #[test]
    fn test_get_content_type_by_extension() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
            <Default Extension="png" ContentType="image/png"/>
            <Default Extension="jpeg" ContentType="image/jpeg"/>
            <Default Extension="emf" ContentType="image/x-emf"/>
        </Types>"#;

        let ct = ContentTypesPart::from_xml(xml).unwrap();

        // Test by extension (Default)
        assert_eq!(
            ct.get_content_type("ppt/media/image1.png"),
            Some("image/png")
        );
        assert_eq!(
            ct.get_content_type("ppt/media/image2.jpeg"),
            Some("image/jpeg")
        );
        assert_eq!(
            ct.get_content_type("ppt/media/image3.emf"),
            Some("image/x-emf")
        );
    }

    #[test]
    fn test_get_content_type_by_override() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
            <Default Extension="xml" ContentType="application/xml"/>
            <Override PartName="/ppt/slides/slide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
        </Types>"#;

        let ct = ContentTypesPart::from_xml(xml).unwrap();

        // Override takes precedence
        assert_eq!(
            ct.get_content_type("ppt/slides/slide1.xml"),
            Some("application/vnd.openxmlformats-officedocument.presentationml.slide+xml")
        );

        // Fallback to default for non-overridden paths
        assert_eq!(
            ct.get_content_type("ppt/other.xml"),
            Some("application/xml")
        );
    }

    #[test]
    fn test_get_content_type_not_found() {
        let ct = ContentTypesPart::new();
        assert_eq!(ct.get_content_type("unknown/file.xyz"), None);
    }
}
