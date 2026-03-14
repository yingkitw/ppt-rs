//! Simplified helper functions for common operations
//!
//! This module provides concise, easy-to-use helper functions that make
//! creating presentations more intuitive and require less boilerplate code.
//!
//! # Examples
//!
//! ```rust
//! use ppt_rs::helpers::*;
//!
//! // Create shapes with simple helpers
//! let rect = rect(0.5, 1.0, 2.0, 1.5)
//!     .fill(hex("4F81BD"))
//!     .text("Hello");
//!
//! // Create images easily
//! let bytes = vec![0u8; 100]; // Example image bytes
//! let img = image(bytes)
//!     .size(914400 * 2, 914400 * 2)  // 2 inches in EMUs
//!     .at(914400, 914400);           // 1 inch in EMUs
//! ```

pub mod colors;
pub mod tables;

use crate::generator::{
    Shape, ShapeType, ShapeFill, ShapeLine,
    ImageBuilder,
    TableBuilder, ChartBuilder, ChartType,
};
use crate::elements::{Color, RgbColor};
use crate::core::Dimension;

// Re-export color utilities
pub use colors::ColorValue;
pub use colors::{
    red, green, blue, yellow, cyan, magenta, white, black, gray, grey,
    light_gray, light_grey, dark_gray, dark_grey, silver,
    orange, purple, pink, brown, navy, teal, olive, maroon, lime, aqua,
    material_red, material_pink, material_purple, material_indigo,
    material_blue, material_cyan, material_teal, material_green,
    material_lime, material_amber, material_orange, material_brown,
    material_gray, material_grey,
    corporate_blue, corporate_green, corporate_red, corporate_orange,
};

// Re-export table utilities
pub use tables::{
    simple_table, table_with_widths, table_from_data, table_with_header,
    QuickTable, cell, header_cell, highlight_cell,
};

// ============================================================================
// Shape Helpers
// ============================================================================

/// Create a rectangle shape at the specified position with the given size.
/// All dimensions are in inches.
///
/// # Example
/// ```
/// use ppt_rs::helpers::rect;
///
/// let shape = rect(0.5, 1.0, 2.0, 1.5);
/// ```
pub fn rect(x: f64, y: f64, width: f64, height: f64) -> Shape {
    Shape::from_dimensions(
        ShapeType::Rectangle,
        Dimension::Inches(x),
        Dimension::Inches(y),
        Dimension::Inches(width),
        Dimension::Inches(height),
    )
}

/// Create a circle shape at the specified position with the given diameter.
/// All dimensions are in inches.
///
/// # Example
/// ```
/// use ppt_rs::helpers::circle;
///
/// let shape = circle(1.0, 1.0, 2.0);
/// ```
pub fn circle(x: f64, y: f64, diameter: f64) -> Shape {
    Shape::from_dimensions(
        ShapeType::Ellipse,
        Dimension::Inches(x),
        Dimension::Inches(y),
        Dimension::Inches(diameter),
        Dimension::Inches(diameter),
    )
}

/// Create an ellipse shape at the specified position with the given size.
/// All dimensions are in inches.
///
/// # Example
/// ```
/// use ppt_rs::helpers::ellipse;
///
/// let shape = ellipse(1.0, 1.0, 3.0, 2.0);
/// ```
pub fn ellipse(x: f64, y: f64, width: f64, height: f64) -> Shape {
    Shape::from_dimensions(
        ShapeType::Ellipse,
        Dimension::Inches(x),
        Dimension::Inches(y),
        Dimension::Inches(width),
        Dimension::Inches(height),
    )
}

/// Create a rounded rectangle shape at the specified position with the given size.
/// All dimensions are in inches.
///
/// # Example
/// ```
/// use ppt_rs::helpers::rounded_rect;
///
/// let shape = rounded_rect(0.5, 1.0, 2.0, 1.5);
/// ```
pub fn rounded_rect(x: f64, y: f64, width: f64, height: f64) -> Shape {
    Shape::from_dimensions(
        ShapeType::RoundedRectangle,
        Dimension::Inches(x),
        Dimension::Inches(y),
        Dimension::Inches(width),
        Dimension::Inches(height),
    )
}

/// Create a triangle shape at the specified position with the given size.
/// All dimensions are in inches.
///
/// # Example
/// ```
/// use ppt_rs::helpers::triangle;
///
/// let shape = triangle(1.0, 1.0, 2.0, 1.5);
/// ```
pub fn triangle(x: f64, y: f64, width: f64, height: f64) -> Shape {
    Shape::from_dimensions(
        ShapeType::Triangle,
        Dimension::Inches(x),
        Dimension::Inches(y),
        Dimension::Inches(width),
        Dimension::Inches(height),
    )
}

/// Create a diamond shape at the specified position with the given size.
/// All dimensions are in inches.
///
/// # Example
/// ```
/// use ppt_rs::helpers::diamond;
///
/// let shape = diamond(1.0, 1.0, 2.0, 2.0);
/// ```
pub fn diamond(x: f64, y: f64, width: f64, height: f64) -> Shape {
    Shape::from_dimensions(
        ShapeType::Diamond,
        Dimension::Inches(x),
        Dimension::Inches(y),
        Dimension::Inches(width),
        Dimension::Inches(height),
    )
}

// ============================================================================
// Image Helpers
// ============================================================================

/// Create an image from raw bytes with automatic format detection.
///
/// # Example
/// ```no_run
/// use ppt_rs::helpers::image;
///
/// let bytes = std::fs::read("photo.jpg").unwrap();
/// let img = image(bytes)
///     .size(914400 * 2, 914400 * 2)  // 2 inches in EMUs
///     .at(914400, 914400);           // 1 inch in EMUs
/// ```
pub fn image<T: Into<Vec<u8>>>(data: T) -> ImageBuilder {
    ImageBuilder::auto(data.into())
}

/// Create an image from a file path.
///
/// # Example
/// ```no_run
/// use ppt_rs::helpers::image_file;
///
/// let img = image_file("photo.jpg").unwrap()
///     .size(914400 * 2, 914400 * 2)  // 2 inches in EMUs
///     .at(914400, 914400);           // 1 inch in EMUs
/// ```
pub fn image_file(path: &str) -> crate::exc::Result<ImageBuilder> {
    let bytes = std::fs::read(path)?;
    Ok(ImageBuilder::auto(bytes))
}

// ============================================================================
// Color Helpers
// ============================================================================

/// Create an RGB color from red, green, and blue components (0-255).
///
/// # Example
/// ```
/// use ppt_rs::helpers::rgb;
///
/// let color = rgb(79, 129, 189); // Blue
/// ```
pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb(RgbColor::new(r, g, b))
}

/// Create a color from a hex string (with or without '#' prefix).
///
/// # Example
/// ```
/// use ppt_rs::helpers::hex;
///
/// let color1 = hex("4F81BD");
/// let color2 = hex("#4F81BD"); // Also works
/// ```
pub fn hex(color: &str) -> Color {
    let color = color.trim_start_matches('#');
    if color.len() == 6 {
        let r = u8::from_str_radix(&color[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&color[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&color[4..6], 16).unwrap_or(0);
        Color::Rgb(RgbColor::new(r, g, b))
    } else {
        Color::Rgb(RgbColor::new(0, 0, 0)) // Default to black
    }
}

// ============================================================================
// Table Helpers
// ============================================================================

/// Create a table builder with the specified column widths.
///
/// # Example
/// ```
/// use ppt_rs::helpers::table;
///
/// let tbl = table(vec![2000000, 2000000, 2000000]); // 3 columns with equal widths
/// ```
pub fn table(column_widths: Vec<u32>) -> TableBuilder {
    TableBuilder::new(column_widths)
}

// ============================================================================
// Chart Helpers
// ============================================================================

/// Create a bar chart builder.
///
/// # Example
/// ```
/// use ppt_rs::helpers::bar_chart;
///
/// let chart = bar_chart("Sales Data");
/// ```
pub fn bar_chart(title: &str) -> ChartBuilder {
    ChartBuilder::new(title, ChartType::Bar)
}

/// Create a line chart builder.
///
/// # Example
/// ```
/// use ppt_rs::helpers::line_chart;
///
/// let chart = line_chart("Trends");
/// ```
pub fn line_chart(title: &str) -> ChartBuilder {
    ChartBuilder::new(title, ChartType::Line)
}

/// Create a pie chart builder.
///
/// # Example
/// ```
/// use ppt_rs::helpers::pie_chart;
///
/// let chart = pie_chart("Distribution");
/// ```
pub fn pie_chart(title: &str) -> ChartBuilder {
    ChartBuilder::new(title, ChartType::Pie)
}

/// Create an area chart builder.
///
/// # Example
/// ```
/// use ppt_rs::helpers::area_chart;
///
/// let chart = area_chart("Growth");
/// ```
pub fn area_chart(title: &str) -> ChartBuilder {
    ChartBuilder::new(title, ChartType::Area)
}

// ============================================================================
// Extension Traits for Fluent API
// ============================================================================

/// Extension trait for shapes to provide shorter method names
pub trait ShapeExt {
    /// Set the fill color (shorter alias for `with_fill`)
    fn fill(self, color: Color) -> Self;
    
    /// Set the stroke/line (shorter alias for `with_line`)
    fn stroke(self, color: Color, width_pt: f64) -> Self;
    
    /// Set the text (shorter alias for `with_text`)
    fn text(self, text: &str) -> Self;
}

impl ShapeExt for Shape {
    fn fill(self, color: Color) -> Self {
        let color_str = match color {
            Color::Rgb(rgb) => format!("{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b),
            _ => "000000".to_string(),
        };
        self.with_fill(ShapeFill::new(&color_str))
    }
    
    fn stroke(self, color: Color, width_pt: f64) -> Self {
        let color_str = match color {
            Color::Rgb(rgb) => format!("{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b),
            _ => "000000".to_string(),
        };
        let width_emu = (width_pt * 12700.0) as u32; // Convert points to EMU
        self.with_line(ShapeLine::new(&color_str, width_emu))
    }
    
    fn text(self, text: &str) -> Self {
        self.with_text(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Positioned, ElementSized};

    #[test]
    fn test_rect_helper() {
        let shape = rect(0.5, 1.0, 2.0, 1.5);
        assert_eq!(shape.x(), (0.5 * 914400.0) as u32);
        assert_eq!(shape.y(), (1.0 * 914400.0) as u32);
    }

    #[test]
    fn test_circle_helper() {
        let shape = circle(2.0, 3.0, 1.5);
        assert_eq!(shape.width(), shape.height()); // Circle is square
    }

    #[test]
    fn test_rgb_helper() {
        let color = rgb(79, 129, 189);
        match color {
            Color::Rgb(rgb) => {
                assert_eq!(rgb.r, 79);
                assert_eq!(rgb.g, 129);
                assert_eq!(rgb.b, 189);
            }
            _ => panic!("Expected RGB color"),
        }
    }

    #[test]
    fn test_hex_helper() {
        let color1 = hex("4F81BD");
        let color2 = hex("#4F81BD");
        
        match (color1, color2) {
            (Color::Rgb(rgb1), Color::Rgb(rgb2)) => {
                assert_eq!(rgb1.r, 79);
                assert_eq!(rgb1.g, 129);
                assert_eq!(rgb1.b, 189);
                assert_eq!(rgb1.r, rgb2.r);
                assert_eq!(rgb1.g, rgb2.g);
                assert_eq!(rgb1.b, rgb2.b);
            }
            _ => panic!("Expected RGB colors"),
        }
    }

    #[test]
    fn test_shape_ext() {
        let shape = rect(0.5, 1.0, 2.0, 1.5)
            .fill(rgb(79, 129, 189))
            .text("Hello");
        
        assert!(shape.text.is_some());
        assert_eq!(shape.text.unwrap(), "Hello");
    }
}
