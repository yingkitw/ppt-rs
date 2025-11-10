//! Core properties part - Dublin Core metadata

use crate::error::Result;
use crate::opc::constants::CONTENT_TYPE;
use crate::opc::part::{BasePart, Part};
use crate::opc::packuri::PackURI;
use crate::opc::relationships::Relationships;

/// Core properties part - contains Dublin Core metadata
pub struct CorePropertiesPart {
    base: BasePart,
    pub(crate) title: Option<String>,
    pub(crate) subject: Option<String>,
    pub(crate) creator: Option<String>,
    pub(crate) keywords: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) last_modified_by: Option<String>,
    pub(crate) revision: Option<u32>,
    pub(crate) created: Option<String>,
    pub(crate) modified: Option<String>,
}

impl CorePropertiesPart {
    /// Create a new core properties part
    pub fn new(partname: PackURI) -> Result<Self> {
        let base = BasePart::new(CONTENT_TYPE::OPC_CORE_PROPERTIES, partname)?;
        Ok(Self {
            base,
            title: None,
            subject: None,
            creator: None,
            keywords: None,
            description: None,
            last_modified_by: None,
            revision: Some(1),
            created: None,
            modified: None,
        })
    }

    /// Set the title
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    /// Get the title
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Set the creator
    pub fn set_creator(&mut self, creator: String) {
        self.creator = Some(creator);
    }

    /// Get the creator
    pub fn creator(&self) -> Option<&str> {
        self.creator.as_deref()
    }

    /// Set the subject
    pub fn set_subject(&mut self, subject: String) {
        self.subject = Some(subject);
    }

    /// Get the subject
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }
}

impl Part for CorePropertiesPart {
    fn content_type(&self) -> &str {
        self.base.content_type()
    }

    fn uri(&self) -> &PackURI {
        self.base.uri()
    }

    fn relationships(&self) -> &Relationships {
        self.base.relationships()
    }

    fn relationships_mut(&mut self) -> &mut Relationships {
        self.base.relationships_mut()
    }

    fn blob(&self) -> Result<Vec<u8>> {
        // Serialize core properties to XML
        let xml = self.to_xml()?;
        Ok(xml.as_bytes().to_vec())
    }

    fn to_xml(&self) -> Result<String> {
        // Generate core properties XML
        let mut xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties"
                   xmlns:dc="http://purl.org/dc/elements/1.1/"
                   xmlns:dcterms="http://purl.org/dc/terms/"
                   xmlns:dcmitype="http://purl.org/dc/dcmitype/"
                   xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">"#.to_string();
        
        if let Some(ref title) = self.title {
            xml.push_str(&format!("\n  <dc:title>{}</dc:title>", escape_xml(title)));
        }
        if let Some(ref subject) = self.subject {
            xml.push_str(&format!("\n  <dc:subject>{}</dc:subject>", escape_xml(subject)));
        }
        if let Some(ref creator) = self.creator {
            xml.push_str(&format!("\n  <dc:creator>{}</dc:creator>", escape_xml(creator)));
        }
        if let Some(ref keywords) = self.keywords {
            xml.push_str(&format!("\n  <cp:keywords>{}</cp:keywords>", escape_xml(keywords)));
        }
        if let Some(ref description) = self.description {
            xml.push_str(&format!("\n  <dc:description>{}</dc:description>", escape_xml(description)));
        }
        if let Some(ref last_modified_by) = self.last_modified_by {
            xml.push_str(&format!("\n  <cp:lastModifiedBy>{}</cp:lastModifiedBy>", escape_xml(last_modified_by)));
        }
        if let Some(revision) = self.revision {
            xml.push_str(&format!("\n  <cp:revision>{}</cp:revision>", revision));
        }
        if let Some(ref created) = self.created {
            xml.push_str(&format!("\n  <dcterms:created xsi:type=\"dcterms:W3CDTF\">{}</dcterms:created>", escape_xml(created)));
        }
        if let Some(ref modified) = self.modified {
            xml.push_str(&format!("\n  <dcterms:modified xsi:type=\"dcterms:W3CDTF\">{}</dcterms:modified>", escape_xml(modified)));
        }
        
        xml.push_str("\n</cp:coreProperties>");
        Ok(xml)
    }

    fn from_xml<R: std::io::Read>(mut reader: R) -> Result<Self> {
        let mut content = String::new();
        reader.read_to_string(&mut content)
            .map_err(|e| crate::error::PptError::ValueError(format!("Failed to read XML: {}", e)))?;
        
        // Parse core properties XML
        let mut part = Self::new(PackURI::new("/docProps/core.xml")?)?;
        
        // Extract values using regex (simplified parsing)
        let title_re = regex::Regex::new(r#"<dc:title>([^<]+)</dc:title>"#).ok();
        let subject_re = regex::Regex::new(r#"<dc:subject>([^<]+)</dc:subject>"#).ok();
        let creator_re = regex::Regex::new(r#"<dc:creator>([^<]+)</dc:creator>"#).ok();
        let keywords_re = regex::Regex::new(r#"<cp:keywords>([^<]+)</cp:keywords>"#).ok();
        let description_re = regex::Regex::new(r#"<dc:description>([^<]+)</dc:description>"#).ok();
        let last_modified_by_re = regex::Regex::new(r#"<cp:lastModifiedBy>([^<]+)</cp:lastModifiedBy>"#).ok();
        let revision_re = regex::Regex::new(r#"<cp:revision>(\d+)</cp:revision>"#).ok();
        
        if let Some(re) = title_re {
            if let Some(cap) = re.captures(&content) {
                if let Some(title) = cap.get(1) {
                    part.set_title(unescape_xml(title.as_str()));
                }
            }
        }
        if let Some(re) = subject_re {
            if let Some(cap) = re.captures(&content) {
                if let Some(subject) = cap.get(1) {
                    part.set_subject(unescape_xml(subject.as_str()));
                }
            }
        }
        if let Some(re) = creator_re {
            if let Some(cap) = re.captures(&content) {
                if let Some(creator) = cap.get(1) {
                    part.set_creator(unescape_xml(creator.as_str()));
                }
            }
        }
        if let Some(re) = keywords_re {
            if let Some(cap) = re.captures(&content) {
                if let Some(keywords) = cap.get(1) {
                    part.keywords = Some(unescape_xml(keywords.as_str()));
                }
            }
        }
        if let Some(re) = description_re {
            if let Some(cap) = re.captures(&content) {
                if let Some(description) = cap.get(1) {
                    part.description = Some(unescape_xml(description.as_str()));
                }
            }
        }
        if let Some(re) = last_modified_by_re {
            if let Some(cap) = re.captures(&content) {
                if let Some(lmb) = cap.get(1) {
                    part.last_modified_by = Some(unescape_xml(lmb.as_str()));
                }
            }
        }
        if let Some(re) = revision_re {
            if let Some(cap) = re.captures(&content) {
                if let Some(rev_str) = cap.get(1) {
                    if let Ok(rev) = rev_str.as_str().parse::<u32>() {
                        part.revision = Some(rev);
                    }
                }
            }
        }
        
        Ok(part)
    }
}

// Helper functions for XML escaping
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn unescape_xml(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

