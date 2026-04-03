//! Presentation-level settings that get embedded into the PPTX package
//!
//! Aggregates slide show settings, print settings, embedded fonts,
//! and digital signatures into a single struct passed to the builder.

use super::digital_signature::DigitalSignature;
use super::embedded_fonts::EmbeddedFontList;
use super::print_settings::PrintSettings;
use super::slide_show_settings::SlideShowSettings;
use crate::generator::constants::{SLIDE_HEIGHT, SLIDE_WIDTH};

/// PowerPoint slide size settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlideSize {
    /// Standard 10" x 7.5" slides.
    Standard4x3,
    /// Standard widescreen 13.333" x 7.5" slides.
    Widescreen16x9,
    /// Custom dimensions in EMU.
    Custom { width: u32, height: u32 },
}

impl SlideSize {
    pub fn dimensions(self) -> (u32, u32) {
        match self {
            SlideSize::Standard4x3 => (SLIDE_WIDTH, SLIDE_HEIGHT),
            SlideSize::Widescreen16x9 => (12_192_000, 6_858_000),
            SlideSize::Custom { width, height } => (width, height),
        }
    }

    pub(crate) fn presentation_type(self) -> Option<&'static str> {
        match self {
            SlideSize::Standard4x3 => Some("screen4x3"),
            SlideSize::Widescreen16x9 => Some("screen16x9"),
            SlideSize::Custom { .. } => None,
        }
    }

    pub(crate) fn app_presentation_format(self) -> &'static str {
        match self {
            SlideSize::Standard4x3 => "On-screen Show (4:3)",
            SlideSize::Widescreen16x9 => "Widescreen",
            SlideSize::Custom { .. } => "Custom",
        }
    }
}

impl Default for SlideSize {
    fn default() -> Self {
        Self::Standard4x3
    }
}

/// Presentation-level settings for the PPTX package
#[derive(Clone, Debug)]
pub struct PresentationSettings {
    /// Presentation slide size (`<p:sldSz>` in presentation.xml)
    pub slide_size: SlideSize,
    /// Slide show settings (generates `<p:showPr>` in presentation.xml)
    pub slide_show: Option<SlideShowSettings>,
    /// Print settings (generates `<p:prnPr>` in presentation.xml)
    pub print: Option<PrintSettings>,
    /// Embedded fonts (generates `<p:embeddedFontLst>` in presentation.xml)
    pub embedded_fonts: Option<EmbeddedFontList>,
    /// Digital signature (generates `_xmlsignatures/` parts in package)
    pub digital_signature: Option<DigitalSignature>,
}

impl Default for PresentationSettings {
    fn default() -> Self {
        Self {
            slide_size: SlideSize::default(),
            slide_show: None,
            print: None,
            embedded_fonts: None,
            digital_signature: None,
        }
    }
}

impl PresentationSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn slide_size(mut self, slide_size: SlideSize) -> Self {
        self.slide_size = slide_size;
        self
    }

    pub fn slide_show(mut self, settings: SlideShowSettings) -> Self {
        self.slide_show = Some(settings);
        self
    }

    pub fn print(mut self, settings: PrintSettings) -> Self {
        self.print = Some(settings);
        self
    }

    pub fn embedded_fonts(mut self, fonts: EmbeddedFontList) -> Self {
        self.embedded_fonts = Some(fonts);
        self
    }

    pub fn digital_signature(mut self, sig: DigitalSignature) -> Self {
        self.digital_signature = Some(sig);
        self
    }

    /// Check if any presentation-level settings are configured
    pub fn has_settings(&self) -> bool {
        self.slide_size != SlideSize::default()
            || self.slide_show.is_some()
            || self.print.is_some()
            || self.embedded_fonts.is_some()
            || self.digital_signature.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_has_no_settings() {
        let settings = PresentationSettings::new();
        assert!(!settings.has_settings());
        assert_eq!(settings.slide_size, SlideSize::Standard4x3);
    }

    #[test]
    fn test_with_slide_show() {
        let settings = PresentationSettings::new().slide_show(SlideShowSettings::new());
        assert!(settings.has_settings());
        assert!(settings.slide_show.is_some());
    }

    #[test]
    fn test_with_print() {
        let settings = PresentationSettings::new().print(PrintSettings::new());
        assert!(settings.has_settings());
        assert!(settings.print.is_some());
    }

    #[test]
    fn test_with_embedded_fonts() {
        let settings = PresentationSettings::new().embedded_fonts(EmbeddedFontList::new());
        assert!(settings.has_settings());
        assert!(settings.embedded_fonts.is_some());
    }

    #[test]
    fn test_with_slide_size() {
        let settings = PresentationSettings::new().slide_size(SlideSize::Widescreen16x9);
        assert!(settings.has_settings());
        assert_eq!(settings.slide_size, SlideSize::Widescreen16x9);
        assert_eq!(settings.slide_size.dimensions(), (12_192_000, 6_858_000));
    }
}
