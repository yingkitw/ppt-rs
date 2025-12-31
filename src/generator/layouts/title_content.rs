//! Title and content slide layouts

use super::common::{SlideXmlBuilder, generate_text_props, escape_xml};
use crate::generator::slide_content::SlideContent;
use crate::generator::shapes_xml::generate_shape_xml;
use crate::generator::charts::xml::generate_chart_frame_xml;
use crate::generator::constants::{
    TITLE_X, TITLE_Y, TITLE_WIDTH, TITLE_HEIGHT, TITLE_HEIGHT_BIG,
    CONTENT_X, CONTENT_Y_START, CONTENT_Y_START_BIG,
    CONTENT_WIDTH, CONTENT_HEIGHT, CONTENT_HEIGHT_BIG,
    TITLE_FONT_SIZE, CONTENT_FONT_SIZE,
};

/// Standard title and content layout
pub struct TitleContentLayout;

impl TitleContentLayout {
    /// Generate title and content slide XML
    pub fn generate(content: &SlideContent) -> String {
        let title_size = content.title_size.unwrap_or((TITLE_FONT_SIZE / 100) as u32) * 100;
        let content_size = content.content_size.unwrap_or((CONTENT_FONT_SIZE / 100) as u32) * 100;

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
            .add_title(2, TITLE_X, TITLE_Y, TITLE_WIDTH, TITLE_HEIGHT, &content.title, &title_props, "title");

        // Add table or bullets
        if let Some(ref table) = content.table {
            builder = builder.raw(&crate::generator::tables_xml::generate_table_xml(table, 3));
        } else if !content.bullets.is_empty() {
            // Use bullets with styles
            builder = builder.start_content_body(3, CONTENT_X, CONTENT_Y_START, CONTENT_WIDTH, CONTENT_HEIGHT);
            for bullet in &content.bullets {
                builder = builder.add_bullet_with_style(&bullet.text, &content_props, bullet.level, bullet.style);
            }
            builder = builder.end_content_body();
        } else if !content.content.is_empty() {
            // Fallback to plain content strings
            builder = builder.start_content_body(3, CONTENT_X, CONTENT_Y_START, CONTENT_WIDTH, CONTENT_HEIGHT);
            for bullet in &content.content {
                builder = builder.add_bullet_with_style(bullet, &content_props, 0, content.bullet_style);
            }
            builder = builder.end_content_body();
        }

        // Add shapes
        for (i, shape) in content.shapes.iter().enumerate() {
            builder = builder.raw("\n").raw(&generate_shape_xml(shape, (i + 10) as u32));
        }

        // Add image placeholders
        let image_start_id = 20 + content.shapes.len();
        for (i, image) in content.images.iter().enumerate() {
            builder = builder.raw(&format!(
                r#"
<p:sp>
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
                escape_xml(&image.filename),
                image.x,
                image.y,
                image.width,
                image.height,
                escape_xml(&image.filename)
            ));
        }

        // Add charts
        let chart_start_id = 30 + content.shapes.len() + content.images.len();
        for (i, chart) in content.charts.iter().enumerate() {
            let relationship_id = format!("rId{}", i + 2); // Start from rId2 (rId1 is usually for slide layout)
            let chart_xml = generate_chart_frame_xml(chart, chart_start_id + i, &relationship_id);
            builder = builder.raw("\n").raw(&chart_xml);
        }

        builder
            .end_sp_tree()
            .end_slide()
            .build()
    }
}

/// Title and big content layout (smaller title, larger content area)
pub struct TitleBigContentLayout;

impl TitleBigContentLayout {
    /// Generate title and big content slide XML
    pub fn generate(content: &SlideContent) -> String {
        let title_size = content.title_size.unwrap_or((TITLE_FONT_SIZE / 100) as u32) * 100;
        let content_size = content.content_size.unwrap_or((CONTENT_FONT_SIZE / 100) as u32) * 100;

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
            .add_title(2, TITLE_X, TITLE_Y, TITLE_WIDTH, TITLE_HEIGHT_BIG, &content.title, &title_props, "title");

        if !content.bullets.is_empty() {
            builder = builder.start_content_body(3, CONTENT_X, CONTENT_Y_START_BIG, CONTENT_WIDTH, CONTENT_HEIGHT_BIG);
            for bullet in &content.bullets {
                builder = builder.add_bullet_with_style(&bullet.text, &content_props, bullet.level, bullet.style);
            }
            builder = builder.end_content_body();
        } else if !content.content.is_empty() {
            builder = builder.start_content_body(3, CONTENT_X, CONTENT_Y_START_BIG, CONTENT_WIDTH, CONTENT_HEIGHT_BIG);
            for bullet in &content.content {
                builder = builder.add_bullet_with_style(bullet, &content_props, 0, content.bullet_style);
            }
            builder = builder.end_content_body();
        }

        // Add charts
        let chart_start_id = 30 + content.shapes.len() + content.images.len();
        for (i, chart) in content.charts.iter().enumerate() {
            let relationship_id = format!("rId{}", i + 2); // Start from rId2 (rId1 is usually for slide layout)
            let chart_xml = generate_chart_frame_xml(chart, chart_start_id + i, &relationship_id);
            builder = builder.raw("\n").raw(&chart_xml);
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
    fn test_title_content_layout() {
        let content = SlideContent::new("Test")
            .add_bullet("Point 1")
            .add_bullet("Point 2");
        let xml = TitleContentLayout::generate(&content);
        
        assert!(xml.contains("Test"));
        assert!(xml.contains("Point 1"));
        assert!(xml.contains("Point 2"));
    }

    #[test]
    fn test_title_big_content_layout() {
        let content = SlideContent::new("Big Content")
            .add_bullet("Item");
        let xml = TitleBigContentLayout::generate(&content);
        
        assert!(xml.contains("Big Content"));
        assert!(xml.contains("5668800")); // Larger content height
    }
}
