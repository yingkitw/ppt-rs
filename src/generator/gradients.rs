//! Gradient fill support for PPTX shapes
//!
//! Provides gradient types and XML generation for shape fills.

/// Gradient types
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum GradientType {
    /// Linear gradient
    Linear,
    /// Radial gradient
    Radial,
    /// Rectangular gradient
    Rectangular,
    /// Path gradient
    Path,
}

impl GradientType {
    /// Get OOXML gradient type value
    pub fn xml_value(&self) -> &'static str {
        match self {
            GradientType::Linear => "lin",
            GradientType::Radial => "circle",
            GradientType::Rectangular => "rect",
            GradientType::Path => "path",
        }
    }
}

/// Gradient direction for linear gradients (in degrees)
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum GradientDirection {
    /// Left to right (0°)
    Horizontal,
    /// Top to bottom (90°)
    Vertical,
    /// Top-left to bottom-right (45°)
    DiagonalDown,
    /// Bottom-left to top-right (315°)
    DiagonalUp,
    /// Custom angle in degrees
    Custom(u32),
}

impl GradientDirection {
    /// Get angle in 60000ths of a degree (OOXML format)
    pub fn angle(&self) -> u32 {
        match self {
            GradientDirection::Horizontal => 0,
            GradientDirection::Vertical => 5400000,
            GradientDirection::DiagonalDown => 2700000,
            GradientDirection::DiagonalUp => 18900000,
            GradientDirection::Custom(deg) => deg * 60000,
        }
    }
}

/// A color stop in a gradient
#[derive(Clone, Debug)]
pub struct GradientStop {
    /// Position (0-100000, where 100000 = 100%)
    pub position: u32,
    /// Color (RGB hex)
    pub color: String,
    /// Transparency (0-100000, where 100000 = fully transparent)
    pub transparency: Option<u32>,
}

impl GradientStop {
    /// Create a new gradient stop
    pub fn new(position: u32, color: &str) -> Self {
        GradientStop {
            position: position.min(100000),
            color: color.trim_start_matches('#').to_uppercase(),
            transparency: None,
        }
    }

    /// Create stop at start (0%)
    pub fn start(color: &str) -> Self {
        Self::new(0, color)
    }

    /// Create stop at middle (50%)
    pub fn middle(color: &str) -> Self {
        Self::new(50000, color)
    }

    /// Create stop at end (100%)
    pub fn end(color: &str) -> Self {
        Self::new(100000, color)
    }

    /// Set transparency (0-100 percent)
    pub fn with_transparency(mut self, percent: u32) -> Self {
        self.transparency = Some(percent.min(100) * 1000);
        self
    }
}

/// Gradient fill definition
#[derive(Clone, Debug)]
pub struct GradientFill {
    /// Gradient type
    pub gradient_type: GradientType,
    /// Direction (for linear gradients)
    pub direction: GradientDirection,
    /// Color stops
    pub stops: Vec<GradientStop>,
    /// Rotate with shape
    pub rotate_with_shape: bool,
}

impl GradientFill {
    /// Create a new gradient fill
    pub fn new(gradient_type: GradientType) -> Self {
        GradientFill {
            gradient_type,
            direction: GradientDirection::Vertical,
            stops: Vec::new(),
            rotate_with_shape: true,
        }
    }

    /// Create a linear gradient
    pub fn linear(direction: GradientDirection) -> Self {
        let mut fill = Self::new(GradientType::Linear);
        fill.direction = direction;
        fill
    }

    /// Create a radial gradient
    pub fn radial() -> Self {
        Self::new(GradientType::Radial)
    }

    /// Create a simple two-color gradient
    pub fn two_color(start_color: &str, end_color: &str) -> Self {
        Self::linear(GradientDirection::Vertical)
            .add_stop(GradientStop::start(start_color))
            .add_stop(GradientStop::end(end_color))
    }

    /// Create a three-color gradient
    pub fn three_color(start_color: &str, middle_color: &str, end_color: &str) -> Self {
        Self::linear(GradientDirection::Vertical)
            .add_stop(GradientStop::start(start_color))
            .add_stop(GradientStop::middle(middle_color))
            .add_stop(GradientStop::end(end_color))
    }

    /// Add a gradient stop
    pub fn add_stop(mut self, stop: GradientStop) -> Self {
        self.stops.push(stop);
        self
    }

    /// Set direction
    pub fn with_direction(mut self, direction: GradientDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set rotate with shape
    pub fn with_rotate(mut self, rotate: bool) -> Self {
        self.rotate_with_shape = rotate;
        self
    }

    /// Sort stops by position
    pub fn sorted(mut self) -> Self {
        self.stops.sort_by_key(|s| s.position);
        self
    }
}

/// Preset gradient definitions
pub struct PresetGradients;

impl PresetGradients {
    /// Blue gradient
    pub fn blue() -> GradientFill {
        GradientFill::two_color("0066CC", "003366")
    }

    /// Green gradient
    pub fn green() -> GradientFill {
        GradientFill::two_color("00CC66", "006633")
    }

    /// Red gradient
    pub fn red() -> GradientFill {
        GradientFill::two_color("CC0000", "660000")
    }

    /// Orange gradient
    pub fn orange() -> GradientFill {
        GradientFill::two_color("FF9900", "CC6600")
    }

    /// Purple gradient
    pub fn purple() -> GradientFill {
        GradientFill::two_color("9933CC", "660099")
    }

    /// Gray gradient
    pub fn gray() -> GradientFill {
        GradientFill::two_color("999999", "333333")
    }

    /// Sunrise gradient (orange to yellow)
    pub fn sunrise() -> GradientFill {
        GradientFill::three_color("FF6600", "FFCC00", "FFFF66")
    }

    /// Ocean gradient (dark blue to light blue)
    pub fn ocean() -> GradientFill {
        GradientFill::three_color("003366", "0066CC", "66CCFF")
    }

    /// Forest gradient (dark green to light green)
    pub fn forest() -> GradientFill {
        GradientFill::three_color("003300", "006600", "66CC66")
    }

    /// Rainbow gradient
    pub fn rainbow() -> GradientFill {
        GradientFill::linear(GradientDirection::Horizontal)
            .add_stop(GradientStop::new(0, "FF0000"))
            .add_stop(GradientStop::new(17000, "FF9900"))
            .add_stop(GradientStop::new(33000, "FFFF00"))
            .add_stop(GradientStop::new(50000, "00FF00"))
            .add_stop(GradientStop::new(67000, "0000FF"))
            .add_stop(GradientStop::new(83000, "9900FF"))
            .add_stop(GradientStop::new(100000, "FF00FF"))
    }
}

/// Generate gradient fill XML
pub fn generate_gradient_fill_xml(gradient: &GradientFill) -> String {
    let mut xml = String::from(r#"<a:gradFill rotWithShape=""#);
    xml.push_str(if gradient.rotate_with_shape { "1" } else { "0" });
    xml.push_str(r#"">"#);

    // Generate gradient stop list
    xml.push_str("<a:gsLst>");
    for stop in &gradient.stops {
        xml.push_str(&format!(
            r#"<a:gs pos="{}"><a:srgbClr val="{}""#,
            stop.position, stop.color
        ));

        if let Some(alpha) = stop.transparency {
            xml.push_str(&format!(r#"><a:alpha val="{}"/></a:srgbClr>"#, 100000 - alpha));
        } else {
            xml.push_str("/>");
        }

        xml.push_str("</a:gs>");
    }
    xml.push_str("</a:gsLst>");

    // Generate gradient type-specific elements
    match gradient.gradient_type {
        GradientType::Linear => {
            xml.push_str(&format!(
                r#"<a:lin ang="{}" scaled="1"/>"#,
                gradient.direction.angle()
            ));
        }
        GradientType::Radial | GradientType::Rectangular | GradientType::Path => {
            xml.push_str(&format!(
                r#"<a:path path="{}"><a:fillToRect l="50000" t="50000" r="50000" b="50000"/></a:path>"#,
                gradient.gradient_type.xml_value()
            ));
        }
    }

    xml.push_str("</a:gradFill>");
    xml
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_type_xml() {
        assert_eq!(GradientType::Linear.xml_value(), "lin");
        assert_eq!(GradientType::Radial.xml_value(), "circle");
    }

    #[test]
    fn test_gradient_direction_angle() {
        assert_eq!(GradientDirection::Horizontal.angle(), 0);
        assert_eq!(GradientDirection::Vertical.angle(), 5400000);
        assert_eq!(GradientDirection::Custom(45).angle(), 2700000);
    }

    #[test]
    fn test_gradient_stop() {
        let stop = GradientStop::new(50000, "#FF0000");
        assert_eq!(stop.position, 50000);
        assert_eq!(stop.color, "FF0000");
    }

    #[test]
    fn test_gradient_stop_with_transparency() {
        let stop = GradientStop::new(0, "000000").with_transparency(50);
        assert_eq!(stop.transparency, Some(50000));
    }

    #[test]
    fn test_two_color_gradient() {
        let gradient = GradientFill::two_color("FF0000", "0000FF");
        assert_eq!(gradient.stops.len(), 2);
        assert_eq!(gradient.stops[0].color, "FF0000");
        assert_eq!(gradient.stops[1].color, "0000FF");
    }

    #[test]
    fn test_three_color_gradient() {
        let gradient = GradientFill::three_color("FF0000", "00FF00", "0000FF");
        assert_eq!(gradient.stops.len(), 3);
    }

    #[test]
    fn test_preset_gradients() {
        let blue = PresetGradients::blue();
        assert_eq!(blue.stops.len(), 2);

        let rainbow = PresetGradients::rainbow();
        assert_eq!(rainbow.stops.len(), 7);
    }

    #[test]
    fn test_generate_gradient_xml() {
        let gradient = GradientFill::two_color("FF0000", "0000FF");
        let xml = generate_gradient_fill_xml(&gradient);
        assert!(xml.contains("gradFill"));
        assert!(xml.contains("gsLst"));
        assert!(xml.contains("FF0000"));
        assert!(xml.contains("0000FF"));
    }

    #[test]
    fn test_radial_gradient_xml() {
        let gradient = GradientFill::radial()
            .add_stop(GradientStop::start("FFFFFF"))
            .add_stop(GradientStop::end("000000"));
        let xml = generate_gradient_fill_xml(&gradient);
        assert!(xml.contains("path"));
        assert!(xml.contains("circle"));
    }
}
