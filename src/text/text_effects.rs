//! Text effects for advanced text formatting
//!
//! Provides comprehensive text effects including:
//! - Text rotation (0-360 degrees)
//! - Text wrapping options
//! - Text shadow
//! - Text outline

use crate::error::Result;
use crate::dml::color::RGBColor;

/// Text effects for advanced formatting
#[derive(Clone, Debug)]
pub struct TextEffects {
    /// Text rotation in degrees (0-360)
    rotation: Option<f64>,
    /// Text wrapping mode
    wrapping: TextWrapping,
    /// Text shadow
    shadow: Option<TextShadow>,
    /// Text outline
    outline: Option<TextOutline>,
    /// Vertical alignment
    vertical_align: VerticalAlignment,
}

/// Text wrapping mode
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextWrapping {
    /// No wrapping
    None,
    /// Wrap text
    Wrap,
    /// Wrap and shrink text
    WrapAndShrink,
    /// Overflow (no wrapping)
    Overflow,
}

/// Text shadow
#[derive(Clone, Debug, PartialEq)]
pub struct TextShadow {
    /// Shadow color
    color: RGBColor,
    /// Shadow offset X in EMU
    offset_x: i32,
    /// Shadow offset Y in EMU
    offset_y: i32,
    /// Shadow blur radius in EMU
    blur_radius: u32,
    /// Shadow opacity (0-100)
    opacity: u32,
}

/// Text outline
#[derive(Clone, Debug, PartialEq)]
pub struct TextOutline {
    /// Outline color
    color: RGBColor,
    /// Outline width in EMU
    width: u32,
    /// Outline dash style
    dash_style: OutlineDashStyle,
}

/// Outline dash style
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutlineDashStyle {
    /// Solid line
    Solid,
    /// Dashed line
    Dash,
    /// Dotted line
    Dot,
    /// Dash-dot line
    DashDot,
}

/// Vertical alignment
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VerticalAlignment {
    /// Top alignment
    Top,
    /// Middle alignment
    Middle,
    /// Bottom alignment
    Bottom,
}

impl TextEffects {
    /// Create new text effects with defaults
    pub fn new() -> Self {
        Self {
            rotation: None,
            wrapping: TextWrapping::Wrap,
            shadow: None,
            outline: None,
            vertical_align: VerticalAlignment::Top,
        }
    }

    /// Set text rotation in degrees (0-360)
    pub fn set_rotation(&mut self, degrees: f64) -> Result<()> {
        if degrees < 0.0 || degrees > 360.0 {
            return Err(crate::error::PptError::ValueError(
                "Rotation must be between 0 and 360 degrees".to_string(),
            ));
        }
        self.rotation = Some(degrees);
        Ok(())
    }

    /// Get text rotation
    pub fn rotation(&self) -> Option<f64> {
        self.rotation
    }

    /// Clear text rotation
    pub fn clear_rotation(&mut self) {
        self.rotation = None;
    }

    /// Set text wrapping mode
    pub fn set_wrapping(&mut self, wrapping: TextWrapping) {
        self.wrapping = wrapping;
    }

    /// Get text wrapping mode
    pub fn wrapping(&self) -> &TextWrapping {
        &self.wrapping
    }

    /// Set text shadow
    pub fn set_shadow(&mut self, shadow: TextShadow) {
        self.shadow = Some(shadow);
    }

    /// Get text shadow
    pub fn shadow(&self) -> Option<&TextShadow> {
        self.shadow.as_ref()
    }

    /// Clear text shadow
    pub fn clear_shadow(&mut self) {
        self.shadow = None;
    }

    /// Set text outline
    pub fn set_outline(&mut self, outline: TextOutline) {
        self.outline = Some(outline);
    }

    /// Get text outline
    pub fn outline(&self) -> Option<&TextOutline> {
        self.outline.as_ref()
    }

    /// Clear text outline
    pub fn clear_outline(&mut self) {
        self.outline = None;
    }

    /// Set vertical alignment
    pub fn set_vertical_align(&mut self, align: VerticalAlignment) {
        self.vertical_align = align;
    }

    /// Get vertical alignment
    pub fn vertical_align(&self) -> &VerticalAlignment {
        &self.vertical_align
    }

    /// Generate XML for text effects
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();

        // Rotation
        if let Some(rotation) = self.rotation {
            xml.push_str(&format!(r#"<a:rot val="{}"/>"#, (rotation * 60000.0) as i32));
        }

        // Wrapping
        match self.wrapping {
            TextWrapping::None => xml.push_str(r#"<a:wrap>none</a:wrap>"#),
            TextWrapping::Wrap => xml.push_str(r#"<a:wrap>wrap</a:wrap>"#),
            TextWrapping::WrapAndShrink => xml.push_str(r#"<a:wrap>shrinkToFit</a:wrap>"#),
            TextWrapping::Overflow => xml.push_str(r#"<a:wrap>overflow</a:wrap>"#),
        }

        // Shadow
        if let Some(shadow) = &self.shadow {
            xml.push_str(&format!(
                r#"<a:effectLst><a:outerShdw blurRad="{}" dist="{}" dir="{}" algn="tl" rotWithShape="0"><a:srgbClr val="{}"/></a:outerShdw></a:effectLst>"#,
                shadow.blur_radius,
                shadow.offset_x.abs(),
                (shadow.offset_y.abs() as f64).atan2(shadow.offset_x.abs() as f64).to_degrees() as i32,
                shadow.color.to_hex()
            ));
        }

        // Outline
        if let Some(outline) = &self.outline {
            let dash_style = match outline.dash_style {
                OutlineDashStyle::Solid => "solid",
                OutlineDashStyle::Dash => "dash",
                OutlineDashStyle::Dot => "dot",
                OutlineDashStyle::DashDot => "dashDot",
            };
            xml.push_str(&format!(
                r#"<a:ln w="{}"><a:solidFill><a:srgbClr val="{}"/></a:solidFill><a:prstDash val="{}"/></a:ln>"#,
                outline.width,
                outline.color.to_hex(),
                dash_style
            ));
        }

        xml
    }
}

impl Default for TextEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl TextShadow {
    /// Create a new text shadow
    pub fn new(color: RGBColor, offset_x: i32, offset_y: i32, blur_radius: u32) -> Self {
        Self {
            color,
            offset_x,
            offset_y,
            blur_radius,
            opacity: 100,
        }
    }

    /// Set shadow opacity (0-100)
    pub fn set_opacity(&mut self, opacity: u32) -> Result<()> {
        if opacity > 100 {
            return Err(crate::error::PptError::ValueError(
                "Opacity must be between 0 and 100".to_string(),
            ));
        }
        self.opacity = opacity;
        Ok(())
    }

    /// Get shadow properties
    pub fn properties(&self) -> (&RGBColor, i32, i32, u32, u32) {
        (&self.color, self.offset_x, self.offset_y, self.blur_radius, self.opacity)
    }
}

impl TextOutline {
    /// Create a new text outline
    pub fn new(color: RGBColor, width: u32) -> Self {
        Self {
            color,
            width,
            dash_style: OutlineDashStyle::Solid,
        }
    }

    /// Set dash style
    pub fn set_dash_style(&mut self, dash_style: OutlineDashStyle) {
        self.dash_style = dash_style;
    }

    /// Get outline properties
    pub fn properties(&self) -> (&RGBColor, u32, &OutlineDashStyle) {
        (&self.color, self.width, &self.dash_style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_effects_creation() {
        let effects = TextEffects::new();
        assert_eq!(effects.wrapping(), &TextWrapping::Wrap);
        assert_eq!(effects.rotation(), None);
        assert_eq!(effects.shadow(), None);
        assert_eq!(effects.outline(), None);
    }

    #[test]
    fn test_text_rotation() {
        let mut effects = TextEffects::new();
        assert!(effects.set_rotation(45.0).is_ok());
        assert_eq!(effects.rotation(), Some(45.0));
        
        assert!(effects.set_rotation(361.0).is_err());
        assert!(effects.set_rotation(-1.0).is_err());
    }

    #[test]
    fn test_text_wrapping() {
        let mut effects = TextEffects::new();
        effects.set_wrapping(TextWrapping::None);
        assert_eq!(effects.wrapping(), &TextWrapping::None);
        
        effects.set_wrapping(TextWrapping::Overflow);
        assert_eq!(effects.wrapping(), &TextWrapping::Overflow);
    }

    #[test]
    fn test_text_shadow() {
        let mut effects = TextEffects::new();
        let shadow = TextShadow::new(RGBColor::new(0, 0, 0), 10000, 10000, 5000);
        effects.set_shadow(shadow);
        assert!(effects.shadow().is_some());
    }

    #[test]
    fn test_text_outline() {
        let mut effects = TextEffects::new();
        let mut outline = TextOutline::new(RGBColor::new(0, 0, 0), 25400);
        outline.set_dash_style(OutlineDashStyle::Dash);
        effects.set_outline(outline);
        assert!(effects.outline().is_some());
    }

    #[test]
    fn test_vertical_alignment() {
        let mut effects = TextEffects::new();
        effects.set_vertical_align(VerticalAlignment::Middle);
        assert_eq!(effects.vertical_align(), &VerticalAlignment::Middle);
    }

    #[test]
    fn test_text_effects_to_xml() {
        let mut effects = TextEffects::new();
        effects.set_rotation(45.0).unwrap();
        let xml = effects.to_xml();
        assert!(xml.contains("rot"));
        assert!(xml.contains("wrap"));
    }

    #[test]
    fn test_shadow_opacity() {
        let mut shadow = TextShadow::new(RGBColor::new(0, 0, 0), 10000, 10000, 5000);
        assert!(shadow.set_opacity(50).is_ok());
        assert!(shadow.set_opacity(101).is_err());
    }
}
