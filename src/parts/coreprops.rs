//! Core properties part
//!
//! Represents docProps/core.xml with document metadata.

use super::base::{Part, PartType, ContentType};
use crate::exc::PptxError;
use crate::oxml::XmlParser;
use chrono::Utc;

/// Core properties part (docProps/core.xml)
#[derive(Debug, Clone)]
pub struct CorePropertiesPart {
    path: String,
    pub title: Option<String>,
    pub subject: Option<String>,
    pub creator: Option<String>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub last_modified_by: Option<String>,
    pub revision: Option<u32>,
    pub created: Option<String>,
    pub modified: Option<String>,
}

impl CorePropertiesPart {
    /// Create a new core properties part
    pub fn new() -> Self {
        let now = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        CorePropertiesPart {
            path: "docProps/core.xml".to_string(),
            title: None,
            subject: None,
            creator: Some("pptx-rs".to_string()),
            keywords: None,
            description: None,
            last_modified_by: Some("pptx-rs".to_string()),
            revision: Some(1),
            created: Some(now.clone()),
            modified: Some(now),
        }
    }

    /// Set title
    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set subject
    pub fn set_subject(&mut self, subject: &str) -> &mut Self {
        self.subject = Some(subject.to_string());
        self
    }

    /// Set creator
    pub fn set_creator(&mut self, creator: &str) -> &mut Self {
        self.creator = Some(creator.to_string());
        self
    }

    /// Set keywords
    pub fn set_keywords(&mut self, keywords: &str) -> &mut Self {
        self.keywords = Some(keywords.to_string());
        self
    }

    /// Set description
    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = Some(description.to_string());
        self
    }

    /// Update modified timestamp
    pub fn touch(&mut self) {
        self.modified = Some(Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());
        if let Some(ref mut rev) = self.revision {
            *rev += 1;
        }
    }

    fn escape_xml(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}

impl Default for CorePropertiesPart {
    fn default() -> Self {
        Self::new()
    }
}

impl Part for CorePropertiesPart {
    fn path(&self) -> &str {
        &self.path
    }

    fn part_type(&self) -> PartType {
        PartType::CoreProperties
    }

    fn content_type(&self) -> ContentType {
        ContentType::CoreProperties
    }

    fn to_xml(&self) -> Result<String, PptxError> {
        let mut elements = Vec::new();

        if let Some(ref title) = self.title {
            elements.push(format!("<dc:title>{}</dc:title>", Self::escape_xml(title)));
        }
        if let Some(ref subject) = self.subject {
            elements.push(format!("<dc:subject>{}</dc:subject>", Self::escape_xml(subject)));
        }
        if let Some(ref creator) = self.creator {
            elements.push(format!("<dc:creator>{}</dc:creator>", Self::escape_xml(creator)));
        }
        if let Some(ref keywords) = self.keywords {
            elements.push(format!("<cp:keywords>{}</cp:keywords>", Self::escape_xml(keywords)));
        }
        if let Some(ref description) = self.description {
            elements.push(format!("<dc:description>{}</dc:description>", Self::escape_xml(description)));
        }
        if let Some(ref last_modified_by) = self.last_modified_by {
            elements.push(format!("<cp:lastModifiedBy>{}</cp:lastModifiedBy>", Self::escape_xml(last_modified_by)));
        }
        if let Some(revision) = self.revision {
            elements.push(format!("<cp:revision>{}</cp:revision>", revision));
        }
        if let Some(ref created) = self.created {
            elements.push(format!(r#"<dcterms:created xsi:type="dcterms:W3CDTF">{}</dcterms:created>"#, created));
        }
        if let Some(ref modified) = self.modified {
            elements.push(format!(r#"<dcterms:modified xsi:type="dcterms:W3CDTF">{}</dcterms:modified>"#, modified));
        }

        let xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
{}
</cp:coreProperties>"#,
            elements.join("\n")
        );

        Ok(xml)
    }

    fn from_xml(xml: &str) -> Result<Self, PptxError> {
        let root = XmlParser::parse_str(xml)?;
        let mut part = CorePropertiesPart::new();

        part.title = root.find_descendant("title").map(|e| e.text_content()).filter(|s| !s.is_empty());
        part.subject = root.find_descendant("subject").map(|e| e.text_content()).filter(|s| !s.is_empty());
        part.creator = root.find_descendant("creator").map(|e| e.text_content()).filter(|s| !s.is_empty());
        part.keywords = root.find_descendant("keywords").map(|e| e.text_content()).filter(|s| !s.is_empty());
        part.description = root.find_descendant("description").map(|e| e.text_content()).filter(|s| !s.is_empty());
        part.last_modified_by = root.find_descendant("lastModifiedBy").map(|e| e.text_content()).filter(|s| !s.is_empty());
        part.revision = root.find_descendant("revision").and_then(|e| e.text_content().parse().ok());
        part.created = root.find_descendant("created").map(|e| e.text_content()).filter(|s| !s.is_empty());
        part.modified = root.find_descendant("modified").map(|e| e.text_content()).filter(|s| !s.is_empty());

        Ok(part)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_props_new() {
        let part = CorePropertiesPart::new();
        assert_eq!(part.path(), "docProps/core.xml");
        assert!(part.creator.is_some());
        assert!(part.created.is_some());
    }

    #[test]
    fn test_core_props_set_title() {
        let mut part = CorePropertiesPart::new();
        part.set_title("My Presentation");
        
        assert_eq!(part.title, Some("My Presentation".to_string()));
    }

    #[test]
    fn test_core_props_to_xml() {
        let mut part = CorePropertiesPart::new();
        part.set_title("Test Title");
        part.set_creator("Test Author");
        
        let xml = part.to_xml().unwrap();
        assert!(xml.contains("dc:title"));
        assert!(xml.contains("Test Title"));
        assert!(xml.contains("dc:creator"));
    }

    #[test]
    fn test_core_props_from_xml() {
        let xml = r#"<?xml version="1.0"?>
        <cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties"
                          xmlns:dc="http://purl.org/dc/elements/1.1/"
                          xmlns:dcterms="http://purl.org/dc/terms/">
            <dc:title>Parsed Title</dc:title>
            <dc:creator>Parsed Author</dc:creator>
            <cp:revision>5</cp:revision>
        </cp:coreProperties>"#;
        
        let part = CorePropertiesPart::from_xml(xml).unwrap();
        assert_eq!(part.title, Some("Parsed Title".to_string()));
        assert_eq!(part.creator, Some("Parsed Author".to_string()));
        assert_eq!(part.revision, Some(5));
    }

    #[test]
    fn test_core_props_touch() {
        let mut part = CorePropertiesPart::new();
        let original_rev = part.revision;
        
        part.touch();
        
        assert_eq!(part.revision, Some(original_rev.unwrap() + 1));
    }
}
