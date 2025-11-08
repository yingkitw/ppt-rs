//! Text run functionality with hyperlink support

use crate::shapes::hyperlink::Hyperlink;
use crate::text::fonts::Font;
use crate::dml::color::RGBColor;

/// A run of text with consistent formatting
/// 
/// A run is a contiguous sequence of text with the same formatting properties.
/// Runs can have hyperlinks attached to them.
pub struct Run {
    text: String,
    font: Font,
    hyperlink: Option<Hyperlink>,
    bold: bool,
    italic: bool,
    underline: bool,
    color: Option<RGBColor>,
    font_size: Option<u32>,
}

impl Run {
    /// Create a new text run
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            font: Font::new(),
            hyperlink: None,
            bold: false,
            italic: false,
            underline: false,
            color: None,
            font_size: None,
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

    /// Set bold formatting
    pub fn set_bold(&mut self, bold: bool) {
        self.bold = bold;
    }

    /// Get bold formatting
    pub fn is_bold(&self) -> bool {
        self.bold
    }

    /// Set italic formatting
    pub fn set_italic(&mut self, italic: bool) {
        self.italic = italic;
    }

    /// Get italic formatting
    pub fn is_italic(&self) -> bool {
        self.italic
    }

    /// Set underline formatting
    pub fn set_underline(&mut self, underline: bool) {
        self.underline = underline;
    }

    /// Get underline formatting
    pub fn is_underline(&self) -> bool {
        self.underline
    }

    /// Set text color
    pub fn set_color(&mut self, color: RGBColor) {
        self.color = Some(color);
    }

    /// Get text color
    pub fn color(&self) -> Option<&RGBColor> {
        self.color.as_ref()
    }

    /// Set font size (in points)
    pub fn set_font_size(&mut self, size: u32) {
        self.font_size = Some(size);
    }

    /// Get font size
    pub fn font_size(&self) -> Option<u32> {
        self.font_size
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
    fn test_run_bold() {
        let mut run = Run::new("Bold text");
        assert!(!run.is_bold());
        run.set_bold(true);
        assert!(run.is_bold());
    }

    #[test]
    fn test_run_italic() {
        let mut run = Run::new("Italic text");
        assert!(!run.is_italic());
        run.set_italic(true);
        assert!(run.is_italic());
    }

    #[test]
    fn test_run_underline() {
        let mut run = Run::new("Underlined text");
        assert!(!run.is_underline());
        run.set_underline(true);
        assert!(run.is_underline());
    }

    #[test]
    fn test_run_color() {
        let mut run = Run::new("Colored text");
        assert!(run.color().is_none());
        let color = RGBColor::new(255, 0, 0);
        run.set_color(color);
        assert!(run.color().is_some());
    }

    #[test]
    fn test_run_font_size() {
        let mut run = Run::new("Sized text");
        assert!(run.font_size().is_none());
        run.set_font_size(24);
        assert_eq!(run.font_size(), Some(24));
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
