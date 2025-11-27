//! Centered title slide layout

use super::common::{SlideXmlBuilder, generate_text_props};
use crate::generator::slide_content::SlideContent;

/// Centered title slide layout generator
pub struct CenteredTitleLayout;

impl CenteredTitleLayout {
    /// Generate centered title slide XML
    pub fn generate(content: &SlideContent) -> String {
        let title_size = content.title_size.unwrap_or(54) * 100;
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
            .add_centered_title(2, 457200, 2743200, 8230200, 1371600, &content.title, &title_props)
            .end_sp_tree()
            .end_slide()
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centered_title_layout() {
        let content = SlideContent::new("Centered Title");
        let xml = CenteredTitleLayout::generate(&content);
        
        assert!(xml.contains("Centered Title"));
        assert!(xml.contains("ctrTitle"));
        assert!(xml.contains("algn=\"ctr\""));
    }
}
