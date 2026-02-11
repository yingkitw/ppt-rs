//! Presentation-level settings that get embedded into the PPTX package
//!
//! Aggregates slide show settings, print settings, embedded fonts,
//! and digital signatures into a single struct passed to the builder.

use super::slide_show_settings::SlideShowSettings;
use super::print_settings::PrintSettings;
use super::embedded_fonts::EmbeddedFontList;
use super::digital_signature::DigitalSignature;

/// Presentation-level settings for the PPTX package
#[derive(Clone, Debug, Default)]
pub struct PresentationSettings {
    /// Slide show settings (generates `<p:showPr>` in presentation.xml)
    pub slide_show: Option<SlideShowSettings>,
    /// Print settings (generates `<p:prnPr>` in presentation.xml)
    pub print: Option<PrintSettings>,
    /// Embedded fonts (generates `<p:embeddedFontLst>` in presentation.xml)
    pub embedded_fonts: Option<EmbeddedFontList>,
    /// Digital signature (generates `_xmlsignatures/` parts in package)
    pub digital_signature: Option<DigitalSignature>,
}

impl PresentationSettings {
    pub fn new() -> Self {
        Self::default()
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
        self.slide_show.is_some()
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
    }

    #[test]
    fn test_with_slide_show() {
        let settings = PresentationSettings::new()
            .slide_show(SlideShowSettings::new());
        assert!(settings.has_settings());
        assert!(settings.slide_show.is_some());
    }

    #[test]
    fn test_with_print() {
        let settings = PresentationSettings::new()
            .print(PrintSettings::new());
        assert!(settings.has_settings());
        assert!(settings.print.is_some());
    }

    #[test]
    fn test_with_embedded_fonts() {
        let settings = PresentationSettings::new()
            .embedded_fonts(EmbeddedFontList::new());
        assert!(settings.has_settings());
        assert!(settings.embedded_fonts.is_some());
    }
}
