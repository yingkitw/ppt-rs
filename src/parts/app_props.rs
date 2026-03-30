//! Application properties part
//!
//! Represents docProps/app.xml with application-specific metadata.

use super::base::{ContentType, Part, PartType};
use crate::exc::PptxError;

/// Application properties part (docProps/app.xml)
#[derive(Debug, Clone)]
pub struct AppPropertiesPart {
    path: String,
    pub application: String,
    pub app_version: String,
    pub company: Option<String>,
    pub presentation_format: String,
    pub slides: u32,
    pub paragraphs: u32,
    pub words: u32,
    pub notes: u32,
    pub hidden_slides: u32,
    pub mm_clips: u32,
    pub scale_crop: bool,
    pub links_up_to_date: bool,
    pub shared_doc: bool,
    pub hyperlinks_changed: bool,
    xml_content: Option<String>,
}

impl AppPropertiesPart {
    /// Create a new app properties part
    pub fn new() -> Self {
        AppPropertiesPart {
            path: "docProps/app.xml".to_string(),
            application: "Microsoft Office PowerPoint".to_string(),
            app_version: "16.0000".to_string(),
            company: None,
            presentation_format: "On-screen Show (4:3)".to_string(),
            slides: 0,
            paragraphs: 0,
            words: 0,
            notes: 0,
            hidden_slides: 0,
            mm_clips: 0,
            scale_crop: false,
            links_up_to_date: false,
            shared_doc: false,
            hyperlinks_changed: false,
            xml_content: None,
        }
    }

    /// Set application name
    pub fn set_application(&mut self, app: impl Into<String>) {
        self.application = app.into();
    }

    /// Set company
    pub fn set_company(&mut self, company: impl Into<String>) {
        self.company = Some(company.into());
    }

    /// Set slide count
    pub fn set_slides(&mut self, count: u32) {
        self.slides = count;
    }

    /// Set presentation format
    pub fn set_presentation_format(&mut self, format: impl Into<String>) {
        self.presentation_format = format.into();
    }

    fn generate_xml(&self) -> String {
        let company_xml = self
            .company
            .as_ref()
            .map(|c| format!("<Company>{}</Company>", c))
            .unwrap_or_default();

        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
  <Application>{}</Application>
  <AppVersion>{}</AppVersion>
  {}
  <PresentationFormat>{}</PresentationFormat>
  <Slides>{}</Slides>
  <Paragraphs>{}</Paragraphs>
  <Words>{}</Words>
  <Notes>{}</Notes>
  <HiddenSlides>{}</HiddenSlides>
  <MMClips>{}</MMClips>
  <ScaleCrop>{}</ScaleCrop>
  <LinksUpToDate>{}</LinksUpToDate>
  <SharedDoc>{}</SharedDoc>
  <HyperlinksChanged>{}</HyperlinksChanged>
</Properties>"#,
            self.application,
            self.app_version,
            company_xml,
            self.presentation_format,
            self.slides,
            self.paragraphs,
            self.words,
            self.notes,
            self.hidden_slides,
            self.mm_clips,
            self.scale_crop,
            self.links_up_to_date,
            self.shared_doc,
            self.hyperlinks_changed
        )
    }
}

impl Default for AppPropertiesPart {
    fn default() -> Self {
        Self::new()
    }
}

impl Part for AppPropertiesPart {
    fn path(&self) -> &str {
        &self.path
    }

    fn part_type(&self) -> PartType {
        PartType::ExtendedProperties
    }

    fn content_type(&self) -> ContentType {
        ContentType::ExtendedProperties
    }

    fn to_xml(&self) -> Result<String, PptxError> {
        if let Some(ref xml) = self.xml_content {
            return Ok(xml.clone());
        }
        Ok(self.generate_xml())
    }

    fn from_xml(xml: &str) -> Result<Self, PptxError> {
        let mut part = AppPropertiesPart::new();
        part.xml_content = Some(xml.to_string());
        Ok(part)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_props_new() {
        let props = AppPropertiesPart::new();
        assert_eq!(props.path(), "docProps/app.xml");
        assert_eq!(props.slides, 0);
    }

    #[test]
    fn test_app_props_set_company() {
        let mut props = AppPropertiesPart::new();
        props.set_company("Acme Corp");
        assert_eq!(props.company, Some("Acme Corp".to_string()));
    }

    #[test]
    fn test_app_props_set_slides() {
        let mut props = AppPropertiesPart::new();
        props.set_slides(10);
        assert_eq!(props.slides, 10);
    }

    #[test]
    fn test_app_props_to_xml() {
        let mut props = AppPropertiesPart::new();
        props.set_slides(5);
        props.set_company("Test Company");
        let xml = props.to_xml().unwrap();
        assert!(xml.contains("<Slides>5</Slides>"));
        assert!(xml.contains("<Company>Test Company</Company>"));
    }

    #[test]
    fn test_app_props_content_type() {
        let props = AppPropertiesPart::new();
        assert_eq!(props.content_type(), ContentType::ExtendedProperties);
    }
}
