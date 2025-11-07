//! Picture shape functionality

use crate::shapes::base::{BaseShape, Shape};

/// Picture shape - contains an image
pub struct Picture {
    base: BaseShape,
    image_part_id: Option<String>,
    crop_left: f64,
    crop_right: f64,
    crop_top: f64,
    crop_bottom: f64,
}

impl Picture {
    /// Create a new Picture shape
    pub fn new(id: u32, name: String) -> Self {
        Self {
            base: BaseShape::new(id, name),
            image_part_id: None,
            crop_left: 0.0,
            crop_right: 0.0,
            crop_top: 0.0,
            crop_bottom: 0.0,
        }
    }
    
    /// Create a new Picture shape with image
    pub fn with_image(id: u32, name: String, image_part_id: String) -> Self {
        Self {
            base: BaseShape::new(id, name),
            image_part_id: Some(image_part_id),
            crop_left: 0.0,
            crop_right: 0.0,
            crop_top: 0.0,
            crop_bottom: 0.0,
        }
    }
    
    /// Get the image part ID
    pub fn image_part_id(&self) -> Option<&str> {
        self.image_part_id.as_deref()
    }
    
    /// Set the image part ID
    pub fn set_image_part_id(&mut self, image_part_id: String) {
        self.image_part_id = Some(image_part_id);
    }
    
    /// Get crop left (0.0 to 1.0)
    pub fn crop_left(&self) -> f64 {
        self.crop_left
    }
    
    /// Set crop left
    pub fn set_crop_left(&mut self, value: f64) {
        self.crop_left = value;
    }
    
    /// Get crop right
    pub fn crop_right(&self) -> f64 {
        self.crop_right
    }
    
    /// Set crop right
    pub fn set_crop_right(&mut self, value: f64) {
        self.crop_right = value;
    }
    
    /// Get crop top
    pub fn crop_top(&self) -> f64 {
        self.crop_top
    }
    
    /// Set crop top
    pub fn set_crop_top(&mut self, value: f64) {
        self.crop_top = value;
    }
    
    /// Get crop bottom
    pub fn crop_bottom(&self) -> f64 {
        self.crop_bottom
    }
    
    /// Set crop bottom
    pub fn set_crop_bottom(&mut self, value: f64) {
        self.crop_bottom = value;
    }
}

impl Shape for Picture {
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

