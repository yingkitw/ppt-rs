//! Shadow Effects - Shadow support for shapes

use crate::dml::color::RGBColor;

/// Shadow Type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShadowType {
    /// Outer shadow
    Outer,
    /// Inner shadow
    Inner,
}

impl ShadowType {
    /// Get shadow type string
    pub fn type_str(&self) -> &str {
        match self {
            ShadowType::Outer => "outerShdw",
            ShadowType::Inner => "innerShdw",
        }
    }
}

/// Shadow Effect
#[derive(Clone, Debug)]
pub struct Shadow {
    /// Shadow type
    shadow_type: ShadowType,
    /// Blur radius in EMUs (default: 38100)
    blur_radius: i32,
    /// Distance in EMUs (default: 38100)
    distance: i32,
    /// Direction in degrees (0-360, default: 2700000 = 45 degrees in 60000ths)
    direction: i32,
    /// Alignment (default: "tl" for top-left)
    alignment: String,
    /// Rotate with shape
    rotate_with_shape: bool,
    /// Shadow color
    color: RGBColor,
    /// Shadow opacity (0.0-1.0)
    opacity: f64,
}

impl Shadow {
    /// Create a new outer shadow
    pub fn outer() -> Self {
        Self {
            shadow_type: ShadowType::Outer,
            blur_radius: 38100,
            distance: 38100,
            direction: 2700000,
            alignment: "tl".to_string(),
            rotate_with_shape: true,
            color: RGBColor::new(0, 0, 0),
            opacity: 0.5,
        }
    }

    /// Create a new inner shadow
    pub fn inner() -> Self {
        Self {
            shadow_type: ShadowType::Inner,
            blur_radius: 38100,
            distance: 38100,
            direction: 2700000,
            alignment: "tl".to_string(),
            rotate_with_shape: true,
            color: RGBColor::new(0, 0, 0),
            opacity: 0.5,
        }
    }

    /// Set blur radius
    pub fn set_blur_radius(&mut self, blur_radius: i32) {
        self.blur_radius = blur_radius;
    }

    /// Get blur radius
    pub fn blur_radius(&self) -> i32 {
        self.blur_radius
    }

    /// Set distance
    pub fn set_distance(&mut self, distance: i32) {
        self.distance = distance;
    }

    /// Get distance
    pub fn distance(&self) -> i32 {
        self.distance
    }

    /// Set direction
    pub fn set_direction(&mut self, direction: i32) {
        self.direction = direction;
    }

    /// Get direction
    pub fn direction(&self) -> i32 {
        self.direction
    }

    /// Set color
    pub fn set_color(&mut self, color: RGBColor) {
        self.color = color;
    }

    /// Get color
    pub fn color(&self) -> &RGBColor {
        &self.color
    }

    /// Set opacity
    pub fn set_opacity(&mut self, opacity: f64) {
        self.opacity = opacity.max(0.0).min(1.0);
    }

    /// Get opacity
    pub fn opacity(&self) -> f64 {
        self.opacity
    }

    /// Generate XML for shadow effect
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(&format!(
            r#"<a:{} blurRad="{}" dist="{}" dir="{}" algn="{}" rotWithShape="{}">"#,
            self.shadow_type.type_str(),
            self.blur_radius,
            self.distance,
            self.direction,
            self.alignment,
            if self.rotate_with_shape { "1" } else { "0" }
        ));
        xml.push('\n');
        
        // Add color with opacity
        let alpha = (self.opacity * 100000.0) as i32;
        xml.push_str(&format!(
            r#"  <a:srgbClr val="{:06X}"><a:alpha val="{}"/></a:srgbClr>"#,
            (self.color.r as u32) << 16 | (self.color.g as u32) << 8 | (self.color.b as u32),
            alpha
        ));
        xml.push('\n');
        
        xml.push_str(&format!(r#"</a:{}>"#, self.shadow_type.type_str()));
        xml
    }
}

/// Shadow Manager
#[derive(Clone, Debug)]
pub struct ShadowManager {
    /// Shadow effects
    shadows: Vec<Shadow>,
}

impl ShadowManager {
    /// Create a new shadow manager
    pub fn new() -> Self {
        Self {
            shadows: vec![],
        }
    }

    /// Add a shadow
    pub fn add_shadow(&mut self, shadow: Shadow) -> usize {
        self.shadows.push(shadow);
        self.shadows.len() - 1
    }

    /// Add outer shadow
    pub fn add_outer_shadow(&mut self) -> usize {
        self.add_shadow(Shadow::outer())
    }

    /// Add inner shadow
    pub fn add_inner_shadow(&mut self) -> usize {
        self.add_shadow(Shadow::inner())
    }

    /// Get shadow by index
    pub fn get(&self, index: usize) -> Option<&Shadow> {
        self.shadows.get(index)
    }

    /// Get mutable shadow by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Shadow> {
        self.shadows.get_mut(index)
    }

    /// Get all shadows
    pub fn all(&self) -> &[Shadow] {
        &self.shadows
    }

    /// Get number of shadows
    pub fn len(&self) -> usize {
        self.shadows.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.shadows.is_empty()
    }

    /// Clear all shadows
    pub fn clear(&mut self) {
        self.shadows.clear();
    }

    /// Generate XML for all shadows
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        for shadow in &self.shadows {
            xml.push_str(&shadow.to_xml());
            xml.push('\n');
        }
        xml
    }
}

impl Default for ShadowManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow_outer() {
        let shadow = Shadow::outer();
        assert_eq!(shadow.shadow_type, ShadowType::Outer);
        assert_eq!(shadow.blur_radius(), 38100);
        assert_eq!(shadow.distance(), 38100);
    }

    #[test]
    fn test_shadow_inner() {
        let shadow = Shadow::inner();
        assert_eq!(shadow.shadow_type, ShadowType::Inner);
    }

    #[test]
    fn test_shadow_properties() {
        let mut shadow = Shadow::outer();
        shadow.set_blur_radius(50000);
        shadow.set_distance(60000);
        shadow.set_direction(3600000);
        shadow.set_opacity(0.8);
        
        assert_eq!(shadow.blur_radius(), 50000);
        assert_eq!(shadow.distance(), 60000);
        assert_eq!(shadow.direction(), 3600000);
        assert_eq!(shadow.opacity(), 0.8);
    }

    #[test]
    fn test_shadow_color() {
        let mut shadow = Shadow::outer();
        let color = RGBColor::new(255, 0, 0);
        shadow.set_color(color);
        
        assert_eq!(shadow.color().r, 255);
        assert_eq!(shadow.color().g, 0);
        assert_eq!(shadow.color().b, 0);
    }

    #[test]
    fn test_shadow_opacity_clamping() {
        let mut shadow = Shadow::outer();
        shadow.set_opacity(1.5);
        assert_eq!(shadow.opacity(), 1.0);
        
        shadow.set_opacity(-0.5);
        assert_eq!(shadow.opacity(), 0.0);
    }

    #[test]
    fn test_shadow_to_xml() {
        let shadow = Shadow::outer();
        let xml = shadow.to_xml();
        
        assert!(xml.contains(r#"<a:outerShdw"#));
        assert!(xml.contains(r#"blurRad="38100""#));
        assert!(xml.contains(r#"dist="38100""#));
        assert!(xml.contains(r#"<a:srgbClr"#));
        assert!(xml.contains(r#"</a:outerShdw>"#));
    }

    #[test]
    fn test_shadow_manager_creation() {
        let manager = ShadowManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_shadow_manager_add() {
        let mut manager = ShadowManager::new();
        manager.add_outer_shadow();
        manager.add_inner_shadow();
        
        assert_eq!(manager.len(), 2);
    }

    #[test]
    fn test_shadow_manager_get() {
        let mut manager = ShadowManager::new();
        manager.add_outer_shadow();
        
        let shadow = manager.get(0);
        assert!(shadow.is_some());
        assert_eq!(shadow.unwrap().shadow_type, ShadowType::Outer);
    }

    #[test]
    fn test_shadow_manager_get_mut() {
        let mut manager = ShadowManager::new();
        manager.add_outer_shadow();
        
        if let Some(shadow) = manager.get_mut(0) {
            shadow.set_blur_radius(50000);
        }
        
        assert_eq!(manager.get(0).unwrap().blur_radius(), 50000);
    }

    #[test]
    fn test_shadow_manager_clear() {
        let mut manager = ShadowManager::new();
        manager.add_outer_shadow();
        manager.add_inner_shadow();
        assert_eq!(manager.len(), 2);
        
        manager.clear();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_shadow_manager_default() {
        let manager = ShadowManager::default();
        assert!(manager.is_empty());
    }
}
