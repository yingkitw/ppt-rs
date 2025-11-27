//! Title-only slide layout

use super::common::{SlideXmlBuilder, generate_text_props};
use crate::generator::slide_content::SlideContent;

/// Title-only slide layout generator
pub struct TitleOnlyLayout;

impl TitleOnlyLayout {
    /// Generate title-only slide XML
    pub fn generate(content: &SlideContent) -> String {
        let title_size = content.title_size.unwrap_or(44) * 100;
        let title_props = generate_text_props(
            title_size,
            content.title_bold,
            content.title_italic,
            content.title_underline,
            content.title_color.as_deref(),
        );

        SlideXmlBuilder::new()
            .start_slide_with_bg()
            .start_sp_tree()
            .add_title(2, 457200, 274638, 8230200, 1143000, &content.title, &title_props, "title")
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
