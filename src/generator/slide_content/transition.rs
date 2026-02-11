//! Slide transition types

use crate::core::ToXml;

/// Slide transition effects
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum TransitionType {
    #[default]
    None,
    Fade,
    Cut,
    Push,
    Wipe,
    Split,
    Reveal,
    Cover,
    Zoom,
}

impl TransitionType {
    /// Generate XML for the transition
    pub fn to_xml(&self) -> String {
        match self {
            TransitionType::None => String::new(),
            TransitionType::Cut => String::new(), // Default is cut/instant
            TransitionType::Fade => r#"<p:transition><p:fade/></p:transition>"#.to_string(),
            TransitionType::Push => r#"<p:transition><p:push dir="r"/></p:transition>"#.to_string(), // Default right
            TransitionType::Wipe => r#"<p:transition><p:wipe dir="r"/></p:transition>"#.to_string(), // Default right
            TransitionType::Split => r#"<p:transition><p:split dir="out" orient="horz"/></p:transition>"#.to_string(),
            TransitionType::Reveal => r#"<p:transition><p:reveal dir="r"/></p:transition>"#.to_string(),
            TransitionType::Cover => r#"<p:transition><p:cover dir="r"/></p:transition>"#.to_string(),
            TransitionType::Zoom => r#"<p:transition><p:zoom dir="in"/></p:transition>"#.to_string(),
        }
    }
}

impl ToXml for TransitionType {
    fn to_xml(&self) -> String {
        TransitionType::to_xml(self)
    }
}
