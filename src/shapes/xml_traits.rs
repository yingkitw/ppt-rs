//! XML trait implementations for shapes
//!
//! Provides OpenXmlSerialize and OpenXmlDeserialize implementations for shapes
//! using the XML builder pattern.

use crate::oxml::{XmlBuilder, OpenXmlSerialize, OpenXmlDeserialize};
use crate::shapes::base::Shape;
use crate::error::Result;

/// Shape XML serializer using builder pattern
pub struct ShapeXmlSerializer;

impl ShapeXmlSerializer {
    /// Serialize a shape to XML using the builder pattern
    pub fn serialize(shape: &dyn Shape) -> String {
        let mut builder = XmlBuilder::new("p:sp");

        // Non-visual shape properties
        let nvsppr = format!(
            r#"<p:nvSpPr><p:cNvPr id="{}" name="{}"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr/></p:nvSpPr>"#,
            shape.id(),
            shape.name()
        );

        // Shape properties (position and size)
        let sppr = format!(
            r#"<p:spPr><a:xfrm><a:off x="{}" y="{}"/><a:ext cx="{}" cy="{}"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom></p:spPr>"#,
            shape.left(),
            shape.top(),
            shape.width(),
            shape.height()
        );

        // Text body (if shape has text frame)
        let txbody = if shape.has_text_frame() {
            r#"<p:txBody><a:bodyPr/><a:lstStyle/><a:p/></p:txBody>"#.to_string()
        } else {
            String::new()
        };

        builder = builder
            .add_child(nvsppr)
            .add_child(sppr)
            .add_child(txbody);

        builder.build()
    }

    /// Serialize a shape with XML declaration
    pub fn serialize_with_declaration(shape: &dyn Shape) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>{}"#,
            Self::serialize(shape)
        )
    }

    /// Serialize multiple shapes
    pub fn serialize_multiple(shapes: &[&dyn Shape]) -> String {
        let mut xml = String::new();
        for shape in shapes {
            xml.push_str(&Self::serialize(*shape));
        }
        xml
    }
}

/// Trait implementation for shapes to support XML serialization
pub trait ShapeXmlExt {
    /// Serialize this shape to XML
    fn to_xml_string(&self) -> String;

    /// Serialize this shape with XML declaration
    fn to_xml_with_declaration(&self) -> String;
}

/// Implement ShapeXmlExt for all types that implement Shape
impl<T: Shape> ShapeXmlExt for T {
    fn to_xml_string(&self) -> String {
        ShapeXmlSerializer::serialize(self)
    }

    fn to_xml_with_declaration(&self) -> String {
        ShapeXmlSerializer::serialize_with_declaration(self)
    }
}

/// Implement ShapeXmlExt for trait objects
impl ShapeXmlExt for dyn Shape {
    fn to_xml_string(&self) -> String {
        ShapeXmlSerializer::serialize(self)
    }

    fn to_xml_with_declaration(&self) -> String {
        ShapeXmlSerializer::serialize_with_declaration(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::autoshape::{AutoShape, AutoShapeType};
    use crate::shapes::base::BaseShape;

    #[test]
    fn test_serialize_base_shape() {
        let shape = BaseShape::new(1, "Test Shape".to_string());
        let xml = ShapeXmlSerializer::serialize(&shape);
        assert!(xml.contains("<p:sp"));
        assert!(xml.contains(r#"id="1""#));
        assert!(xml.contains(r#"name="Test Shape""#));
        assert!(xml.contains("</p:sp>"));
    }

    #[test]
    fn test_serialize_with_position() {
        let mut shape = BaseShape::new(2, "Positioned Shape".to_string());
        shape.set_left(100000);
        shape.set_top(200000);
        shape.set_width(300000);
        shape.set_height(400000);

        let xml = ShapeXmlSerializer::serialize(&shape);
        assert!(xml.contains(r#"x="100000""#));
        assert!(xml.contains(r#"y="200000""#));
        assert!(xml.contains(r#"cx="300000""#));
        assert!(xml.contains(r#"cy="400000""#));
    }

    #[test]
    fn test_serialize_with_declaration() {
        let shape = BaseShape::new(1, "Test".to_string());
        let xml = ShapeXmlSerializer::serialize_with_declaration(&shape);
        assert!(xml.starts_with(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#));
        assert!(xml.contains("<p:sp"));
    }

    #[test]
    fn test_serialize_multiple() {
        let shape1 = BaseShape::new(1, "Shape1".to_string());
        let shape2 = BaseShape::new(2, "Shape2".to_string());
        let shapes: Vec<&dyn Shape> = vec![&shape1, &shape2];

        let xml = ShapeXmlSerializer::serialize_multiple(&shapes);
        assert!(xml.contains(r#"id="1""#));
        assert!(xml.contains(r#"id="2""#));
        // Count opening tags only (not closing tags or self-closing)
        assert_eq!(xml.matches("<p:sp>").count(), 2);
    }

    #[test]
    fn test_shape_xml_ext_trait() {
        let shape = BaseShape::new(1, "Test".to_string());
        let xml = shape.to_xml_string();
        assert!(xml.contains("<p:sp"));
        assert!(xml.contains(r#"id="1""#));
    }

    #[test]
    fn test_autoshape_serialization() {
        let shape = AutoShape::new(3, "AutoShape".to_string(), AutoShapeType::Rectangle);
        let xml = ShapeXmlSerializer::serialize(&shape);
        assert!(xml.contains(r#"id="3""#));
        assert!(xml.contains(r#"name="AutoShape""#));
    }
}
