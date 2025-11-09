//! DrawingML fill functionality

use crate::dml::color::ColorFormat;
use crate::enums::dml::FillType;
use crate::dml::gradient::Gradient;
use crate::dml::pattern::Pattern;
use crate::error::Result;

/// Fill format - provides access to fill properties
pub struct FillFormat {
    fill_type: FillType,
    fore_color: Option<ColorFormat>,
    back_color: Option<ColorFormat>,
    gradient: Option<Gradient>,
    pattern: Option<Pattern>,
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

    /// Set gradient fill (linear)
    pub fn set_gradient_linear(&mut self, start_color: crate::dml::color::RGBColor, end_color: crate::dml::color::RGBColor) -> Result<()> {
        let gradient = Gradient::linear_with_colors(start_color, end_color)?;
        self.fill_type = FillType::Gradient;
        self.gradient = Some(gradient);
        self.pattern = None;
        Ok(())
    }

    /// Set gradient fill (radial)
    pub fn set_gradient_radial(&mut self, start_color: crate::dml::color::RGBColor, end_color: crate::dml::color::RGBColor) -> Result<()> {
        let gradient = Gradient::radial_with_colors(start_color, end_color)?;
        self.fill_type = FillType::Gradient;
        self.gradient = Some(gradient);
        self.pattern = None;
        Ok(())
    }

    /// Get gradient
    pub fn gradient(&self) -> Option<&Gradient> {
        self.gradient.as_ref()
    }

    /// Get mutable gradient
    pub fn gradient_mut(&mut self) -> Option<&mut Gradient> {
        self.gradient.as_mut()
    }

    /// Set pattern fill
    pub fn set_pattern_fill(&mut self, pattern: Pattern) {
        self.fill_type = FillType::Pattern;
        self.pattern = Some(pattern);
        self.gradient = None;
    }

    /// Get pattern
    pub fn pattern(&self) -> Option<&Pattern> {
        self.pattern.as_ref()
    }

    /// Get mutable pattern
    pub fn pattern_mut(&mut self) -> Option<&mut Pattern> {
        self.pattern.as_mut()
    }
}

