//! PPTX file generator - creates proper ZIP-based PPTX files

// Layout constants (shared positioning and sizing values)
pub mod constants;

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

// Table module (modularized)
pub mod table;
// Keep old modules for backward compatibility
pub mod tables;
pub mod tables_xml;

pub mod images;
pub mod images_xml;

// Charts module (modularized)
#[path = "charts/mod.rs"]
pub mod charts;

// Notes module for speaker notes
pub mod notes_xml;

// Slide utilities (formatting, etc.)
pub mod slide;

// New element modules
pub mod connectors;
pub mod hyperlinks;
pub mod gradients;
pub mod media;

pub use builder::{create_pptx, create_pptx_with_content};
pub use notes_xml::{create_notes_xml, create_notes_rels_xml, create_notes_master_xml, create_notes_master_rels_xml};
pub use xml::{SlideContent, SlideLayout};
pub use slide_content::CodeBlock;
pub use text::{TextFormat, FormattedText, TextFrame, Paragraph, Run, TextAlign, TextAnchor};
pub use shapes::{Shape, ShapeType, ShapeFill, ShapeLine, GradientFill as ShapeGradientFill, GradientStop as ShapeGradientStop, GradientDirection as ShapeGradientDirection, FillType, emu_to_inches, inches_to_emu, cm_to_emu};
pub use shapes_xml::{generate_shape_xml, generate_shapes_xml, generate_connector_xml};
pub use tables::{Table, TableRow, TableCell, TableBuilder, CellAlign, CellVAlign};
pub use images::{Image, ImageBuilder};
pub use images_xml::{generate_image_xml, generate_image_relationship, generate_image_content_type};
pub use charts::{Chart, ChartType, ChartSeries, ChartBuilder, generate_chart_xml};

// New element exports
pub use connectors::{Connector, ConnectorType, ConnectorLine, ArrowType, ArrowSize, ConnectionSite, LineDash, generate_connector_xml as generate_cxn_xml};
pub use hyperlinks::{Hyperlink, HyperlinkAction, generate_text_hyperlink_xml, generate_shape_hyperlink_xml, generate_hyperlink_relationship_xml};
pub use gradients::{GradientFill, GradientType, GradientDirection, GradientStop, PresetGradients, generate_gradient_fill_xml};
pub use media::{Video, Audio, VideoFormat, AudioFormat, VideoOptions, AudioOptions, generate_video_xml, generate_audio_xml};

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
