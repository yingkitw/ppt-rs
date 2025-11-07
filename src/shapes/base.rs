//! Base shape functionality

use crate::text::TextFrame;
use crate::shapes::hyperlink::Hyperlink;

/// Base trait for all shapes
pub trait Shape: Send + Sync {
    /// Get the shape ID
    fn id(&self) -> u32;
    
    /// Get the shape name
    fn name(&self) -> &str;
    
    /// Set the shape name
    fn set_name(&mut self, name: String);
    
    /// Get the left position in EMU
    fn left(&self) -> i64;
    
    /// Set the left position in EMU
    fn set_left(&mut self, left: i64);
    
    /// Get the top position in EMU
    fn top(&self) -> i64;
    
    /// Set the top position in EMU
    fn set_top(&mut self, top: i64);
    
    /// Get the width in EMU
    fn width(&self) -> u32;
    
    /// Set the width in EMU
    fn set_width(&mut self, width: u32);
    
    /// Get the height in EMU
    fn height(&self) -> u32;
    
    /// Set the height in EMU
    fn set_height(&mut self, height: u32);
    
    /// Check if this shape has a text frame
    fn has_text_frame(&self) -> bool {
        false
    }
    
    /// Get the text frame (if available)
    fn text_frame(&self) -> Option<&TextFrame> {
        None
    }
    
    /// Get mutable text frame (if available)
    fn text_frame_mut(&mut self) -> Option<&mut TextFrame> {
        None
    }
    
    /// Check if this shape is a placeholder
    fn is_placeholder(&self) -> bool {
        false
    }
    
    /// Get the hyperlink (if available)
    fn hyperlink(&self) -> Option<&Hyperlink> {
        None
    }
    
    /// Get mutable hyperlink (if available)
    fn hyperlink_mut(&mut self) -> Option<&mut Hyperlink> {
        None
    }
    
    /// Set the hyperlink
    fn set_hyperlink(&mut self, _hyperlink: Option<Hyperlink>) {
        // Default implementation does nothing
        // Shapes that support hyperlinks should override this
    }
    
    /// Get the image part ID (if this is a picture shape)
    fn image_part_id(&self) -> Option<&str> {
        None
    }
}

/// Base shape implementation
pub struct BaseShape {
    id: u32,
    name: String,
    left: i64,
    top: i64,
    width: u32,
    height: u32,
}

impl BaseShape {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            left: 0,
            top: 0,
            width: 914400,  // Default 1 inch
            height: 914400, // Default 1 inch
        }
    }
    
    pub fn with_position(id: u32, name: String, left: i64, top: i64, width: u32, height: u32) -> Self {
        Self {
            id,
            name,
            left,
            top,
            width,
            height,
        }
    }
}

impl Shape for BaseShape {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
    
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    fn left(&self) -> i64 {
        self.left
    }
    
    fn set_left(&mut self, left: i64) {
        self.left = left;
    }
    
    fn top(&self) -> i64 {
        self.top
    }
    
    fn set_top(&mut self, top: i64) {
        self.top = top;
    }
    
    fn width(&self) -> u32 {
        self.width
    }
    
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    
    fn height(&self) -> u32 {
        self.height
    }
    
    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

