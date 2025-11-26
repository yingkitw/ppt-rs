//! PPTX file generator - creates proper ZIP-based PPTX files

mod builder;
mod xml;

pub use builder::{create_pptx, create_pptx_with_content};
pub use xml::SlideContent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_content_builder() {
        let slide = SlideContent::new("Title")
            .add_bullet("Point 1")
            .add_bullet("Point 2");
        
        assert_eq!(slide.title, "Title");
        assert_eq!(slide.content.len(), 2);
        assert_eq!(slide.content[0], "Point 1");
    }
}
