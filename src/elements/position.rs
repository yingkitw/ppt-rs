//! Position and size types for PPTX elements
//!
//! All measurements are in EMU (English Metric Units).
//! 1 inch = 914400 EMU
//! 1 cm = 360000 EMU
//! 1 pt = 12700 EMU

use crate::core::ToXml;

/// EMU conversion constants
pub const EMU_PER_INCH: i64 = 914400;
pub const EMU_PER_CM: i64 = 360000;
pub const EMU_PER_MM: i64 = 36000;
pub const EMU_PER_PT: i64 = 12700;

/// Position in EMU
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    /// Create position from EMU values
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// Create position from inches
    pub fn from_inches(x: f64, y: f64) -> Self {
        Self {
            x: (x * EMU_PER_INCH as f64) as i64,
            y: (y * EMU_PER_INCH as f64) as i64,
        }
    }

    /// Create position from centimeters
    pub fn from_cm(x: f64, y: f64) -> Self {
        Self {
            x: (x * EMU_PER_CM as f64) as i64,
            y: (y * EMU_PER_CM as f64) as i64,
        }
    }

    /// Get X in inches
    pub fn x_inches(&self) -> f64 {
        self.x as f64 / EMU_PER_INCH as f64
    }

    /// Get Y in inches
    pub fn y_inches(&self) -> f64 {
        self.y as f64 / EMU_PER_INCH as f64
    }
}

/// Size in EMU
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Size {
    pub width: i64,
    pub height: i64,
}

impl Size {
    /// Create size from EMU values
    pub fn new(width: i64, height: i64) -> Self {
        Self { width, height }
    }

    /// Create size from inches
    pub fn from_inches(width: f64, height: f64) -> Self {
        Self {
            width: (width * EMU_PER_INCH as f64) as i64,
            height: (height * EMU_PER_INCH as f64) as i64,
        }
    }

    /// Create size from centimeters
    pub fn from_cm(width: f64, height: f64) -> Self {
        Self {
            width: (width * EMU_PER_CM as f64) as i64,
            height: (height * EMU_PER_CM as f64) as i64,
        }
    }

    /// Get width in inches
    pub fn width_inches(&self) -> f64 {
        self.width as f64 / EMU_PER_INCH as f64
    }

    /// Get height in inches
    pub fn height_inches(&self) -> f64 {
        self.height as f64 / EMU_PER_INCH as f64
    }
}

/// Transform (position + size) for shapes
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Transform {
    pub position: Position,
    pub size: Size,
    pub rotation: i32, // in 60000ths of a degree
}

impl Transform {
    /// Create a new transform
    pub fn new(position: Position, size: Size) -> Self {
        Self {
            position,
            size,
            rotation: 0,
        }
    }

    /// Create from inches
    pub fn from_inches(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            position: Position::from_inches(x, y),
            size: Size::from_inches(width, height),
            rotation: 0,
        }
    }

    /// Set rotation in degrees
    pub fn with_rotation(mut self, degrees: f64) -> Self {
        self.rotation = (degrees * 60000.0) as i32;
        self
    }
}

impl ToXml for Transform {
    fn to_xml(&self) -> String {
        let mut xml = String::from("<a:xfrm");
        if self.rotation != 0 {
            xml.push_str(&format!(r#" rot="{}""#, self.rotation));
        }
        xml.push_str(&format!(
            r#"><a:off x="{}" y="{}"/><a:ext cx="{}" cy="{}"/></a:xfrm>"#,
            self.position.x, self.position.y, self.size.width, self.size.height
        ));
        xml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_from_inches() {
        let pos = Position::from_inches(1.0, 2.0);
        assert_eq!(pos.x, 914400);
        assert_eq!(pos.y, 1828800);
    }

    #[test]
    fn test_size_from_inches() {
        let size = Size::from_inches(3.0, 2.0);
        assert_eq!(size.width, 2743200);
        assert_eq!(size.height, 1828800);
    }

    #[test]
    fn test_transform_to_xml() {
        let transform = Transform::from_inches(1.0, 1.0, 2.0, 1.5);
        let xml = transform.to_xml();
        assert!(xml.contains("a:xfrm"));
        assert!(xml.contains("a:off"));
        assert!(xml.contains("a:ext"));
    }

    #[test]
    fn test_transform_with_rotation() {
        let transform = Transform::from_inches(0.0, 0.0, 1.0, 1.0)
            .with_rotation(45.0);
        let xml = transform.to_xml();
        assert!(xml.contains("rot=\"2700000\"")); // 45 * 60000
    }

    #[test]
    fn test_emu_constants() {
        assert_eq!(EMU_PER_INCH, 914400);
        assert_eq!(EMU_PER_CM, 360000);
        assert_eq!(EMU_PER_PT, 12700);
    }
}
