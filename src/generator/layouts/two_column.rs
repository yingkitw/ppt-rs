//! Two-column slide layout

use super::common::{SlideXmlBuilder, generate_text_props, ShapePosition, TextContent};
use crate::generator::slide_content::SlideContent;

/// Two-column slide layout generator
pub struct TwoColumnLayout;

impl TwoColumnLayout {
    /// Generate two-column slide XML
    /// Bullets are automatically split between left and right columns
    pub fn generate(content: &SlideContent) -> String {
        let title_size = content.title_size.unwrap_or(44) * 100;
        let content_size = content.content_size.unwrap_or(24) * 100;

        let title_props = generate_text_props(
            title_size,
            content.title_bold,
            content.title_italic,
            content.title_underline,
            content.title_color.as_deref(),
        );

        let content_props = generate_text_props(
            content_size,
            content.content_bold,
            content.content_italic,
            content.content_underline,
            content.content_color.as_deref(),
        );

        let mut builder = SlideXmlBuilder::new()
            .start_slide_with_bg()
            .start_sp_tree()
            .add_title(2, ShapePosition::new(457200, 274638, 8230200, 914400), TextContent::new(&content.title, &title_props), "title");

        // Determine which bullets to use
        let use_styled_bullets = !content.bullets.is_empty();
        let bullet_count = if use_styled_bullets { content.bullets.len() } else { content.content.len() };
        
        if bullet_count > 0 {
            let mid = bullet_count.div_ceil(2);

            // Left column
            builder = builder.raw(r#"
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
<a:lstStyle/>
"#);

            if use_styled_bullets {
                for bullet in &content.bullets[..mid] {
                    builder = builder.add_bullet_with_style(&bullet.text, &content_props, bullet.level, bullet.style);
                }
            } else {
                for bullet in &content.content[..mid] {
                    builder = builder.add_bullet_with_style(bullet, &content_props, 0, content.bullet_style);
                }
            }
            builder = builder.raw("</p:txBody>\n</p:sp>\n");

            // Right column
            if mid < bullet_count {
                builder = builder.raw(r#"
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
<a:lstStyle/>
"#);

                if use_styled_bullets {
                    for bullet in &content.bullets[mid..] {
                        builder = builder.add_bullet_with_style(&bullet.text, &content_props, bullet.level, bullet.style);
                    }
                } else {
                    for bullet in &content.content[mid..] {
                        builder = builder.add_bullet_with_style(bullet, &content_props, 0, content.bullet_style);
                    }
                }
                builder = builder.raw("</p:txBody>\n</p:sp>\n");
            }
        }

        builder
            .end_sp_tree()
            .end_slide()
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_column_layout() {
        let content = SlideContent::new("Comparison")
            .add_bullet("Left 1")
            .add_bullet("Left 2")
            .add_bullet("Right 1")
            .add_bullet("Right 2");
        let xml = TwoColumnLayout::generate(&content);
        
        assert!(xml.contains("Left Content"));
        assert!(xml.contains("Right Content"));
        assert!(xml.contains("Left 1"));
        assert!(xml.contains("Right 1"));
    }
}
