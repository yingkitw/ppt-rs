//! AutoShape functionality

use crate::shapes::base::{BaseShape, Shape};
use crate::text::TextFrame;

/// AutoShape - predefined shapes like rectangles, circles, etc.
pub struct AutoShape {
    base: BaseShape,
    shape_type: AutoShapeType,
    text_frame: Option<TextFrame>,
}

/// AutoShape types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoShapeType {
    Rectangle,
    Oval,
    Line,
    RoundedRectangle,
    Triangle,
    RightTriangle,
    Parallelogram,
    Trapezoid,
    Diamond,
    Pentagon,
    Hexagon,
    Octagon,
    Star,
    Arrow,
    // TODO: Add more shape types
}

impl AutoShape {
    /// Create a new AutoShape
    pub fn new(id: u32, name: String, shape_type: AutoShapeType) -> Self {
        Self {
            base: BaseShape::new(id, name),
            shape_type,
            text_frame: None,
        }
    }
    
    /// Create a new AutoShape with text frame
    pub fn with_text_frame(id: u32, name: String, shape_type: AutoShapeType) -> Self {
        Self {
            base: BaseShape::new(id, name),
            shape_type,
            text_frame: Some(TextFrame::new()),
        }
    }
    
    /// Get the shape type
    pub fn shape_type(&self) -> AutoShapeType {
        self.shape_type
    }
    
    /// Set the shape type
    pub fn set_shape_type(&mut self, shape_type: AutoShapeType) {
        self.shape_type = shape_type;
    }
}

impl Shape for AutoShape {
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
        self.text_frame.is_some()
    }
    
    fn text_frame(&self) -> Option<&TextFrame> {
        self.text_frame.as_ref()
    }
    
    fn text_frame_mut(&mut self) -> Option<&mut TextFrame> {
        self.text_frame.as_mut()
    }
}

