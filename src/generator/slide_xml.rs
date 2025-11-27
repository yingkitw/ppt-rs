//! Slide XML generation for different layouts

use super::slide_content::{SlideContent, SlideLayout};
use super::package_xml::escape_xml;
use super::shapes_xml::generate_shape_xml;

/// Generate text properties XML with formatting
fn generate_text_props(
    size: u32,
    bold: bool,
    italic: bool,
    underline: bool,
    color: Option<&str>,
) -> String {
    let mut props = format!(
        r#"<a:rPr lang="en-US" sz="{}" b="{}" i="{}""#,
        size,
        if bold { "1" } else { "0" },
        if italic { "1" } else { "0" }
    );

    if underline {
        props.push_str(r#" u="sng""#);
    }

    props.push('>');

    if let Some(hex_color) = color {
        props.push_str(&format!(
            r#"<a:solidFill><a:srgbClr val="{}"/></a:solidFill>"#,
            hex_color
        ));
    }

    props.push_str("</a:rPr>");
    props
}

/// Create simple slide XML
pub fn create_slide_xml(slide_num: usize, title: &str) -> String {
    let slide_title = if slide_num == 1 {
        title.to_string()
    } else {
        format!("Slide {slide_num}")
    };
    
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
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
<p:sp>
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
<a:p>
<a:r>
<a:rPr lang="en-US" smtClean="0"/>
<a:t>{}</a:t>
</a:r>
<a:endParaRPr lang="en-US"/>
</a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#,
        slide_title
    )
}

/// Create slide XML with content based on layout
pub fn create_slide_xml_with_content(_slide_num: usize, content: &SlideContent) -> String {
    match content.layout {
        SlideLayout::Blank => create_blank_slide(),
        SlideLayout::TitleOnly => create_title_only_slide(content),
        SlideLayout::CenteredTitle => create_centered_title_slide(content),
        SlideLayout::TitleAndBigContent => create_title_and_big_content_slide(content),
        SlideLayout::TwoColumn => create_two_column_slide(content),
        SlideLayout::TitleAndContent => create_title_and_content_slide(content),
    }
}

fn create_blank_slide() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
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
</p:grpSpPr>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#.to_string()
}

fn create_title_only_slide(content: &SlideContent) -> String {
    let title_size = content.title_size.unwrap_or(44) * 100;
    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );

    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
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
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="title"/></p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="274638"/>
<a:ext cx="8230200" cy="1143000"/>
</a:xfrm>
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
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#,
        title_props, escape_xml(&content.title)
    )
}

fn create_centered_title_slide(content: &SlideContent) -> String {
    let title_size = content.title_size.unwrap_or(54) * 100;
    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );

    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
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
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="ctrTitle"/></p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="2743200"/>
<a:ext cx="8230200" cy="1371600"/>
</a:xfrm>
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
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#,
        title_props, escape_xml(&content.title)
    )
}

fn create_title_and_big_content_slide(content: &SlideContent) -> String {
    let title_size = content.title_size.unwrap_or(44) * 100;
    let content_size = content.content_size.unwrap_or(28) * 100;

    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );

    let mut xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
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
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="title"/></p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="274638"/>
<a:ext cx="8230200" cy="914400"/>
</a:xfrm>
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
</p:sp>"#,
        title_props, escape_xml(&content.title)
    );

    if !content.content.is_empty() {
        xml.push_str(
            r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Content"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="body" idx="1"/></p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="1189200"/>
<a:ext cx="8230200" cy="5668800"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>"#
        );

        let content_props = generate_text_props(
            content_size,
            content.content_bold,
            content.content_italic,
            content.content_underline,
            content.content_color.as_deref(),
        );

        for bullet in content.content.iter() {
            xml.push_str(&format!(
                r#"
<a:p>
<a:pPr lvl="0"/>
<a:r>
{}
<a:t>{}</a:t>
</a:r>
</a:p>"#,
                content_props, escape_xml(bullet)
            ));
        }

        xml.push_str(
            r#"
</p:txBody>
</p:sp>"#
        );
    }

    xml.push_str(
        r#"
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#
    );

    xml
}

fn create_two_column_slide(content: &SlideContent) -> String {
    let title_size = content.title_size.unwrap_or(44) * 100;
    let content_size = content.content_size.unwrap_or(24) * 100;

    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );

    let mut xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
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
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="title"/></p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="274638"/>
<a:ext cx="8230200" cy="914400"/>
</a:xfrm>
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
</p:sp>"#,
        title_props, escape_xml(&content.title)
    );

    if !content.content.is_empty() {
        let content_props = generate_text_props(
            content_size,
            content.content_bold,
            content.content_italic,
            content.content_underline,
            content.content_color.as_deref(),
        );

        let mid = (content.content.len() + 1) / 2;
        let left_content = &content.content[..mid];
        let right_content = &content.content[mid..];

        // Left column
        xml.push_str(
            r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Left Content"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="body" idx="1"/></p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="1189200"/>
<a:ext cx="4115100" cy="5668800"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>"#
        );

        for bullet in left_content.iter() {
            xml.push_str(&format!(
                r#"
<a:p>
<a:pPr lvl="0"/>
<a:r>
{}
<a:t>{}</a:t>
</a:r>
</a:p>"#,
                content_props, escape_xml(bullet)
            ));
        }

        xml.push_str(
            r#"
</p:txBody>
</p:sp>"#
        );

        // Right column
        if !right_content.is_empty() {
            xml.push_str(
                r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="Right Content"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="body" idx="2"/></p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="4572300" y="1189200"/>
<a:ext cx="4115100" cy="5668800"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>"#
            );

            for bullet in right_content.iter() {
                xml.push_str(&format!(
                    r#"
<a:p>
<a:pPr lvl="0"/>
<a:r>
{}
<a:t>{}</a:t>
</a:r>
</a:p>"#,
                    content_props, escape_xml(bullet)
                ));
            }

            xml.push_str(
                r#"
</p:txBody>
</p:sp>"#
            );
        }
    }

    xml.push_str(
        r#"
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#
    );

    xml
}

fn create_title_and_content_slide(content: &SlideContent) -> String {
    let title_size = content.title_size.unwrap_or(44) * 100;
    let content_size = content.content_size.unwrap_or(28) * 100;

    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );

    let mut xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
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
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="title"/></p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="274638"/>
<a:ext cx="8230200" cy="1143000"/>
</a:xfrm>
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
</p:sp>"#,
        title_props, escape_xml(&content.title)
    );

    // Render table if present
    if let Some(ref table) = content.table {
        xml.push('\n');
        xml.push_str(&super::tables_xml::generate_table_xml(table, 3));
    } else if !content.content.is_empty() {
        // Render bullets if no table
        xml.push_str(
            r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Content"/>
<p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
<p:nvPr><p:ph type="body" idx="1"/></p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="1600200"/>
<a:ext cx="8230200" cy="4572000"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>"#
        );

        let content_props = generate_text_props(
            content_size,
            content.content_bold,
            content.content_italic,
            content.content_underline,
            content.content_color.as_deref(),
        );

        for bullet in content.content.iter() {
            xml.push_str(&format!(
                r#"
<a:p>
<a:pPr lvl="0"/>
<a:r>
{}
<a:t>{}</a:t>
</a:r>
</a:p>"#,
                content_props, escape_xml(bullet)
            ));
        }

        xml.push_str(
            r#"
</p:txBody>
</p:sp>"#
        );
    }

    // Render shapes if present
    // Start shape IDs after title (2) and content (3)
    for (i, shape) in content.shapes.iter().enumerate() {
        xml.push('\n');
        xml.push_str(&generate_shape_xml(shape, (i + 10) as u32));
    }

    // Note: Images require actual image data embedded in ppt/media/ and 
    // corresponding relationships. For now, we add a placeholder shape showing
    // where the image would be placed.
    let image_start_id = 20 + content.shapes.len();
    for (i, image) in content.images.iter().enumerate() {
        xml.push('\n');
        // Create a placeholder rectangle showing image location
        xml.push_str(&format!(
            r#"<p:sp>
<p:nvSpPr>
<p:cNvPr id="{}" name="Image Placeholder: {}"/>
<p:cNvSpPr/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="{}" y="{}"/>
<a:ext cx="{}" cy="{}"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:solidFill><a:srgbClr val="E0E0E0"/></a:solidFill>
<a:ln w="12700"><a:solidFill><a:srgbClr val="808080"/></a:solidFill></a:ln>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0" anchor="ctr"/>
<a:lstStyle/>
<a:p>
<a:pPr algn="ctr"/>
<a:r>
<a:rPr lang="en-US" sz="1400"/>
<a:t>📷 {}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>"#,
            image_start_id + i,
            image.filename,
            image.x,
            image.y,
            image.width,
            image.height,
            image.filename
        ));
    }

    xml.push_str(
        r#"
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#
    );

    xml
}

/// Create slide relationships XML
pub fn create_slide_rels_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>
</Relationships>"#.to_string()
}
