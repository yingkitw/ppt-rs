//! Blank slide layout

use super::common::SlideXmlBuilder;
use crate::generator::slide_content::SlideContent;
use crate::generator::charts::xml::generate_chart_frame_xml;

/// Blank slide layout generator
pub struct BlankLayout;

impl BlankLayout {
    /// Generate blank slide XML
    pub fn generate() -> String {
        SlideXmlBuilder::new()
            .start_slide_with_bg()
            .start_sp_tree()
            .end_sp_tree()
            .end_slide()
            .build()
    }

    /// Generate blank slide XML with chart support
    pub fn generate_with_content(content: &SlideContent) -> String {
        let mut builder = SlideXmlBuilder::new()
            .start_slide_with_bg()
            .start_sp_tree();

        // Add charts
        let chart_start_id = 2;
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
    fn test_blank_layout() {
        let xml = BlankLayout::generate();
        assert!(xml.contains("p:sld"));
        assert!(xml.contains("p:spTree"));
        assert!(!xml.contains("p:ph type=\"title\""));
    }
}
