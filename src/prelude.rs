//! Prelude module for easy imports
//!
//! This module provides a simplified API for common use cases.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use ppt_rs::prelude::*;
//! use ppt_rs::pptx;
//!
//! // Create a simple presentation
//! let pptx_data = pptx!("My Presentation")
//!     .slide("Welcome", &["Point 1", "Point 2"])
//!     .slide("Details", &["More info"])
//!     .build()
//!     .unwrap();
//!
//! std::fs::write("output.pptx", pptx_data).unwrap();
//! ```

// Re-export commonly used types
pub use crate::generator::{
    SlideContent, SlideLayout,
    Shape, ShapeType, ShapeFill, ShapeLine,
    Image,
    Connector, ConnectorType, ArrowType,
    create_pptx, create_pptx_with_content,
    BulletStyle, BulletPoint,
    TextFormat, FormattedText,
};

pub use crate::generator::shapes::{
    GradientFill, GradientDirection, GradientStop,
};

pub use crate::elements::{Color, RgbColor, Position, Size};
pub use crate::core::{Dimension, FlexPosition, FlexSize};
pub use crate::exc::Result;

/// Font size module with common presets (in points)
/// 
/// These values can be used directly with `content_size()` and `title_size()`:
/// ```
/// use ppt_rs::prelude::{SlideContent, font_sizes};
/// 
/// let slide = SlideContent::new("Title")
///     .title_size(font_sizes::TITLE)
///     .content_size(font_sizes::BODY);
/// 
/// assert_eq!(font_sizes::TITLE, 44);
/// assert_eq!(font_sizes::BODY, 18);
/// ```
pub mod font_sizes {
    /// Title font size (44pt)
    pub const TITLE: u32 = 44;
    /// Subtitle font size (32pt)
    pub const SUBTITLE: u32 = 32;
    /// Heading font size (28pt)
    pub const HEADING: u32 = 28;
    /// Body font size (18pt)
    pub const BODY: u32 = 18;
    /// Small font size (14pt)
    pub const SMALL: u32 = 14;
    /// Caption font size (12pt)
    pub const CAPTION: u32 = 12;
    /// Code font size (14pt)
    pub const CODE: u32 = 14;
    /// Large font size (36pt)
    pub const LARGE: u32 = 36;
    /// Extra large font size (48pt)
    pub const XLARGE: u32 = 48;
    
    /// Convert points to OOXML size units (hundredths of a point)
    pub fn to_emu(pt: u32) -> u32 {
        pt * 100
    }
}

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
    
    /// Create a rectangle at EMU coordinates
    pub fn rect_emu(x: u32, y: u32, width: u32, height: u32) -> Shape {
        Shape::new(ShapeType::Rectangle, x, y, width, height)
    }
    
    /// Create a circle
    pub fn circle(x: f64, y: f64, diameter: f64) -> Shape {
        Shape::new(ShapeType::Circle, inches(x), inches(y), inches(diameter), inches(diameter))
    }
    
    /// Create a circle at EMU coordinates
    pub fn circle_emu(x: u32, y: u32, diameter: u32) -> Shape {
        Shape::new(ShapeType::Circle, x, y, diameter, diameter)
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
    
    /// Create an arrow shape pointing right
    pub fn arrow_right(x: f64, y: f64, width: f64, height: f64) -> Shape {
        Shape::new(ShapeType::RightArrow, inches(x), inches(y), inches(width), inches(height))
    }
    
    /// Create an arrow shape pointing left
    pub fn arrow_left(x: f64, y: f64, width: f64, height: f64) -> Shape {
        Shape::new(ShapeType::LeftArrow, inches(x), inches(y), inches(width), inches(height))
    }
    
    /// Create an arrow shape pointing up
    pub fn arrow_up(x: f64, y: f64, width: f64, height: f64) -> Shape {
        Shape::new(ShapeType::UpArrow, inches(x), inches(y), inches(width), inches(height))
    }
    
    /// Create an arrow shape pointing down
    pub fn arrow_down(x: f64, y: f64, width: f64, height: f64) -> Shape {
        Shape::new(ShapeType::DownArrow, inches(x), inches(y), inches(width), inches(height))
    }
    
    /// Create a diamond shape
    pub fn diamond(x: f64, y: f64, size: f64) -> Shape {
        Shape::new(ShapeType::Diamond, inches(x), inches(y), inches(size), inches(size))
    }
    
    /// Create a triangle shape
    pub fn triangle(x: f64, y: f64, width: f64, height: f64) -> Shape {
        Shape::new(ShapeType::Triangle, inches(x), inches(y), inches(width), inches(height))
    }
    
    /// Create a star shape (5-pointed)
    pub fn star(x: f64, y: f64, size: f64) -> Shape {
        Shape::new(ShapeType::Star5, inches(x), inches(y), inches(size), inches(size))
    }
    
    /// Create a heart shape
    pub fn heart(x: f64, y: f64, size: f64) -> Shape {
        Shape::new(ShapeType::Heart, inches(x), inches(y), inches(size), inches(size))
    }
    
    /// Create a cloud shape
    pub fn cloud(x: f64, y: f64, width: f64, height: f64) -> Shape {
        Shape::new(ShapeType::Cloud, inches(x), inches(y), inches(width), inches(height))
    }
    
    /// Create a callout shape with text
    pub fn callout(x: f64, y: f64, width: f64, height: f64, text: &str) -> Shape {
        Shape::new(ShapeType::WedgeRectCallout, inches(x), inches(y), inches(width), inches(height))
            .with_text(text)
    }
    
    /// Create a badge (colored rounded rectangle with text)
    pub fn badge(x: f64, y: f64, text: &str, fill_color: &str) -> Shape {
        Shape::new(ShapeType::RoundedRectangle, inches(x), inches(y), inches(1.5), inches(0.4))
            .with_fill(ShapeFill::new(fill_color))
            .with_text(text)
    }
    
    /// Create a process box (flowchart)
    pub fn process(x: f64, y: f64, width: f64, height: f64, text: &str) -> Shape {
        Shape::new(ShapeType::FlowChartProcess, inches(x), inches(y), inches(width), inches(height))
            .with_text(text)
    }
    
    /// Create a decision diamond (flowchart)
    pub fn decision(x: f64, y: f64, size: f64, text: &str) -> Shape {
        Shape::new(ShapeType::FlowChartDecision, inches(x), inches(y), inches(size), inches(size))
            .with_text(text)
    }
    
    /// Create a document shape (flowchart)
    pub fn document(x: f64, y: f64, width: f64, height: f64, text: &str) -> Shape {
        Shape::new(ShapeType::FlowChartDocument, inches(x), inches(y), inches(width), inches(height))
            .with_text(text)
    }
    
    /// Create a data shape (parallelogram, flowchart)
    pub fn data(x: f64, y: f64, width: f64, height: f64, text: &str) -> Shape {
        Shape::new(ShapeType::FlowChartData, inches(x), inches(y), inches(width), inches(height))
            .with_text(text)
    }
    
    /// Create a terminator shape (flowchart)
    pub fn terminator(x: f64, y: f64, width: f64, height: f64, text: &str) -> Shape {
        Shape::new(ShapeType::FlowChartTerminator, inches(x), inches(y), inches(width), inches(height))
            .with_text(text)
    }

    /// Create any shape using flexible Dimension units.
    ///
    /// ```
    /// use ppt_rs::prelude::*;
    ///
    /// // 10% from left, 20% from top, 80% wide, 2 inches tall
    /// let shape = shapes::dim(
    ///     ShapeType::Rectangle,
    ///     Dimension::Ratio(0.1), Dimension::Ratio(0.2),
    ///     Dimension::Ratio(0.8), Dimension::Inches(2.0),
    /// );
    /// ```
    pub fn dim(shape_type: ShapeType, x: Dimension, y: Dimension, width: Dimension, height: Dimension) -> Shape {
        Shape::from_dimensions(shape_type, x, y, width, height)
    }

    /// Create a rectangle using ratio (0.0–1.0) of slide dimensions.
    ///
    /// ```
    /// use ppt_rs::prelude::*;
    ///
    /// // Centered rectangle: 10% margin on each side, 20% from top, 30% tall
    /// let shape = shapes::rect_ratio(0.1, 0.2, 0.8, 0.3);
    /// ```
    pub fn rect_ratio(x: f64, y: f64, width: f64, height: f64) -> Shape {
        Shape::from_dimensions(
            ShapeType::Rectangle,
            Dimension::Ratio(x), Dimension::Ratio(y),
            Dimension::Ratio(width), Dimension::Ratio(height),
        )
    }

    /// Create a text box using ratio (0.0–1.0) of slide dimensions.
    pub fn text_box_ratio(x: f64, y: f64, width: f64, height: f64, text: &str) -> Shape {
        rect_ratio(x, y, width, height).with_text(text)
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
    
    // Material Design colors
    pub const MATERIAL_RED: &str = "F44336";
    pub const MATERIAL_PINK: &str = "E91E63";
    pub const MATERIAL_PURPLE: &str = "9C27B0";
    pub const MATERIAL_INDIGO: &str = "3F51B5";
    pub const MATERIAL_BLUE: &str = "2196F3";
    pub const MATERIAL_CYAN: &str = "00BCD4";
    pub const MATERIAL_TEAL: &str = "009688";
    pub const MATERIAL_GREEN: &str = "4CAF50";
    pub const MATERIAL_LIME: &str = "CDDC39";
    pub const MATERIAL_AMBER: &str = "FFC107";
    pub const MATERIAL_ORANGE: &str = "FF9800";
    pub const MATERIAL_BROWN: &str = "795548";
    pub const MATERIAL_GRAY: &str = "9E9E9E";
    
    // IBM Carbon Design colors
    pub const CARBON_BLUE_60: &str = "0043CE";
    pub const CARBON_BLUE_40: &str = "4589FF";
    pub const CARBON_GRAY_100: &str = "161616";
    pub const CARBON_GRAY_80: &str = "393939";
    pub const CARBON_GRAY_20: &str = "E0E0E0";
    pub const CARBON_GREEN_50: &str = "24A148";
    pub const CARBON_RED_60: &str = "DA1E28";
    pub const CARBON_PURPLE_60: &str = "8A3FFC";
}

/// Theme presets for presentations
pub mod themes {
    /// Theme definition with color palette
    #[derive(Debug, Clone)]
    pub struct Theme {
        pub name: &'static str,
        pub primary: &'static str,
        pub secondary: &'static str,
        pub accent: &'static str,
        pub background: &'static str,
        pub text: &'static str,
        pub light: &'static str,
        pub dark: &'static str,
    }

    /// Corporate blue theme - Professional and trustworthy
    pub const CORPORATE: Theme = Theme {
        name: "Corporate",
        primary: "1565C0",
        secondary: "1976D2",
        accent: "FF6F00",
        background: "FFFFFF",
        text: "212121",
        light: "E3F2FD",
        dark: "0D47A1",
    };

    /// Modern minimalist theme - Clean and simple
    pub const MODERN: Theme = Theme {
        name: "Modern",
        primary: "212121",
        secondary: "757575",
        accent: "00BCD4",
        background: "FAFAFA",
        text: "212121",
        light: "F5F5F5",
        dark: "424242",
    };

    /// Vibrant creative theme - Bold and colorful
    pub const VIBRANT: Theme = Theme {
        name: "Vibrant",
        primary: "E91E63",
        secondary: "9C27B0",
        accent: "FF9800",
        background: "FFFFFF",
        text: "212121",
        light: "FCE4EC",
        dark: "880E4F",
    };

    /// Dark mode theme - Easy on the eyes
    pub const DARK: Theme = Theme {
        name: "Dark",
        primary: "BB86FC",
        secondary: "03DAC6",
        accent: "CF6679",
        background: "121212",
        text: "FFFFFF",
        light: "1E1E1E",
        dark: "000000",
    };

    /// Nature green theme - Fresh and organic
    pub const NATURE: Theme = Theme {
        name: "Nature",
        primary: "2E7D32",
        secondary: "4CAF50",
        accent: "8BC34A",
        background: "FFFFFF",
        text: "1B5E20",
        light: "E8F5E9",
        dark: "1B5E20",
    };

    /// Tech blue theme - Modern technology feel
    pub const TECH: Theme = Theme {
        name: "Tech",
        primary: "0D47A1",
        secondary: "1976D2",
        accent: "00E676",
        background: "FAFAFA",
        text: "263238",
        light: "E3F2FD",
        dark: "01579B",
    };

    /// Carbon Design theme - IBM's design system
    pub const CARBON: Theme = Theme {
        name: "Carbon",
        primary: "0043CE",
        secondary: "4589FF",
        accent: "24A148",
        background: "FFFFFF",
        text: "161616",
        light: "E0E0E0",
        dark: "161616",
    };

    /// Get all available themes
    pub fn all() -> Vec<Theme> {
        vec![CORPORATE, MODERN, VIBRANT, DARK, NATURE, TECH, CARBON]
    }
}

/// Layout helpers for positioning shapes
pub mod layouts {
    /// Slide dimensions in EMU
    pub const SLIDE_WIDTH: u32 = 9144000;   // 10 inches
    pub const SLIDE_HEIGHT: u32 = 6858000;  // 7.5 inches
    
    /// Common margins
    pub const MARGIN: u32 = 457200;          // 0.5 inch
    pub const MARGIN_SMALL: u32 = 228600;    // 0.25 inch
    pub const MARGIN_LARGE: u32 = 914400;    // 1 inch

    /// Center a shape on the slide (horizontal)
    pub fn center_x(shape_width: u32) -> u32 {
        (SLIDE_WIDTH - shape_width) / 2
    }

    /// Center a shape on the slide (vertical)
    pub fn center_y(shape_height: u32) -> u32 {
        (SLIDE_HEIGHT - shape_height) / 2
    }

    /// Get position for centering a shape both horizontally and vertically
    pub fn center(shape_width: u32, shape_height: u32) -> (u32, u32) {
        (center_x(shape_width), center_y(shape_height))
    }

    /// Calculate grid positions for arranging shapes
    /// Returns Vec of (x, y) positions
    pub fn grid(rows: usize, cols: usize, cell_width: u32, cell_height: u32) -> Vec<(u32, u32)> {
        let mut positions = Vec::new();
        let total_width = cell_width * cols as u32;
        let total_height = cell_height * rows as u32;
        let start_x = center_x(total_width);
        let start_y = center_y(total_height);
        
        for row in 0..rows {
            for col in 0..cols {
                let x = start_x + (col as u32 * cell_width);
                let y = start_y + (row as u32 * cell_height);
                positions.push((x, y));
            }
        }
        positions
    }

    /// Calculate positions for a horizontal stack of shapes
    pub fn stack_horizontal(count: usize, shape_width: u32, spacing: u32, y: u32) -> Vec<(u32, u32)> {
        let total_width = (shape_width * count as u32) + (spacing * (count - 1) as u32);
        let start_x = center_x(total_width);
        
        (0..count)
            .map(|i| (start_x + (i as u32 * (shape_width + spacing)), y))
            .collect()
    }

    /// Calculate positions for a vertical stack of shapes
    pub fn stack_vertical(count: usize, shape_height: u32, spacing: u32, x: u32) -> Vec<(u32, u32)> {
        let total_height = (shape_height * count as u32) + (spacing * (count - 1) as u32);
        let start_y = center_y(total_height);
        
        (0..count)
            .map(|i| (x, start_y + (i as u32 * (shape_height + spacing))))
            .collect()
    }

    /// Calculate positions to evenly distribute shapes across slide width
    pub fn distribute_horizontal(count: usize, shape_width: u32, y: u32) -> Vec<(u32, u32)> {
        if count == 0 {
            return vec![];
        }
        if count == 1 {
            return vec![(center_x(shape_width), y)];
        }
        
        let usable_width = SLIDE_WIDTH - (2 * MARGIN);
        let spacing = (usable_width - (shape_width * count as u32)) / (count as u32 - 1);
        
        (0..count)
            .map(|i| (MARGIN + (i as u32 * (shape_width + spacing)), y))
            .collect()
    }
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
    
    #[test]
    fn test_arrow_shapes() {
        let arrow = shapes::arrow_right(1.0, 1.0, 2.0, 1.0);
        assert_eq!(arrow.width, inches(2.0));
        
        let up = shapes::arrow_up(1.0, 1.0, 1.0, 2.0);
        assert_eq!(up.height, inches(2.0));
    }
    
    #[test]
    fn test_flowchart_shapes() {
        let process = shapes::process(1.0, 1.0, 2.0, 1.0, "Process");
        assert!(process.text.is_some());
        
        let decision = shapes::decision(1.0, 1.0, 1.5, "Yes/No");
        assert!(decision.text.is_some());
    }
    
    #[test]
    fn test_badge_shape() {
        let badge = shapes::badge(1.0, 1.0, "NEW", colors::MATERIAL_GREEN);
        assert!(badge.text.is_some());
        assert!(badge.fill.is_some());
    }
    
    #[test]
    fn test_themes() {
        let all_themes = crate::prelude::themes::all();
        assert_eq!(all_themes.len(), 7);
        
        assert_eq!(crate::prelude::themes::CORPORATE.name, "Corporate");
        assert_eq!(crate::prelude::themes::DARK.background, "121212");
    }
    
    #[test]
    fn test_layouts_center() {
        let (x, y) = crate::prelude::layouts::center(1000000, 500000);
        assert!(x > 0);
        assert!(y > 0);
        
        // Should be centered
        assert_eq!(x, (crate::prelude::layouts::SLIDE_WIDTH - 1000000) / 2);
        assert_eq!(y, (crate::prelude::layouts::SLIDE_HEIGHT - 500000) / 2);
    }
    
    #[test]
    fn test_layouts_grid() {
        let positions = crate::prelude::layouts::grid(2, 3, 1000000, 800000);
        assert_eq!(positions.len(), 6);
    }
    
    #[test]
    fn test_layouts_stack_horizontal() {
        let positions = crate::prelude::layouts::stack_horizontal(4, 500000, 100000, 2000000);
        assert_eq!(positions.len(), 4);
        
        // Check that positions are evenly spaced
        for i in 1..positions.len() {
            let diff = positions[i].0 - positions[i-1].0;
            assert_eq!(diff, 600000); // shape_width + spacing
        }
    }
    
    #[test]
    fn test_layouts_distribute_horizontal() {
        let positions = crate::prelude::layouts::distribute_horizontal(3, 500000, 2000000);
        assert_eq!(positions.len(), 3);
    }
    
    #[test]
    fn test_material_colors() {
        assert_eq!(colors::MATERIAL_RED, "F44336");
        assert_eq!(colors::MATERIAL_BLUE, "2196F3");
        assert_eq!(colors::MATERIAL_GREEN, "4CAF50");
    }
    
    #[test]
    fn test_carbon_colors() {
        assert_eq!(colors::CARBON_BLUE_60, "0043CE");
        assert_eq!(colors::CARBON_GRAY_100, "161616");
    }
}
