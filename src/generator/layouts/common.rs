//! Common utilities for slide XML generation

use crate::core::XmlWriter;

/// Standard slide dimensions in EMU
pub const SLIDE_WIDTH: u32 = 9144000;
pub const SLIDE_HEIGHT: u32 = 6858000;

/// XML declaration and namespaces
pub const XML_DECL: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#;
pub const SLIDE_NS: &str = r#"xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main""#;

/// Generate text run properties XML
pub fn generate_text_props(
    size: u32,
    bold: bool,
    italic: bool,
    underline: bool,
    color: Option<&str>,
) -> String {
    let mut props = format!(
        r#"<a:rPr lang="en-US" sz="{}" b="{}" i="{}" dirty="0""#,
        size,
        if bold { "1" } else { "0" },
        if italic { "1" } else { "0" }
    );

    if underline {
        props.push_str(r#" u="sng""#);
    }

    props.push('>');

    if let Some(hex_color) = color {
        let clean_color = hex_color.trim_start_matches('#').to_uppercase();
        props.push_str(&format!(
            r#"<a:solidFill><a:srgbClr val="{}"/></a:solidFill>"#,
            clean_color
        ));
    }

    props.push_str("</a:rPr>");
    props
}

/// Builder for slide XML with common structure
pub struct SlideXmlBuilder {
    writer: XmlWriter,
}

impl SlideXmlBuilder {
    pub fn new() -> Self {
        Self {
            writer: XmlWriter::new(),
        }
    }

    /// Start slide with background
    pub fn start_slide_with_bg(mut self) -> Self {
        self.writer.raw(XML_DECL);
        self.writer.raw("\n<p:sld ");
        self.writer.raw(SLIDE_NS);
        self.writer.raw(">\n<p:cSld>\n");
        self.writer.raw("<p:bg><p:bgRef idx=\"1001\"><a:schemeClr val=\"bg1\"/></p:bgRef></p:bg>\n");
        self
    }

    /// Start shape tree
    pub fn start_sp_tree(mut self) -> Self {
        self.writer.raw("<p:spTree>\n");
        self.writer.raw("<p:nvGrpSpPr><p:cNvPr id=\"1\" name=\"\"/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>\n");
        self.writer.raw(&format!(
            "<p:grpSpPr><a:xfrm><a:off x=\"0\" y=\"0\"/><a:ext cx=\"{}\" cy=\"{}\"/><a:chOff x=\"0\" y=\"0\"/><a:chExt cx=\"{}\" cy=\"{}\"/></a:xfrm></p:grpSpPr>\n",
            SLIDE_WIDTH, SLIDE_HEIGHT, SLIDE_WIDTH, SLIDE_HEIGHT
        ));
        self
    }

    /// Add title shape
    pub fn add_title(mut self, id: u32, x: u32, y: u32, cx: u32, cy: u32, text: &str, props: &str, ph_type: &str) -> Self {
        self.writer.raw(&format!(
            r#"<p:sp>
<p:nvSpPr>
<p:cNvPr id="{}" name="Title"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="{}"/></p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm><a:off x="{}" y="{}"/><a:ext cx="{}" cy="{}"/></a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
{}
<a:t>{}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>
"#,
            id, ph_type, x, y, cx, cy, props, escape_xml(text)
        ));
        self
    }

    /// Add centered title
    pub fn add_centered_title(mut self, id: u32, x: u32, y: u32, cx: u32, cy: u32, text: &str, props: &str) -> Self {
        self.writer.raw(&format!(
            r#"<p:sp>
<p:nvSpPr>
<p:cNvPr id="{}" name="Title"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="ctrTitle"/></p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm><a:off x="{}" y="{}"/><a:ext cx="{}" cy="{}"/></a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:pPr algn="ctr"/>
<a:r>
{}
<a:t>{}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>
"#,
            id, x, y, cx, cy, props, escape_xml(text)
        ));
        self
    }

    /// Start content body shape
    pub fn start_content_body(mut self, id: u32, x: u32, y: u32, cx: u32, cy: u32) -> Self {
        self.writer.raw(&format!(
            r#"<p:sp>
<p:nvSpPr>
<p:cNvPr id="{}" name="Content"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="body" idx="1"/></p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm><a:off x="{}" y="{}"/><a:ext cx="{}" cy="{}"/></a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
"#,
            id, x, y, cx, cy
        ));
        self
    }

    /// Add bullet paragraph
    pub fn add_bullet(mut self, text: &str, props: &str, level: u32) -> Self {
        self.writer.raw(&format!(
            r#"<a:p>
<a:pPr lvl="{}"/>
<a:r>
{}
<a:t>{}</a:t>
</a:r>
</a:p>
"#,
            level, props, escape_xml(text)
        ));
        self
    }

    /// End content body
    pub fn end_content_body(mut self) -> Self {
        self.writer.raw("</p:txBody>\n</p:sp>\n");
        self
    }

    /// Add raw XML
    pub fn raw(mut self, xml: &str) -> Self {
        self.writer.raw(xml);
        self
    }

    /// End shape tree
    pub fn end_sp_tree(mut self) -> Self {
        self.writer.raw("</p:spTree>\n");
        self
    }

    /// End slide
    pub fn end_slide(mut self) -> Self {
        self.writer.raw("</p:cSld>\n<p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr>\n</p:sld>");
        self
    }

    /// Build final XML string
    pub fn build(self) -> String {
        self.writer.finish()
    }
}

impl Default for SlideXmlBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Escape XML special characters
pub fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_text_props() {
        let props = generate_text_props(2400, true, false, false, Some("FF0000"));
        assert!(props.contains("b=\"1\""));
        assert!(props.contains("sz=\"2400\""));
        assert!(props.contains("FF0000"));
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("a & b"), "a &amp; b");
        assert_eq!(escape_xml("<tag>"), "&lt;tag&gt;");
    }

    #[test]
    fn test_slide_builder() {
        let xml = SlideXmlBuilder::new()
            .start_slide_with_bg()
            .start_sp_tree()
            .end_sp_tree()
            .end_slide()
            .build();
        
        assert!(xml.contains("p:sld"));
        assert!(xml.contains("p:spTree"));
    }
}
