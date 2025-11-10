//! DrawingML module - colors, fills, lines, effects

pub mod color;
pub mod fill;
pub mod line;
pub mod gradient;
pub mod pattern;
pub mod picture_fill;
pub mod theme;

pub use color::ColorFormat;
pub use fill::FillFormat;
pub use line::{LineFormat, DashStyle, ArrowType};
pub use gradient::{Gradient, GradientStop, GradientType};
pub use pattern::{Pattern, PatternType};
pub use picture_fill::PictureFill;
pub use theme::{Theme, ColorScheme, FontScheme};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dml::color::{ColorFormat, RGBColor};
    use crate::dml::fill::FillFormat;
    use crate::dml::line::{LineFormat, DashStyle};
    use crate::enums::dml::{ColorType, FillType};

    #[test]
    fn test_rgb_color_new() {
        let color = RGBColor::new(255, 0, 0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_rgb_color_from_hex() {
        let color = RGBColor::from_hex("FF0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        
        let color2 = RGBColor::from_hex("#00FF00").unwrap();
        assert_eq!(color2.r, 0);
        assert_eq!(color2.g, 255);
        assert_eq!(color2.b, 0);
    }

    #[test]
    fn test_rgb_color_to_hex() {
        let color = RGBColor::new(255, 128, 64);
        assert_eq!(color.to_hex(), "FF8040");
    }

    #[test]
    fn test_color_format_rgb() {
        let color = RGBColor::new(255, 0, 0);
        let mut cf = ColorFormat::from_rgb(color);
        assert_eq!(cf.color_type(), ColorType::Rgb);
        assert_eq!(cf.rgb().unwrap().r, 255);
        
        cf.set_brightness(0.5);
        assert_eq!(cf.brightness(), 0.5);
    }

    #[test]
    fn test_color_format_theme() {
        let mut cf = ColorFormat::from_theme(1);
        assert_eq!(cf.color_type(), ColorType::Theme);
        assert_eq!(cf.theme_color(), Some(1));
    }

    #[test]
    fn test_fill_format() {
        let mut fill = FillFormat::new();
        assert_eq!(fill.fill_type(), FillType::NoFill);
        
        fill.set_solid(RGBColor::new(255, 0, 0));
        assert_eq!(fill.fill_type(), FillType::Solid);
        assert!(fill.fore_color().is_some());
    }

    #[test]
    fn test_line_format() {
        let mut line = LineFormat::new();
        assert_eq!(line.width(), 12700);
        assert_eq!(line.dash_style(), Some(DashStyle::Solid));
        
        line.set_width(25400);
        line.set_dash_style(Some(DashStyle::Dash));
        assert_eq!(line.width(), 25400);
        assert_eq!(line.dash_style(), Some(DashStyle::Dash));
    }
}
