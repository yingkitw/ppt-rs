//! Core traits for PPTX elements
//!
//! These traits provide a consistent interface for XML generation
//! and element manipulation across the library.

/// Trait for types that can be converted to XML
pub trait ToXml {
    /// Generate XML representation of this element
    fn to_xml(&self) -> String;
    
    /// Write XML to a string buffer (more efficient for large documents)
    fn write_xml(&self, writer: &mut String) {
        writer.push_str(&self.to_xml());
    }
}

/// Trait for XML elements with a tag name
pub trait XmlElement: ToXml {
    /// Get the XML tag name for this element
    fn tag_name(&self) -> &'static str;
    
    /// Get XML namespace prefix (e.g., "a", "p", "r")
    fn namespace_prefix(&self) -> &'static str {
        ""
    }
    
    /// Get the fully qualified tag name
    fn qualified_name(&self) -> String {
        let prefix = self.namespace_prefix();
        if prefix.is_empty() {
            self.tag_name().to_string()
        } else {
            format!("{}:{}", prefix, self.tag_name())
        }
    }
}

/// Trait for positioned elements (x, y coordinates)
pub trait Positioned {
    /// Get X position in EMU
    fn x(&self) -> u32;
    
    /// Get Y position in EMU
    fn y(&self) -> u32;
    
    /// Set position
    fn set_position(&mut self, x: u32, y: u32);
}

/// Trait for sized elements (width, height)
pub trait Sized {
    /// Get width in EMU
    fn width(&self) -> u32;
    
    /// Get height in EMU
    fn height(&self) -> u32;
    
    /// Set size
    fn set_size(&mut self, width: u32, height: u32);
}

/// Trait for styled elements (color, formatting)
pub trait Styled {
    /// Get the primary color (if any)
    fn color(&self) -> Option<&str>;
    
    /// Set the primary color
    fn set_color(&mut self, color: &str);
}

/// RGB color representation
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(dead_code)]
impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    
    /// Parse from hex string (e.g., "FF0000" or "#FF0000")
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self { r, g, b })
    }
    
    /// Convert to hex string (uppercase, no #)
    pub fn to_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl ToXml for RgbColor {
    fn to_xml(&self) -> String {
        format!(r#"<a:srgbClr val="{}"/>"#, self.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_color_from_hex() {
        let color = RgbColor::from_hex("FF0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        
        let color = RgbColor::from_hex("#00FF00").unwrap();
        assert_eq!(color.to_hex(), "00FF00");
    }

    #[test]
    fn test_rgb_color_to_xml() {
        let color = RgbColor::new(255, 0, 0);
        assert_eq!(color.to_xml(), r#"<a:srgbClr val="FF0000"/>"#);
    }

    /// Verify generic dispatch works via ToXml trait objects
    #[test]
    fn test_to_xml_trait_dispatch() {
        use crate::generator::text::{Run, Paragraph, TextFrame};

        let items: Vec<Box<dyn ToXml>> = vec![
            Box::new(Run::new("hello")),
            Box::new(Paragraph::with_text("world")),
            Box::new(TextFrame::with_text("frame")),
            Box::new(RgbColor::new(255, 0, 0)),
        ];

        for item in &items {
            let xml = item.to_xml();
            assert!(!xml.is_empty(), "ToXml dispatch should produce non-empty XML");
        }

        assert!(items[0].to_xml().contains("hello"));
        assert!(items[1].to_xml().contains("world"));
        assert!(items[2].to_xml().contains("frame"));
        assert!(items[3].to_xml().contains("FF0000"));
    }

    /// Verify Positioned trait works generically
    #[test]
    fn test_positioned_trait_dispatch() {
        use crate::generator::shapes::{Shape, ShapeType};
        use crate::generator::images::Image;

        fn move_element(elem: &mut dyn Positioned, x: u32, y: u32) {
            elem.set_position(x, y);
        }

        let mut shape = Shape::new(ShapeType::Rectangle, 0, 0, 1000, 1000);
        let mut image = Image::new("test.png", 500, 500, "PNG");

        move_element(&mut shape, 100, 200);
        move_element(&mut image, 300, 400);

        assert_eq!(shape.x(), 100);
        assert_eq!(shape.y(), 200);
        assert_eq!(image.x(), 300);
        assert_eq!(image.y(), 400);
    }

    /// Verify ElementSized trait works generically
    #[test]
    fn test_element_sized_trait_dispatch() {
        use crate::generator::shapes::{Shape, ShapeType};
        use crate::generator::images::Image;

        fn resize(elem: &mut dyn Sized, w: u32, h: u32) {
            elem.set_size(w, h);
        }

        let mut shape = Shape::new(ShapeType::Rectangle, 0, 0, 1000, 1000);
        let mut image = Image::new("test.png", 500, 500, "PNG");

        resize(&mut shape, 2000, 3000);
        resize(&mut image, 4000, 5000);

        assert_eq!(shape.width(), 2000);
        assert_eq!(shape.height(), 3000);
        assert_eq!(image.width(), 4000);
        assert_eq!(image.height(), 5000);
    }
}
