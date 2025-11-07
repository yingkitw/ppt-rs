//! Connector shape functionality

use crate::shapes::base::{BaseShape, Shape};

/// Connector shape - connects two shapes
pub struct Connector {
    base: BaseShape,
    start_shape_id: Option<u32>,
    end_shape_id: Option<u32>,
}

impl Connector {
    /// Create a new Connector shape
    pub fn new(id: u32, name: String) -> Self {
        Self {
            base: BaseShape::new(id, name),
            start_shape_id: None,
            end_shape_id: None,
        }
    }
    
    /// Create a connector between two shapes
    pub fn between(id: u32, name: String, start_shape_id: u32, end_shape_id: u32) -> Self {
        Self {
            base: BaseShape::new(id, name),
            start_shape_id: Some(start_shape_id),
            end_shape_id: Some(end_shape_id),
        }
    }
    
    /// Get the start shape ID
    pub fn start_shape_id(&self) -> Option<u32> {
        self.start_shape_id
    }
    
    /// Set the start shape ID
    pub fn set_start_shape_id(&mut self, shape_id: u32) {
        self.start_shape_id = Some(shape_id);
    }
    
    /// Get the end shape ID
    pub fn end_shape_id(&self) -> Option<u32> {
        self.end_shape_id
    }
    
    /// Set the end shape ID
    pub fn set_end_shape_id(&mut self, shape_id: u32) {
        self.end_shape_id = Some(shape_id);
    }
}

impl Shape for Connector {
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
}

