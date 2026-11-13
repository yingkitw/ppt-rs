//! Slide layout implementations

use super::common::{
    SLIDE_FOOTER, SLIDE_HEADER, TitleContent, TitlePlacement, generate_title_shape,
};
use super::content::render_additional_content;
use crate::core::escape_xml;
use crate::generator::layouts::ExtendedTextProps;
use crate::generator::media::MediaRelIds;
use crate::generator::slide::formatting::{generate_rich_text_runs, generate_text_props};
use crate::generator::slide_content::{BulletPoint, SlideContent};

/// Generate a bullet paragraph from a BulletPoint with full formatting
fn generate_bullet_paragraph_from_point(
    bullet: &BulletPoint,
    default_props: &ExtendedTextProps,
) -> String {
    let indent = 457200 + (bullet.level * 457200);
    let margin_left = bullet.level * 457200 + indent;
    let bullet_xml = bullet.style.to_xml();

    let rich_runs = generate_rich_text_runs(
        &bullet.text,
        default_props.size,
        default_props.bold,
        default_props.italic,
        default_props.color.as_deref(),
    );

    format!(
        r#"
<a:p>
<a:pPr lvl="{}" marL="{}" indent="-{}">
{}
</a:pPr>
{}
</a:p>"#,
        bullet.level,
        margin_left,
        indent,
        bullet_xml,
        rich_runs
    )
}

fn body_placeholder_geometry(content: &SlideContent) -> (u32, u32) {
    let y = content.content_body_y.unwrap_or(1_600_200);
    let height = (6_200_000u32).saturating_sub(y).max(800_000);
    (y, height)
}

fn push_body_bullets(
    xml: &mut String,
    content: &SlideContent,
    shape_id: u32,
    y: u32,
    height: u32,
) {
    let content_size = content.content_size.unwrap_or(28) * 100;
    let default_props = ExtendedTextProps::with_basic(
        content_size,
        content.content_bold,
        content.content_italic,
        false,
        content.content_color.as_deref(),
    );

    xml.push_str(&format!(
        r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="{shape_id}" name="Content"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr><p:ph type="body" idx="1"/></p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="{y}"/>
<a:ext cx="8230200" cy="{height}"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0"/>
<a:lstStyle/>"#
    ));

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
</p:sp>"#,
    );
}

/// Create a blank slide
pub fn create_blank_slide(
    content: &SlideContent,
    chart_rids: &[String],
    ink_rel_id: Option<&str>,
    media_rids: &[MediaRelIds],
) -> String {
    let mut xml = String::from(SLIDE_HEADER);
    render_additional_content(&mut xml, content, chart_rids, ink_rel_id, media_rids);
    xml.push_str(SLIDE_FOOTER);
    xml
}

/// Create a title-only slide
pub fn create_title_only_slide(
    content: &SlideContent,
    chart_rids: &[String],
    ink_rel_id: Option<&str>,
    media_rids: &[MediaRelIds],
) -> String {
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
        TitleContent::new(&title_text, &title_props),
        TitlePlacement::new(457_200, 274_638, 8_230_200, 1_143_000),
        "l",     // align left
        "title", // placeholder type — fill the layout's title slot
    );

    let mut xml = format!("{}\n{}", SLIDE_HEADER, title_shape);
    render_additional_content(&mut xml, content, chart_rids, ink_rel_id, media_rids);
    xml.push_str(SLIDE_FOOTER);
    xml
}

/// Create a section header slide (large centered title)
pub fn create_section_header_slide(
    content: &SlideContent,
    chart_rids: &[String],
    ink_rel_id: Option<&str>,
    media_rids: &[MediaRelIds],
) -> String {
    let title_size = content.title_size.unwrap_or(48) * 100;
    let title_props = generate_text_props(
        title_size,
        true,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );
    let title_text = escape_xml(&content.title);

    let title_shape = generate_title_shape(
        TitleContent::new(&title_text, &title_props),
        TitlePlacement::new(457_200, 1_600_200, 8_229_600, 1_828_800),
        "ctr",
        "title",
    );

    let mut xml = format!("{}\n{}", SLIDE_HEADER, title_shape);
    render_additional_content(&mut xml, content, chart_rids, ink_rel_id, media_rids);
    xml.push_str(SLIDE_FOOTER);
    xml
}

/// Create a centered title slide
pub fn create_centered_title_slide(
    content: &SlideContent,
    chart_rids: &[String],
    ink_rel_id: Option<&str>,
    media_rids: &[MediaRelIds],
) -> String {
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
        TitleContent::new(&title_text, &title_props),
        TitlePlacement::new(457_200, 2_743_200, 8_230_200, 1_371_600),
        "ctr",     // align center
        "ctrTitle",
    );

    let mut xml = format!("{}\n{}", SLIDE_HEADER, title_shape);
    render_additional_content(&mut xml, content, chart_rids, ink_rel_id, media_rids);
    xml.push_str(SLIDE_FOOTER);
    xml
}

/// Create a title and big content slide
pub fn create_title_and_big_content_slide(
    content: &SlideContent,
    chart_rids: &[String],
    ink_rel_id: Option<&str>,
    media_rids: &[MediaRelIds],
) -> String {
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

    // Title shape — fill the layout's title placeholder
    xml.push_str(&format!(
        r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr txBox="1"/>
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

    // Content — fill the layout's body placeholder (idx="1")
    if !content.bullets.is_empty() || !content.content.is_empty() {
        xml.push_str(
            r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Content"/>
<p:cNvSpPr txBox="1"/>
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
<a:bodyPr wrap="square" rtlCol="0"/>
<a:lstStyle/>"#,
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
                xml.push_str(&generate_bullet_paragraph_from_point(
                    bullet,
                    &default_props,
                ));
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
</p:sp>"#,
        );
    }

    render_additional_content(&mut xml, content, chart_rids, ink_rel_id, media_rids);
    xml.push_str(SLIDE_FOOTER);
    xml
}

/// Create a two-column slide
pub fn create_two_column_slide(
    content: &SlideContent,
    chart_rids: &[String],
    ink_rel_id: Option<&str>,
    media_rids: &[MediaRelIds],
) -> String {
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

    // Title — fill the layout's title placeholder
    xml.push_str(&format!(
        r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr txBox="1"/>
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
    let bullet_count = if use_styled_bullets {
        content.bullets.len()
    } else {
        content.content.len()
    };

    if bullet_count > 0 {
        let mid = bullet_count.div_ceil(2);

        // Left column — fill the layout's body placeholder idx="1"
        xml.push_str(
            r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Left Content"/>
<p:cNvSpPr txBox="1"/>
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
<a:bodyPr wrap="square" rtlCol="0"/>
<a:lstStyle/>"#,
        );

        if use_styled_bullets {
            for bullet in &content.bullets[..mid] {
                xml.push_str(&generate_bullet_paragraph_from_point(
                    bullet,
                    &default_props,
                ));
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
</p:sp>"#,
        );

        // Right column — fill the layout's body placeholder idx="2"
        if mid < bullet_count {
            xml.push_str(
                r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="Right Content"/>
<p:cNvSpPr txBox="1"/>
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
<a:bodyPr wrap="square" rtlCol="0"/>
<a:lstStyle/>"#,
            );

            if use_styled_bullets {
                for bullet in &content.bullets[mid..] {
                    xml.push_str(&generate_bullet_paragraph_from_point(
                        bullet,
                        &default_props,
                    ));
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
</p:sp>"#,
            );
        }
    }

    render_additional_content(&mut xml, content, chart_rids, ink_rel_id, media_rids);
    xml.push_str(SLIDE_FOOTER);
    xml
}

/// Create a title and content slide (most common layout)
pub fn create_title_and_content_slide(
    content: &SlideContent,
    chart_rids: &[String],
    ink_rel_id: Option<&str>,
    media_rids: &[MediaRelIds],
) -> String {
    let title_size = content.title_size.unwrap_or(44) * 100;

    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );
    let title_text = escape_xml(&content.title);

    let mut xml = String::from(SLIDE_HEADER);

    // Title — fill the layout's title placeholder
    xml.push_str(&format!(
        r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr txBox="1"/>
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

    if let Some(ref table) = content.table {
        xml.push('\n');
        xml.push_str(&crate::generator::table::generate_table_xml(table, 3));
    }

    if !content.bullets.is_empty() || !content.content.is_empty() {
        let (body_y, body_h) = body_placeholder_geometry(content);
        push_body_bullets(&mut xml, content, 3, body_y, body_h);
    }

    // Render additional content (shapes, images, code blocks, connectors)
    render_additional_content(&mut xml, content, chart_rids, ink_rel_id, media_rids);

    xml.push_str(SLIDE_FOOTER);
    xml
}
