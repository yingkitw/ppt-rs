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

pub use builder::{create_pptx, create_pptx_with_content, create_pptx_with_settings};
pub use notes_xml::{create_notes_xml, create_notes_rels_xml, create_notes_master_xml, create_notes_master_rels_xml};
pub use xml::{SlideContent, SlideLayout};
pub use slide_content::{CodeBlock, BulletStyle, BulletPoint, BulletTextFormat, TransitionType, Comment, CommentAuthor, CommentAuthorList, SlideComments, SlideSection, SectionManager, DigitalSignature, SignerInfo, HashAlgorithm, SignatureCommitment, InkAnnotations, InkStroke, InkPen, InkPoint, PenTip, SlideShowSettings, ShowType, PenColor, SlideRange, PrintSettings, HandoutLayout, PrintColorMode, PrintWhat, Orientation, TableMergeMap, MergeRegion, CellMergeState, EmbeddedFontList, EmbeddedFont, FontStyle, FontCharset, PresentationSettings};
pub use text::{TextFormat, FormattedText, TextFrame, Paragraph, Run, TextAlign, TextAnchor, TextDirection, RtlLanguage, RtlTextProps};
pub use shapes::{Shape, ShapeType, ShapeFill, ShapeLine, GradientFill as ShapeGradientFill, GradientStop as ShapeGradientStop, GradientDirection as ShapeGradientDirection, FillType, emu_to_inches, inches_to_emu, cm_to_emu};
pub use shapes_xml::{generate_shape_xml, generate_shapes_xml, generate_connector_xml};
pub use tables::{Table, TableRow, TableCell, TableBuilder, CellAlign, CellVAlign};
pub use images::{Image, ImageBuilder, ImageSource};
pub use images_xml::{generate_image_xml, generate_image_relationship, generate_image_content_type};
pub use charts::{Chart, ChartType, ChartSeries, ChartBuilder, generate_chart_part_xml, generate_chart_ref_xml};

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
    
    #[test]
    fn test_bullet_styles() {
        let bullet = BulletStyle::Bullet;
        assert!(bullet.to_xml().contains("buChar"));
        
        let number = BulletStyle::Number;
        assert!(number.to_xml().contains("arabicPeriod"));
        
        let letter = BulletStyle::LetterLower;
        assert!(letter.to_xml().contains("alphaLcPeriod"));
        
        let roman = BulletStyle::RomanUpper;
        assert!(roman.to_xml().contains("romanUcPeriod"));
        
        let custom = BulletStyle::Custom('★');
        assert!(custom.to_xml().contains("★"));
        
        let none = BulletStyle::None;
        assert!(none.to_xml().contains("buNone"));
    }
    
    #[test]
    fn test_numbered_slide() {
        let slide = SlideContent::new("Steps")
            .with_bullet_style(BulletStyle::Number)
            .add_bullet("First step")
            .add_bullet("Second step")
            .add_bullet("Third step");
        
        assert_eq!(slide.bullets.len(), 3);
        assert_eq!(slide.bullets[0].style, BulletStyle::Number);
    }
    
    #[test]
    fn test_mixed_bullet_styles() {
        let slide = SlideContent::new("Mixed")
            .add_numbered("Step 1")
            .add_numbered("Step 2")
            .add_lettered("Option a")
            .add_lettered("Option b");
        
        assert_eq!(slide.bullets.len(), 4);
        assert_eq!(slide.bullets[0].style, BulletStyle::Number);
        assert_eq!(slide.bullets[2].style, BulletStyle::LetterLower);
    }
    
    #[test]
    fn test_sub_bullets() {
        let slide = SlideContent::new("Hierarchy")
            .add_bullet("Main point")
            .add_sub_bullet("Sub point 1")
            .add_sub_bullet("Sub point 2")
            .add_bullet("Another main point");
        
        assert_eq!(slide.bullets.len(), 4);
        assert_eq!(slide.bullets[0].level, 0);
        assert_eq!(slide.bullets[1].level, 1);
        assert_eq!(slide.bullets[2].level, 1);
        assert_eq!(slide.bullets[3].level, 0);
    }
}
