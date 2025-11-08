//! DrawingML gradient fill functionality

use crate::dml::color::{ColorFormat, RGBColor};

/// Gradient stop - represents a color stop in a gradient
pub struct GradientStop {
    position: f64,  // 0.0 to 1.0
    color: ColorFormat,
}

impl GradientStop {
    /// Create a new gradient stop
    pub fn new(position: f64, color: ColorFormat) -> crate::error::Result<Self> {
        if position < 0.0 || position > 1.0 {
            return Err(crate::error::PptError::ValueError(
                format!("Gradient stop position must be between 0.0 and 1.0, got {}", position)
            ));
        }
        Ok(Self { position, color })
    }

    /// Get the position (0.0 to 1.0)
    pub fn position(&self) -> f64 {
        self.position
    }

    /// Set the position
    pub fn set_position(&mut self, position: f64) -> crate::error::Result<()> {
        if position < 0.0 || position > 1.0 {
            return Err(crate::error::PptError::ValueError(
                format!("Gradient stop position must be between 0.0 and 1.0, got {}", position)
            ));
        }
        self.position = position;
        Ok(())
    }

    /// Get the color
    pub fn color(&self) -> &ColorFormat {
        &self.color
    }

    /// Get mutable color
    pub fn color_mut(&mut self) -> &mut ColorFormat {
        &mut self.color
    }
}

/// Gradient fill type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GradientType {
    /// Linear gradient
    Linear,
    /// Radial gradient (from center outward)
    Radial,
    /// Rectangular gradient
    Rectangular,
    /// Path gradient
    Path,
}

/// Gradient fill - provides access to gradient fill properties
pub struct GradientFill {
    gradient_type: GradientType,
    angle: f64,  // 0.0 to 360.0 degrees (for linear gradients)
    stops: Vec<GradientStop>,
}

impl GradientFill {
    /// Create a new linear gradient fill
    pub fn linear() -> Self {
        Self {
            gradient_type: GradientType::Linear,
            angle: 0.0,
            stops: vec![],
        }
    }

    /// Create a new radial gradient fill
    pub fn radial() -> Self {
        Self {
            gradient_type: GradientType::Radial,
            angle: 0.0,
            stops: vec![],
        }
    }

    /// Create a linear gradient with two colors
    pub fn linear_with_colors(start_color: RGBColor, end_color: RGBColor) -> crate::error::Result<Self> {
        let mut gradient = Self::linear();
        gradient.add_stop(0.0, ColorFormat::from_rgb(start_color))?;
        gradient.add_stop(1.0, ColorFormat::from_rgb(end_color))?;
        Ok(gradient)
    }

    /// Get the gradient type
    pub fn gradient_type(&self) -> GradientType {
        self.gradient_type
    }

    /// Set the gradient type
    pub fn set_gradient_type(&mut self, gradient_type: GradientType) {
        self.gradient_type = gradient_type;
    }

    /// Get the angle (for linear gradients)
    pub fn angle(&self) -> f64 {
        self.angle
    }

    /// Set the angle (0.0 to 360.0 degrees)
    pub fn set_angle(&mut self, angle: f64) -> crate::error::Result<()> {
        if angle < 0.0 || angle > 360.0 {
            return Err(crate::error::PptError::ValueError(
                format!("Gradient angle must be between 0.0 and 360.0, got {}", angle)
            ));
        }
        self.angle = angle;
        Ok(())
    }

    /// Add a gradient stop
    pub fn add_stop(&mut self, position: f64, color: ColorFormat) -> crate::error::Result<()> {
        let stop = GradientStop::new(position, color)?;
        self.stops.push(stop);
        // Sort stops by position
        self.stops.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
        Ok(())
    }

    /// Get all gradient stops
    pub fn stops(&self) -> &[GradientStop] {
        &self.stops
    }

    /// Get mutable gradient stops
    pub fn stops_mut(&mut self) -> &mut [GradientStop] {
        &mut self.stops
    }

    /// Get a specific gradient stop
    pub fn stop(&self, index: usize) -> Option<&GradientStop> {
        self.stops.get(index)
    }

    /// Get a mutable gradient stop
    pub fn stop_mut(&mut self, index: usize) -> Option<&mut GradientStop> {
        self.stops.get_mut(index)
    }

    /// Clear all gradient stops
    pub fn clear_stops(&mut self) {
        self.stops.clear();
    }

    /// Get the number of gradient stops
    pub fn stop_count(&self) -> usize {
        self.stops.len()
    }

    /// Check if gradient has at least 2 stops
    pub fn is_valid(&self) -> bool {
        self.stops.len() >= 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_stop_creation() {
        let color = ColorFormat::from_rgb(RGBColor::new(255, 0, 0));
        let stop = GradientStop::new(0.5, color).unwrap();
        assert_eq!(stop.position(), 0.5);
    }

    #[test]
    fn test_gradient_stop_invalid_position() {
        let color1 = ColorFormat::from_rgb(RGBColor::new(255, 0, 0));
        let color2 = ColorFormat::from_rgb(RGBColor::new(255, 0, 0));
        assert!(GradientStop::new(1.5, color1).is_err());
        assert!(GradientStop::new(-0.1, color2).is_err());
    }

    #[test]
    fn test_linear_gradient() {
        let gradient = GradientFill::linear();
        assert_eq!(gradient.gradient_type(), GradientType::Linear);
        assert_eq!(gradient.angle(), 0.0);
        assert_eq!(gradient.stop_count(), 0);
    }

    #[test]
    fn test_radial_gradient() {
        let gradient = GradientFill::radial();
        assert_eq!(gradient.gradient_type(), GradientType::Radial);
    }

    #[test]
    fn test_gradient_with_colors() {
        let start = RGBColor::new(255, 0, 0);
        let end = RGBColor::new(0, 0, 255);
        let gradient = GradientFill::linear_with_colors(start, end).unwrap();
        
        assert_eq!(gradient.stop_count(), 2);
        assert_eq!(gradient.stop(0).unwrap().position(), 0.0);
        assert_eq!(gradient.stop(1).unwrap().position(), 1.0);
    }

    #[test]
    fn test_add_gradient_stop() {
        let mut gradient = GradientFill::linear();
        let color1 = ColorFormat::from_rgb(RGBColor::new(255, 0, 0));
        let color2 = ColorFormat::from_rgb(RGBColor::new(0, 255, 0));
        let color3 = ColorFormat::from_rgb(RGBColor::new(0, 0, 255));
        
        gradient.add_stop(0.0, color1).unwrap();
        gradient.add_stop(1.0, color2).unwrap();
        gradient.add_stop(0.5, color3).unwrap();
        
        assert_eq!(gradient.stop_count(), 3);
        // Verify stops are sorted
        assert_eq!(gradient.stop(0).unwrap().position(), 0.0);
        assert_eq!(gradient.stop(1).unwrap().position(), 0.5);
        assert_eq!(gradient.stop(2).unwrap().position(), 1.0);
    }

    #[test]
    fn test_gradient_angle() {
        let mut gradient = GradientFill::linear();
        gradient.set_angle(45.0).unwrap();
        assert_eq!(gradient.angle(), 45.0);
    }

    #[test]
    fn test_gradient_angle_invalid() {
        let mut gradient = GradientFill::linear();
        assert!(gradient.set_angle(361.0).is_err());
        assert!(gradient.set_angle(-1.0).is_err());
    }

    #[test]
    fn test_gradient_validity() {
        let mut gradient = GradientFill::linear();
        assert!(!gradient.is_valid());
        
        let color1 = ColorFormat::from_rgb(RGBColor::new(255, 0, 0));
        let color2 = ColorFormat::from_rgb(RGBColor::new(0, 0, 255));
        
        gradient.add_stop(0.0, color1).unwrap();
        assert!(!gradient.is_valid());
        
        gradient.add_stop(1.0, color2).unwrap();
        assert!(gradient.is_valid());
    }

    #[test]
    fn test_clear_stops() {
        let mut gradient = GradientFill::linear();
        let color1 = ColorFormat::from_rgb(RGBColor::new(255, 0, 0));
        let color2 = ColorFormat::from_rgb(RGBColor::new(0, 0, 255));
        
        gradient.add_stop(0.0, color1).unwrap();
        gradient.add_stop(1.0, color2).unwrap();
        
        assert_eq!(gradient.stop_count(), 2);
        gradient.clear_stops();
        assert_eq!(gradient.stop_count(), 0);
    }
}
