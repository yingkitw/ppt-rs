//! Text formatting support for PPTX generation
//!
//! Modular text system with atomic capabilities:
//! - `format` - Text formatting options (bold, italic, color, etc.)
//! - `run` - A run of text with consistent formatting
//! - `paragraph` - A paragraph with alignment and spacing
//! - `frame` - Container for text content

mod format;
mod run;
mod paragraph;
mod frame;

pub use format::{TextFormat, FormattedText, color_to_xml};
pub use run::Run;
pub use paragraph::Paragraph;
pub use frame::TextFrame;

/// Text alignment options
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

impl TextAlign {
    /// Get the OOXML alignment value
    pub fn to_xml(&self) -> &'static str {
        match self {
            TextAlign::Left => "l",
            TextAlign::Center => "ctr",
            TextAlign::Right => "r",
            TextAlign::Justify => "just",
        }
    }
}

/// Vertical text anchor
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum TextAnchor {
    #[default]
    Top,
    Middle,
    Bottom,
}

impl TextAnchor {
    /// Get the OOXML anchor value
    pub fn to_xml(&self) -> &'static str {
        match self {
            TextAnchor::Top => "t",
            TextAnchor::Middle => "ctr",
            TextAnchor::Bottom => "b",
        }
    }
}

/// Escape XML special characters
pub(crate) fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_align() {
        assert_eq!(TextAlign::Left.to_xml(), "l");
        assert_eq!(TextAlign::Center.to_xml(), "ctr");
        assert_eq!(TextAlign::Right.to_xml(), "r");
        assert_eq!(TextAlign::Justify.to_xml(), "just");
    }

    #[test]
    fn test_text_anchor() {
        assert_eq!(TextAnchor::Top.to_xml(), "t");
        assert_eq!(TextAnchor::Middle.to_xml(), "ctr");
        assert_eq!(TextAnchor::Bottom.to_xml(), "b");
    }
}
