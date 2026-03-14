//! Enhanced color utilities with aliases, adjustments, and conversions
//!
//! This module provides a rich color API with:
//! - Popular color aliases (red, blue, green, etc.)
//! - RGB color creation and manipulation
//! - Color adjustment methods (lighter, darker, opacity)
//! - Color conversion utilities

use crate::elements::{Color, RgbColor};

/// A color value that can be manipulated and converted
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorValue {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8, // Alpha channel (0-255, 255 = opaque)
}

impl ColorValue {
    /// Create a new color from RGB values
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a new color from RGBA values
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create a color from a hex string (with or without #)
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            Self::rgb(r, g, b)
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
            Self::rgba(r, g, b, a)
        } else {
            Self::rgb(0, 0, 0)
        }
    }

    /// Convert to hex string (RRGGBB format)
    pub fn to_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Convert to hex string with alpha (RRGGBBAA format)
    pub fn to_hex_alpha(&self) -> String {
        format!("{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }

    /// Make the color lighter by a percentage (0.0 - 1.0)
    pub fn lighter(&self, amount: f32) -> Self {
        let amount = amount.clamp(0.0, 1.0);
        let r = (self.r as f32 + (255.0 - self.r as f32) * amount) as u8;
        let g = (self.g as f32 + (255.0 - self.g as f32) * amount) as u8;
        let b = (self.b as f32 + (255.0 - self.b as f32) * amount) as u8;
        Self::rgba(r, g, b, self.a)
    }

    /// Make the color darker by a percentage (0.0 - 1.0)
    pub fn darker(&self, amount: f32) -> Self {
        let amount = amount.clamp(0.0, 1.0);
        let r = (self.r as f32 * (1.0 - amount)) as u8;
        let g = (self.g as f32 * (1.0 - amount)) as u8;
        let b = (self.b as f32 * (1.0 - amount)) as u8;
        Self::rgba(r, g, b, self.a)
    }

    /// Adjust the opacity (0.0 = transparent, 1.0 = opaque)
    pub fn opacity(&self, alpha: f32) -> Self {
        let alpha = (alpha.clamp(0.0, 1.0) * 255.0) as u8;
        Self::rgba(self.r, self.g, self.b, alpha)
    }

    /// Set transparency percentage (0 = opaque, 100 = transparent)
    pub fn transparent(&self, percent: u8) -> Self {
        let percent = percent.min(100);
        let alpha = ((100 - percent) as f32 / 100.0 * 255.0) as u8;
        Self::rgba(self.r, self.g, self.b, alpha)
    }

    /// Mix this color with another color
    pub fn mix(&self, other: &ColorValue, ratio: f32) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        let r = (self.r as f32 * (1.0 - ratio) + other.r as f32 * ratio) as u8;
        let g = (self.g as f32 * (1.0 - ratio) + other.g as f32 * ratio) as u8;
        let b = (self.b as f32 * (1.0 - ratio) + other.b as f32 * ratio) as u8;
        let a = (self.a as f32 * (1.0 - ratio) + other.a as f32 * ratio) as u8;
        Self::rgba(r, g, b, a)
    }

    /// Convert to grayscale
    pub fn grayscale(&self) -> Self {
        let gray = (0.299 * self.r as f32 + 0.587 * self.g as f32 + 0.114 * self.b as f32) as u8;
        Self::rgba(gray, gray, gray, self.a)
    }

    /// Invert the color
    pub fn invert(&self) -> Self {
        Self::rgba(255 - self.r, 255 - self.g, 255 - self.b, self.a)
    }

    /// Convert to Color enum for use in the library
    pub fn to_color(&self) -> Color {
        Color::Rgb(RgbColor::new(self.r, self.g, self.b))
    }
}

// Popular color aliases
pub fn red() -> ColorValue { ColorValue::rgb(255, 0, 0) }
pub fn green() -> ColorValue { ColorValue::rgb(0, 255, 0) }
pub fn blue() -> ColorValue { ColorValue::rgb(0, 0, 255) }
pub fn yellow() -> ColorValue { ColorValue::rgb(255, 255, 0) }
pub fn cyan() -> ColorValue { ColorValue::rgb(0, 255, 255) }
pub fn magenta() -> ColorValue { ColorValue::rgb(255, 0, 255) }
pub fn white() -> ColorValue { ColorValue::rgb(255, 255, 255) }
pub fn black() -> ColorValue { ColorValue::rgb(0, 0, 0) }
pub fn gray() -> ColorValue { ColorValue::rgb(128, 128, 128) }
pub fn grey() -> ColorValue { ColorValue::rgb(128, 128, 128) }

// Shades of gray
pub fn light_gray() -> ColorValue { ColorValue::rgb(211, 211, 211) }
pub fn light_grey() -> ColorValue { ColorValue::rgb(211, 211, 211) }
pub fn dark_gray() -> ColorValue { ColorValue::rgb(64, 64, 64) }
pub fn dark_grey() -> ColorValue { ColorValue::rgb(64, 64, 64) }
pub fn silver() -> ColorValue { ColorValue::rgb(192, 192, 192) }

// Common web colors
pub fn orange() -> ColorValue { ColorValue::rgb(255, 165, 0) }
pub fn purple() -> ColorValue { ColorValue::rgb(128, 0, 128) }
pub fn pink() -> ColorValue { ColorValue::rgb(255, 192, 203) }
pub fn brown() -> ColorValue { ColorValue::rgb(165, 42, 42) }
pub fn navy() -> ColorValue { ColorValue::rgb(0, 0, 128) }
pub fn teal() -> ColorValue { ColorValue::rgb(0, 128, 128) }
pub fn olive() -> ColorValue { ColorValue::rgb(128, 128, 0) }
pub fn maroon() -> ColorValue { ColorValue::rgb(128, 0, 0) }
pub fn lime() -> ColorValue { ColorValue::rgb(0, 255, 0) }
pub fn aqua() -> ColorValue { ColorValue::rgb(0, 255, 255) }

// Material Design colors
pub fn material_red() -> ColorValue { ColorValue::from_hex("F44336") }
pub fn material_pink() -> ColorValue { ColorValue::from_hex("E91E63") }
pub fn material_purple() -> ColorValue { ColorValue::from_hex("9C27B0") }
pub fn material_indigo() -> ColorValue { ColorValue::from_hex("3F51B5") }
pub fn material_blue() -> ColorValue { ColorValue::from_hex("2196F3") }
pub fn material_cyan() -> ColorValue { ColorValue::from_hex("00BCD4") }
pub fn material_teal() -> ColorValue { ColorValue::from_hex("009688") }
pub fn material_green() -> ColorValue { ColorValue::from_hex("4CAF50") }
pub fn material_lime() -> ColorValue { ColorValue::from_hex("CDDC39") }
pub fn material_amber() -> ColorValue { ColorValue::from_hex("FFC107") }
pub fn material_orange() -> ColorValue { ColorValue::from_hex("FF9800") }
pub fn material_brown() -> ColorValue { ColorValue::from_hex("795548") }
pub fn material_gray() -> ColorValue { ColorValue::from_hex("9E9E9E") }
pub fn material_grey() -> ColorValue { ColorValue::from_hex("9E9E9E") }

// Corporate/Professional colors
pub fn corporate_blue() -> ColorValue { ColorValue::from_hex("1565C0") }
pub fn corporate_green() -> ColorValue { ColorValue::from_hex("2E7D32") }
pub fn corporate_red() -> ColorValue { ColorValue::from_hex("C62828") }
pub fn corporate_orange() -> ColorValue { ColorValue::from_hex("EF6C00") }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_creation() {
        let color = ColorValue::rgb(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_hex_conversion() {
        let color = ColorValue::from_hex("#FF8040");
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.to_hex(), "FF8040");
    }

    #[test]
    fn test_lighter() {
        let color = ColorValue::rgb(100, 100, 100);
        let lighter = color.lighter(0.5);
        assert!(lighter.r > color.r);
        assert!(lighter.g > color.g);
        assert!(lighter.b > color.b);
    }

    #[test]
    fn test_darker() {
        let color = ColorValue::rgb(200, 200, 200);
        let darker = color.darker(0.5);
        assert!(darker.r < color.r);
        assert!(darker.g < color.g);
        assert!(darker.b < color.b);
    }

    #[test]
    fn test_opacity() {
        let color = ColorValue::rgb(255, 0, 0);
        let semi = color.opacity(0.5);
        assert_eq!(semi.a, 127);
    }

    #[test]
    fn test_transparent() {
        let color = ColorValue::rgb(255, 0, 0);
        let trans = color.transparent(50);
        assert_eq!(trans.a, 127);
    }

    #[test]
    fn test_mix() {
        let red = ColorValue::rgb(255, 0, 0);
        let blue = ColorValue::rgb(0, 0, 255);
        let purple = red.mix(&blue, 0.5);
        assert_eq!(purple.r, 127);
        assert_eq!(purple.b, 127);
    }

    #[test]
    fn test_grayscale() {
        let color = ColorValue::rgb(255, 128, 64);
        let gray = color.grayscale();
        assert_eq!(gray.r, gray.g);
        assert_eq!(gray.g, gray.b);
    }

    #[test]
    fn test_invert() {
        let color = ColorValue::rgb(100, 150, 200);
        let inverted = color.invert();
        assert_eq!(inverted.r, 155);
        assert_eq!(inverted.g, 105);
        assert_eq!(inverted.b, 55);
    }

    #[test]
    fn test_color_aliases() {
        assert_eq!(red().to_hex(), "FF0000");
        assert_eq!(green().to_hex(), "00FF00");
        assert_eq!(blue().to_hex(), "0000FF");
        assert_eq!(white().to_hex(), "FFFFFF");
        assert_eq!(black().to_hex(), "000000");
    }
}
