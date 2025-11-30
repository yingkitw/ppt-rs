//! Hyperlink support for PPTX elements
//!
//! Provides hyperlink types for shapes, text, and images.

use crate::core::escape_xml;

/// Hyperlink action types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HyperlinkAction {
    /// Link to external URL
    Url(String),
    /// Link to another slide in the presentation
    Slide(u32),
    /// Link to first slide
    FirstSlide,
    /// Link to last slide
    LastSlide,
    /// Link to next slide
    NextSlide,
    /// Link to previous slide
    PreviousSlide,
    /// Link to end show
    EndShow,
    /// Link to email address
    Email { address: String, subject: Option<String> },
    /// Link to file
    File(String),
}

impl HyperlinkAction {
    /// Create URL hyperlink
    pub fn url(url: &str) -> Self {
        HyperlinkAction::Url(url.to_string())
    }

    /// Create slide hyperlink
    pub fn slide(slide_num: u32) -> Self {
        HyperlinkAction::Slide(slide_num)
    }

    /// Create email hyperlink
    pub fn email(address: &str) -> Self {
        HyperlinkAction::Email {
            address: address.to_string(),
            subject: None,
        }
    }

    /// Create email hyperlink with subject
    pub fn email_with_subject(address: &str, subject: &str) -> Self {
        HyperlinkAction::Email {
            address: address.to_string(),
            subject: Some(subject.to_string()),
        }
    }

    /// Create file hyperlink
    pub fn file(path: &str) -> Self {
        HyperlinkAction::File(path.to_string())
    }

    /// Get the relationship target for this action
    pub fn relationship_target(&self) -> String {
        match self {
            HyperlinkAction::Url(url) => url.clone(),
            HyperlinkAction::Slide(num) => format!("slide{}.xml", num),
            HyperlinkAction::FirstSlide => "ppaction://hlinkshowjump?jump=firstslide".to_string(),
            HyperlinkAction::LastSlide => "ppaction://hlinkshowjump?jump=lastslide".to_string(),
            HyperlinkAction::NextSlide => "ppaction://hlinkshowjump?jump=nextslide".to_string(),
            HyperlinkAction::PreviousSlide => "ppaction://hlinkshowjump?jump=previousslide".to_string(),
            HyperlinkAction::EndShow => "ppaction://hlinkshowjump?jump=endshow".to_string(),
            HyperlinkAction::Email { address, subject } => {
                let mut mailto = format!("mailto:{}", address);
                if let Some(subj) = subject {
                    mailto.push_str(&format!("?subject={}", subj));
                }
                mailto
            }
            HyperlinkAction::File(path) => format!("file:///{}", path.replace('\\', "/")),
        }
    }

    /// Check if this is an external link
    pub fn is_external(&self) -> bool {
        matches!(
            self,
            HyperlinkAction::Url(_) | HyperlinkAction::Email { .. } | HyperlinkAction::File(_)
        )
    }

    /// Get the action type for internal links
    pub fn action_type(&self) -> Option<&'static str> {
        match self {
            HyperlinkAction::FirstSlide => Some("ppaction://hlinkshowjump?jump=firstslide"),
            HyperlinkAction::LastSlide => Some("ppaction://hlinkshowjump?jump=lastslide"),
            HyperlinkAction::NextSlide => Some("ppaction://hlinkshowjump?jump=nextslide"),
            HyperlinkAction::PreviousSlide => Some("ppaction://hlinkshowjump?jump=previousslide"),
            HyperlinkAction::EndShow => Some("ppaction://hlinkshowjump?jump=endshow"),
            _ => None,
        }
    }
}

/// Hyperlink definition
#[derive(Clone, Debug)]
pub struct Hyperlink {
    /// The action to perform when clicked
    pub action: HyperlinkAction,
    /// Tooltip text shown on hover
    pub tooltip: Option<String>,
    /// Highlight click (visual feedback)
    pub highlight_click: bool,
    /// Relationship ID (set during XML generation)
    pub r_id: Option<String>,
}

impl Hyperlink {
    /// Create a new hyperlink
    pub fn new(action: HyperlinkAction) -> Self {
        Hyperlink {
            action,
            tooltip: None,
            highlight_click: true,
            r_id: None,
        }
    }

    /// Create URL hyperlink
    pub fn url(url: &str) -> Self {
        Self::new(HyperlinkAction::url(url))
    }

    /// Create slide hyperlink
    pub fn slide(slide_num: u32) -> Self {
        Self::new(HyperlinkAction::slide(slide_num))
    }

    /// Create email hyperlink
    pub fn email(address: &str) -> Self {
        Self::new(HyperlinkAction::email(address))
    }

    /// Set tooltip
    pub fn with_tooltip(mut self, tooltip: &str) -> Self {
        self.tooltip = Some(tooltip.to_string());
        self
    }

    /// Set highlight click
    pub fn with_highlight_click(mut self, highlight: bool) -> Self {
        self.highlight_click = highlight;
        self
    }

    /// Set relationship ID
    pub fn with_r_id(mut self, r_id: &str) -> Self {
        self.r_id = Some(r_id.to_string());
        self
    }
}

/// Generate hyperlink XML for text run
pub fn generate_text_hyperlink_xml(hyperlink: &Hyperlink, r_id: &str) -> String {
    let mut xml = format!(r#"<a:hlinkClick r:id="{}""#, r_id);

    if let Some(tooltip) = &hyperlink.tooltip {
        xml.push_str(&format!(r#" tooltip="{}""#, escape_xml(tooltip)));
    }

    if hyperlink.highlight_click {
        xml.push_str(r#" highlightClick="1""#);
    }

    // Add action for internal navigation
    if let Some(action) = hyperlink.action.action_type() {
        xml.push_str(&format!(r#" action="{}""#, action));
    }

    xml.push_str("/>");
    xml
}

/// Generate hyperlink XML for shape
pub fn generate_shape_hyperlink_xml(hyperlink: &Hyperlink, r_id: &str) -> String {
    let mut xml = format!(r#"<a:hlinkClick r:id="{}""#, r_id);

    if let Some(tooltip) = &hyperlink.tooltip {
        xml.push_str(&format!(r#" tooltip="{}""#, escape_xml(tooltip)));
    }

    if hyperlink.highlight_click {
        xml.push_str(r#" highlightClick="1""#);
    }

    if let Some(action) = hyperlink.action.action_type() {
        xml.push_str(&format!(r#" action="{}""#, action));
    }

    xml.push_str("/>");
    xml
}

/// Generate relationship XML for hyperlink
pub fn generate_hyperlink_relationship_xml(hyperlink: &Hyperlink, r_id: &str) -> String {
    let target = hyperlink.action.relationship_target();
    let target_mode = if hyperlink.action.is_external() {
        r#" TargetMode="External""#
    } else {
        ""
    };

    format!(
        r#"<Relationship Id="{}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" Target="{}"{}/>"#,
        r_id,
        escape_xml(&target),
        target_mode
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyperlink_url() {
        let link = Hyperlink::url("https://example.com");
        assert!(matches!(link.action, HyperlinkAction::Url(_)));
        assert!(link.action.is_external());
    }

    #[test]
    fn test_hyperlink_slide() {
        let link = Hyperlink::slide(3);
        assert!(matches!(link.action, HyperlinkAction::Slide(3)));
        assert!(!link.action.is_external());
    }

    #[test]
    fn test_hyperlink_email() {
        let link = Hyperlink::email("test@example.com");
        assert!(link.action.is_external());
        assert!(link.action.relationship_target().starts_with("mailto:"));
    }

    #[test]
    fn test_hyperlink_with_tooltip() {
        let link = Hyperlink::url("https://example.com")
            .with_tooltip("Click here");
        assert_eq!(link.tooltip, Some("Click here".to_string()));
    }

    #[test]
    fn test_hyperlink_action_types() {
        assert!(HyperlinkAction::FirstSlide.action_type().is_some());
        assert!(HyperlinkAction::LastSlide.action_type().is_some());
        assert!(HyperlinkAction::NextSlide.action_type().is_some());
        assert!(HyperlinkAction::PreviousSlide.action_type().is_some());
        assert!(HyperlinkAction::EndShow.action_type().is_some());
        assert!(HyperlinkAction::url("test").action_type().is_none());
    }

    #[test]
    fn test_generate_text_hyperlink_xml() {
        let link = Hyperlink::url("https://example.com")
            .with_tooltip("Example");
        let xml = generate_text_hyperlink_xml(&link, "rId1");
        assert!(xml.contains("hlinkClick"));
        assert!(xml.contains("rId1"));
        assert!(xml.contains("Example"));
    }

    #[test]
    fn test_generate_relationship_xml() {
        let link = Hyperlink::url("https://example.com");
        let xml = generate_hyperlink_relationship_xml(&link, "rId1");
        assert!(xml.contains("Relationship"));
        assert!(xml.contains("hyperlink"));
        assert!(xml.contains("External"));
    }

    #[test]
    fn test_email_with_subject() {
        let action = HyperlinkAction::email_with_subject("test@example.com", "Hello");
        let target = action.relationship_target();
        assert!(target.contains("mailto:"));
        assert!(target.contains("subject=Hello"));
    }
}
