//! Shape effects for advanced shape formatting
//!
//! Provides comprehensive shape effects including:
//! - Glow effect
//! - Reflection effect
//! - Bevel effect

use crate::error::Result;
use crate::dml::color::RGBColor;

/// Shape effects
#[derive(Clone, Debug)]
pub struct ShapeEffects {
    /// Glow effect
    glow: Option<GlowEffect>,
    /// Reflection effect
    reflection: Option<ReflectionEffect>,
    /// Bevel effect
    bevel: Option<BevelEffect>,
}

/// Glow effect
#[derive(Clone, Debug, PartialEq)]
pub struct GlowEffect {
    /// Glow color
    color: RGBColor,
    /// Glow size in EMU
    size: u32,
    /// Glow opacity (0-100)
    opacity: u32,
}

/// Reflection effect
#[derive(Clone, Debug, PartialEq)]
pub struct ReflectionEffect {
    /// Reflection opacity (0-100)
    opacity: u32,
    /// Reflection distance in EMU
    distance: u32,
    /// Reflection blur radius in EMU
    blur_radius: u32,
    /// Reflection scale (0-100)
    scale: u32,
}

/// Bevel effect
#[derive(Clone, Debug, PartialEq)]
pub struct BevelEffect {
    /// Bevel type
    bevel_type: BevelType,
    /// Bevel width in EMU
    width: u32,
    /// Bevel height in EMU
    height: u32,
    /// Bevel depth in EMU
    depth: u32,
}

/// Bevel type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BevelType {
    /// Circle bevel
    Circle,
    /// Relaxed inset bevel
    RelaxedInset,
    /// Cross bevel
    Cross,
    /// Cool slant bevel
    CoolSlant,
    /// Angle bevel
    Angle,
    /// Soft round bevel
    SoftRound,
    /// Convex bevel
    Convex,
    /// Slope bevel
    Slope,
    /// Divot bevel
    Divot,
}

impl ShapeEffects {
    /// Create new shape effects
    pub fn new() -> Self {
        Self {
            glow: None,
            reflection: None,
            bevel: None,
        }
    }

    /// Set glow effect
    pub fn set_glow(&mut self, glow: GlowEffect) {
        self.glow = Some(glow);
    }

    /// Get glow effect
    pub fn glow(&self) -> Option<&GlowEffect> {
        self.glow.as_ref()
    }

    /// Clear glow effect
    pub fn clear_glow(&mut self) {
        self.glow = None;
    }

    /// Set reflection effect
    pub fn set_reflection(&mut self, reflection: ReflectionEffect) {
        self.reflection = Some(reflection);
    }

    /// Get reflection effect
    pub fn reflection(&self) -> Option<&ReflectionEffect> {
        self.reflection.as_ref()
    }

    /// Clear reflection effect
    pub fn clear_reflection(&mut self) {
        self.reflection = None;
    }

    /// Set bevel effect
    pub fn set_bevel(&mut self, bevel: BevelEffect) {
        self.bevel = Some(bevel);
    }

    /// Get bevel effect
    pub fn bevel(&self) -> Option<&BevelEffect> {
        self.bevel.as_ref()
    }

    /// Clear bevel effect
    pub fn clear_bevel(&mut self) {
        self.bevel = None;
    }

    /// Check if any effects are applied
    pub fn has_effects(&self) -> bool {
        self.glow.is_some() || self.reflection.is_some() || self.bevel.is_some()
    }

    /// Generate XML for shape effects
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();

        if !self.has_effects() {
            return xml;
        }

        xml.push_str(r#"<a:effectLst>"#);

        // Glow
        if let Some(glow) = &self.glow {
            xml.push_str(&format!(
                r#"<a:glow rad="{}"><a:srgbClr val="{}"><a:alpha val="{}"/></a:srgbClr></a:glow>"#,
                glow.size,
                glow.color.to_hex(),
                glow.opacity * 1000
            ));
        }

        // Reflection
        if let Some(reflection) = &self.reflection {
            xml.push_str(&format!(
                r#"<a:reflection blurRad="{}" dist="{}" dir="2700000" algn="b" rotWithShape="0"><a:alpha val="{}"/></a:reflection>"#,
                reflection.blur_radius,
                reflection.distance,
                reflection.opacity * 1000
            ));
        }

        // Bevel
        if let Some(bevel) = &self.bevel {
            let bevel_name = match bevel.bevel_type {
                BevelType::Circle => "circle",
                BevelType::RelaxedInset => "relaxedInset",
                BevelType::Cross => "cross",
                BevelType::CoolSlant => "coolSlant",
                BevelType::Angle => "angle",
                BevelType::SoftRound => "softRound",
                BevelType::Convex => "convex",
                BevelType::Slope => "slope",
                BevelType::Divot => "divot",
            };
            xml.push_str(&format!(
                r#"<a:cell3D w="{}" h="{}" prst="{}"/>"#,
                bevel.width, bevel.height, bevel_name
            ));
        }

        xml.push_str(r#"</a:effectLst>"#);
        xml
    }
}

impl Default for ShapeEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl GlowEffect {
    /// Create a new glow effect
    pub fn new(color: RGBColor, size: u32) -> Self {
        Self {
            color,
            size,
            opacity: 100,
        }
    }

    /// Set glow opacity (0-100)
    pub fn set_opacity(&mut self, opacity: u32) -> Result<()> {
        if opacity > 100 {
            return Err(crate::error::PptError::ValueError(
                "Opacity must be between 0 and 100".to_string(),
            ));
        }
        self.opacity = opacity;
        Ok(())
    }

    /// Get glow properties
    pub fn properties(&self) -> (&RGBColor, u32, u32) {
        (&self.color, self.size, self.opacity)
    }
}

impl ReflectionEffect {
    /// Create a new reflection effect
    pub fn new() -> Self {
        Self {
            opacity: 50,
            distance: 20000,
            blur_radius: 5000,
            scale: 100,
        }
    }

    /// Set reflection opacity (0-100)
    pub fn set_opacity(&mut self, opacity: u32) -> Result<()> {
        if opacity > 100 {
            return Err(crate::error::PptError::ValueError(
                "Opacity must be between 0 and 100".to_string(),
            ));
        }
        self.opacity = opacity;
        Ok(())
    }

    /// Set reflection distance
    pub fn set_distance(&mut self, distance: u32) {
        self.distance = distance;
    }

    /// Set reflection blur radius
    pub fn set_blur_radius(&mut self, blur_radius: u32) {
        self.blur_radius = blur_radius;
    }

    /// Set reflection scale (0-100)
    pub fn set_scale(&mut self, scale: u32) -> Result<()> {
        if scale > 100 {
            return Err(crate::error::PptError::ValueError(
                "Scale must be between 0 and 100".to_string(),
            ));
        }
        self.scale = scale;
        Ok(())
    }

    /// Get reflection properties
    pub fn properties(&self) -> (u32, u32, u32, u32) {
        (self.opacity, self.distance, self.blur_radius, self.scale)
    }
}

impl Default for ReflectionEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl BevelEffect {
    /// Create a new bevel effect
    pub fn new(bevel_type: BevelType, width: u32, height: u32, depth: u32) -> Self {
        Self {
            bevel_type,
            width,
            height,
            depth,
        }
    }

    /// Get bevel type
    pub fn bevel_type(&self) -> &BevelType {
        &self.bevel_type
    }

    /// Get bevel properties
    pub fn properties(&self) -> (&BevelType, u32, u32, u32) {
        (&self.bevel_type, self.width, self.height, self.depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_effects_creation() {
        let effects = ShapeEffects::new();
        assert!(!effects.has_effects());
        assert_eq!(effects.glow(), None);
        assert_eq!(effects.reflection(), None);
        assert_eq!(effects.bevel(), None);
    }

    #[test]
    fn test_glow_effect() {
        let mut effects = ShapeEffects::new();
        let glow = GlowEffect::new(RGBColor::new(255, 0, 0), 50000);
        effects.set_glow(glow);
        assert!(effects.has_effects());
        assert!(effects.glow().is_some());
    }

    #[test]
    fn test_reflection_effect() {
        let mut effects = ShapeEffects::new();
        let reflection = ReflectionEffect::new();
        effects.set_reflection(reflection);
        assert!(effects.has_effects());
        assert!(effects.reflection().is_some());
    }

    #[test]
    fn test_bevel_effect() {
        let mut effects = ShapeEffects::new();
        let bevel = BevelEffect::new(BevelType::Circle, 38100, 38100, 38100);
        effects.set_bevel(bevel);
        assert!(effects.has_effects());
        assert!(effects.bevel().is_some());
    }

    #[test]
    fn test_glow_opacity() {
        let mut glow = GlowEffect::new(RGBColor::new(255, 0, 0), 50000);
        assert!(glow.set_opacity(75).is_ok());
        assert!(glow.set_opacity(101).is_err());
    }

    #[test]
    fn test_reflection_properties() {
        let mut reflection = ReflectionEffect::new();
        reflection.set_opacity(60).unwrap();
        reflection.set_distance(30000);
        reflection.set_blur_radius(10000);
        reflection.set_scale(80).unwrap();
        
        let (opacity, distance, blur, scale) = reflection.properties();
        assert_eq!(opacity, 60);
        assert_eq!(distance, 30000);
        assert_eq!(blur, 10000);
        assert_eq!(scale, 80);
    }

    #[test]
    fn test_shape_effects_to_xml() {
        let mut effects = ShapeEffects::new();
        let glow = GlowEffect::new(RGBColor::new(255, 0, 0), 50000);
        effects.set_glow(glow);
        
        let xml = effects.to_xml();
        assert!(xml.contains("effectLst"));
        assert!(xml.contains("glow"));
    }

    #[test]
    fn test_all_bevel_types() {
        let types = vec![
            BevelType::Circle,
            BevelType::RelaxedInset,
            BevelType::Cross,
            BevelType::CoolSlant,
            BevelType::Angle,
            BevelType::SoftRound,
            BevelType::Convex,
            BevelType::Slope,
            BevelType::Divot,
        ];
        
        for bevel_type in types {
            let bevel = BevelEffect::new(bevel_type.clone(), 38100, 38100, 38100);
            assert_eq!(bevel.bevel_type(), &bevel_type);
        }
    }
}
