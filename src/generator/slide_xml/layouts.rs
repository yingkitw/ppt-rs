//! Slide layout implementations

use crate::generator::slide_content::{SlideContent, BulletStyle, BulletPoint, BulletTextFormat};
use crate::generator::package_xml::escape_xml;
use crate::generator::slide::formatting::generate_text_props;
use super::common::{SLIDE_HEADER, SLIDE_FOOTER, generate_title_shape};
use crate::generator::layouts::ExtendedTextProps;
use super::content::render_additional_content;

/// Generate text properties XML for a bullet, merging slide defaults with bullet-specific format
fn generate_bullet_text_props(
    default_props: &ExtendedTextProps,
    bullet_format: Option<&BulletTextFormat>,
) -> String {
    if let Some(fmt) = bullet_format {
        let props = ExtendedTextProps {
            size: fmt.font_size.map(|s| s * 100).unwrap_or(default_props.size),
            bold: fmt.bold || default_props.bold,
            italic: fmt.italic || default_props.italic,
            underline: fmt.underline || default_props.underline,
            strikethrough: fmt.strikethrough,
            subscript: fmt.subscript,
            superscript: fmt.superscript,
            color: fmt.color.clone().or_else(|| default_props.color.clone()),
            highlight: fmt.highlight.clone(),
            font_family: fmt.font_family.clone().or_else(|| default_props.font_family.clone()),
        };
        props.to_xml()
    } else {
        default_props.to_xml()
    }
}

/// Generate a bullet paragraph with style
#[allow(dead_code)]
fn generate_bullet_paragraph(text: &str, level: u32, style: BulletStyle, text_props: &str) -> String {
    let indent = 457200 + (level * 457200);
    let margin_left = level * 457200 + indent;
    let bullet_xml = style.to_xml();
    
    format!(
        r#"
<a:p>
<a:pPr lvl="{}" marL="{}" indent="-{}">
{}
</a:pPr>
<a:r>
{}
<a:t>{}</a:t>
</a:r>
</a:p>"#,
        level, margin_left, indent, bullet_xml, text_props, escape_xml(text)
    )
}

/// Generate a bullet paragraph from a BulletPoint with full formatting
fn generate_bullet_paragraph_from_point(
    bullet: &BulletPoint,
    default_props: &ExtendedTextProps,
) -> String {
    let indent = 457200 + (bullet.level * 457200);
    let margin_left = bullet.level * 457200 + indent;
    let bullet_xml = bullet.style.to_xml();
    let text_props = generate_bullet_text_props(default_props, bullet.format.as_ref());
    
    format!(
        r#"
<a:p>
<a:pPr lvl="{}" marL="{}" indent="-{}">
{}
</a:pPr>
<a:r>
{}
<a:t>{}</a:t>
</a:r>
</a:p>"#,
        bullet.level, margin_left, indent, bullet_xml, text_props, escape_xml(&bullet.text)
    )
}

/// Create a blank slide
pub fn create_blank_slide(content: &SlideContent, chart_rids: &[String]) -> String {
    let mut xml = String::from(SLIDE_HEADER);
    render_additional_content(&mut xml, content, chart_rids);
    xml.push_str(SLIDE_FOOTER);
    xml
}

/// Create a title-only slide
pub fn create_title_only_slide(content: &SlideContent, chart_rids: &[String]) -> String {
    let title_size = content.title_size.unwrap_or(44) * 100;
    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );
    let title_text = escape_xml(&content.title);

    let title_shape = generate_title_shape(
        &title_text,
        &title_props,
        457200,   // x
        274638,   // y
        8230200,  // width
        1143000,  // height
        "l",      // align left
    );

    let mut xml = format!("{}\n{}", SLIDE_HEADER, title_shape);
    render_additional_content(&mut xml, content, chart_rids);
    xml.push_str(SLIDE_FOOTER);
    xml
}

/// Create a centered title slide
pub fn create_centered_title_slide(content: &SlideContent, chart_rids: &[String]) -> String {
    let title_size = content.title_size.unwrap_or(54) * 100;
    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );
    let title_text = escape_xml(&content.title);

    let title_shape = generate_title_shape(
        &title_text,
        &title_props,
        457200,   // x
        2743200,  // y (centered vertically)
        8230200,  // width
        1371600,  // height
        "ctr",    // align center
    );

    let mut xml = format!("{}\n{}", SLIDE_HEADER, title_shape);
    render_additional_content(&mut xml, content, chart_rids);
    xml.push_str(SLIDE_FOOTER);
    xml
}

/// Create a title and big content slide
pub fn create_title_and_big_content_slide(content: &SlideContent, chart_rids: &[String]) -> String {
    let title_size = content.title_size.unwrap_or(44) * 100;
    let content_size = content.content_size.unwrap_or(28) * 100;

    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );
    let title_text = escape_xml(&content.title);

    let mut xml = String::from(SLIDE_HEADER);
    
    // Title shape
    xml.push_str(&format!(
        r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
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
<a:bodyPr wrap="square" rtlCol="0" anchor="ctr"/>
<a:lstStyle/>
<a:p>
<a:pPr algn="l"/>
<a:r>
{title_props}
<a:t>{title_text}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>"#
    ));

    // Content
    if !content.bullets.is_empty() || !content.content.is_empty() {
        xml.push_str(
            r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Content"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
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
<a:bodyPr wrap="square" rtlCol="0"/>
<a:lstStyle/>"#
        );

        let default_props = ExtendedTextProps::with_basic(
            content_size,
            content.content_bold,
            content.content_italic,
            false,
            content.content_color.as_deref(),
        );

        // Use styled bullets if available, otherwise use plain content
        if !content.bullets.is_empty() {
            for bullet in &content.bullets {
                xml.push_str(&generate_bullet_paragraph_from_point(bullet, &default_props));
            }
        } else {
            for bullet in &content.content {
                let bp = BulletPoint::new(bullet).with_style(content.bullet_style);
                xml.push_str(&generate_bullet_paragraph_from_point(&bp, &default_props));
            }
        }

        xml.push_str(
            r#"
</p:txBody>
</p:sp>"#
        );
    }

    render_additional_content(&mut xml, content, chart_rids);
    xml.push_str(SLIDE_FOOTER);
    xml
}

/// Create a two-column slide
pub fn create_two_column_slide(content: &SlideContent, chart_rids: &[String]) -> String {
    let title_size = content.title_size.unwrap_or(44) * 100;
    let content_size = content.content_size.unwrap_or(24) * 100;

    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );
    let title_text = escape_xml(&content.title);

    let mut xml = String::from(SLIDE_HEADER);
    
    // Title
    xml.push_str(&format!(
        r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
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
<a:bodyPr wrap="square" rtlCol="0" anchor="ctr"/>
<a:lstStyle/>
<a:p>
<a:pPr algn="l"/>
<a:r>
{title_props}
<a:t>{title_text}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>"#
    ));

    let default_props = ExtendedTextProps::with_basic(
        content_size,
        content.content_bold,
        content.content_italic,
        false,
        content.content_color.as_deref(),
    );

    // Determine which bullets to use
    let use_styled_bullets = !content.bullets.is_empty();
    let bullet_count = if use_styled_bullets { content.bullets.len() } else { content.content.len() };

    if bullet_count > 0 {
        let mid = bullet_count.div_ceil(2);

        // Left column
        xml.push_str(
            r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Left Content"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
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
<a:bodyPr wrap="square" rtlCol="0"/>
<a:lstStyle/>"#
        );

        if use_styled_bullets {
            for bullet in &content.bullets[..mid] {
                xml.push_str(&generate_bullet_paragraph_from_point(bullet, &default_props));
            }
        } else {
            for bullet in &content.content[..mid] {
                let bp = BulletPoint::new(bullet).with_style(content.bullet_style);
                xml.push_str(&generate_bullet_paragraph_from_point(&bp, &default_props));
            }
        }

        xml.push_str(
            r#"
</p:txBody>
</p:sp>"#
        );

        // Right column
        if mid < bullet_count {
            xml.push_str(
                r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="Right Content"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
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
<a:bodyPr wrap="square" rtlCol="0"/>
<a:lstStyle/>"#
            );

            if use_styled_bullets {
                for bullet in &content.bullets[mid..] {
                    xml.push_str(&generate_bullet_paragraph_from_point(bullet, &default_props));
                }
            } else {
                for bullet in &content.content[mid..] {
                    let bp = BulletPoint::new(bullet).with_style(content.bullet_style);
                    xml.push_str(&generate_bullet_paragraph_from_point(&bp, &default_props));
                }
            }

            xml.push_str(
                r#"
</p:txBody>
</p:sp>"#
            );
        }
    }

    render_additional_content(&mut xml, content, chart_rids);
    xml.push_str(SLIDE_FOOTER);
    xml
}

/// Create a title and content slide (most common layout)
pub fn create_title_and_content_slide(content: &SlideContent, chart_rids: &[String]) -> String {
    let title_size = content.title_size.unwrap_or(44) * 100;
    let content_size = content.content_size.unwrap_or(28) * 100;

    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );
    let title_text = escape_xml(&content.title);

    let mut xml = String::from(SLIDE_HEADER);
    
    // Title
    xml.push_str(&format!(
        r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
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
<a:bodyPr wrap="square" rtlCol="0" anchor="ctr"/>
<a:lstStyle/>
<a:p>
<a:pPr algn="l"/>
<a:r>
{title_props}
<a:t>{title_text}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>"#
    ));

    // Render table if present
    if let Some(ref table) = content.table {
        xml.push('\n');
        xml.push_str(&crate::generator::table::generate_table_xml(table, 3));
    } else if !content.bullets.is_empty() || !content.content.is_empty() {
        // Render bullets if no table
        xml.push_str(
            r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Content"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
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
<a:bodyPr wrap="square" rtlCol="0"/>
<a:lstStyle/>"#
        );

        let default_props = ExtendedTextProps::with_basic(
            content_size,
            content.content_bold,
            content.content_italic,
            false,
            content.content_color.as_deref(),
        );

        // Use styled bullets if available, otherwise use plain content
        if !content.bullets.is_empty() {
            for bullet in &content.bullets {
                xml.push_str(&generate_bullet_paragraph_from_point(bullet, &default_props));
            }
        } else {
            for bullet in &content.content {
                let bp = BulletPoint::new(bullet).with_style(content.bullet_style);
                xml.push_str(&generate_bullet_paragraph_from_point(&bp, &default_props));
            }
        }

        xml.push_str(
            r#"
</p:txBody>
</p:sp>"#
        );
    }

    // Render additional content (shapes, images, code blocks, connectors)
    render_additional_content(&mut xml, content, chart_rids);

    xml.push_str(SLIDE_FOOTER);
    xml
}
