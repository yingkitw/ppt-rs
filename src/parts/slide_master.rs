//! Slide master part
//!
//! Represents a slide master template (ppt/slideMasters/slideMasterN.xml).

use super::base::{ContentType, Part, PartType};
use crate::exc::PptxError;

/// Slide master part (ppt/slideMasters/slideMasterN.xml)
#[derive(Debug, Clone)]
pub struct SlideMasterPart {
    path: String,
    master_number: usize,
    name: String,
    theme_rel_id: String,
    layout_rel_ids: Vec<String>,
    xml_content: Option<String>,
}

impl SlideMasterPart {
    /// Create a new slide master part
    pub fn new(master_number: usize) -> Self {
        SlideMasterPart {
            path: format!("ppt/slideMasters/slideMaster{}.xml", master_number),
            master_number,
            name: "Office Theme".to_string(),
            theme_rel_id: "rId1".to_string(),
            layout_rel_ids: vec![],
            xml_content: None,
        }
    }

    /// Get master number
    pub fn master_number(&self) -> usize {
        self.master_number
    }

    /// Get master name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set master name
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Add a layout relationship ID
    pub fn add_layout_rel_id(&mut self, rel_id: impl Into<String>) {
        self.layout_rel_ids.push(rel_id.into());
    }

    /// Get layout relationship IDs
    pub fn layout_rel_ids(&self) -> &[String] {
        &self.layout_rel_ids
    }

    /// Set theme relationship ID
    pub fn set_theme_rel_id(&mut self, rel_id: impl Into<String>) {
        self.theme_rel_id = rel_id.into();
    }

    /// Get relative path for relationships
    pub fn rel_target(&self) -> String {
        format!("slideMasters/slideMaster{}.xml", self.master_number)
    }

    fn generate_xml(&self) -> String {
        let layout_ids: String = self
            .layout_rel_ids
            .iter()
            .map(|id| {
                format!(
                    r#"<p:sldLayoutId id="{}" r:id="{}"/>"#,
                    2147483649 + self.layout_rel_ids.iter().position(|x| x == id).unwrap() as u64,
                    id
                )
            })
            .collect::<Vec<_>>()
            .join("\n      ");

        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:bg>
      <p:bgRef idx="1001">
        <a:schemeClr val="bg1"/>
      </p:bgRef>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>
    </p:spTree>
  </p:cSld>
  <p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>
  <p:sldLayoutIdLst>
    {}</p:sldLayoutIdLst>
  <p:txStyles>
    <p:titleStyle/>
    <p:bodyStyle/>
    <p:otherStyle/>
  </p:txStyles>
</p:sldMaster>"#,
            if layout_ids.is_empty() {
                "".to_string()
            } else {
                format!("\n      {}\n  ", layout_ids)
            }
        )
    }
}

impl Part for SlideMasterPart {
    fn path(&self) -> &str {
        &self.path
    }

    fn part_type(&self) -> PartType {
        PartType::SlideMaster
    }

    fn content_type(&self) -> ContentType {
        ContentType::SlideMaster
    }

    fn to_xml(&self) -> Result<String, PptxError> {
        if let Some(ref xml) = self.xml_content {
            return Ok(xml.clone());
        }
        Ok(self.generate_xml())
    }

    fn from_xml(xml: &str) -> Result<Self, PptxError> {
        Ok(SlideMasterPart {
            path: "ppt/slideMasters/slideMaster1.xml".to_string(),
            master_number: 1,
            name: "Office Theme".to_string(),
            theme_rel_id: "rId1".to_string(),
            layout_rel_ids: vec![],
            xml_content: Some(xml.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_master_new() {
        let master = SlideMasterPart::new(1);
        assert_eq!(master.master_number(), 1);
        assert_eq!(master.path(), "ppt/slideMasters/slideMaster1.xml");
    }

    #[test]
    fn test_slide_master_name() {
        let mut master = SlideMasterPart::new(1);
        assert_eq!(master.name(), "Office Theme");
        master.set_name("Custom Theme");
        assert_eq!(master.name(), "Custom Theme");
    }

    #[test]
    fn test_slide_master_layouts() {
        let mut master = SlideMasterPart::new(1);
        master.add_layout_rel_id("rId2");
        master.add_layout_rel_id("rId3");
        assert_eq!(master.layout_rel_ids().len(), 2);
    }

    #[test]
    fn test_slide_master_to_xml() {
        let master = SlideMasterPart::new(1);
        let xml = master.to_xml().unwrap();
        assert!(xml.contains("p:sldMaster"));
        assert!(xml.contains("p:clrMap"));
    }

    #[test]
    fn test_slide_master_rel_target() {
        let master = SlideMasterPart::new(2);
        assert_eq!(master.rel_target(), "slideMasters/slideMaster2.xml");
    }
}
