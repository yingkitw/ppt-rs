//! DrawingML line functionality

use crate::dml::fill::FillFormat;

/// Line format - provides access to line properties
pub struct LineFormat {
    width: u32, // in EMU
    dash_style: Option<DashStyle>,
    fill: FillFormat,
}

/// Line dash styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashStyle {
    Solid,
    Dash,
    DashDot,
    Dot,
    LongDash,
    LongDashDot,
    LongDashDotDot,
    // TODO: Add more dash styles
}

impl LineFormat {
    /// Create a new line format
    pub fn new() -> Self {
        Self {
            width: 12700, // Default 1 point
            dash_style: Some(DashStyle::Solid),
            fill: FillFormat::new(),
        }
    }

    /// Get line width in EMU
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Set line width in EMU
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    /// Get dash style
    pub fn dash_style(&self) -> Option<DashStyle> {
        self.dash_style
    }

    /// Set dash style
    pub fn set_dash_style(&mut self, style: Option<DashStyle>) {
        self.dash_style = style;
    }

    /// Get fill format (for line color)
    pub fn fill(&self) -> &FillFormat {
        &self.fill
    }

    /// Get mutable fill format
    pub fn fill_mut(&mut self) -> &mut FillFormat {
        &mut self.fill
    }

    /// Get color format (convenience method)
    pub fn color(&self) -> Option<&crate::dml::color::ColorFormat> {
        self.fill.fore_color()
    }

    /// Get mutable color format
    pub fn color_mut(&mut self) -> Option<&mut crate::dml::color::ColorFormat> {
        self.fill.fore_color_mut()
    }
}

