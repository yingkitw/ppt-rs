//! DrawingML fill functionality

use crate::dml::color::ColorFormat;
use crate::dml::gradient::GradientFill;
use crate::dml::pattern::PatternFill;
use crate::enums::dml::FillType;

/// Fill format - provides access to fill properties
pub struct FillFormat {
    fill_type: FillType,
    fore_color: Option<ColorFormat>,
    back_color: Option<ColorFormat>,
    gradient: Option<GradientFill>,
    pattern: Option<PatternFill>,
}

impl FillFormat {
    /// Create a new fill format
    pub fn new() -> Self {
        Self {
            fill_type: FillType::NoFill,
            fore_color: None,
            back_color: None,
            gradient: None,
            pattern: None,
        }
    }

    /// Create a solid fill
    pub fn solid(color: crate::dml::color::RGBColor) -> Self {
        Self {
            fill_type: FillType::Solid,
            fore_color: Some(ColorFormat::from_rgb(color)),
            back_color: None,
            gradient: None,
            pattern: None,
        }
    }

    /// Get the fill type
    pub fn fill_type(&self) -> FillType {
        self.fill_type
    }

    /// Set fill type to no fill (transparent)
    pub fn set_no_fill(&mut self) {
        self.fill_type = FillType::NoFill;
        self.fore_color = None;
        self.back_color = None;
    }

    /// Set fill type to solid
    pub fn set_solid(&mut self, color: crate::dml::color::RGBColor) {
        self.fill_type = FillType::Solid;
        self.fore_color = Some(ColorFormat::from_rgb(color));
    }

    /// Get foreground color
    pub fn fore_color(&self) -> Option<&ColorFormat> {
        self.fore_color.as_ref()
    }

    /// Get mutable foreground color
    pub fn fore_color_mut(&mut self) -> Option<&mut ColorFormat> {
        self.fore_color.as_mut()
    }

    /// Get background color (for pattern fills)
    pub fn back_color(&self) -> Option<&ColorFormat> {
        self.back_color.as_ref()
    }

    /// Get mutable background color
    pub fn back_color_mut(&mut self) -> Option<&mut ColorFormat> {
        self.back_color.as_mut()
    }

    /// Set fill type to gradient
    pub fn set_gradient(&mut self, gradient: GradientFill) {
        self.fill_type = FillType::Gradient;
        self.gradient = Some(gradient);
        self.fore_color = None;
        self.back_color = None;
    }

    /// Get gradient fill
    pub fn gradient(&self) -> Option<&GradientFill> {
        self.gradient.as_ref()
    }

    /// Get mutable gradient fill
    pub fn gradient_mut(&mut self) -> Option<&mut GradientFill> {
        self.gradient.as_mut()
    }

    /// Create a linear gradient fill
    pub fn set_gradient_linear(&mut self, start_color: crate::dml::color::RGBColor, end_color: crate::dml::color::RGBColor) -> crate::error::Result<()> {
        let gradient = GradientFill::linear_with_colors(start_color, end_color)?;
        self.set_gradient(gradient);
        Ok(())
    }

    /// Create a radial gradient fill
    pub fn set_gradient_radial(&mut self, start_color: crate::dml::color::RGBColor, end_color: crate::dml::color::RGBColor) -> crate::error::Result<()> {
        let mut gradient = GradientFill::radial();
        gradient.add_stop(0.0, ColorFormat::from_rgb(start_color))?;
        gradient.add_stop(1.0, ColorFormat::from_rgb(end_color))?;
        self.set_gradient(gradient);
        Ok(())
    }

    /// Set fill type to pattern
    pub fn set_pattern(&mut self, pattern: PatternFill) {
        self.fill_type = FillType::Pattern;
        self.pattern = Some(pattern);
        self.fore_color = None;
        self.back_color = None;
        self.gradient = None;
    }

    /// Get pattern fill
    pub fn pattern(&self) -> Option<&PatternFill> {
        self.pattern.as_ref()
    }

    /// Get mutable pattern fill
    pub fn pattern_mut(&mut self) -> Option<&mut PatternFill> {
        self.pattern.as_mut()
    }

    /// Create a pattern fill
    pub fn set_pattern_fill(&mut self, pattern_type: crate::dml::pattern::PatternType, fore_color: crate::dml::color::RGBColor, back_color: crate::dml::color::RGBColor) {
        let pattern = PatternFill::with_rgb(pattern_type, fore_color, back_color);
        self.set_pattern(pattern);
    }
}

