//! Slide layout XML generators
//!
//! Each layout is a separate module with atomic responsibility.

mod common;
mod blank;
mod title_only;
mod centered_title;
mod title_content;
mod two_column;

pub use common::{SlideXmlBuilder, generate_text_props};
pub use blank::BlankLayout;
pub use title_only::TitleOnlyLayout;
pub use centered_title::CenteredTitleLayout;
pub use title_content::{TitleContentLayout, TitleBigContentLayout};
pub use two_column::TwoColumnLayout;

use super::slide_content::{SlideContent, SlideLayout};

/// Generate slide XML based on layout type
pub fn create_slide_xml_for_layout(content: &SlideContent) -> String {
    match content.layout {
        SlideLayout::Blank => BlankLayout::generate(),
        SlideLayout::TitleOnly => TitleOnlyLayout::generate(content),
        SlideLayout::CenteredTitle => CenteredTitleLayout::generate(content),
        SlideLayout::TitleAndBigContent => TitleBigContentLayout::generate(content),
        SlideLayout::TwoColumn => TwoColumnLayout::generate(content),
        SlideLayout::TitleAndContent => TitleContentLayout::generate(content),
    }
}
