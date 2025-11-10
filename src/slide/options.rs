//! Options for slide content (text, shapes, images)
//!
//! Provides simplified, intuitive options for adding content to slides,
//! inspired by PptxGenJS design patterns.

/// Options for adding text to a slide
#[derive(Debug, Clone)]
pub struct TextOptions {
    /// X position in inches
    pub x: f64,
    /// Y position in inches
    pub y: f64,
    /// Width in inches
    pub width: Option<f64>,
    /// Height in inches
    pub height: Option<f64>,
    /// Text color (hex format: "FF0000" or "FFFFFF")
    pub color: String,
    /// Font size in points
    pub font_size: u32,
    /// Font name
    pub font_name: String,
    /// Bold text
    pub bold: bool,
    /// Italic text
    pub italic: bool,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: None,
            height: None,
            color: "000000".to_string(),
            font_size: 12,
            font_name: "Calibri".to_string(),
            bold: false,
            italic: false,
        }
    }
}

impl TextOptions {
    /// Create a new TextOptions with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set position
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Set color
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    /// Set font size
    pub fn font_size(mut self, size: u32) -> Self {
        self.font_size = size;
        self
    }

    /// Set font name
    pub fn font_name(mut self, name: impl Into<String>) -> Self {
        self.font_name = name.into();
        self
    }

    /// Set bold
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Set italic
    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }
}

/// Options for adding shapes to a slide
#[derive(Debug, Clone)]
pub struct ShapeOptions {
    /// X position in inches
    pub x: f64,
    /// Y position in inches
    pub y: f64,
    /// Width in inches
    pub width: f64,
    /// Height in inches
    pub height: f64,
    /// Fill color (hex format)
    pub fill_color: Option<String>,
    /// Line color (hex format)
    pub line_color: Option<String>,
    /// Line width in points
    pub line_width: f64,
}

impl Default for ShapeOptions {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            fill_color: Some("4472C4".to_string()),
            line_color: Some("000000".to_string()),
            line_width: 1.0,
        }
    }
}

impl ShapeOptions {
    /// Create a new ShapeOptions with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set position
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set fill color
    pub fn fill_color(mut self, color: impl Into<String>) -> Self {
        self.fill_color = Some(color.into());
        self
    }

    /// Set line color
    pub fn line_color(mut self, color: impl Into<String>) -> Self {
        self.line_color = Some(color.into());
        self
    }

    /// Set line width
    pub fn line_width(mut self, width: f64) -> Self {
        self.line_width = width;
        self
    }
}

/// Options for adding images to a slide
#[derive(Debug, Clone)]
pub struct ImageOptions {
    /// X position in inches
    pub x: f64,
    /// Y position in inches
    pub y: f64,
    /// Width in inches
    pub width: Option<f64>,
    /// Height in inches
    pub height: Option<f64>,
}

impl Default for ImageOptions {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: None,
            height: None,
        }
    }
}

impl ImageOptions {
    /// Create a new ImageOptions with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set position
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_options_default() {
        let opts = TextOptions::default();
        assert_eq!(opts.x, 0.0);
        assert_eq!(opts.font_size, 12);
        assert_eq!(opts.color, "000000");
    }

    #[test]
    fn test_text_options_builder() {
        let opts = TextOptions::new()
            .position(1.0, 2.0)
            .font_size(24)
            .color("FF0000")
            .bold(true);
        
        assert_eq!(opts.x, 1.0);
        assert_eq!(opts.y, 2.0);
        assert_eq!(opts.font_size, 24);
        assert_eq!(opts.color, "FF0000");
        assert!(opts.bold);
    }

    #[test]
    fn test_shape_options_default() {
        let opts = ShapeOptions::default();
        assert_eq!(opts.width, 1.0);
        assert_eq!(opts.height, 1.0);
        assert_eq!(opts.fill_color, Some("4472C4".to_string()));
    }

    #[test]
    fn test_shape_options_builder() {
        let opts = ShapeOptions::new()
            .position(1.0, 1.0)
            .size(2.0, 1.0)
            .fill_color("FF0000");
        
        assert_eq!(opts.x, 1.0);
        assert_eq!(opts.width, 2.0);
        assert_eq!(opts.fill_color, Some("FF0000".to_string()));
    }

    #[test]
    fn test_image_options_default() {
        let opts = ImageOptions::default();
        assert_eq!(opts.x, 0.0);
        assert_eq!(opts.width, None);
    }
}
