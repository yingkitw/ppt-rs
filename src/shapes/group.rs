//! GroupShape - a shape that acts as a container for other shapes

use crate::shapes::base::{BaseShape, Shape};

/// GroupShape - a shape that acts as a container for other shapes
pub struct GroupShape {
    base: BaseShape,
    shapes: Vec<Box<dyn Shape>>,
}

impl GroupShape {
    /// Create a new GroupShape
    pub fn new(id: u32, name: String) -> Self {
        Self {
            base: BaseShape::new(id, name),
            shapes: Vec::new(),
        }
    }

    /// Add a shape to this group
    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    /// Get the number of shapes in this group
    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }

    /// Get shapes in this group
    pub fn shapes(&self) -> &[Box<dyn Shape>] {
        &self.shapes
    }

    /// Get mutable shapes in this group
    pub fn shapes_mut(&mut self) -> &mut [Box<dyn Shape>] {
        &mut self.shapes
    }

    /// Remove a shape by index
    pub fn remove_shape(&mut self, index: usize) -> Option<Box<dyn Shape>> {
        if index < self.shapes.len() {
            Some(self.shapes.remove(index))
        } else {
            None
        }
    }

    /// Clear all shapes from this group
    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}

impl Shape for GroupShape {
    fn id(&self) -> u32 {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn set_name(&mut self, name: String) {
        self.base.set_name(name);
    }

    fn left(&self) -> i64 {
        self.base.left()
    }

    fn set_left(&mut self, left: i64) {
        self.base.set_left(left);
    }

    fn top(&self) -> i64 {
        self.base.top()
    }

    fn set_top(&mut self, top: i64) {
        self.base.set_top(top);
    }

    fn width(&self) -> u32 {
        self.base.width()
    }

    fn set_width(&mut self, width: u32) {
        self.base.set_width(width);
    }

    fn height(&self) -> u32 {
        self.base.height()
    }

    fn set_height(&mut self, height: u32) {
        self.base.set_height(height);
    }

    fn has_text_frame(&self) -> bool {
        false // GroupShape doesn't have its own text frame
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::autoshape::{AutoShape, AutoShapeType};
    use crate::shapes::picture::Picture;

    #[test]
    fn test_group_shape_new() {
        let group = GroupShape::new(1, "Group1".to_string());
        assert_eq!(group.id(), 1);
        assert_eq!(group.shape_count(), 0);
        assert!(!group.has_text_frame());
    }

    #[test]
    fn test_group_shape_add_shape() {
        let mut group = GroupShape::new(1, "Group1".to_string());
        let shape1 = Box::new(AutoShape::new(2, "Rect1".to_string(), AutoShapeType::Rectangle));
        let shape2 = Box::new(Picture::new(3, "Pic1".to_string()));
        
        group.add_shape(shape1);
        group.add_shape(shape2);
        
        assert_eq!(group.shape_count(), 2);
    }

    #[test]
    fn test_group_shape_remove_shape() {
        let mut group = GroupShape::new(1, "Group1".to_string());
        let shape1 = Box::new(AutoShape::new(2, "Rect1".to_string(), AutoShapeType::Rectangle));
        group.add_shape(shape1);
        
        assert_eq!(group.shape_count(), 1);
        let removed = group.remove_shape(0);
        assert!(removed.is_some());
        assert_eq!(group.shape_count(), 0);
    }

    #[test]
    fn test_group_shape_clear() {
        let mut group = GroupShape::new(1, "Group1".to_string());
        group.add_shape(Box::new(AutoShape::new(2, "Rect1".to_string(), AutoShapeType::Rectangle)));
        group.add_shape(Box::new(Picture::new(3, "Pic1".to_string())));
        
        assert_eq!(group.shape_count(), 2);
        group.clear();
        assert_eq!(group.shape_count(), 0);
    }
}

