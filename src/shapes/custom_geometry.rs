//! Custom geometry for freeform shapes
//!
//! Supports creating custom shapes with points, curves, and paths.
//! Based on PptxGenJS custom geometry implementation.

use std::f64;

/// Curve type for custom geometry points
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurveType {
    /// Quadratic Bezier curve
    Quadratic,
    /// Cubic Bezier curve
    Cubic,
}

/// A point in custom geometry
#[derive(Debug, Clone)]
pub struct GeometryPoint {
    /// X coordinate (0.0 to 1.0 normalized)
    pub x: f64,
    /// Y coordinate (0.0 to 1.0 normalized)
    pub y: f64,
    /// Optional curve information
    pub curve: Option<CurveInfo>,
    /// Whether this point closes the path
    pub close: bool,
}

/// Curve information for a point
#[derive(Debug, Clone)]
pub struct CurveInfo {
    /// Curve type (Quadratic or Cubic)
    pub curve_type: CurveType,
    /// First control point X (for quadratic) or first control point X (for cubic)
    pub x1: f64,
    /// First control point Y
    pub y1: f64,
    /// Second control point X (for cubic only)
    pub x2: Option<f64>,
    /// Second control point Y (for cubic only)
    pub y2: Option<f64>,
}

impl GeometryPoint {
    /// Create a new point
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            curve: None,
            close: false,
        }
    }

    /// Create a point with quadratic curve
    pub fn with_quadratic_curve(x: f64, y: f64, x1: f64, y1: f64) -> Self {
        Self {
            x,
            y,
            curve: Some(CurveInfo {
                curve_type: CurveType::Quadratic,
                x1,
                y1,
                x2: None,
                y2: None,
            }),
            close: false,
        }
    }

    /// Create a point with cubic curve
    pub fn with_cubic_curve(x: f64, y: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            x,
            y,
            curve: Some(CurveInfo {
                curve_type: CurveType::Cubic,
                x1,
                y1,
                x2: Some(x2),
                y2: Some(y2),
            }),
            close: false,
        }
    }

    /// Create a closing point
    pub fn close() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            curve: None,
            close: true,
        }
    }

    /// Clamp value to 0.0-1.0 range
    fn clamp(value: f64) -> f64 {
        if value < 0.0 {
            0.0
        } else if value > 1.0 {
            1.0
        } else {
            value
        }
    }

    /// Check if this is a valid point
    pub fn is_valid(&self) -> bool {
        if self.close {
            return true;
        }
        self.x >= 0.0 && self.x <= 1.0 && self.y >= 0.0 && self.y <= 1.0
    }
}

/// Custom geometry shape
#[derive(Debug, Clone)]
pub struct CustomGeometry {
    /// Points that define the shape
    points: Vec<GeometryPoint>,
}

impl CustomGeometry {
    /// Create a new custom geometry
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }

    /// Add a point to the geometry
    pub fn add_point(&mut self, point: GeometryPoint) -> Result<(), String> {
        if !point.is_valid() {
            return Err(format!(
                "Invalid point: x={}, y={} (must be 0.0-1.0)",
                point.x, point.y
            ));
        }
        self.points.push(point);
        Ok(())
    }

    /// Add a simple point
    pub fn add_simple_point(&mut self, x: f64, y: f64) -> Result<(), String> {
        self.add_point(GeometryPoint::new(x, y))
    }

    /// Add a point with quadratic curve
    pub fn add_quadratic_point(&mut self, x: f64, y: f64, x1: f64, y1: f64) -> Result<(), String> {
        self.add_point(GeometryPoint::with_quadratic_curve(x, y, x1, y1))
    }

    /// Add a point with cubic curve
    pub fn add_cubic_point(
        &mut self,
        x: f64,
        y: f64,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) -> Result<(), String> {
        self.add_point(GeometryPoint::with_cubic_curve(x, y, x1, y1, x2, y2))
    }

    /// Close the path
    pub fn close_path(&mut self) {
        self.points.push(GeometryPoint::close());
    }

    /// Get all points
    pub fn points(&self) -> &[GeometryPoint] {
        &self.points
    }

    /// Get mutable points
    pub fn points_mut(&mut self) -> &mut Vec<GeometryPoint> {
        &mut self.points
    }

    /// Clear all points
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// Get number of points
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Check if geometry is empty
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Validate geometry
    pub fn validate(&self) -> Result<(), String> {
        if self.points.is_empty() {
            return Err("Geometry must have at least one point".to_string());
        }

        // Check for at least one close point
        let has_close = self.points.iter().any(|p| p.close);
        if !has_close {
            return Err("Geometry must have at least one close point".to_string());
        }

        Ok(())
    }
}

impl Default for CustomGeometry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry_point_new() {
        let point = GeometryPoint::new(0.5, 0.5);
        assert_eq!(point.x, 0.5);
        assert_eq!(point.y, 0.5);
        assert!(point.curve.is_none());
        assert!(!point.close);
    }

    #[test]
    fn test_geometry_point_out_of_range() {
        let point = GeometryPoint::new(1.5, -0.5);
        assert_eq!(point.x, 1.5);
        assert_eq!(point.y, -0.5);
        assert!(!point.is_valid());
    }

    #[test]
    fn test_geometry_point_quadratic_curve() {
        let point = GeometryPoint::with_quadratic_curve(0.5, 0.5, 0.3, 0.3);
        assert_eq!(point.x, 0.5);
        assert_eq!(point.y, 0.5);
        assert!(point.curve.is_some());
        let curve = point.curve.unwrap();
        assert_eq!(curve.curve_type, CurveType::Quadratic);
        assert_eq!(curve.x1, 0.3);
        assert_eq!(curve.y1, 0.3);
    }

    #[test]
    fn test_geometry_point_cubic_curve() {
        let point = GeometryPoint::with_cubic_curve(0.5, 0.5, 0.3, 0.3, 0.7, 0.7);
        assert_eq!(point.x, 0.5);
        assert_eq!(point.y, 0.5);
        assert!(point.curve.is_some());
        let curve = point.curve.unwrap();
        assert_eq!(curve.curve_type, CurveType::Cubic);
        assert_eq!(curve.x1, 0.3);
        assert_eq!(curve.y1, 0.3);
        assert_eq!(curve.x2, Some(0.7));
        assert_eq!(curve.y2, Some(0.7));
    }

    #[test]
    fn test_geometry_point_close() {
        let point = GeometryPoint::close();
        assert!(point.close);
    }

    #[test]
    fn test_custom_geometry_new() {
        let geom = CustomGeometry::new();
        assert!(geom.is_empty());
        assert_eq!(geom.len(), 0);
    }

    #[test]
    fn test_custom_geometry_add_point() {
        let mut geom = CustomGeometry::new();
        let point = GeometryPoint::new(0.5, 0.5);
        assert!(geom.add_point(point).is_ok());
        assert_eq!(geom.len(), 1);
    }

    #[test]
    fn test_custom_geometry_add_invalid_point() {
        let mut geom = CustomGeometry::new();
        let point = GeometryPoint::new(1.5, 0.5);
        assert!(geom.add_point(point).is_err());
    }

    #[test]
    fn test_custom_geometry_add_simple_point() {
        let mut geom = CustomGeometry::new();
        assert!(geom.add_simple_point(0.5, 0.5).is_ok());
        assert_eq!(geom.len(), 1);
    }

    #[test]
    fn test_custom_geometry_add_quadratic_point() {
        let mut geom = CustomGeometry::new();
        assert!(geom.add_quadratic_point(0.5, 0.5, 0.3, 0.3).is_ok());
        assert_eq!(geom.len(), 1);
    }

    #[test]
    fn test_custom_geometry_add_cubic_point() {
        let mut geom = CustomGeometry::new();
        assert!(geom.add_cubic_point(0.5, 0.5, 0.3, 0.3, 0.7, 0.7).is_ok());
        assert_eq!(geom.len(), 1);
    }

    #[test]
    fn test_custom_geometry_close_path() {
        let mut geom = CustomGeometry::new();
        geom.add_simple_point(0.0, 0.0).unwrap();
        geom.add_simple_point(1.0, 0.0).unwrap();
        geom.add_simple_point(0.5, 1.0).unwrap();
        geom.close_path();
        assert_eq!(geom.len(), 4);
        assert!(geom.points()[3].close);
    }

    #[test]
    fn test_custom_geometry_clear() {
        let mut geom = CustomGeometry::new();
        geom.add_simple_point(0.5, 0.5).unwrap();
        assert_eq!(geom.len(), 1);
        geom.clear();
        assert!(geom.is_empty());
    }

    #[test]
    fn test_custom_geometry_validate_empty() {
        let geom = CustomGeometry::new();
        assert!(geom.validate().is_err());
    }

    #[test]
    fn test_custom_geometry_validate_no_close() {
        let mut geom = CustomGeometry::new();
        geom.add_simple_point(0.5, 0.5).unwrap();
        assert!(geom.validate().is_err());
    }

    #[test]
    fn test_custom_geometry_validate_valid() {
        let mut geom = CustomGeometry::new();
        geom.add_simple_point(0.0, 0.0).unwrap();
        geom.add_simple_point(1.0, 0.0).unwrap();
        geom.add_simple_point(0.5, 1.0).unwrap();
        geom.close_path();
        assert!(geom.validate().is_ok());
    }

    #[test]
    fn test_custom_geometry_triangle() {
        let mut geom = CustomGeometry::new();
        geom.add_simple_point(0.0, 0.0).unwrap();
        geom.add_simple_point(1.0, 0.0).unwrap();
        geom.add_simple_point(0.5, 1.0).unwrap();
        geom.close_path();
        
        assert_eq!(geom.len(), 4);
        assert!(geom.validate().is_ok());
    }

    #[test]
    fn test_custom_geometry_with_curves() {
        let mut geom = CustomGeometry::new();
        geom.add_simple_point(0.0, 0.0).unwrap();
        geom.add_quadratic_point(0.5, 1.0, 0.25, 0.5).unwrap();
        geom.add_simple_point(1.0, 0.0).unwrap();
        geom.close_path();
        
        assert_eq!(geom.len(), 4);
        assert!(geom.validate().is_ok());
        assert!(geom.points()[1].curve.is_some());
    }

    #[test]
    fn test_custom_geometry_complex_shape() {
        let mut geom = CustomGeometry::new();
        
        // Create a wavy shape with valid coordinates
        geom.add_simple_point(0.0, 0.0).unwrap();
        geom.add_quadratic_point(0.5, 1.0, 0.25, 0.5).unwrap();
        geom.add_simple_point(1.0, 0.0).unwrap();
        geom.add_cubic_point(0.5, 0.5, 0.75, 0.25, 0.25, 0.25).unwrap();
        geom.close_path();
        
        assert_eq!(geom.len(), 5);
        assert!(geom.validate().is_ok());
    }
}
