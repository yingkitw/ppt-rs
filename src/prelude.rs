//! Prelude module for easy imports
//!
//! This module provides a simplified API for common use cases.
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use ppt_rs::prelude::*;
//!
//! // Create a simple presentation
//! let pptx = pptx!("My Presentation")
//!     .slide("Welcome", &["Point 1", "Point 2"])
//!     .slide("Details", &["More info"])
//!     .build()
//!     .unwrap();
//!
//! std::fs::write("output.pptx", pptx).unwrap();
//! ```

// Re-export commonly used types
pub use crate::generator::{
    SlideContent, SlideLayout,
    Shape, ShapeType, ShapeFill, ShapeLine,
    Image,
    Connector, ConnectorType, ArrowType,
    create_pptx, create_pptx_with_content,
};

pub use crate::generator::shapes::{
    GradientFill, GradientDirection, GradientStop,
};

pub use crate::elements::{Color, RgbColor, Position, Size};
pub use crate::exc::Result;

/// Quick presentation builder macro
#[macro_export]
macro_rules! pptx {
    ($title:expr) => {
        $crate::prelude::QuickPptx::new($title)
    };
}

/// Quick shape creation
#[macro_export]
macro_rules! shape {
    // Rectangle with position and size (in inches)
    (rect $x:expr, $y:expr, $w:expr, $h:expr) => {
        $crate::prelude::Shape::new(
            $crate::prelude::ShapeType::Rectangle,
            $crate::prelude::inches($x),
            $crate::prelude::inches($y),
            $crate::prelude::inches($w),
            $crate::prelude::inches($h),
        )
    };
    // Circle with position and size (in inches)
    (circle $x:expr, $y:expr, $size:expr) => {
        $crate::prelude::Shape::new(
            $crate::prelude::ShapeType::Circle,
            $crate::prelude::inches($x),
            $crate::prelude::inches($y),
            $crate::prelude::inches($size),
            $crate::prelude::inches($size),
        )
    };
}

/// Convert inches to EMU (English Metric Units)
pub fn inches(val: f64) -> u32 {
    (val * 914400.0) as u32
}

/// Convert centimeters to EMU
pub fn cm(val: f64) -> u32 {
    (val * 360000.0) as u32
}

/// Convert points to EMU
pub fn pt(val: f64) -> u32 {
    (val * 12700.0) as u32
}

/// Quick presentation builder for simple use cases
pub struct QuickPptx {
    title: String,
    slides: Vec<SlideContent>,
}

impl QuickPptx {
    /// Create a new presentation with a title
    pub fn new(title: &str) -> Self {
        QuickPptx {
            title: title.to_string(),
            slides: Vec::new(),
        }
    }
    
    /// Add a slide with title and bullet points
    pub fn slide(mut self, title: &str, bullets: &[&str]) -> Self {
        let mut slide = SlideContent::new(title);
        for bullet in bullets {
            slide = slide.add_bullet(*bullet);
        }
        self.slides.push(slide);
        self
    }
    
    /// Add a slide with just a title
    pub fn title_slide(mut self, title: &str) -> Self {
        self.slides.push(SlideContent::new(title));
        self
    }
    
    /// Add a slide with title and custom content
    pub fn content_slide(mut self, slide: SlideContent) -> Self {
        self.slides.push(slide);
        self
    }
    
    /// Add a slide with shapes
    pub fn shapes_slide(mut self, title: &str, shapes: Vec<Shape>) -> Self {
        let slide = SlideContent::new(title).with_shapes(shapes);
        self.slides.push(slide);
        self
    }
    
    /// Build the presentation and return the PPTX data
    pub fn build(self) -> std::result::Result<Vec<u8>, Box<dyn std::error::Error>> {
        if self.slides.is_empty() {
            // Create at least one slide
            create_pptx(&self.title, 1)
        } else {
            create_pptx_with_content(&self.title, self.slides)
        }
    }
    
    /// Build and save to a file
    pub fn save(self, path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let data = self.build()?;
        std::fs::write(path, data)?;
        Ok(())
    }
}


/// Quick shape builders
pub mod shapes {
    use super::*;
    
    /// Create a rectangle
    pub fn rect(x: f64, y: f64, width: f64, height: f64) -> Shape {
        Shape::new(ShapeType::Rectangle, inches(x), inches(y), inches(width), inches(height))
    }
    
    /// Create a circle
    pub fn circle(x: f64, y: f64, diameter: f64) -> Shape {
        Shape::new(ShapeType::Circle, inches(x), inches(y), inches(diameter), inches(diameter))
    }
    
    /// Create a rounded rectangle
    pub fn rounded_rect(x: f64, y: f64, width: f64, height: f64) -> Shape {
        Shape::new(ShapeType::RoundedRectangle, inches(x), inches(y), inches(width), inches(height))
    }
    
    /// Create a text box (rectangle with text)
    pub fn text_box(x: f64, y: f64, width: f64, height: f64, text: &str) -> Shape {
        Shape::new(ShapeType::Rectangle, inches(x), inches(y), inches(width), inches(height))
            .with_text(text)
    }
    
    /// Create a colored shape
    pub fn colored(shape: Shape, fill: &str, line: Option<&str>) -> Shape {
        let mut s = shape.with_fill(ShapeFill::new(fill));
        if let Some(l) = line {
            s = s.with_line(ShapeLine::new(l, 12700));
        }
        s
    }
    
    /// Create a gradient shape
    pub fn gradient(shape: Shape, start: &str, end: &str, direction: GradientDirection) -> Shape {
        shape.with_gradient(GradientFill::linear(start, end, direction))
    }
}

/// Color constants for convenience
pub mod colors {
    pub const RED: &str = "FF0000";
    pub const GREEN: &str = "00FF00";
    pub const BLUE: &str = "0000FF";
    pub const WHITE: &str = "FFFFFF";
    pub const BLACK: &str = "000000";
    pub const GRAY: &str = "808080";
    pub const LIGHT_GRAY: &str = "D3D3D3";
    pub const DARK_GRAY: &str = "404040";
    pub const YELLOW: &str = "FFFF00";
    pub const ORANGE: &str = "FFA500";
    pub const PURPLE: &str = "800080";
    pub const CYAN: &str = "00FFFF";
    pub const MAGENTA: &str = "FF00FF";
    pub const NAVY: &str = "000080";
    pub const TEAL: &str = "008080";
    pub const OLIVE: &str = "808000";
    
    // Corporate colors
    pub const CORPORATE_BLUE: &str = "1565C0";
    pub const CORPORATE_GREEN: &str = "2E7D32";
    pub const CORPORATE_RED: &str = "C62828";
    pub const CORPORATE_ORANGE: &str = "EF6C00";
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quick_pptx() {
        let result = QuickPptx::new("Test")
            .slide("Slide 1", &["Point 1", "Point 2"])
            .build();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_inches_conversion() {
        assert_eq!(inches(1.0), 914400);
        assert_eq!(cm(2.54), 914400); // 1 inch = 2.54 cm
    }
    
    #[test]
    fn test_shape_builders() {
        let rect = shapes::rect(1.0, 1.0, 2.0, 1.0);
        assert_eq!(rect.width, inches(2.0));
        
        let circle = shapes::circle(1.0, 1.0, 1.0);
        assert_eq!(circle.width, circle.height);
    }
}
