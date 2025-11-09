//! DrawingML pattern fill functionality

use crate::dml::color::ColorFormat;

/// Pattern fill type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatternType {
    /// Solid fill (no pattern)
    Solid,
    /// Horizontal lines
    Horizontal,
    /// Vertical lines
    Vertical,
    /// Diagonal down (top-left to bottom-right)
    DiagonalDown,
    /// Diagonal up (bottom-left to top-right)
    DiagonalUp,
    /// Cross pattern
    Cross,
    /// Diagonal cross
    DiagonalCross,
    /// Horizontal brick
    HorizontalBrick,
    /// Vertical brick
    VerticalBrick,
    /// Checker board
    Checker,
    /// Dots
    Dots,
    /// Dashes
    Dashes,
    /// Waves
    Waves,
    /// Trellis
    Trellis,
    /// Thick horizontal
    ThickHorizontal,
    /// Thick vertical
    ThickVertical,
    /// Thick diagonal down
    ThickDiagonalDown,
    /// Thick diagonal up
    ThickDiagonalUp,
    /// Thick cross
    ThickCross,
    /// Thick diagonal cross
    ThickDiagonalCross,
}

/// Pattern fill - provides access to pattern fill properties
pub struct PatternFill {
    pattern_type: PatternType,
    fore_color: ColorFormat,
    back_color: ColorFormat,
}

/// Type alias for convenience
pub type Pattern = PatternFill;

impl PatternFill {
    /// Create a new pattern fill
    pub fn new(pattern_type: PatternType, fore_color: ColorFormat, back_color: ColorFormat) -> Self {
        Self {
            pattern_type,
            fore_color,
            back_color,
        }
    }

    /// Create a pattern fill with RGB colors
    pub fn with_rgb(
        pattern_type: PatternType,
        fore_rgb: crate::dml::color::RGBColor,
        back_rgb: crate::dml::color::RGBColor,
    ) -> Self {
        Self {
            pattern_type,
            fore_color: ColorFormat::from_rgb(fore_rgb),
            back_color: ColorFormat::from_rgb(back_rgb),
        }
    }

    /// Get the pattern type
    pub fn pattern_type(&self) -> PatternType {
        self.pattern_type
    }

    /// Set the pattern type
    pub fn set_pattern_type(&mut self, pattern_type: PatternType) {
        self.pattern_type = pattern_type;
    }

    /// Get the foreground color
    pub fn fore_color(&self) -> &ColorFormat {
        &self.fore_color
    }

    /// Get mutable foreground color
    pub fn fore_color_mut(&mut self) -> &mut ColorFormat {
        &mut self.fore_color
    }

    /// Get the background color
    pub fn back_color(&self) -> &ColorFormat {
        &self.back_color
    }

    /// Get mutable background color
    pub fn back_color_mut(&mut self) -> &mut ColorFormat {
        &mut self.back_color
    }

    /// Swap foreground and background colors
    pub fn swap_colors(&mut self) {
        std::mem::swap(&mut self.fore_color, &mut self.back_color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dml::color::RGBColor;

    #[test]
    fn test_pattern_fill_creation() {
        let fore = ColorFormat::from_rgb(RGBColor::new(255, 0, 0));
        let back = ColorFormat::from_rgb(RGBColor::new(0, 0, 255));
        let pattern = PatternFill::new(PatternType::Horizontal, fore, back);
        
        assert_eq!(pattern.pattern_type(), PatternType::Horizontal);
    }

    #[test]
    fn test_pattern_fill_with_rgb() {
        let pattern = PatternFill::with_rgb(
            PatternType::Vertical,
            RGBColor::new(255, 0, 0),
            RGBColor::new(0, 0, 255),
        );
        
        assert_eq!(pattern.pattern_type(), PatternType::Vertical);
    }

    #[test]
    fn test_pattern_type_equality() {
        assert_eq!(PatternType::Horizontal, PatternType::Horizontal);
        assert_ne!(PatternType::Horizontal, PatternType::Vertical);
    }

    #[test]
    fn test_pattern_fill_set_pattern_type() {
        let mut pattern = PatternFill::with_rgb(
            PatternType::Horizontal,
            RGBColor::new(255, 0, 0),
            RGBColor::new(0, 0, 255),
        );
        
        pattern.set_pattern_type(PatternType::DiagonalDown);
        assert_eq!(pattern.pattern_type(), PatternType::DiagonalDown);
    }

    #[test]
    fn test_pattern_fill_color_access() {
        let fore = ColorFormat::from_rgb(RGBColor::new(255, 0, 0));
        let back = ColorFormat::from_rgb(RGBColor::new(0, 0, 255));
        let pattern = PatternFill::new(PatternType::Checker, fore, back);
        
        assert!(pattern.fore_color().rgb().is_some());
        assert!(pattern.back_color().rgb().is_some());
    }

    #[test]
    fn test_pattern_fill_swap_colors() {
        let mut pattern = PatternFill::with_rgb(
            PatternType::Cross,
            RGBColor::new(255, 0, 0),
            RGBColor::new(0, 0, 255),
        );
        
        let fore_rgb_before = pattern.fore_color().rgb().unwrap();
        let back_rgb_before = pattern.back_color().rgb().unwrap();
        
        pattern.swap_colors();
        
        let fore_rgb_after = pattern.fore_color().rgb().unwrap();
        let back_rgb_after = pattern.back_color().rgb().unwrap();
        
        assert_eq!(fore_rgb_before.r, back_rgb_after.r);
        assert_eq!(back_rgb_before.r, fore_rgb_after.r);
    }

    #[test]
    fn test_all_pattern_types() {
        let patterns = vec![
            PatternType::Solid,
            PatternType::Horizontal,
            PatternType::Vertical,
            PatternType::DiagonalDown,
            PatternType::DiagonalUp,
            PatternType::Cross,
            PatternType::DiagonalCross,
            PatternType::HorizontalBrick,
            PatternType::VerticalBrick,
            PatternType::Checker,
            PatternType::Dots,
            PatternType::Dashes,
            PatternType::Waves,
            PatternType::Trellis,
            PatternType::ThickHorizontal,
            PatternType::ThickVertical,
            PatternType::ThickDiagonalDown,
            PatternType::ThickDiagonalUp,
            PatternType::ThickCross,
            PatternType::ThickDiagonalCross,
        ];
        
        assert_eq!(patterns.len(), 20);
    }
}
