//! Theme Customization - Custom color schemes and fonts

use crate::dml::color::RGBColor;

/// Color scheme
#[derive(Clone, Debug)]
pub struct ColorScheme {
    /// Scheme name
    name: String,
    /// Accent 1 color
    accent1: RGBColor,
    /// Accent 2 color
    accent2: RGBColor,
    /// Accent 3 color
    accent3: RGBColor,
    /// Accent 4 color
    accent4: RGBColor,
    /// Accent 5 color
    accent5: RGBColor,
    /// Accent 6 color
    accent6: RGBColor,
    /// Dark 1 color
    dark1: RGBColor,
    /// Light 1 color
    light1: RGBColor,
}

impl ColorScheme {
    /// Create a new color scheme
    pub fn new(name: String) -> Self {
        Self {
            name,
            accent1: RGBColor::new(0, 0, 0),
            accent2: RGBColor::new(0, 0, 0),
            accent3: RGBColor::new(0, 0, 0),
            accent4: RGBColor::new(0, 0, 0),
            accent5: RGBColor::new(0, 0, 0),
            accent6: RGBColor::new(0, 0, 0),
            dark1: RGBColor::new(0, 0, 0),
            light1: RGBColor::new(255, 255, 255),
        }
    }

    /// Get scheme name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set accent 1 color
    pub fn set_accent1(&mut self, color: RGBColor) {
        self.accent1 = color;
    }

    /// Get accent 1 color
    pub fn accent1(&self) -> RGBColor {
        self.accent1
    }

    /// Set accent 2 color
    pub fn set_accent2(&mut self, color: RGBColor) {
        self.accent2 = color;
    }

    /// Get accent 2 color
    pub fn accent2(&self) -> RGBColor {
        self.accent2
    }

    /// Set accent 3 color
    pub fn set_accent3(&mut self, color: RGBColor) {
        self.accent3 = color;
    }

    /// Get accent 3 color
    pub fn accent3(&self) -> RGBColor {
        self.accent3
    }

    /// Set accent 4 color
    pub fn set_accent4(&mut self, color: RGBColor) {
        self.accent4 = color;
    }

    /// Get accent 4 color
    pub fn accent4(&self) -> RGBColor {
        self.accent4
    }

    /// Set accent 5 color
    pub fn set_accent5(&mut self, color: RGBColor) {
        self.accent5 = color;
    }

    /// Get accent 5 color
    pub fn accent5(&self) -> RGBColor {
        self.accent5
    }

    /// Set accent 6 color
    pub fn set_accent6(&mut self, color: RGBColor) {
        self.accent6 = color;
    }

    /// Get accent 6 color
    pub fn accent6(&self) -> RGBColor {
        self.accent6
    }

    /// Set dark 1 color
    pub fn set_dark1(&mut self, color: RGBColor) {
        self.dark1 = color;
    }

    /// Get dark 1 color
    pub fn dark1(&self) -> RGBColor {
        self.dark1
    }

    /// Set light 1 color
    pub fn set_light1(&mut self, color: RGBColor) {
        self.light1 = color;
    }

    /// Get light 1 color
    pub fn light1(&self) -> RGBColor {
        self.light1
    }

    /// Generate XML for color scheme
    pub fn to_xml(&self) -> String {
        let mut xml = format!(r#"<a:clrScheme name="{}">"#, self.name);

        xml.push_str(&format!(
            r#"<a:dk1><a:srgbClr val="{}"/></a:dk1>"#,
            self.dark1.to_hex()
        ));
        xml.push_str(&format!(
            r#"<a:lt1><a:srgbClr val="{}"/></a:lt1>"#,
            self.light1.to_hex()
        ));
        xml.push_str(&format!(
            r#"<a:accent1><a:srgbClr val="{}"/></a:accent1>"#,
            self.accent1.to_hex()
        ));
        xml.push_str(&format!(
            r#"<a:accent2><a:srgbClr val="{}"/></a:accent2>"#,
            self.accent2.to_hex()
        ));
        xml.push_str(&format!(
            r#"<a:accent3><a:srgbClr val="{}"/></a:accent3>"#,
            self.accent3.to_hex()
        ));
        xml.push_str(&format!(
            r#"<a:accent4><a:srgbClr val="{}"/></a:accent4>"#,
            self.accent4.to_hex()
        ));
        xml.push_str(&format!(
            r#"<a:accent5><a:srgbClr val="{}"/></a:accent5>"#,
            self.accent5.to_hex()
        ));
        xml.push_str(&format!(
            r#"<a:accent6><a:srgbClr val="{}"/></a:accent6>"#,
            self.accent6.to_hex()
        ));

        xml.push_str("</a:clrScheme>");
        xml
    }
}

/// Font scheme
#[derive(Clone, Debug)]
pub struct FontScheme {
    /// Scheme name
    name: String,
    /// Major font (for headings)
    major_font: String,
    /// Minor font (for body)
    minor_font: String,
}

impl FontScheme {
    /// Create a new font scheme
    pub fn new(name: String, major_font: String, minor_font: String) -> Self {
        Self {
            name,
            major_font,
            minor_font,
        }
    }

    /// Get scheme name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get major font
    pub fn major_font(&self) -> &str {
        &self.major_font
    }

    /// Set major font
    pub fn set_major_font(&mut self, font: String) {
        self.major_font = font;
    }

    /// Get minor font
    pub fn minor_font(&self) -> &str {
        &self.minor_font
    }

    /// Set minor font
    pub fn set_minor_font(&mut self, font: String) {
        self.minor_font = font;
    }

    /// Generate XML for font scheme
    pub fn to_xml(&self) -> String {
        format!(
            r#"<a:fontScheme name="{}"><a:majorFont><a:latin typeface="{}"/></a:majorFont><a:minorFont><a:latin typeface="{}"/></a:minorFont></a:fontScheme>"#,
            self.name, self.major_font, self.minor_font
        )
    }
}

/// Theme
#[derive(Clone, Debug)]
pub struct Theme {
    /// Theme name
    name: String,
    /// Color scheme
    color_scheme: ColorScheme,
    /// Font scheme
    font_scheme: FontScheme,
}

impl Theme {
    /// Create a new theme
    pub fn new(name: String, color_scheme: ColorScheme, font_scheme: FontScheme) -> Self {
        Self {
            name,
            color_scheme,
            font_scheme,
        }
    }

    /// Get theme name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get color scheme
    pub fn color_scheme(&self) -> &ColorScheme {
        &self.color_scheme
    }

    /// Get mutable color scheme
    pub fn color_scheme_mut(&mut self) -> &mut ColorScheme {
        &mut self.color_scheme
    }

    /// Get font scheme
    pub fn font_scheme(&self) -> &FontScheme {
        &self.font_scheme
    }

    /// Get mutable font scheme
    pub fn font_scheme_mut(&mut self) -> &mut FontScheme {
        &mut self.font_scheme
    }

    /// Generate XML for theme
    pub fn to_xml(&self) -> String {
        let mut xml = format!(r#"<a:theme name="{}">"#, self.name);
        xml.push_str(&self.color_scheme.to_xml());
        xml.push_str(&self.font_scheme.to_xml());
        xml.push_str("</a:theme>");
        xml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scheme_creation() {
        let scheme = ColorScheme::new("Custom".to_string());
        assert_eq!(scheme.name(), "Custom");
    }

    #[test]
    fn test_color_scheme_set_colors() {
        let mut scheme = ColorScheme::new("Custom".to_string());
        scheme.set_accent1(RGBColor::new(255, 0, 0));
        assert_eq!(scheme.accent1(), RGBColor::new(255, 0, 0));
    }

    #[test]
    fn test_color_scheme_xml() {
        let scheme = ColorScheme::new("Custom".to_string());
        let xml = scheme.to_xml();
        assert!(xml.contains(r#"<a:clrScheme name="Custom""#));
        assert!(xml.contains("</a:clrScheme>"));
    }

    #[test]
    fn test_font_scheme_creation() {
        let scheme = FontScheme::new(
            "Custom".to_string(),
            "Calibri".to_string(),
            "Cambria".to_string(),
        );
        assert_eq!(scheme.name(), "Custom");
        assert_eq!(scheme.major_font(), "Calibri");
        assert_eq!(scheme.minor_font(), "Cambria");
    }

    #[test]
    fn test_font_scheme_set_fonts() {
        let mut scheme = FontScheme::new(
            "Custom".to_string(),
            "Calibri".to_string(),
            "Cambria".to_string(),
        );
        scheme.set_major_font("Arial".to_string());
        assert_eq!(scheme.major_font(), "Arial");
    }

    #[test]
    fn test_font_scheme_xml() {
        let scheme = FontScheme::new(
            "Custom".to_string(),
            "Calibri".to_string(),
            "Cambria".to_string(),
        );
        let xml = scheme.to_xml();
        assert!(xml.contains(r#"<a:fontScheme name="Custom""#));
        assert!(xml.contains("Calibri"));
        assert!(xml.contains("Cambria"));
    }

    #[test]
    fn test_theme_creation() {
        let color_scheme = ColorScheme::new("Colors".to_string());
        let font_scheme = FontScheme::new(
            "Fonts".to_string(),
            "Calibri".to_string(),
            "Cambria".to_string(),
        );
        let theme = Theme::new("MyTheme".to_string(), color_scheme, font_scheme);
        assert_eq!(theme.name(), "MyTheme");
    }

    #[test]
    fn test_theme_xml() {
        let color_scheme = ColorScheme::new("Colors".to_string());
        let font_scheme = FontScheme::new(
            "Fonts".to_string(),
            "Calibri".to_string(),
            "Cambria".to_string(),
        );
        let theme = Theme::new("MyTheme".to_string(), color_scheme, font_scheme);
        let xml = theme.to_xml();
        assert!(xml.contains(r#"<a:theme name="MyTheme""#));
        assert!(xml.contains("</a:theme>"));
    }
}
