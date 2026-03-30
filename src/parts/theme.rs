//! Theme part
//!
//! Represents a theme (ppt/theme/themeN.xml).

use super::base::{ContentType, Part, PartType};
use crate::exc::PptxError;

/// Theme color
#[derive(Debug, Clone)]
pub struct ThemeColor {
    pub name: String,
    pub value: String, // RGB hex value
}

impl ThemeColor {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        ThemeColor {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// Theme font
#[derive(Debug, Clone)]
pub struct ThemeFont {
    pub typeface: String,
    pub panose: Option<String>,
}

impl ThemeFont {
    pub fn new(typeface: impl Into<String>) -> Self {
        ThemeFont {
            typeface: typeface.into(),
            panose: None,
        }
    }
}

/// Theme part (ppt/theme/themeN.xml)
#[derive(Debug, Clone)]
pub struct ThemePart {
    path: String,
    theme_number: usize,
    name: String,
    major_font: ThemeFont,
    minor_font: ThemeFont,
    colors: Vec<ThemeColor>,
    xml_content: Option<String>,
}

impl ThemePart {
    /// Create a new theme part with default Office theme
    pub fn new(theme_number: usize) -> Self {
        ThemePart {
            path: format!("ppt/theme/theme{}.xml", theme_number),
            theme_number,
            name: "Office Theme".to_string(),
            major_font: ThemeFont::new("Calibri Light"),
            minor_font: ThemeFont::new("Calibri"),
            colors: Self::default_colors(),
            xml_content: None,
        }
    }

    fn default_colors() -> Vec<ThemeColor> {
        vec![
            ThemeColor::new("dk1", "000000"),
            ThemeColor::new("lt1", "FFFFFF"),
            ThemeColor::new("dk2", "44546A"),
            ThemeColor::new("lt2", "E7E6E6"),
            ThemeColor::new("accent1", "4472C4"),
            ThemeColor::new("accent2", "ED7D31"),
            ThemeColor::new("accent3", "A5A5A5"),
            ThemeColor::new("accent4", "FFC000"),
            ThemeColor::new("accent5", "5B9BD5"),
            ThemeColor::new("accent6", "70AD47"),
            ThemeColor::new("hlink", "0563C1"),
            ThemeColor::new("folHlink", "954F72"),
        ]
    }

    /// Get theme number
    pub fn theme_number(&self) -> usize {
        self.theme_number
    }

    /// Get theme name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set theme name
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Set major font (headings)
    pub fn set_major_font(&mut self, typeface: impl Into<String>) {
        self.major_font = ThemeFont::new(typeface);
    }

    /// Set minor font (body)
    pub fn set_minor_font(&mut self, typeface: impl Into<String>) {
        self.minor_font = ThemeFont::new(typeface);
    }

    /// Set a theme color
    pub fn set_color(&mut self, name: impl Into<String>, value: impl Into<String>) {
        let name = name.into();
        if let Some(color) = self.colors.iter_mut().find(|c| c.name == name) {
            color.value = value.into();
        } else {
            self.colors.push(ThemeColor::new(name, value));
        }
    }

    /// Get relative path for relationships
    pub fn rel_target(&self) -> String {
        format!("../theme/theme{}.xml", self.theme_number)
    }

    fn generate_xml(&self) -> String {
        let colors_xml: String = self
            .colors
            .iter()
            .map(|c| {
                format!(
                    r#"<a:{} val="{}"><a:srgbClr val="{}"/></a:{}>"#,
                    c.name, c.name, c.value, c.name
                )
            })
            .collect::<Vec<_>>()
            .join("\n        ");

        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="{}">
  <a:themeElements>
    <a:clrScheme name="Office">
      {}
    </a:clrScheme>
    <a:fontScheme name="Office">
      <a:majorFont>
        <a:latin typeface="{}"/>
        <a:ea typeface=""/>
        <a:cs typeface=""/>
      </a:majorFont>
      <a:minorFont>
        <a:latin typeface="{}"/>
        <a:ea typeface=""/>
        <a:cs typeface=""/>
      </a:minorFont>
    </a:fontScheme>
    <a:fmtScheme name="Office">
      <a:fillStyleLst>
        <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
        <a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:tint val="50000"/><a:satMod val="300000"/></a:schemeClr></a:gs><a:gs pos="35000"><a:schemeClr val="phClr"><a:tint val="37000"/><a:satMod val="300000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:tint val="15000"/><a:satMod val="350000"/></a:schemeClr></a:gs></a:gsLst><a:lin ang="16200000" scaled="1"/></a:gradFill>
        <a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:shade val="51000"/><a:satMod val="130000"/></a:schemeClr></a:gs><a:gs pos="80000"><a:schemeClr val="phClr"><a:shade val="93000"/><a:satMod val="130000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:shade val="94000"/><a:satMod val="135000"/></a:schemeClr></a:gs></a:gsLst><a:lin ang="16200000" scaled="0"/></a:gradFill>
      </a:fillStyleLst>
      <a:lnStyleLst>
        <a:ln w="6350" cap="flat" cmpd="sng" algn="ctr"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:prstDash val="solid"/><a:miter lim="800000"/></a:ln>
        <a:ln w="12700" cap="flat" cmpd="sng" algn="ctr"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:prstDash val="solid"/><a:miter lim="800000"/></a:ln>
        <a:ln w="19050" cap="flat" cmpd="sng" algn="ctr"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:prstDash val="solid"/><a:miter lim="800000"/></a:ln>
      </a:lnStyleLst>
      <a:effectStyleLst>
        <a:effectStyle><a:effectLst/></a:effectStyle>
        <a:effectStyle><a:effectLst/></a:effectStyle>
        <a:effectStyle><a:effectLst><a:outerShdw blurRad="57150" dist="19050" dir="5400000" algn="ctr" rotWithShape="0"><a:srgbClr val="000000"><a:alpha val="63000"/></a:srgbClr></a:outerShdw></a:effectLst></a:effectStyle>
      </a:effectStyleLst>
      <a:bgFillStyleLst>
        <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
        <a:solidFill><a:schemeClr val="phClr"><a:tint val="95000"/><a:satMod val="170000"/></a:schemeClr></a:solidFill>
        <a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:tint val="93000"/><a:satMod val="150000"/><a:shade val="98000"/><a:lumMod val="102000"/></a:schemeClr></a:gs><a:gs pos="50000"><a:schemeClr val="phClr"><a:tint val="98000"/><a:satMod val="130000"/><a:shade val="90000"/><a:lumMod val="103000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:shade val="63000"/><a:satMod val="120000"/></a:schemeClr></a:gs></a:gsLst><a:lin ang="5400000" scaled="0"/></a:gradFill>
      </a:bgFillStyleLst>
    </a:fmtScheme>
  </a:themeElements>
  <a:objectDefaults/>
  <a:extraClrSchemeLst/>
</a:theme>"#,
            self.name, colors_xml, self.major_font.typeface, self.minor_font.typeface
        )
    }
}

impl Part for ThemePart {
    fn path(&self) -> &str {
        &self.path
    }

    fn part_type(&self) -> PartType {
        PartType::Theme
    }

    fn content_type(&self) -> ContentType {
        ContentType::Theme
    }

    fn to_xml(&self) -> Result<String, PptxError> {
        if let Some(ref xml) = self.xml_content {
            return Ok(xml.clone());
        }
        Ok(self.generate_xml())
    }

    fn from_xml(xml: &str) -> Result<Self, PptxError> {
        Ok(ThemePart {
            path: "ppt/theme/theme1.xml".to_string(),
            theme_number: 1,
            name: "Office Theme".to_string(),
            major_font: ThemeFont::new("Calibri Light"),
            minor_font: ThemeFont::new("Calibri"),
            colors: Self::default_colors(),
            xml_content: Some(xml.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_new() {
        let theme = ThemePart::new(1);
        assert_eq!(theme.theme_number(), 1);
        assert_eq!(theme.path(), "ppt/theme/theme1.xml");
        assert_eq!(theme.name(), "Office Theme");
    }

    #[test]
    fn test_theme_set_fonts() {
        let mut theme = ThemePart::new(1);
        theme.set_major_font("Arial");
        theme.set_minor_font("Times New Roman");
        let xml = theme.to_xml().unwrap();
        assert!(xml.contains("Arial"));
        assert!(xml.contains("Times New Roman"));
    }

    #[test]
    fn test_theme_set_color() {
        let mut theme = ThemePart::new(1);
        theme.set_color("accent1", "FF0000");
        let xml = theme.to_xml().unwrap();
        assert!(xml.contains("FF0000"));
    }

    #[test]
    fn test_theme_to_xml() {
        let theme = ThemePart::new(1);
        let xml = theme.to_xml().unwrap();
        assert!(xml.contains("a:theme"));
        assert!(xml.contains("a:clrScheme"));
        assert!(xml.contains("a:fontScheme"));
    }

    #[test]
    fn test_theme_rel_target() {
        let theme = ThemePart::new(1);
        assert_eq!(theme.rel_target(), "../theme/theme1.xml");
    }
}
