//! Core traits for PPTX elements

/// Trait for types that can be converted to XML
pub trait ToXml {
    /// Generate XML representation of this element
    fn to_xml(&self) -> String;

    /// Write XML to a string buffer (more efficient for large documents)
    fn write_xml(&self, writer: &mut String) {
        writer.push_str(&self.to_xml());
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify generic dispatch works via ToXml trait objects
    #[test]
    fn test_to_xml_trait_dispatch() {
        use crate::generator::text::{Paragraph, Run, TextFrame};

        let items: Vec<Box<dyn ToXml>> = vec![
            Box::new(Run::new("hello")),
            Box::new(Paragraph::with_text("world")),
            Box::new(TextFrame::with_text("frame")),
        ];

        for item in &items {
            let xml = item.to_xml();
            assert!(
                !xml.is_empty(),
                "ToXml dispatch should produce non-empty XML"
            );
        }

        assert!(items[0].to_xml().contains("hello"));
        assert!(items[1].to_xml().contains("world"));
        assert!(items[2].to_xml().contains("frame"));
    }

    /// Verify Positioned trait works generically
    #[test]
    fn test_positioned_trait_dispatch() {
        use crate::generator::images::Image;
        use crate::generator::shapes::{Shape, ShapeType};

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
        use crate::generator::images::Image;
        use crate::generator::shapes::{Shape, ShapeType};

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
