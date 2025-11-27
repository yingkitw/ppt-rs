//! PPTX file generator - creates proper ZIP-based PPTX files

// Core XML generation modules
pub mod slide_content;
pub mod package_xml;
pub mod slide_xml;
pub mod theme_xml;
pub mod props_xml;

// Modular layout system
pub mod layouts;

// Re-export module for backward compatibility
pub mod xml;

// Builder and content modules
pub mod builder;

// Text module (modularized)
pub mod text;

pub mod shapes;
pub mod shapes_xml;
pub mod tables;
pub mod tables_xml;
pub mod images;
pub mod images_xml;

// Charts module (modularized)
#[path = "charts/mod.rs"]
pub mod charts;

pub use builder::{create_pptx, create_pptx_with_content};
pub use xml::{SlideContent, SlideLayout};
pub use text::{TextFormat, FormattedText, TextFrame, Paragraph, Run, TextAlign, TextAnchor};
pub use shapes::{Shape, ShapeType, ShapeFill, ShapeLine, emu_to_inches, inches_to_emu, cm_to_emu};
pub use shapes_xml::{generate_shape_xml, generate_shapes_xml, generate_connector_xml};
pub use tables::{Table, TableRow, TableCell, TableBuilder};
pub use images::{Image, ImageBuilder};
pub use images_xml::{generate_image_xml, generate_image_relationship, generate_image_content_type};
pub use charts::{Chart, ChartType, ChartSeries, ChartBuilder, generate_chart_xml};

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
