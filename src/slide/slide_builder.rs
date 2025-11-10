//! Slide builder for streamlined slide creation
//!
//! Simplifies the process of creating and adding slides to presentations

use crate::error::Result;
use crate::opc::packuri::PackURI;
use crate::parts::slide::SlidePart;
use crate::opc::part::Part;

/// Slide builder for creating slides with a fluent API
#[derive(Clone, Debug)]
pub struct SlideBuilder {
    /// Slide index
    index: usize,
    /// Include default placeholders
    with_placeholders: bool,
}

impl SlideBuilder {
    /// Create a new slide builder
    pub fn new(index: usize) -> Self {
        Self {
            index,
            with_placeholders: true,
        }
    }

    /// Set whether to include default placeholders
    pub fn with_placeholders(mut self, include: bool) -> Self {
        self.with_placeholders = include;
        self
    }

    /// Get the slide index
    pub fn index(&self) -> usize {
        self.index
    }

    /// Check if placeholders should be included
    pub fn has_placeholders(&self) -> bool {
        self.with_placeholders
    }

    /// Generate placeholder XML
    pub fn generate_placeholder_xml(&self) -> String {
        if !self.with_placeholders {
            return String::new();
        }

        r#"      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Title 1"/>
          <p:cNvSpPr>
            <a:spLocks noGrp="1"/>
          </p:cNvSpPr>
          <p:nvPr>
            <p:ph type="ctrTitle"/>
          </p:nvPr>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p/>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="3" name="Subtitle 2"/>
          <p:cNvSpPr>
            <a:spLocks noGrp="1"/>
          </p:cNvSpPr>
          <p:nvPr>
            <p:ph type="subTitle" idx="1"/>
          </p:nvPr>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p/>
        </p:txBody>
      </p:sp>"#
            .to_string()
    }

    /// Generate slide XML
    pub fn generate_slide_xml(&self) -> String {
        let placeholders = self.generate_placeholder_xml();
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr/>
{}
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>"#,
            placeholders
        )
    }

    /// Build a slide part
    pub fn build_part(&self, layout_part: &dyn Part) -> Result<SlidePart> {
        let slide_uri = PackURI::new(&format!("/ppt/slides/slide{}.xml", self.index + 1))?;
        let mut slide_part = SlidePart::new(slide_uri, layout_part)?;
        slide_part.update_xml(self.generate_slide_xml())?;
        Ok(slide_part)
    }
}

impl Default for SlideBuilder {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_builder_creation() {
        let builder = SlideBuilder::new(0);
        assert_eq!(builder.index(), 0);
        assert!(builder.has_placeholders());
    }

    #[test]
    fn test_slide_builder_without_placeholders() {
        let builder = SlideBuilder::new(1).with_placeholders(false);
        assert_eq!(builder.index(), 1);
        assert!(!builder.has_placeholders());
    }

    #[test]
    fn test_placeholder_xml_generation() {
        let builder = SlideBuilder::new(0);
        let xml = builder.generate_placeholder_xml();
        assert!(xml.contains("<p:sp>"));
        assert!(xml.contains("Title 1"));
        assert!(xml.contains("Subtitle 2"));
    }

    #[test]
    fn test_slide_xml_generation() {
        let builder = SlideBuilder::new(0);
        let xml = builder.generate_slide_xml();
        assert!(xml.contains(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#));
        assert!(xml.contains("<p:sld"));
        assert!(xml.contains("</p:sld>"));
    }

    #[test]
    fn test_slide_xml_without_placeholders() {
        let builder = SlideBuilder::new(0).with_placeholders(false);
        let xml = builder.generate_slide_xml();
        assert!(xml.contains("<p:sld"));
        assert!(!xml.contains("Title 1"));
    }
}
