//! Shapes module - handles all shape types in PowerPoint

pub mod base;
pub mod autoshape;
pub mod picture;
pub mod connector;
pub mod graphfrm;
pub mod group;
pub mod xml;
pub mod hyperlink;
pub mod xml_traits;
pub mod custom_geometry;

pub use base::{BaseShape, Shape};
pub use autoshape::{AutoShape, AutoShapeType};
pub use picture::Picture;
pub use connector::Connector;
pub use graphfrm::{GraphicFrame, GraphicFrameContentType};
pub use group::GroupShape;
pub use xml::{parse_shapes_from_xml, shape_to_xml, next_shape_id};
pub use hyperlink::{Hyperlink, hyperlink_to_xml, parse_hyperlink_from_xml};
pub use xml_traits::{ShapeXmlSerializer, ShapeXmlExt};
pub use custom_geometry::{CustomGeometry, GeometryPoint, CurveType, CurveInfo};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::base::BaseShape;
    use crate::shapes::autoshape::AutoShapeType;

    #[test]
    fn test_base_shape_new() {
        let shape = BaseShape::new(1, "Shape1".to_string());
        assert_eq!(shape.id(), 1);
        assert_eq!(shape.name(), "Shape1");
        assert_eq!(shape.width(), 914400);
        assert_eq!(shape.height(), 914400);
    }

    #[test]
    fn test_base_shape_position() {
        let mut shape = BaseShape::new(1, "Shape1".to_string());
        shape.set_left(1000000);
        shape.set_top(2000000);
        shape.set_width(3000000);
        shape.set_height(4000000);
        
        assert_eq!(shape.left(), 1000000);
        assert_eq!(shape.top(), 2000000);
        assert_eq!(shape.width(), 3000000);
        assert_eq!(shape.height(), 4000000);
    }

    #[test]
    fn test_autoshape_new() {
        let shape = AutoShape::new(1, "Rectangle".to_string(), AutoShapeType::Rectangle);
        assert_eq!(shape.id(), 1);
        assert_eq!(shape.shape_type(), AutoShapeType::Rectangle);
        assert!(!shape.has_text_frame());
    }

    #[test]
    fn test_autoshape_with_text_frame() {
        let shape = AutoShape::with_text_frame(1, "Rectangle".to_string(), AutoShapeType::Rectangle);
        assert!(shape.has_text_frame());
        assert!(shape.text_frame().is_some());
    }

    #[test]
    fn test_picture_new() {
        let pic = Picture::new(1, "Picture1".to_string());
        assert_eq!(pic.id(), 1);
        assert_eq!(pic.crop_left(), 0.0);
        assert_eq!(pic.crop_right(), 0.0);
    }

    #[test]
    fn test_picture_crop() {
        let mut pic = Picture::new(1, "Picture1".to_string());
        pic.set_crop_left(0.1);
        pic.set_crop_right(0.2);
        pic.set_crop_top(0.3);
        pic.set_crop_bottom(0.4);
        
        assert_eq!(pic.crop_left(), 0.1);
        assert_eq!(pic.crop_right(), 0.2);
        assert_eq!(pic.crop_top(), 0.3);
        assert_eq!(pic.crop_bottom(), 0.4);
    }

    #[test]
    fn test_connector_new() {
        let conn = Connector::new(1, "Connector1".to_string());
        assert_eq!(conn.id(), 1);
        assert!(conn.start_shape_id().is_none());
        assert!(conn.end_shape_id().is_none());
    }

    #[test]
    fn test_connector_between() {
        let conn = Connector::between(1, "Connector1".to_string(), 10, 20);
        assert_eq!(conn.start_shape_id(), Some(10));
        assert_eq!(conn.end_shape_id(), Some(20));
    }

    #[test]
    fn test_graphic_frame_with_chart() {
        use crate::shapes::graphfrm::{GraphicFrame, GraphicFrameContentType};
        use crate::chart::Chart;
        use crate::enums::chart::ChartType;
        
        let chart = Chart::new(ChartType::ColumnClustered);
        let gf = GraphicFrame::with_chart(1, "Chart1".to_string(), chart);
        assert!(gf.has_chart());
        assert!(!gf.has_table());
    }

    #[test]
    fn test_group_shape_add_shape() {
        use crate::shapes::group::GroupShape;
        
        let mut group = GroupShape::new(1, "Group1".to_string());
        let shape1 = Box::new(AutoShape::new(2, "Rect1".to_string(), AutoShapeType::Rectangle));
        group.add_shape(shape1);
        assert_eq!(group.shape_count(), 1);
    }

    #[test]
    fn test_autoshape_hyperlink() {
        let mut shape = AutoShape::new(1, "Link".to_string(), AutoShapeType::Rectangle);
        assert!(shape.hyperlink().is_none());
        
        let mut hlink = crate::shapes::hyperlink::Hyperlink::with_address("https://example.com".to_string());
        hlink.set_screen_tip(Some("Example".to_string()));
        shape.set_hyperlink(Some(hlink));
        
        assert!(shape.hyperlink().is_some());
        assert_eq!(shape.hyperlink().unwrap().address(), Some("https://example.com"));
        assert_eq!(shape.hyperlink().unwrap().screen_tip(), Some("Example"));
    }

    #[test]
    fn test_picture_hyperlink() {
        let mut pic = Picture::new(1, "Pic".to_string());
        assert!(pic.hyperlink().is_none());
        
        let hlink = crate::shapes::hyperlink::Hyperlink::with_address("https://example.com".to_string());
        pic.set_hyperlink(Some(hlink));
        
        assert!(pic.hyperlink().is_some());
        assert_eq!(pic.hyperlink().unwrap().address(), Some("https://example.com"));
    }

    #[test]
    fn test_autoshape_remove_hyperlink() {
        let mut shape = AutoShape::new(1, "Shape".to_string(), AutoShapeType::Rectangle);
        let hlink = crate::shapes::hyperlink::Hyperlink::with_address("https://example.com".to_string());
        shape.set_hyperlink(Some(hlink));
        assert!(shape.hyperlink().is_some());
        
        shape.set_hyperlink(None);
        assert!(shape.hyperlink().is_none());
    }
}
