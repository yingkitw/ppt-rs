//! Master slide customization for presentation templates
//!
//! Provides comprehensive master slide customization including:
//! - Custom layouts
//! - Theme colors and fonts
//! - Placeholder positioning
//! - Background customization
//! - Text styles

use crate::dml::color::RGBColor;

/// Master slide customization
#[derive(Clone, Debug)]
pub struct MasterSlideCustomization {
    /// Master slide name
    name: String,
    /// Custom layouts
    layouts: Vec<CustomLayout>,
    /// Theme colors
    theme_colors: Vec<RGBColor>,
    /// Theme fonts (major, minor)
    theme_fonts: (String, String),
    /// Background color
    background_color: Option<RGBColor>,
    /// Placeholder styles
    placeholder_styles: Vec<PlaceholderStyle>,
}

/// Custom layout for master slide
#[derive(Clone, Debug)]
pub struct CustomLayout {
    /// Layout name
    name: String,
    /// Layout ID
    id: u32,
    /// Layout type
    layout_type: LayoutType,
    /// Placeholder positions
    placeholders: Vec<PlaceholderPosition>,
}

/// Layout type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LayoutType {
    /// Title slide layout
    TitleSlide,
    /// Title and content layout
    TitleAndContent,
    /// Blank layout
    Blank,
    /// Title only layout
    TitleOnly,
    /// Custom layout
    Custom(String),
}

/// Placeholder position on layout
#[derive(Clone, Debug)]
pub struct PlaceholderPosition {
    /// Placeholder type
    placeholder_type: String,
    /// X position in EMU
    x: i32,
    /// Y position in EMU
    y: i32,
    /// Width in EMU
    width: u32,
    /// Height in EMU
    height: u32,
}

/// Placeholder style
#[derive(Clone, Debug)]
pub struct PlaceholderStyle {
    /// Placeholder type
    placeholder_type: String,
    /// Font name
    font_name: String,
    /// Font size in points
    font_size: u32,
    /// Font color
    color: RGBColor,
    /// Bold
    bold: bool,
    /// Italic
    italic: bool,
}

impl MasterSlideCustomization {
    /// Create a new master slide customization
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            layouts: Vec::new(),
            theme_colors: vec![
                RGBColor::new(0, 0, 0),     // Dark 1
                RGBColor::new(255, 255, 255), // Light 1
                RGBColor::new(68, 114, 196),  // Accent 1
                RGBColor::new(237, 125, 49),  // Accent 2
                RGBColor::new(165, 165, 165), // Accent 3
                RGBColor::new(112, 48, 160),  // Accent 4
                RGBColor::new(37, 150, 185),  // Accent 5
                RGBColor::new(0, 176, 80),    // Accent 6
            ],
            theme_fonts: ("Calibri".to_string(), "Calibri".to_string()),
            background_color: None,
            placeholder_styles: Vec::new(),
        }
    }

    /// Get master slide name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set master slide name
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Add a custom layout
    pub fn add_layout(&mut self, layout: CustomLayout) -> usize {
        self.layouts.push(layout);
        self.layouts.len() - 1
    }

    /// Get layout by index
    pub fn get_layout(&self, index: usize) -> Option<&CustomLayout> {
        self.layouts.get(index)
    }

    /// Get all layouts
    pub fn layouts(&self) -> &[CustomLayout] {
        &self.layouts
    }

    /// Set theme colors
    pub fn set_theme_colors(&mut self, colors: Vec<RGBColor>) {
        if colors.len() == 8 {
            self.theme_colors = colors;
        }
    }

    /// Get theme colors
    pub fn theme_colors(&self) -> &[RGBColor] {
        &self.theme_colors
    }

    /// Set theme fonts (major, minor)
    pub fn set_theme_fonts(&mut self, major: impl Into<String>, minor: impl Into<String>) {
        self.theme_fonts = (major.into(), minor.into());
    }

    /// Get theme fonts
    pub fn theme_fonts(&self) -> (&str, &str) {
        (&self.theme_fonts.0, &self.theme_fonts.1)
    }

    /// Set background color
    pub fn set_background_color(&mut self, color: RGBColor) {
        self.background_color = Some(color);
    }

    /// Get background color
    pub fn background_color(&self) -> Option<&RGBColor> {
        self.background_color.as_ref()
    }

    /// Add placeholder style
    pub fn add_placeholder_style(&mut self, style: PlaceholderStyle) {
        self.placeholder_styles.push(style);
    }

    /// Get placeholder styles
    pub fn placeholder_styles(&self) -> &[PlaceholderStyle] {
        &self.placeholder_styles
    }

    /// Generate XML for master slide
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
        xml.push_str(r#"<p:sldMaster xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#);
        
        // Background
        if let Some(color) = &self.background_color {
            xml.push_str(&format!(
                r#"<p:cSld><p:bg><p:bgPr><a:solidFill><a:srgbClr val="{}"/></a:solidFill><a:effectLst/></p:bgPr></p:bg>"#,
                color.to_hex()
            ));
        }
        
        // Layouts
        xml.push_str(r#"<p:sldLayoutIdLst>"#);
        for (idx, layout) in self.layouts.iter().enumerate() {
            xml.push_str(&format!(
                r#"<p:sldLayoutId id="{}" r:id="rId{}"/>"#,
                layout.id,
                idx + 2
            ));
        }
        xml.push_str(r#"</p:sldLayoutIdLst>"#);
        
        xml.push_str(r#"</p:sldMaster>"#);
        xml
    }
}

impl CustomLayout {
    /// Create a new custom layout
    pub fn new(name: impl Into<String>, id: u32, layout_type: LayoutType) -> Self {
        Self {
            name: name.into(),
            id,
            layout_type,
            placeholders: Vec::new(),
        }
    }

    /// Get layout name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get layout ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get layout type
    pub fn layout_type(&self) -> &LayoutType {
        &self.layout_type
    }

    /// Add placeholder position
    pub fn add_placeholder(&mut self, placeholder: PlaceholderPosition) {
        self.placeholders.push(placeholder);
    }

    /// Get placeholders
    pub fn placeholders(&self) -> &[PlaceholderPosition] {
        &self.placeholders
    }
}

impl PlaceholderPosition {
    /// Create a new placeholder position
    pub fn new(
        placeholder_type: impl Into<String>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            placeholder_type: placeholder_type.into(),
            x,
            y,
            width,
            height,
        }
    }

    /// Get placeholder type
    pub fn placeholder_type(&self) -> &str {
        &self.placeholder_type
    }

    /// Get position and size
    pub fn bounds(&self) -> (i32, i32, u32, u32) {
        (self.x, self.y, self.width, self.height)
    }
}

impl PlaceholderStyle {
    /// Create a new placeholder style
    pub fn new(
        placeholder_type: impl Into<String>,
        font_name: impl Into<String>,
        font_size: u32,
        color: RGBColor,
    ) -> Self {
        Self {
            placeholder_type: placeholder_type.into(),
            font_name: font_name.into(),
            font_size,
            color,
            bold: false,
            italic: false,
        }
    }

    /// Set bold
    pub fn set_bold(&mut self, bold: bool) {
        self.bold = bold;
    }

    /// Set italic
    pub fn set_italic(&mut self, italic: bool) {
        self.italic = italic;
    }

    /// Get placeholder type
    pub fn placeholder_type(&self) -> &str {
        &self.placeholder_type
    }

    /// Get font properties
    pub fn font_properties(&self) -> (&str, u32, bool, bool) {
        (&self.font_name, self.font_size, self.bold, self.italic)
    }

    /// Get color
    pub fn color(&self) -> &RGBColor {
        &self.color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_master_slide_creation() {
        let master = MasterSlideCustomization::new("Master 1");
        assert_eq!(master.name(), "Master 1");
        assert_eq!(master.layouts().len(), 0);
    }

    #[test]
    fn test_add_layout() {
        let mut master = MasterSlideCustomization::new("Master 1");
        let layout = CustomLayout::new("Title Slide", 256, LayoutType::TitleSlide);
        master.add_layout(layout);
        assert_eq!(master.layouts().len(), 1);
    }

    #[test]
    fn test_theme_colors() {
        let mut master = MasterSlideCustomization::new("Master 1");
        let colors = vec![
            RGBColor::new(0, 0, 0),
            RGBColor::new(255, 255, 255),
            RGBColor::new(68, 114, 196),
            RGBColor::new(237, 125, 49),
            RGBColor::new(165, 165, 165),
            RGBColor::new(112, 48, 160),
            RGBColor::new(37, 150, 185),
            RGBColor::new(0, 176, 80),
        ];
        master.set_theme_colors(colors.clone());
        assert_eq!(master.theme_colors().len(), 8);
    }

    #[test]
    fn test_theme_fonts() {
        let mut master = MasterSlideCustomization::new("Master 1");
        master.set_theme_fonts("Calibri", "Cambria");
        let (major, minor) = master.theme_fonts();
        assert_eq!(major, "Calibri");
        assert_eq!(minor, "Cambria");
    }

    #[test]
    fn test_placeholder_style() {
        let mut style = PlaceholderStyle::new(
            "Title",
            "Calibri",
            44,
            RGBColor::new(0, 0, 0),
        );
        style.set_bold(true);
        style.set_italic(false);
        
        let (font_name, font_size, bold, italic) = style.font_properties();
        assert_eq!(font_name, "Calibri");
        assert_eq!(font_size, 44);
        assert!(bold);
        assert!(!italic);
    }

    #[test]
    fn test_master_to_xml() {
        let master = MasterSlideCustomization::new("Master 1");
        let xml = master.to_xml();
        assert!(xml.contains(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#));
        assert!(xml.contains(r#"<p:sldMaster"#));
        assert!(xml.contains(r#"</p:sldMaster>"#));
    }
}
