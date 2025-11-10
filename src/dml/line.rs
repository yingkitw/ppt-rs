//! DrawingML line functionality

use crate::dml::fill::FillFormat;

/// Line format - provides access to line properties
pub struct LineFormat {
    width: u32, // in EMU
    dash_style: Option<DashStyle>,
    fill: FillFormat,
    begin_arrow_type: Option<ArrowType>,
    end_arrow_type: Option<ArrowType>,
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

/// Arrow types for line ends
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowType {
    None,
    Triangle,
    Diamond,
    Oval,
    Arrow,
    Stealth,
    Chevron,
    DoubleChevron,
}

impl LineFormat {
    /// Create a new line format
    pub fn new() -> Self {
        Self {
            width: 12700, // Default 1 point
            dash_style: Some(DashStyle::Solid),
            fill: FillFormat::new(),
            begin_arrow_type: None,
            end_arrow_type: None,
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

    /// Get begin arrow type
    pub fn begin_arrow_type(&self) -> Option<ArrowType> {
        self.begin_arrow_type
    }

    /// Set begin arrow type
    pub fn set_begin_arrow_type(&mut self, arrow_type: Option<ArrowType>) {
        self.begin_arrow_type = arrow_type;
    }

    /// Get end arrow type
    pub fn end_arrow_type(&self) -> Option<ArrowType> {
        self.end_arrow_type
    }

    /// Set end arrow type
    pub fn set_end_arrow_type(&mut self, arrow_type: Option<ArrowType>) {
        self.end_arrow_type = arrow_type;
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
    }

    #[test]
    fn test_line_format_begin_arrow() {
        let mut lf = LineFormat::new();
        assert_eq!(lf.begin_arrow_type(), None);
        
        lf.set_begin_arrow_type(Some(ArrowType::Triangle));
        assert_eq!(lf.begin_arrow_type(), Some(ArrowType::Triangle));
        
        lf.set_begin_arrow_type(Some(ArrowType::Diamond));
        assert_eq!(lf.begin_arrow_type(), Some(ArrowType::Diamond));
        
        lf.set_begin_arrow_type(None);
        assert_eq!(lf.begin_arrow_type(), None);
    }

    #[test]
    fn test_line_format_end_arrow() {
        let mut lf = LineFormat::new();
        assert_eq!(lf.end_arrow_type(), None);
        
        lf.set_end_arrow_type(Some(ArrowType::Oval));
        assert_eq!(lf.end_arrow_type(), Some(ArrowType::Oval));
        
        lf.set_end_arrow_type(Some(ArrowType::Stealth));
        assert_eq!(lf.end_arrow_type(), Some(ArrowType::Stealth));
        
        lf.set_end_arrow_type(None);
        assert_eq!(lf.end_arrow_type(), None);
    }

    #[test]
    fn test_line_format_both_arrows() {
        let mut lf = LineFormat::new();
        
        lf.set_begin_arrow_type(Some(ArrowType::Triangle));
        lf.set_end_arrow_type(Some(ArrowType::Arrow));
        
        assert_eq!(lf.begin_arrow_type(), Some(ArrowType::Triangle));
        assert_eq!(lf.end_arrow_type(), Some(ArrowType::Arrow));
    }

    #[test]
    fn test_arrow_type_variants() {
        let arrow_types = vec![
            ArrowType::None,
            ArrowType::Triangle,
            ArrowType::Diamond,
            ArrowType::Oval,
            ArrowType::Arrow,
            ArrowType::Stealth,
            ArrowType::Chevron,
            ArrowType::DoubleChevron,
        ];
        
        for arrow_type in arrow_types {
            let mut lf = LineFormat::new();
            lf.set_end_arrow_type(Some(arrow_type));
            assert_eq!(lf.end_arrow_type(), Some(arrow_type));
        }
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

