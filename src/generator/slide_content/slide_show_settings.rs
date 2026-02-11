//! Slide show settings for presentations
//!
//! Controls how the presentation is displayed in slide show mode.
//! Generates `<p:showPr>` XML in presentation.xml.

/// Slide show type
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum ShowType {
    #[default]
    Speaker,
    Kiosk,
    Browsed,
}

impl ShowType {
    pub fn to_xml_element(&self) -> &'static str {
        match self {
            ShowType::Speaker => r#"<p:present/>"#,
            ShowType::Kiosk => r#"<p:kiosk restart="300000"/>"#,
            ShowType::Browsed => r#"<p:browse showScrollbar="1"/>"#,
        }
    }
}

/// Pen color used during slide show
#[derive(Clone, Debug)]
pub struct PenColor {
    pub color: String,
}

impl PenColor {
    pub fn new(color: &str) -> Self {
        Self {
            color: color.trim_start_matches('#').to_uppercase(),
        }
    }

    pub fn red() -> Self { Self::new("FF0000") }
    pub fn blue() -> Self { Self::new("0000FF") }
    pub fn black() -> Self { Self::new("000000") }
}

impl Default for PenColor {
    fn default() -> Self {
        Self::red()
    }
}

/// Slide range for the show
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SlideRange {
    All,
    Range { start: u32, end: u32 },
    Custom(Vec<u32>),
}

impl Default for SlideRange {
    fn default() -> Self {
        SlideRange::All
    }
}

/// Slide show settings
#[derive(Clone, Debug, Default)]
pub struct SlideShowSettings {
    pub show_type: ShowType,
    pub loop_continuously: bool,
    pub show_without_narration: bool,
    pub show_without_animation: bool,
    pub pen_color: PenColor,
    pub slide_range: SlideRange,
    pub use_timings: bool,
    pub show_media_controls: bool,
}

impl SlideShowSettings {
    pub fn new() -> Self {
        Self {
            use_timings: true,
            show_media_controls: true,
            ..Default::default()
        }
    }

    pub fn show_type(mut self, show_type: ShowType) -> Self {
        self.show_type = show_type;
        self
    }

    pub fn loop_continuously(mut self, looping: bool) -> Self {
        self.loop_continuously = looping;
        self
    }

    pub fn without_narration(mut self, val: bool) -> Self {
        self.show_without_narration = val;
        self
    }

    pub fn without_animation(mut self, val: bool) -> Self {
        self.show_without_animation = val;
        self
    }

    pub fn pen_color(mut self, color: PenColor) -> Self {
        self.pen_color = color;
        self
    }

    pub fn slide_range(mut self, range: SlideRange) -> Self {
        self.slide_range = range;
        self
    }

    pub fn use_timings(mut self, val: bool) -> Self {
        self.use_timings = val;
        self
    }

    pub fn show_media_controls(mut self, val: bool) -> Self {
        self.show_media_controls = val;
        self
    }

    /// Kiosk mode preset (loops, no narration)
    pub fn kiosk() -> Self {
        Self::new()
            .show_type(ShowType::Kiosk)
            .loop_continuously(true)
            .without_narration(true)
    }

    /// Generate `<p:showPr>` XML for presentation.xml
    pub fn to_xml(&self) -> String {
        let mut attrs = Vec::new();

        if self.loop_continuously {
            attrs.push(r#"loop="1""#.to_string());
        }
        if self.show_without_narration {
            attrs.push(r#"showNarration="0""#.to_string());
        }
        if self.show_without_animation {
            attrs.push(r#"showAnimation="0""#.to_string());
        }
        if self.use_timings {
            attrs.push(r#"useTimings="1""#.to_string());
        }

        let attrs_str = if attrs.is_empty() {
            String::new()
        } else {
            format!(" {}", attrs.join(" "))
        };

        let mut xml = format!(r#"<p:showPr{}>"#, attrs_str);

        // Show type
        xml.push_str(self.show_type.to_xml_element());

        // Slide range
        match &self.slide_range {
            SlideRange::All => xml.push_str(r#"<p:sldAll/>"#),
            SlideRange::Range { start, end } => {
                xml.push_str(&format!(r#"<p:sldRg st="{}" end="{}"/>"#, start, end));
            }
            SlideRange::Custom(slides) => {
                xml.push_str("<p:custShow>");
                for s in slides {
                    xml.push_str(&format!(r#"<p:sldId id="{}"/>"#, s));
                }
                xml.push_str("</p:custShow>");
            }
        }

        // Pen color
        xml.push_str(&format!(
            r#"<p:penClr><a:srgbClr val="{}"/></p:penClr>"#,
            self.pen_color.color
        ));

        xml.push_str("</p:showPr>");
        xml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_type_default() {
        assert_eq!(ShowType::default(), ShowType::Speaker);
    }

    #[test]
    fn test_show_type_xml() {
        assert!(ShowType::Speaker.to_xml_element().contains("present"));
        assert!(ShowType::Kiosk.to_xml_element().contains("kiosk"));
        assert!(ShowType::Browsed.to_xml_element().contains("browse"));
    }

    #[test]
    fn test_pen_color_presets() {
        assert_eq!(PenColor::red().color, "FF0000");
        assert_eq!(PenColor::blue().color, "0000FF");
        assert_eq!(PenColor::black().color, "000000");
    }

    #[test]
    fn test_pen_color_custom() {
        let pc = PenColor::new("#00FF00");
        assert_eq!(pc.color, "00FF00");
    }

    #[test]
    fn test_slide_range_default() {
        assert_eq!(SlideRange::default(), SlideRange::All);
    }

    #[test]
    fn test_slide_show_settings_new() {
        let s = SlideShowSettings::new();
        assert_eq!(s.show_type, ShowType::Speaker);
        assert!(!s.loop_continuously);
        assert!(s.use_timings);
        assert!(s.show_media_controls);
    }

    #[test]
    fn test_slide_show_settings_builder() {
        let s = SlideShowSettings::new()
            .show_type(ShowType::Kiosk)
            .loop_continuously(true)
            .without_narration(true)
            .without_animation(true)
            .pen_color(PenColor::blue())
            .slide_range(SlideRange::Range { start: 1, end: 5 })
            .use_timings(false)
            .show_media_controls(false);
        assert_eq!(s.show_type, ShowType::Kiosk);
        assert!(s.loop_continuously);
        assert!(s.show_without_narration);
        assert!(s.show_without_animation);
        assert_eq!(s.pen_color.color, "0000FF");
        assert!(!s.use_timings);
    }

    #[test]
    fn test_kiosk_preset() {
        let s = SlideShowSettings::kiosk();
        assert_eq!(s.show_type, ShowType::Kiosk);
        assert!(s.loop_continuously);
        assert!(s.show_without_narration);
    }

    #[test]
    fn test_xml_basic() {
        let s = SlideShowSettings::new();
        let xml = s.to_xml();
        assert!(xml.contains("<p:showPr"));
        assert!(xml.contains("<p:present/>"));
        assert!(xml.contains("<p:sldAll/>"));
        assert!(xml.contains("penClr"));
        assert!(xml.contains("</p:showPr>"));
    }

    #[test]
    fn test_xml_kiosk() {
        let s = SlideShowSettings::kiosk();
        let xml = s.to_xml();
        assert!(xml.contains("loop=\"1\""));
        assert!(xml.contains("showNarration=\"0\""));
        assert!(xml.contains("<p:kiosk"));
    }

    #[test]
    fn test_xml_slide_range() {
        let s = SlideShowSettings::new()
            .slide_range(SlideRange::Range { start: 2, end: 8 });
        let xml = s.to_xml();
        assert!(xml.contains(r#"st="2""#));
        assert!(xml.contains(r#"end="8""#));
    }

    #[test]
    fn test_xml_custom_slides() {
        let s = SlideShowSettings::new()
            .slide_range(SlideRange::Custom(vec![256, 258, 260]));
        let xml = s.to_xml();
        assert!(xml.contains("custShow"));
        assert!(xml.contains(r#"id="256""#));
        assert!(xml.contains(r#"id="260""#));
    }

    #[test]
    fn test_xml_pen_color() {
        let s = SlideShowSettings::new().pen_color(PenColor::new("00FF00"));
        let xml = s.to_xml();
        assert!(xml.contains("00FF00"));
    }
}
