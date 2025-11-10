//! Text module - handles text formatting and layout

pub mod text;
pub mod fonts;
pub mod layout;
pub mod run;

pub use text::TextFrame;
pub use fonts::{Font, UnderlineStyle};
pub use layout::Paragraph;
pub use run::Run;

