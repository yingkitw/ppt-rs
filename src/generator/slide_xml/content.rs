//! Additional content rendering (shapes, images, code blocks, connectors)

use crate::generator::slide_content::SlideContent;
use crate::generator::shapes_xml::generate_shape_xml;

/// Render additional content elements (shapes, images, code blocks, connectors, charts)
pub fn render_additional_content(xml: &mut String, content: &SlideContent, chart_rids: &[String]) {
    // Render shapes - use shape's fixed ID if set, otherwise auto-assign
    for (i, shape) in content.shapes.iter().enumerate() {
        xml.push('\n');
        let shape_id = shape.id.unwrap_or((i + 10) as u32);
        xml.push_str(&generate_shape_xml(shape, shape_id));
    }

    // Render image placeholders
    let image_start_id = 20 + content.shapes.len();
    for (i, image) in content.images.iter().enumerate() {
        xml.push('\n');
        xml.push_str(&generate_image_placeholder(image_start_id + i, image));
    }

    // Render code blocks with syntax highlighting
    let code_start_id = 30 + content.shapes.len() + content.images.len();
    for (i, code_block) in content.code_blocks.iter().enumerate() {
        xml.push('\n');
        xml.push_str(&generate_code_block(code_start_id + i, code_block));
    }

    // Render connectors
    let connector_start_id = 50 + content.shapes.len() + content.images.len() + content.code_blocks.len();
    for (i, connector) in content.connectors.iter().enumerate() {
        xml.push('\n');
        let id = connector_start_id + i;
        xml.push_str(&crate::generator::connectors::generate_connector_xml(connector, id));
    }

    // Render charts
    let chart_start_id = 100 + content.shapes.len() + content.images.len() + content.code_blocks.len() + content.connectors.len();
    for (i, chart) in content.charts.iter().enumerate() {
        if i < chart_rids.len() {
            xml.push('\n');
            let r_id = &chart_rids[i];
            xml.push_str(&crate::generator::charts::generate_chart_ref_xml(chart, r_id, chart_start_id + i));
        }
    }
}

/// Generate image placeholder XML
fn generate_image_placeholder(id: usize, image: &crate::generator::images::Image) -> String {
    let filename = &image.filename;
    let x = image.x;
    let y = image.y;
    let width = image.width;
    let height = image.height;
    
    format!(
        r#"<p:sp>
<p:nvSpPr>
<p:cNvPr id="{id}" name="Image Placeholder: {filename}"/>
<p:cNvSpPr/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="{x}" y="{y}"/>
<a:ext cx="{width}" cy="{height}"/>
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
<a:t>📷 {filename}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>"#
    )
}

/// Generate code block XML with syntax highlighting
fn generate_code_block(id: usize, code_block: &crate::generator::slide_content::CodeBlock) -> String {
    let highlighted_xml = crate::cli::syntax::generate_highlighted_code_xml(&code_block.code, &code_block.language);
    let x = code_block.x;
    let y = code_block.y;
    let width = code_block.width;
    let height = code_block.height;
    
    format!(
        r#"<p:sp>
<p:nvSpPr>
<p:cNvPr id="{id}" name="Code Block"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="{x}" y="{y}"/>
<a:ext cx="{width}" cy="{height}"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:solidFill><a:srgbClr val="002B36"/></a:solidFill>
<a:ln w="12700"><a:solidFill><a:srgbClr val="073642"/></a:solidFill></a:ln>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0" anchor="t" lIns="91440" tIns="45720" rIns="91440" bIns="45720"/>
<a:lstStyle/>
{highlighted_xml}</p:txBody>
</p:sp>"#
    )
}
