//! Slide background formatting

use crate::dml::color::ColorFormat;
use crate::dml::fill::FillFormat;

/// Slide background - provides access to background properties
pub struct SlideBackground {
    fill: FillFormat,
}

impl SlideBackground {
    /// Create a new slide background with no fill
    pub fn new() -> Self {
        Self {
            fill: FillFormat::new(),
        }
    }

    /// Create a slide background with solid color
    pub fn solid(color: crate::dml::color::RGBColor) -> Self {
        Self {
            fill: FillFormat::solid(color),
        }
    }

    /// Get the fill format
    pub fn fill(&self) -> &FillFormat {
        &self.fill
    }

    /// Get mutable fill format
    pub fn fill_mut(&mut self) -> &mut FillFormat {
        &mut self.fill
    }

    /// Set solid color background
    pub fn set_solid(&mut self, color: crate::dml::color::RGBColor) {
        self.fill.set_solid(color);
    }

    /// Set gradient background
    pub fn set_gradient_linear(&mut self, start_color: crate::dml::color::RGBColor, end_color: crate::dml::color::RGBColor) -> crate::error::Result<()> {
        self.fill.set_gradient_linear(start_color, end_color)
    }

    /// Set radial gradient background
    pub fn set_gradient_radial(&mut self, start_color: crate::dml::color::RGBColor, end_color: crate::dml::color::RGBColor) -> crate::error::Result<()> {
        self.fill.set_gradient_radial(start_color, end_color)
    }

    /// Set pattern background
    pub fn set_pattern(&mut self, pattern_type: crate::dml::pattern::PatternType, fore_color: crate::dml::color::RGBColor, back_color: crate::dml::color::RGBColor) {
        self.fill.set_pattern_fill(pattern_type, fore_color, back_color);
    }

    /// Clear background (set to no fill)
    pub fn clear(&mut self) {
        self.fill.set_no_fill();
    }

    /// Generate background XML for slide
    pub fn to_xml(&self) -> String {
        match self.fill.fill_type() {
            crate::enums::dml::FillType::NoFill => {
                // No background element needed
                String::new()
            }
            crate::enums::dml::FillType::Solid => {
                if let Some(color) = self.fill.fore_color() {
                    if let Some(rgb) = color.rgb() {
                        format!(
                            r#"<p:bg>
    <p:bgPr>
      <a:solidFill>
        <a:srgbClr val="{}"/>
      </a:solidFill>
      <a:effectLst/>
    </p:bgPr>
  </p:bg>"#,
                            rgb.to_hex()
                        )
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            }
            crate::enums::dml::FillType::Gradient => {
                // Gradient background XML
                if let Some(gradient) = self.fill.gradient() {
                    let mut stops_xml = String::new();
                    for (idx, stop) in gradient.stops().iter().enumerate() {
                        let position = (stop.position() * 100000.0) as u32;
                        if let Some(rgb) = stop.color().rgb() {
                            stops_xml.push_str(&format!(
                                r#"        <a:gsLst>
          <a:gs pos="{}">
            <a:srgbClr val="{}"/>
          </a:gs>
        </a:gsLst>"#,
                                position,
                                rgb.to_hex()
                            ));
                        }
                    }
                    
                    format!(
                        r#"<p:bg>
    <p:bgPr>
      <a:gradFill>
        {}
        <a:lin ang="{}" scaled="1"/>
      </a:gradFill>
      <a:effectLst/>
    </p:bgPr>
  </p:bg>"#,
                        stops_xml,
                        ((gradient.angle() * 60000.0) as u32)
                    )
                } else {
                    String::new()
                }
            }
            crate::enums::dml::FillType::Pattern => {
                // Pattern background XML
                if let Some(pattern) = self.fill.pattern() {
                    let pattern_name = match pattern.pattern_type() {
                        crate::dml::pattern::PatternType::Horizontal => "dnDnDnDn",
                        crate::dml::pattern::PatternType::Vertical => "upUpUpUp",
                        crate::dml::pattern::PatternType::DiagonalDown => "dnDnDnDn",
                        crate::dml::pattern::PatternType::DiagonalUp => "upUpUpUp",
                        crate::dml::pattern::PatternType::Cross => "cross",
                        crate::dml::pattern::PatternType::DiagonalCross => "dnDnDnDn",
                        crate::dml::pattern::PatternType::Checker => "checker",
                        crate::dml::pattern::PatternType::Dots => "dot",
                        _ => "dnDnDnDn",
                    };
                    
                    let fore_hex = pattern.fore_color().rgb().map(|c| c.to_hex()).unwrap_or_default();
                    let back_hex = pattern.back_color().rgb().map(|c| c.to_hex()).unwrap_or_default();
                    
                    format!(
                        r#"<p:bg>
    <p:bgPr>
      <a:pattFill prst="{}">
        <a:fgClr>
          <a:srgbClr val="{}"/>
        </a:fgClr>
        <a:bgClr>
          <a:srgbClr val="{}"/>
        </a:bgClr>
      </a:pattFill>
      <a:effectLst/>
    </p:bgPr>
  </p:bg>"#,
                        pattern_name, fore_hex, back_hex
                    )
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dml::color::RGBColor;

    #[test]
    fn test_background_new() {
        let bg = SlideBackground::new();
        assert_eq!(bg.fill().fill_type(), crate::enums::dml::FillType::NoFill);
    }

    #[test]
    fn test_background_solid() {
        let bg = SlideBackground::solid(RGBColor::new(255, 0, 0));
        assert_eq!(bg.fill().fill_type(), crate::enums::dml::FillType::Solid);
    }

    #[test]
    fn test_background_set_solid() {
        let mut bg = SlideBackground::new();
        bg.set_solid(RGBColor::new(0, 255, 0));
        assert_eq!(bg.fill().fill_type(), crate::enums::dml::FillType::Solid);
    }

    #[test]
    fn test_background_gradient_linear() {
        let mut bg = SlideBackground::new();
        bg.set_gradient_linear(
            RGBColor::new(255, 0, 0),
            RGBColor::new(0, 0, 255),
        ).unwrap();
        assert_eq!(bg.fill().fill_type(), crate::enums::dml::FillType::Gradient);
    }

    #[test]
    fn test_background_gradient_radial() {
        let mut bg = SlideBackground::new();
        bg.set_gradient_radial(
            RGBColor::new(255, 0, 0),
            RGBColor::new(0, 0, 255),
        ).unwrap();
        assert_eq!(bg.fill().fill_type(), crate::enums::dml::FillType::Gradient);
    }

    #[test]
    fn test_background_pattern() {
        let mut bg = SlideBackground::new();
        bg.set_pattern(
            crate::dml::pattern::PatternType::Horizontal,
            RGBColor::new(255, 0, 0),
            RGBColor::new(0, 0, 255),
        );
        assert_eq!(bg.fill().fill_type(), crate::enums::dml::FillType::Pattern);
    }

    #[test]
    fn test_background_clear() {
        let mut bg = SlideBackground::solid(RGBColor::new(255, 0, 0));
        assert_eq!(bg.fill().fill_type(), crate::enums::dml::FillType::Solid);
        
        bg.clear();
        assert_eq!(bg.fill().fill_type(), crate::enums::dml::FillType::NoFill);
    }

    #[test]
    fn test_background_xml_solid() {
        let bg = SlideBackground::solid(RGBColor::new(255, 0, 0));
        let xml = bg.to_xml();
        assert!(xml.contains("<p:bg>"));
        assert!(xml.contains("FF0000"));
    }

    #[test]
    fn test_background_xml_no_fill() {
        let bg = SlideBackground::new();
        let xml = bg.to_xml();
        assert_eq!(xml, "");
    }
}
