//! Common XML templates and utilities for slide generation

use crate::generator::theme_xml::layout_rel_target;

/// Standard slide header with background
pub const SLIDE_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
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
<a:ext cx="9144000" cy="6858000"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="9144000" cy="6858000"/>
</a:xfrm>
</p:grpSpPr>"#;

/// Standard slide footer
pub const SLIDE_FOOTER: &str = r#"
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#;

/// Create slide relationships XML (default layout 2 = Title and Content)
pub fn create_slide_rels_xml() -> String {
    create_slide_rels_xml_for_layout(2)
}

/// Create slide relationships XML for a specific layout part.
pub fn create_slide_rels_xml_for_layout(layout_number: usize) -> String {
    let layout_target = layout_rel_target(layout_number);
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="{layout_target}"/>
</Relationships>"#
    )
}

/// Generate title shape XML
pub fn generate_title_shape(
    title_text: &str,
    title_props: &str,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    align: &str,
) -> String {
    format!(
        r#"<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="{x}" y="{y}"/>
<a:ext cx="{width}" cy="{height}"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0" anchor="ctr"/>
<a:lstStyle/>
<a:p>
<a:pPr algn="{align}"/>
<a:r>
{title_props}
<a:t>{title_text}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>"#
    )
}

