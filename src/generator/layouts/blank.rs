//! Blank slide layout

use super::common::SlideXmlBuilder;

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
