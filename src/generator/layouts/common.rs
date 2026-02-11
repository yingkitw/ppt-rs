//! Common utilities for slide XML generation

use crate::core::XmlWriter;
use crate::generator::constants::{
    SLIDE_WIDTH, SLIDE_HEIGHT,
};
use crate::generator::slide_content::BulletStyle;

/// XML declaration and namespaces
pub const XML_DECL: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#;
pub const SLIDE_NS: &str = r#"xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main""#;

/// Extended text properties for full formatting support
#[derive(Clone, Debug, Default)]
pub struct ExtendedTextProps {
    pub size: u32,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub subscript: bool,
    pub superscript: bool,
    pub color: Option<String>,
    pub highlight: Option<String>,
    pub font_family: Option<String>,
}

impl ExtendedTextProps {
    pub fn new(size: u32) -> Self {
        Self {
            size,
            ..Default::default()
        }
    }
    
    pub fn with_basic(size: u32, bold: bool, italic: bool, underline: bool, color: Option<&str>) -> Self {
        Self {
            size,
            bold,
            italic,
            underline,
            color: color.map(|c| c.trim_start_matches('#').to_uppercase()),
            ..Default::default()
        }
    }
    
    pub fn to_xml(&self) -> String {
        let mut attrs = format!(
            r#"<a:rPr lang="en-US" sz="{}" b="{}" i="{}" dirty="0""#,
            self.size,
            if self.bold { "1" } else { "0" },
            if self.italic { "1" } else { "0" }
        );

        if self.underline {
            attrs.push_str(r#" u="sng""#);
        }
        
        if self.strikethrough {
            attrs.push_str(r#" strike="sngStrike""#);
        }
        
        if self.subscript {
            attrs.push_str(r#" baseline="-25000""#);
        } else if self.superscript {
            attrs.push_str(r#" baseline="30000""#);
        }

        attrs.push('>');

        if let Some(ref hex_color) = self.color {
            let clean_color = hex_color.trim_start_matches('#').to_uppercase();
            attrs.push_str(&format!(
                r#"<a:solidFill><a:srgbClr val="{clean_color}"/></a:solidFill>"#
            ));
        }
        
        if let Some(ref highlight) = self.highlight {
            let clean_color = highlight.trim_start_matches('#').to_uppercase();
            attrs.push_str(&format!(
                r#"<a:highlight><a:srgbClr val="{clean_color}"/></a:highlight>"#
            ));
        }
        
        if let Some(ref font) = self.font_family {
            attrs.push_str(&format!(
                r#"<a:latin typeface="{font}"/><a:cs typeface="{font}"/>"#
            ));
        }

        attrs.push_str("</a:rPr>");
        attrs
    }
}

/// Generate text run properties XML
pub fn generate_text_props(
    size: u32,
    bold: bool,
    italic: bool,
    underline: bool,
    color: Option<&str>,
) -> String {
    ExtendedTextProps::with_basic(size, bold, italic, underline, color).to_xml()
}

/// Generate text run properties XML with extended formatting
pub fn generate_text_props_extended(props: &ExtendedTextProps) -> String {
    props.to_xml()
}

/// Shape position and size
#[derive(Clone, Copy, Debug)]
pub struct ShapePosition {
    pub x: u32,
    pub y: u32,
    pub cx: u32,
    pub cy: u32,
}

impl ShapePosition {
    pub fn new(x: u32, y: u32, cx: u32, cy: u32) -> Self {
        Self { x, y, cx, cy }
    }
}

/// Text content with formatting properties
#[derive(Clone, Debug)]
pub struct TextContent<'a> {
    pub text: &'a str,
    pub props: &'a str,
}

impl<'a> TextContent<'a> {
    pub fn new(text: &'a str, props: &'a str) -> Self {
        Self { text, props }
    }
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
            "<p:grpSpPr><a:xfrm><a:off x=\"0\" y=\"0\"/><a:ext cx=\"{SLIDE_WIDTH}\" cy=\"{SLIDE_HEIGHT}\"/><a:chOff x=\"0\" y=\"0\"/><a:chExt cx=\"{SLIDE_WIDTH}\" cy=\"{SLIDE_HEIGHT}\"/></a:xfrm></p:grpSpPr>\n"
        ));
        self
    }

    /// Add title shape
    pub fn add_title(mut self, id: u32, position: ShapePosition, content: TextContent<'_>, ph_type: &str) -> Self {
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
            id, ph_type, position.x, position.y, position.cx, position.cy, content.props, escape_xml(content.text)
        ));
        self
    }

    /// Add centered title
    pub fn add_centered_title(mut self, id: u32, position: ShapePosition, content: TextContent<'_>) -> Self {
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
            id, position.x, position.y, position.cx, position.cy, content.props, escape_xml(content.text)
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
    pub fn add_bullet(self, text: &str, props: &str, level: u32) -> Self {
        self.add_bullet_with_style(text, props, level, BulletStyle::Bullet)
    }
    
    /// Add bullet paragraph with specific style
    pub fn add_bullet_with_style(mut self, text: &str, props: &str, level: u32, style: BulletStyle) -> Self {
        let indent = 457200 + (level * 457200); // 0.5 inch base + 0.5 inch per level
        let margin_left = level * 457200 + indent;
        let bullet_xml = style.to_xml();
        
        self.writer.raw(&format!(
            r#"<a:p>
<a:pPr lvl="{}" marL="{}" indent="-{}">
{}
</a:pPr>
<a:r>
{}
<a:t>{}</a:t>
</a:r>
</a:p>
"#,
            level, margin_left, indent, bullet_xml, props, escape_xml(text)
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

pub use crate::core::escape_xml;

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
