//! Shapes module - handles all shape types in PowerPoint

pub mod base;
pub mod autoshape;
pub mod picture;
pub mod connector;
pub mod graphfrm;
pub mod group;

pub use base::{BaseShape, Shape};
pub use autoshape::{AutoShape, AutoShapeType};
pub use picture::Picture;
pub use connector::Connector;
pub use graphfrm::{GraphicFrame, GraphicFrameContentType};
pub use group::GroupShape;

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
}
