//! Text run functionality with hyperlink support

use crate::shapes::hyperlink::Hyperlink;
use crate::text::fonts::Font;

/// A run of text with consistent formatting
/// 
/// A run is a contiguous sequence of text with the same formatting properties.
/// Runs can have hyperlinks attached to them.
pub struct Run {
    text: String,
    font: Font,
    hyperlink: Option<Hyperlink>,
}

impl Run {
    /// Create a new text run
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            font: Font::new(),
            hyperlink: None,
        }
    }

    /// Get the text content
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the text content
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// Get the font
    pub fn font(&self) -> &Font {
        &self.font
    }

    /// Get mutable font
    pub fn font_mut(&mut self) -> &mut Font {
        &mut self.font
    }

    /// Get the hyperlink
    pub fn hyperlink(&self) -> Option<&Hyperlink> {
        self.hyperlink.as_ref()
    }

    /// Add a hyperlink to this run
    pub fn add_hyperlink(&mut self, hyperlink: Hyperlink) {
        self.hyperlink = Some(hyperlink);
    }

    /// Remove the hyperlink from this run
    pub fn remove_hyperlink(&mut self) {
        self.hyperlink = None;
    }

    /// Check if this run has a hyperlink
    pub fn has_hyperlink(&self) -> bool {
        self.hyperlink.is_some()
    }

    /// Get the hyperlink address
    pub fn hyperlink_address(&self) -> Option<&str> {
        self.hyperlink.as_ref().and_then(|h| h.address())
    }

    /// Set the hyperlink address
    pub fn set_hyperlink_address(&mut self, address: &str) {
        if let Some(ref mut h) = self.hyperlink {
            h.set_address(Some(address.to_string()));
        } else {
            let mut h = Hyperlink::with_address(address.to_string());
            self.hyperlink = Some(h);
        }
    }

    /// Get the hyperlink screen tip
    pub fn hyperlink_screen_tip(&self) -> Option<&str> {
        self.hyperlink.as_ref().and_then(|h| h.screen_tip())
    }

    /// Set the hyperlink screen tip
    pub fn set_hyperlink_screen_tip(&mut self, screen_tip: &str) {
        if let Some(ref mut h) = self.hyperlink {
            h.set_screen_tip(Some(screen_tip.to_string()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_creation() {
        let run = Run::new("Hello World");
        assert_eq!(run.text(), "Hello World");
        assert!(!run.has_hyperlink());
    }

    #[test]
    fn test_run_set_text() {
        let mut run = Run::new("Initial");
        run.set_text("Updated");
        assert_eq!(run.text(), "Updated");
    }

    #[test]
    fn test_run_add_hyperlink() {
        let mut run = Run::new("Click here");
        let hyperlink = Hyperlink::with_address("https://example.com".to_string());
        run.add_hyperlink(hyperlink);
        
        assert!(run.has_hyperlink());
        assert_eq!(run.hyperlink_address(), Some("https://example.com"));
    }

    #[test]
    fn test_run_remove_hyperlink() {
        let mut run = Run::new("Click here");
        let hyperlink = Hyperlink::with_address("https://example.com".to_string());
        run.add_hyperlink(hyperlink);
        
        assert!(run.has_hyperlink());
        run.remove_hyperlink();
        assert!(!run.has_hyperlink());
    }

    #[test]
    fn test_run_hyperlink_screen_tip() {
        let mut run = Run::new("Click here");
        let mut hyperlink = Hyperlink::with_address("https://example.com".to_string());
        hyperlink.set_screen_tip(Some("Go to Example".to_string()));
        run.add_hyperlink(hyperlink);
        
        assert_eq!(run.hyperlink_screen_tip(), Some("Go to Example"));
    }

    #[test]
    fn test_run_font_access() {
        let mut run = Run::new("Text");
        run.font_mut().set_name("Arial".to_string());
        assert_eq!(run.font().name(), "Arial");
    }
}
