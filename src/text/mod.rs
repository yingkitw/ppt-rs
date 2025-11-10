//! Text module - handles text formatting and layout

pub mod text;
pub mod fonts;
pub mod layout;
pub mod run;
pub mod rtl_support;

pub use text::TextFrame;
pub use fonts::{Font, UnderlineStyle};
pub use layout::Paragraph;
pub use run::Run;
pub use rtl_support::{
    RTLLanguage, TextDirection, RTLTextConfig, RTLParagraph, ParagraphAlignment,
};

