//! Slide Master - Master slide template for presentations

/// Slide Master - defines the master template for slides
#[derive(Clone, Debug)]
pub struct SlideMaster {
    /// Layout IDs for the 11 predefined layouts
    layout_ids: Vec<u32>,
}

impl SlideMaster {
    /// Create a new slide master
    pub fn new() -> Self {
        Self {
            layout_ids: vec![],
        }
    }

    /// Add a layout ID to the master
    pub fn add_layout_id(&mut self, layout_id: u32) {
        self.layout_ids.push(layout_id);
    }

    /// Get layout IDs
    pub fn layout_ids(&self) -> &[u32] {
        &self.layout_ids
    }

    /// Generate slideMaster1.xml content
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
        xml.push('\n');
        xml.push_str(r#"<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">"#);
        xml.push('\n');

        // Common slide data
        xml.push_str(r#"  <p:cSld>"#);
        xml.push('\n');
        xml.push_str(r#"    <p:bg>"#);
        xml.push('\n');
        xml.push_str(r#"      <p:bgPr>"#);
        xml.push('\n');
        xml.push_str(r#"        <a:solidFill><a:schemeClr val="bg1"/></a:solidFill>"#);
        xml.push('\n');
        xml.push_str(r#"        <a:effectLst/>"#);
        xml.push('\n');
        xml.push_str(r#"      </p:bgPr>"#);
        xml.push('\n');
        xml.push_str(r#"      <p:bgPr><a:solidFill><a:schemeClr val="bg1"/></a:solidFill><a:effectLst/></p:bgPr>"#);
        xml.push('\n');
        xml.push_str(r#"    </p:bg>"#);
        xml.push('\n');
        xml.push_str(r#"    <p:spTree>"#);
        xml.push('\n');
        xml.push_str(r#"      <p:nvGrpSpPr>"#);
        xml.push('\n');
        xml.push_str(r#"        <p:cNvPr id="1" name="Title Master"/>"#);
        xml.push('\n');
        xml.push_str(r#"        <p:cNvGrpSpPr/>"#);
        xml.push('\n');
        xml.push_str(r#"        <p:nvPr/>"#);
        xml.push('\n');
        xml.push_str(r#"      </p:nvGrpSpPr>"#);
        xml.push('\n');
        xml.push_str(r#"      <p:grpSpPr>"#);
        xml.push('\n');
        xml.push_str(r#"        <a:xfrm><a:off x="0" y="0"/><a:ext cx="9144000" cy="6858000"/><a:chOff x="0" y="0"/><a:chExt cx="9144000" cy="6858000"/></a:xfrm>"#);
        xml.push('\n');
        xml.push_str(r#"      </p:grpSpPr>"#);
        xml.push('\n');
        xml.push_str(r#"    </p:spTree>"#);
        xml.push('\n');
        xml.push_str(r#"  </p:cSld>"#);
        xml.push('\n');

        // Color map
        xml.push_str(r#"  <p:clrMap accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" dk1="dk1" dk2="dk2" folHyperlink="folHyperlink" hyperlink="hyperlink" lt1="lt1" lt2="lt2"/>"#);
        xml.push('\n');

        // Slide layout ID list
        xml.push_str(r#"  <p:sldLayoutIdLst>"#);
        xml.push('\n');
        for (idx, layout_id) in self.layout_ids.iter().enumerate() {
            let rid = format!("rId{}", idx + 2); // rId2 onwards (rId1 is for master itself)
            xml.push_str(&format!(r#"    <p:sldLayoutId id="{}" r:id="{}"/>"#, layout_id, rid));
            xml.push('\n');
        }
        xml.push_str(r#"  </p:sldLayoutIdLst>"#);
        xml.push('\n');

        // Text styles
        xml.push_str(r#"  <p:txStyles>"#);
        xml.push('\n');
        xml.push_str(r#"    <p:titleStyle><a:lvl1pPr algn="ctr" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1"><a:defRPr sz="4400" kern="1200"><a:solidFill><a:schemeClr val="accent1"/></a:solidFill><a:latin typeface="+mj-lt"/><a:ea typeface="+mj-ea"/><a:cs typeface="+mj-cs"/></a:defRPr></a:lvl1pPr></p:titleStyle>"#);
        xml.push('\n');
        xml.push_str(r#"    <p:bodyStyle><a:lvl1pPr marL="0" marR="0" lvlIndent="0" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1"><a:defRPr sz="2400" kern="1200"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill><a:latin typeface="+mn-lt"/><a:ea typeface="+mn-ea"/><a:cs typeface="+mn-cs"/></a:defRPr></a:lvl1pPr></p:bodyStyle>"#);
        xml.push('\n');
        xml.push_str(r#"    <p:otherStyle><a:defPPr><a:defRPr lang="en-US" sz="1800" kern="1200"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill><a:latin typeface="+mn-lt"/><a:ea typeface="+mn-ea"/><a:cs typeface="+mn-cs"/></a:defRPr></a:defPPr></p:otherStyle>"#);
        xml.push('\n');
        xml.push_str(r#"  </p:txStyles>"#);
        xml.push('\n');

        xml.push_str(r#"</p:sldMaster>"#);
        xml
    }
}

impl Default for SlideMaster {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_master_creation() {
        let master = SlideMaster::new();
        assert_eq!(master.layout_ids().len(), 0);
    }

    #[test]
    fn test_slide_master_add_layout() {
        let mut master = SlideMaster::new();
        master.add_layout_id(256);
        master.add_layout_id(257);
        assert_eq!(master.layout_ids().len(), 2);
        assert_eq!(master.layout_ids()[0], 256);
        assert_eq!(master.layout_ids()[1], 257);
    }

    #[test]
    fn test_slide_master_to_xml() {
        let mut master = SlideMaster::new();
        master.add_layout_id(256);
        let xml = master.to_xml();
        
        assert!(xml.contains(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#));
        assert!(xml.contains(r#"<p:sldMaster"#));
        assert!(xml.contains(r#"<p:sldLayoutIdLst>"#));
        assert!(xml.contains(r#"<p:sldLayoutId id="256" r:id="rId2"/>"#));
        assert!(xml.contains(r#"</p:sldMaster>"#));
    }

    #[test]
    fn test_slide_master_xml_structure() {
        let master = SlideMaster::new();
        let xml = master.to_xml();
        
        // Check for required elements
        assert!(xml.contains(r#"<p:cSld>"#));
        assert!(xml.contains(r#"<p:clrMap"#));
        assert!(xml.contains(r#"<p:txStyles>"#));
        assert!(xml.contains(r#"<p:titleStyle>"#));
        assert!(xml.contains(r#"<p:bodyStyle>"#));
        assert!(xml.contains(r#"<p:otherStyle>"#));
    }
}
