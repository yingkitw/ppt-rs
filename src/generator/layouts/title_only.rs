//! Title-only slide layout

use super::common::{SlideXmlBuilder, generate_text_props};
use crate::generator::slide_content::SlideContent;
use crate::generator::charts::xml::generate_chart_frame_xml;
use crate::generator::constants::{
    TITLE_X, TITLE_Y, TITLE_WIDTH, TITLE_HEIGHT, TITLE_FONT_SIZE,
};

/// Title-only slide layout generator
pub struct TitleOnlyLayout;

impl TitleOnlyLayout {
    /// Generate title-only slide XML
    pub fn generate(content: &SlideContent) -> String {
        let title_size = content.title_size.unwrap_or((TITLE_FONT_SIZE / 100) as u32) * 100;
        let title_props = generate_text_props(
            title_size,
            content.title_bold,
            content.title_italic,
            content.title_underline,
            content.title_color.as_deref(),
        );

        let mut builder = SlideXmlBuilder::new()
            .start_slide_with_bg()
            .start_sp_tree()
            .add_title(2, TITLE_X, TITLE_Y, TITLE_WIDTH, TITLE_HEIGHT, &content.title, &title_props, "title");

        // Add charts
        let chart_start_id = 10;
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
    fn test_title_only_layout() {
        let content = SlideContent::new("Test Title");
        let xml = TitleOnlyLayout::generate(&content);
        
        assert!(xml.contains("Test Title"));
        assert!(xml.contains("p:ph type=\"title\""));
    }
}
