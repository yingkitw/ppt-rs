//! Text run functionality with hyperlink support

use crate::shapes::hyperlink::Hyperlink;
use crate::text::fonts::{Font, UnderlineStyle};
use crate::dml::color::RGBColor;

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

    /// Set bold formatting
    pub fn set_bold(&mut self, bold: bool) {
        self.font.set_bold(bold);
    }

    /// Get bold formatting
    pub fn is_bold(&self) -> bool {
        self.font.is_bold()
    }

    /// Set italic formatting
    pub fn set_italic(&mut self, italic: bool) {
        self.font.set_italic(italic);
    }

    /// Get italic formatting
    pub fn is_italic(&self) -> bool {
        self.font.is_italic()
    }

    /// Set underline formatting
    pub fn set_underline(&mut self, underline: bool) {
        self.font.set_underline(underline);
    }

    /// Set underline style
    pub fn set_underline_style(&mut self, style: UnderlineStyle) {
        self.font.set_underline_style(style);
    }

    /// Get underline style
    pub fn underline_style(&self) -> UnderlineStyle {
        self.font.underline_style()
    }

    /// Get underline formatting
    pub fn is_underline(&self) -> bool {
        self.font.is_underline()
    }

    /// Set text color (hex string)
    pub fn set_color_hex(&mut self, color: String) {
        self.font.set_color(color);
    }

    /// Get text color (hex string)
    pub fn color_hex(&self) -> Option<&str> {
        self.font.color()
    }

    /// Set font size (in points)
    pub fn set_font_size(&mut self, size: u32) {
        self.font.set_size(size);
    }

    /// Get font size
    pub fn font_size(&self) -> u32 {
        self.font.size()
    }

    /// Set character spacing (in EMU)
    pub fn set_character_spacing(&mut self, spacing: i32) {
        self.font.set_character_spacing(spacing);
    }

    /// Get character spacing
    pub fn character_spacing(&self) -> Option<i32> {
        self.font.character_spacing()
    }

    /// Set transparency (0-100%)
    pub fn set_transparency(&mut self, transparency: u32) {
        self.font.set_transparency(transparency);
    }

    /// Get transparency
    pub fn transparency(&self) -> Option<u32> {
        self.font.transparency()
    }

    /// Set subscript
    pub fn set_subscript(&mut self, subscript: bool) {
        self.font.set_subscript(subscript);
    }

    /// Get subscript
    pub fn is_subscript(&self) -> bool {
        self.font.is_subscript()
    }

    /// Set superscript
    pub fn set_superscript(&mut self, superscript: bool) {
        self.font.set_superscript(superscript);
    }

    /// Get superscript
    pub fn is_superscript(&self) -> bool {
        self.font.is_superscript()
    }

    /// Set strikethrough
    pub fn set_strikethrough(&mut self, strikethrough: bool) {
        self.font.set_strikethrough(strikethrough);
    }

    /// Get strikethrough
    pub fn is_strikethrough(&self) -> bool {
        self.font.is_strikethrough()
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
        assert!(run.color_hex().is_none());
        run.set_color_hex("FF0000".to_string());
        assert_eq!(run.color_hex(), Some("FF0000"));
    }

    #[test]
    fn test_run_font_size() {
        let mut run = Run::new("Sized text");
        assert_eq!(run.font_size(), 18); // Default size
        run.set_font_size(24);
        assert_eq!(run.font_size(), 24);
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

    #[test]
    fn test_run_character_spacing() {
        let mut run = Run::new("Spaced text");
        assert_eq!(run.character_spacing(), None);
        run.set_character_spacing(100);
        assert_eq!(run.character_spacing(), Some(100));
    }

    #[test]
    fn test_run_transparency() {
        let mut run = Run::new("Transparent text");
        assert_eq!(run.transparency(), None);
        run.set_transparency(50);
        assert_eq!(run.transparency(), Some(50));
    }

    #[test]
    fn test_run_subscript_superscript() {
        let mut run = Run::new("H2O");
        assert!(!run.is_subscript());
        run.set_subscript(true);
        assert!(run.is_subscript());
        
        let mut run2 = Run::new("E=mc²");
        run2.set_superscript(true);
        assert!(run2.is_superscript());
    }

    #[test]
    fn test_run_strikethrough() {
        let mut run = Run::new("Struck text");
        assert!(!run.is_strikethrough());
        run.set_strikethrough(true);
        assert!(run.is_strikethrough());
    }

    #[test]
    fn test_run_underline_styles() {
        let mut run = Run::new("Styled underline");
        run.set_underline_style(UnderlineStyle::Wavy);
        assert_eq!(run.underline_style(), UnderlineStyle::Wavy);
    }
}
