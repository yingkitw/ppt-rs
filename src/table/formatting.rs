//! Table Formatting Support
//!
//! This module provides comprehensive table formatting capabilities including:
//! - Cell borders (all sides, individual sides)
//! - Cell shading (background colors)
//! - Cell text formatting (alignment, margins)
//! - Row and column formatting
//! - Table-wide formatting

use crate::dml::color::RGBColor;
use crate::error::Result;

/// Border style for table cells
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    /// No border
    None,
    /// Solid line
    Solid,
    /// Dashed line
    Dashed,
    /// Dotted line
    Dotted,
    /// Double line
    Double,
}

impl BorderStyle {
    /// Get the XML representation of the border style
    pub fn to_xml_str(&self) -> &str {
        match self {
            BorderStyle::None => "none",
            BorderStyle::Solid => "solid",
            BorderStyle::Dashed => "dash",
            BorderStyle::Dotted => "dot",
            BorderStyle::Double => "dbl",
        }
    }
}

/// Cell border configuration
#[derive(Debug, Clone)]
pub struct CellBorder {
    style: BorderStyle,
    width: u32,  // in EMU
    color: Option<RGBColor>,
}

impl CellBorder {
    /// Create a new cell border
    pub fn new(style: BorderStyle, width: u32) -> Self {
        Self {
            style,
            width,
            color: None,
        }
    }

    /// Set the border color
    pub fn set_color(mut self, color: RGBColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Get the border style
    pub fn style(&self) -> BorderStyle {
        self.style
    }

    /// Get the border width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the border color
    pub fn color(&self) -> Option<&RGBColor> {
        self.color.as_ref()
    }

    /// Generate XML for the border
    pub fn to_xml(&self, side: &str) -> String {
        if self.style == BorderStyle::None {
            return format!("<a:{} w=\"0\" cap=\"flat\" cmpd=\"sng\" algn=\"ctr\"/>", side);
        }

        let color_xml = if let Some(color) = &self.color {
            format!(
                "<a:solidFill><a:srgbClr val=\"{}\"/></a:solidFill>",
                color.to_hex()
            )
        } else {
            "<a:solidFill><a:srgbClr val=\"000000\"/></a:solidFill>".to_string()
        };

        format!(
            "<a:{} w=\"{}\" cap=\"flat\" cmpd=\"sng\" algn=\"ctr\">{}<a:prstDash val=\"{}\"/><a:round/></a:{}>",
            side,
            self.width,
            color_xml,
            self.style.to_xml_str(),
            side
        )
    }
}

/// Cell shading (background color)
#[derive(Debug, Clone)]
pub struct CellShading {
    color: RGBColor,
}

impl CellShading {
    /// Create a new cell shading
    pub fn new(color: RGBColor) -> Self {
        Self { color }
    }

    /// Get the shading color
    pub fn color(&self) -> &RGBColor {
        &self.color
    }

    /// Generate XML for cell shading
    pub fn to_xml(&self) -> String {
        format!(
            "<a:solidFill><a:srgbClr val=\"{}\"/></a:solidFill>",
            self.color.to_hex()
        )
    }
}

/// Cell text alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellAlignment {
    /// Left alignment
    Left,
    /// Center alignment
    Center,
    /// Right alignment
    Right,
    /// Justified alignment
    Justified,
}

impl CellAlignment {
    /// Get the XML representation of the alignment
    pub fn to_xml_str(&self) -> &str {
        match self {
            CellAlignment::Left => "l",
            CellAlignment::Center => "ctr",
            CellAlignment::Right => "r",
            CellAlignment::Justified => "just",
        }
    }
}

/// Cell vertical alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    /// Top alignment
    Top,
    /// Middle alignment
    Middle,
    /// Bottom alignment
    Bottom,
}

impl VerticalAlignment {
    /// Get the XML representation of the vertical alignment
    pub fn to_xml_str(&self) -> &str {
        match self {
            VerticalAlignment::Top => "t",
            VerticalAlignment::Middle => "ctr",
            VerticalAlignment::Bottom => "b",
        }
    }
}

/// Cell formatting options
#[derive(Debug, Clone)]
pub struct CellFormat {
    /// Left border
    left_border: Option<CellBorder>,
    /// Right border
    right_border: Option<CellBorder>,
    /// Top border
    top_border: Option<CellBorder>,
    /// Bottom border
    bottom_border: Option<CellBorder>,
    /// Cell shading
    shading: Option<CellShading>,
    /// Horizontal alignment
    alignment: CellAlignment,
    /// Vertical alignment
    vertical_alignment: VerticalAlignment,
    /// Left margin in EMU
    left_margin: u32,
    /// Right margin in EMU
    right_margin: u32,
    /// Top margin in EMU
    top_margin: u32,
    /// Bottom margin in EMU
    bottom_margin: u32,
}

impl Default for CellFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl CellFormat {
    /// Create a new cell format with defaults
    pub fn new() -> Self {
        Self {
            left_border: None,
            right_border: None,
            top_border: None,
            bottom_border: None,
            shading: None,
            alignment: CellAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
            left_margin: 91440,    // ~0.1"
            right_margin: 91440,   // ~0.1"
            top_margin: 45720,     // ~0.05"
            bottom_margin: 45720,  // ~0.05"
        }
    }

    /// Set all borders at once
    pub fn set_all_borders(mut self, border: CellBorder) -> Self {
        self.left_border = Some(border.clone());
        self.right_border = Some(border.clone());
        self.top_border = Some(border.clone());
        self.bottom_border = Some(border);
        self
    }

    /// Set left border
    pub fn set_left_border(mut self, border: CellBorder) -> Self {
        self.left_border = Some(border);
        self
    }

    /// Set right border
    pub fn set_right_border(mut self, border: CellBorder) -> Self {
        self.right_border = Some(border);
        self
    }

    /// Set top border
    pub fn set_top_border(mut self, border: CellBorder) -> Self {
        self.top_border = Some(border);
        self
    }

    /// Set bottom border
    pub fn set_bottom_border(mut self, border: CellBorder) -> Self {
        self.bottom_border = Some(border);
        self
    }

    /// Set cell shading
    pub fn set_shading(mut self, shading: CellShading) -> Self {
        self.shading = Some(shading);
        self
    }

    /// Set cell shading with color
    pub fn set_shading_color(mut self, color: RGBColor) -> Self {
        self.shading = Some(CellShading::new(color));
        self
    }

    /// Set horizontal alignment
    pub fn set_alignment(mut self, alignment: CellAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set vertical alignment
    pub fn set_vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Set all margins
    pub fn set_all_margins(mut self, margin: u32) -> Self {
        self.left_margin = margin;
        self.right_margin = margin;
        self.top_margin = margin;
        self.bottom_margin = margin;
        self
    }

    /// Get left border
    pub fn left_border(&self) -> Option<&CellBorder> {
        self.left_border.as_ref()
    }

    /// Get right border
    pub fn right_border(&self) -> Option<&CellBorder> {
        self.right_border.as_ref()
    }

    /// Get top border
    pub fn top_border(&self) -> Option<&CellBorder> {
        self.top_border.as_ref()
    }

    /// Get bottom border
    pub fn bottom_border(&self) -> Option<&CellBorder> {
        self.bottom_border.as_ref()
    }

    /// Get cell shading
    pub fn shading(&self) -> Option<&CellShading> {
        self.shading.as_ref()
    }

    /// Get horizontal alignment
    pub fn alignment(&self) -> CellAlignment {
        self.alignment
    }

    /// Get vertical alignment
    pub fn vertical_alignment(&self) -> VerticalAlignment {
        self.vertical_alignment
    }

    /// Generate XML for cell properties
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<a:tcPr>");

        // Add borders
        if let Some(border) = &self.left_border {
            xml.push_str(&border.to_xml("lnL"));
        }
        if let Some(border) = &self.right_border {
            xml.push_str(&border.to_xml("lnR"));
        }
        if let Some(border) = &self.top_border {
            xml.push_str(&border.to_xml("lnT"));
        }
        if let Some(border) = &self.bottom_border {
            xml.push_str(&border.to_xml("lnB"));
        }

        // Add shading
        if let Some(shading) = &self.shading {
            xml.push_str(&shading.to_xml());
        }

        // Add margins
        xml.push_str(&format!(
            "<a:tcMar l=\"{}\" r=\"{}\" t=\"{}\" b=\"{}\"/>",
            self.left_margin, self.right_margin, self.top_margin, self.bottom_margin
        ));

        xml.push_str("</a:tcPr>");
        xml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_style_solid() {
        let style = BorderStyle::Solid;
        assert_eq!(style.to_xml_str(), "solid");
    }

    #[test]
    fn test_border_style_dashed() {
        let style = BorderStyle::Dashed;
        assert_eq!(style.to_xml_str(), "dash");
    }

    #[test]
    fn test_cell_border_creation() {
        let border = CellBorder::new(BorderStyle::Solid, 12700);
        assert_eq!(border.style(), BorderStyle::Solid);
        assert_eq!(border.width(), 12700);
        assert!(border.color().is_none());
    }

    #[test]
    fn test_cell_border_with_color() {
        let color = RGBColor::new(255, 0, 0);
        let border = CellBorder::new(BorderStyle::Solid, 12700).set_color(color);
        assert!(border.color().is_some());
    }

    #[test]
    fn test_cell_shading_creation() {
        let color = RGBColor::new(200, 200, 200);
        let shading = CellShading::new(color);
        assert_eq!(shading.color(), &color);
    }

    #[test]
    fn test_cell_alignment_left() {
        let align = CellAlignment::Left;
        assert_eq!(align.to_xml_str(), "l");
    }

    #[test]
    fn test_cell_alignment_center() {
        let align = CellAlignment::Center;
        assert_eq!(align.to_xml_str(), "ctr");
    }

    #[test]
    fn test_vertical_alignment_top() {
        let align = VerticalAlignment::Top;
        assert_eq!(align.to_xml_str(), "t");
    }

    #[test]
    fn test_cell_format_default() {
        let format = CellFormat::new();
        assert_eq!(format.alignment(), CellAlignment::Left);
        assert_eq!(format.vertical_alignment(), VerticalAlignment::Top);
        assert!(format.shading().is_none());
    }

    #[test]
    fn test_cell_format_with_borders() {
        let border = CellBorder::new(BorderStyle::Solid, 12700);
        let format = CellFormat::new().set_all_borders(border);
        assert!(format.left_border().is_some());
        assert!(format.right_border().is_some());
        assert!(format.top_border().is_some());
        assert!(format.bottom_border().is_some());
    }

    #[test]
    fn test_cell_format_with_shading() {
        let color = RGBColor::new(200, 200, 200);
        let format = CellFormat::new().set_shading_color(color);
        assert!(format.shading().is_some());
    }

    #[test]
    fn test_cell_format_alignment() {
        let format = CellFormat::new().set_alignment(CellAlignment::Center);
        assert_eq!(format.alignment(), CellAlignment::Center);
    }

    #[test]
    fn test_cell_format_to_xml() {
        let format = CellFormat::new();
        let xml = format.to_xml();
        assert!(xml.contains("<a:tcPr>"));
        assert!(xml.contains("</a:tcPr>"));
        assert!(xml.contains("tcMar"));
    }

    #[test]
    fn test_cell_format_with_all_options() {
        let border = CellBorder::new(BorderStyle::Solid, 12700).set_color(RGBColor::new(0, 0, 0));
        let format = CellFormat::new()
            .set_all_borders(border)
            .set_shading_color(RGBColor::new(200, 200, 200))
            .set_alignment(CellAlignment::Center)
            .set_vertical_alignment(VerticalAlignment::Middle)
            .set_all_margins(91440);

        assert!(format.left_border().is_some());
        assert!(format.shading().is_some());
        assert_eq!(format.alignment(), CellAlignment::Center);
        assert_eq!(format.vertical_alignment(), VerticalAlignment::Middle);
    }
}
