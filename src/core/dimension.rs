//! Flexible dimension types for position and size specification
//!
//! Supports multiple units: EMU, inches, centimeters, points, and ratio (0.0–1.0 of slide).
//!
//! # Examples
//! ```
//! use ppt_rs::core::Dimension;
//!
//! // Different ways to express the same position
//! let d1 = Dimension::Emu(914400);
//! let d2 = Dimension::Inches(1.0);
//! let d3 = Dimension::Cm(2.54);
//! let d4 = Dimension::Ratio(0.1); // 10% of reference (slide width or height)
//!
//! assert_eq!(d1.to_emu(9144000), 914400);
//! assert_eq!(d2.to_emu(9144000), 914400);
//! assert_eq!(d4.to_emu(9144000), 914400); // 10% of 10 inches
//! ```

/// Standard slide width in EMU (10 inches)
pub const SLIDE_WIDTH_EMU: u32 = 9144000;
/// Standard slide height in EMU (7.5 inches)
pub const SLIDE_HEIGHT_EMU: u32 = 6858000;

/// EMU per inch
const EMU_PER_INCH: f64 = 914400.0;
/// EMU per centimeter
const EMU_PER_CM: f64 = 360000.0;
/// EMU per point
const EMU_PER_PT: f64 = 12700.0;

/// A flexible dimension that can be expressed in multiple units.
///
/// All variants resolve to EMU (English Metric Units) at render time.
/// `Ratio` is relative to a reference dimension (slide width for x/width, slide height for y/height).
#[derive(Clone, Debug, PartialEq)]
pub enum Dimension {
    /// Absolute value in EMU (English Metric Units)
    Emu(u32),
    /// Value in inches (1 inch = 914400 EMU)
    Inches(f64),
    /// Value in centimeters (1 cm = 360000 EMU)
    Cm(f64),
    /// Value in points (1 pt = 12700 EMU)
    Pt(f64),
    /// Ratio of reference dimension (0.0–1.0). For x/width, reference is slide width; for y/height, slide height.
    Ratio(f64),
}

impl Dimension {
    /// Resolve to EMU given a reference dimension (used only for `Ratio`).
    ///
    /// For absolute units (Emu, Inches, Cm, Pt), `reference_emu` is ignored.
    pub fn to_emu(&self, reference_emu: u32) -> u32 {
        match self {
            Dimension::Emu(v) => *v,
            Dimension::Inches(v) => (v * EMU_PER_INCH) as u32,
            Dimension::Cm(v) => (v * EMU_PER_CM) as u32,
            Dimension::Pt(v) => (v * EMU_PER_PT) as u32,
            Dimension::Ratio(r) => (r.clamp(0.0, 1.0) * reference_emu as f64) as u32,
        }
    }

    /// Resolve X position or width to EMU (reference = slide width)
    pub fn to_emu_x(&self) -> u32 {
        self.to_emu(SLIDE_WIDTH_EMU)
    }

    /// Resolve Y position or height to EMU (reference = slide height)
    pub fn to_emu_y(&self) -> u32 {
        self.to_emu(SLIDE_HEIGHT_EMU)
    }
}

/// Convenience: convert from u32 (treated as EMU)
impl From<u32> for Dimension {
    fn from(emu: u32) -> Self {
        Dimension::Emu(emu)
    }
}

/// Convenience: convert from f64 (treated as ratio if 0.0–1.0, else inches)
/// This is intentionally NOT implemented to avoid ambiguity.
/// Use the explicit constructors instead.

/// Shorthand constructors for ergonomic API
impl Dimension {
    /// Create from inches
    pub fn inches(v: f64) -> Self { Dimension::Inches(v) }
    /// Create from centimeters
    pub fn cm(v: f64) -> Self { Dimension::Cm(v) }
    /// Create from points
    pub fn pt(v: f64) -> Self { Dimension::Pt(v) }
    /// Create from ratio (0.0–1.0 of slide dimension)
    pub fn ratio(v: f64) -> Self { Dimension::Ratio(v) }
    /// Create from EMU
    pub fn emu(v: u32) -> Self { Dimension::Emu(v) }
    /// Create from percentage (0–100) of slide dimension
    pub fn percent(v: f64) -> Self { Dimension::Ratio(v / 100.0) }
}

/// A 2D position expressed in flexible dimensions.
#[derive(Clone, Debug)]
pub struct FlexPosition {
    pub x: Dimension,
    pub y: Dimension,
}

impl FlexPosition {
    pub fn new(x: Dimension, y: Dimension) -> Self {
        Self { x, y }
    }

    /// Resolve to (x_emu, y_emu) using standard slide dimensions
    pub fn to_emu(&self) -> (u32, u32) {
        (self.x.to_emu_x(), self.y.to_emu_y())
    }

    /// Resolve to (x_emu, y_emu) using custom slide dimensions
    pub fn to_emu_with(&self, slide_width: u32, slide_height: u32) -> (u32, u32) {
        (self.x.to_emu(slide_width), self.y.to_emu(slide_height))
    }
}

/// A 2D size expressed in flexible dimensions.
#[derive(Clone, Debug)]
pub struct FlexSize {
    pub width: Dimension,
    pub height: Dimension,
}

impl FlexSize {
    pub fn new(width: Dimension, height: Dimension) -> Self {
        Self { width, height }
    }

    /// Resolve to (width_emu, height_emu) using standard slide dimensions
    pub fn to_emu(&self) -> (u32, u32) {
        (self.width.to_emu_x(), self.height.to_emu_y())
    }

    /// Resolve to (width_emu, height_emu) using custom slide dimensions
    pub fn to_emu_with(&self, slide_width: u32, slide_height: u32) -> (u32, u32) {
        (self.width.to_emu(slide_width), self.height.to_emu(slide_height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emu_passthrough() {
        assert_eq!(Dimension::Emu(914400).to_emu(0), 914400);
    }

    #[test]
    fn test_inches_to_emu() {
        assert_eq!(Dimension::Inches(1.0).to_emu(0), 914400);
        assert_eq!(Dimension::Inches(0.5).to_emu(0), 457200);
        assert_eq!(Dimension::Inches(10.0).to_emu(0), 9144000);
    }

    #[test]
    fn test_cm_to_emu() {
        assert_eq!(Dimension::Cm(2.54).to_emu(0), 914400);
        assert_eq!(Dimension::Cm(1.0).to_emu(0), 360000);
    }

    #[test]
    fn test_pt_to_emu() {
        assert_eq!(Dimension::Pt(72.0).to_emu(0), 914400); // 72pt = 1 inch
        assert_eq!(Dimension::Pt(1.0).to_emu(0), 12700);
    }

    #[test]
    fn test_ratio_to_emu() {
        // 10% of slide width (10 inches = 9144000 EMU) = 1 inch
        assert_eq!(Dimension::Ratio(0.1).to_emu(SLIDE_WIDTH_EMU), 914400);
        // 50% of slide width = 5 inches
        assert_eq!(Dimension::Ratio(0.5).to_emu(SLIDE_WIDTH_EMU), 4572000);
        // 100% of slide width = 10 inches
        assert_eq!(Dimension::Ratio(1.0).to_emu(SLIDE_WIDTH_EMU), 9144000);
        // 0% = 0
        assert_eq!(Dimension::Ratio(0.0).to_emu(SLIDE_WIDTH_EMU), 0);
    }

    #[test]
    fn test_ratio_clamped() {
        // Values > 1.0 clamped to 1.0
        assert_eq!(Dimension::Ratio(1.5).to_emu(SLIDE_WIDTH_EMU), 9144000);
        // Values < 0.0 clamped to 0.0
        assert_eq!(Dimension::Ratio(-0.5).to_emu(SLIDE_WIDTH_EMU), 0);
    }

    #[test]
    fn test_percent() {
        assert_eq!(Dimension::percent(50.0).to_emu(SLIDE_WIDTH_EMU), 4572000);
        assert_eq!(Dimension::percent(10.0).to_emu(SLIDE_WIDTH_EMU), 914400);
    }

    #[test]
    fn test_to_emu_x_y() {
        let x = Dimension::Ratio(0.5);
        let y = Dimension::Ratio(0.5);
        assert_eq!(x.to_emu_x(), SLIDE_WIDTH_EMU / 2);
        assert_eq!(y.to_emu_y(), SLIDE_HEIGHT_EMU / 2);
    }

    #[test]
    fn test_flex_position() {
        let pos = FlexPosition::new(Dimension::Inches(1.0), Dimension::Ratio(0.5));
        let (x, y) = pos.to_emu();
        assert_eq!(x, 914400);
        assert_eq!(y, SLIDE_HEIGHT_EMU / 2);
    }

    #[test]
    fn test_flex_size() {
        let size = FlexSize::new(Dimension::Ratio(0.8), Dimension::Inches(2.0));
        let (w, h) = size.to_emu();
        assert_eq!(w, (SLIDE_WIDTH_EMU as f64 * 0.8) as u32);
        assert_eq!(h, 914400 * 2);
    }

    #[test]
    fn test_flex_position_custom_slide() {
        let custom_w = 12192000_u32; // 13.33 inches (widescreen)
        let custom_h = 6858000_u32;
        let pos = FlexPosition::new(Dimension::Ratio(0.5), Dimension::Ratio(0.5));
        let (x, y) = pos.to_emu_with(custom_w, custom_h);
        assert_eq!(x, custom_w / 2);
        assert_eq!(y, custom_h / 2);
    }

    #[test]
    fn test_from_u32() {
        let d: Dimension = 914400_u32.into();
        assert_eq!(d, Dimension::Emu(914400));
    }

    #[test]
    fn test_shorthand_constructors() {
        assert_eq!(Dimension::inches(1.0), Dimension::Inches(1.0));
        assert_eq!(Dimension::cm(2.54), Dimension::Cm(2.54));
        assert_eq!(Dimension::pt(72.0), Dimension::Pt(72.0));
        assert_eq!(Dimension::ratio(0.5), Dimension::Ratio(0.5));
        assert_eq!(Dimension::emu(914400), Dimension::Emu(914400));
    }
}
