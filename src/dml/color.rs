//! DrawingML color functionality

use crate::enums::dml::ColorType;

/// RGB color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGBColor {
    /// Create a new RGB color
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create from hex string (e.g., "FF0000" for red)
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err("Hex color must be 6 characters".to_string());
        }
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|e| format!("Invalid hex: {}", e))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|e| format!("Invalid hex: {}", e))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|e| format!("Invalid hex: {}", e))?;
        Ok(Self { r, g, b })
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

/// Color format - provides access to color settings
pub struct ColorFormat {
    color_type: ColorType,
    rgb: Option<RGBColor>,
    theme_color: Option<u32>, // Theme color index
    brightness: f64, // -1.0 to 1.0
}

impl ColorFormat {
    /// Create a new color format with RGB color
    pub fn from_rgb(rgb: RGBColor) -> Self {
        Self {
            color_type: ColorType::Rgb,
            rgb: Some(rgb),
            theme_color: None,
            brightness: 0.0,
        }
    }

    /// Create a new color format with theme color
    pub fn from_theme(theme_color: u32) -> Self {
        Self {
            color_type: ColorType::Theme,
            rgb: None,
            theme_color: Some(theme_color),
            brightness: 0.0,
        }
    }

    /// Get the color type
    pub fn color_type(&self) -> ColorType {
        self.color_type
    }

    /// Get RGB color
    pub fn rgb(&self) -> Option<RGBColor> {
        self.rgb
    }

    /// Set RGB color
    pub fn set_rgb(&mut self, rgb: RGBColor) {
        self.color_type = ColorType::Rgb;
        self.rgb = Some(rgb);
        self.theme_color = None;
        self.brightness = 0.0;
    }

    /// Get theme color index
    pub fn theme_color(&self) -> Option<u32> {
        self.theme_color
    }

    /// Set theme color
    pub fn set_theme_color(&mut self, theme_color: u32) {
        self.color_type = ColorType::Theme;
        self.theme_color = Some(theme_color);
        self.rgb = None;
    }

    /// Get brightness adjustment (-1.0 to 1.0)
    pub fn brightness(&self) -> f64 {
        self.brightness
    }

    /// Set brightness adjustment
    pub fn set_brightness(&mut self, brightness: f64) {
        if brightness < -1.0 || brightness > 1.0 {
            return; // TODO: Return error
        }
        self.brightness = brightness;
    }
}

