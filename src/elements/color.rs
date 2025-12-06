//! Color types for PPTX elements
//!
//! Provides unified color handling for all PPTX elements.

use crate::core::ToXml;

/// RGB color (6-digit hex)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    /// Create a new RGB color
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Parse from hex string (e.g., "FF0000" or "#FF0000")
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self { r, g, b })
    }

    /// Convert to hex string (uppercase, no #)
    pub fn to_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Common colors
    pub fn black() -> Self { Self::new(0, 0, 0) }
    pub fn white() -> Self { Self::new(255, 255, 255) }
    pub fn red() -> Self { Self::new(255, 0, 0) }
    pub fn green() -> Self { Self::new(0, 255, 0) }
    pub fn blue() -> Self { Self::new(0, 0, 255) }
}

impl ToXml for RgbColor {
    fn to_xml(&self) -> String {
        format!(r#"<a:srgbClr val="{}"/>"#, self.to_hex())
    }
}

impl Default for RgbColor {
    fn default() -> Self {
        Self::black()
    }
}

/// Scheme color (theme-based)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchemeColor {
    Accent1,
    Accent2,
    Accent3,
    Accent4,
    Accent5,
    Accent6,
    Dark1,
    Dark2,
    Light1,
    Light2,
    Hyperlink,
    FollowedHyperlink,
    Background1,
    Background2,
    Text1,
    Text2,
}

impl SchemeColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            SchemeColor::Accent1 => "accent1",
            SchemeColor::Accent2 => "accent2",
            SchemeColor::Accent3 => "accent3",
            SchemeColor::Accent4 => "accent4",
            SchemeColor::Accent5 => "accent5",
            SchemeColor::Accent6 => "accent6",
            SchemeColor::Dark1 => "dk1",
            SchemeColor::Dark2 => "dk2",
            SchemeColor::Light1 => "lt1",
            SchemeColor::Light2 => "lt2",
            SchemeColor::Hyperlink => "hlink",
            SchemeColor::FollowedHyperlink => "folHlink",
            SchemeColor::Background1 => "bg1",
            SchemeColor::Background2 => "bg2",
            SchemeColor::Text1 => "tx1",
            SchemeColor::Text2 => "tx2",
        }
    }
}

impl ToXml for SchemeColor {
    fn to_xml(&self) -> String {
        format!(r#"<a:schemeClr val="{}"/>"#, self.as_str())
    }
}

/// Unified color type
#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Rgb(RgbColor),
    Scheme(SchemeColor),
}

impl Color {
    /// Create from RGB values
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb(RgbColor::new(r, g, b))
    }

    /// Create from hex string
    pub fn from_hex(hex: &str) -> Option<Self> {
        RgbColor::from_hex(hex).map(Color::Rgb)
    }

    /// Create from scheme color
    pub fn scheme(color: SchemeColor) -> Self {
        Color::Scheme(color)
    }
}

impl ToXml for Color {
    fn to_xml(&self) -> String {
        match self {
            Color::Rgb(rgb) => rgb.to_xml(),
            Color::Scheme(scheme) => scheme.to_xml(),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Rgb(RgbColor::black())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_from_hex() {
        let color = RgbColor::from_hex("FF0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);

        let color = RgbColor::from_hex("#00FF00").unwrap();
        assert_eq!(color.to_hex(), "00FF00");
    }

    #[test]
    fn test_rgb_to_xml() {
        let color = RgbColor::new(255, 0, 0);
        assert_eq!(color.to_xml(), r#"<a:srgbClr val="FF0000"/>"#);
    }

    #[test]
    fn test_scheme_color() {
        let color = SchemeColor::Accent1;
        assert_eq!(color.to_xml(), r#"<a:schemeClr val="accent1"/>"#);
    }

    #[test]
    fn test_color_enum() {
        let rgb = Color::rgb(255, 0, 0);
        assert_eq!(rgb.to_xml(), r#"<a:srgbClr val="FF0000"/>"#);

        let scheme = Color::scheme(SchemeColor::Dark1);
        assert_eq!(scheme.to_xml(), r#"<a:schemeClr val="dk1"/>"#);
    }

    #[test]
    fn test_common_colors() {
        assert_eq!(RgbColor::black().to_hex(), "000000");
        assert_eq!(RgbColor::white().to_hex(), "FFFFFF");
        assert_eq!(RgbColor::red().to_hex(), "FF0000");
    }
}
