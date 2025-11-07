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
    DashDotDot,
    Dot,
    RoundDot,
    SquareDot,
    LongDash,
    LongDashDot,
    LongDashDotDot,
    SystemDash,
    SystemDot,
    SystemDashDot,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_format_new() {
        let lf = LineFormat::new();
        assert_eq!(lf.width(), 12700);
        assert_eq!(lf.dash_style(), Some(DashStyle::Solid));
    }

    #[test]
    fn test_line_format_width() {
        let mut lf = LineFormat::new();
        lf.set_width(25400);
        assert_eq!(lf.width(), 25400);
    }

    #[test]
    fn test_line_format_dash_styles() {
        let mut lf = LineFormat::new();
        
        lf.set_dash_style(Some(DashStyle::Dash));
        assert_eq!(lf.dash_style(), Some(DashStyle::Dash));
        
        lf.set_dash_style(Some(DashStyle::DashDot));
        assert_eq!(lf.dash_style(), Some(DashStyle::DashDot));
        
        lf.set_dash_style(Some(DashStyle::Dot));
        assert_eq!(lf.dash_style(), Some(DashStyle::Dot));
        
        lf.set_dash_style(Some(DashStyle::LongDash));
        assert_eq!(lf.dash_style(), Some(DashStyle::LongDash));
        
        lf.set_dash_style(None);
        assert_eq!(lf.dash_style(), None);
    }

    #[test]
    fn test_line_format_all_dash_styles() {
        let dash_styles = vec![
            DashStyle::Solid,
            DashStyle::Dash,
            DashStyle::DashDot,
            DashStyle::DashDotDot,
            DashStyle::Dot,
            DashStyle::RoundDot,
            DashStyle::SquareDot,
            DashStyle::LongDash,
            DashStyle::LongDashDot,
            DashStyle::LongDashDotDot,
            DashStyle::SystemDash,
            DashStyle::SystemDot,
            DashStyle::SystemDashDot,
        ];
        
        for style in dash_styles {
            let mut lf = LineFormat::new();
            lf.set_dash_style(Some(style));
            assert_eq!(lf.dash_style(), Some(style));
        }
    }

    #[test]
    fn test_line_format_fill() {
        let lf = LineFormat::new();
        let fill = lf.fill();
        // FillFormat::new() creates NoFill with no fore_color
        // This is correct behavior - line format has a fill but it may not have a color initially
        assert!(fill.fill_type() == crate::enums::dml::FillType::NoFill || fill.fore_color().is_some());
    }
}

