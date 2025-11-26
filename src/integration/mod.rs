//! Integration module - connects all PPTX components

mod builders;
mod helpers;

pub use builders::{PresentationBuilder, SlideBuilder, PresentationMetadata};
pub use helpers::utils;
pub use helpers::enum_helpers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentation_builder() {
        let builder = PresentationBuilder::new("Test")
            .with_slides(5);
        assert_eq!(builder.slides, 5);
        assert_eq!(builder.title, "Test");
    }

    #[test]
    fn test_slide_builder() {
        let slide = SlideBuilder::new("Title")
            .with_content("Content");
        let (title, content) = slide.build();
        assert_eq!(title, "Title");
        assert_eq!(content, "Content");
    }

    #[test]
    fn test_format_size() {
        assert_eq!(utils::format_size(512), "512 B");
        assert_eq!(utils::format_size(1024), "1.0 KB");
        assert_eq!(utils::format_size(1024 * 1024), "1.0 MB");
    }
}
