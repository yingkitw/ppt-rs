//! Table Styles - Table style management and application

use crate::dml::color::RGBColor;

/// Table style type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TableStyleType {
    /// Light style
    Light,
    /// Medium style
    Medium,
    /// Dark style
    Dark,
    /// Accent style
    Accent,
    /// Custom style
    Custom,
}

impl TableStyleType {
    /// Get style name
    pub fn name(&self) -> &str {
        match self {
            TableStyleType::Light => "Light",
            TableStyleType::Medium => "Medium",
            TableStyleType::Dark => "Dark",
            TableStyleType::Accent => "Accent",
            TableStyleType::Custom => "Custom",
        }
    }
}

/// Cell style
#[derive(Clone, Debug)]
pub struct CellStyle {
    /// Fill color
    fill_color: Option<RGBColor>,
    /// Text color
    text_color: Option<RGBColor>,
    /// Border color
    border_color: Option<RGBColor>,
    /// Border width
    border_width: u32,
}

impl CellStyle {
    /// Create a new cell style
    pub fn new() -> Self {
        Self {
            fill_color: None,
            text_color: None,
            border_color: None,
            border_width: 0,
        }
    }

    /// Set fill color
    pub fn set_fill_color(&mut self, color: RGBColor) {
        self.fill_color = Some(color);
    }

    /// Get fill color
    pub fn fill_color(&self) -> Option<&RGBColor> {
        self.fill_color.as_ref()
    }

    /// Set text color
    pub fn set_text_color(&mut self, color: RGBColor) {
        self.text_color = Some(color);
    }

    /// Get text color
    pub fn text_color(&self) -> Option<&RGBColor> {
        self.text_color.as_ref()
    }

    /// Set border color
    pub fn set_border_color(&mut self, color: RGBColor) {
        self.border_color = Some(color);
    }

    /// Get border color
    pub fn border_color(&self) -> Option<&RGBColor> {
        self.border_color.as_ref()
    }

    /// Set border width
    pub fn set_border_width(&mut self, width: u32) {
        self.border_width = width;
    }

    /// Get border width
    pub fn border_width(&self) -> u32 {
        self.border_width
    }

    /// Generate XML for cell style
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(r#"<a:tcStyle>"#);

        if let Some(fill) = &self.fill_color {
            xml.push_str(&format!(
                r#"<a:fill><a:srgbClr val="{:06X}"/></a:fill>"#,
                (fill.r as u32) << 16 | (fill.g as u32) << 8 | (fill.b as u32)
            ));
        }

        if let Some(text) = &self.text_color {
            xml.push_str(&format!(
                r#"<a:textColor><a:srgbClr val="{:06X}"/></a:textColor>"#,
                (text.r as u32) << 16 | (text.g as u32) << 8 | (text.b as u32)
            ));
        }

        xml.push_str(r#"</a:tcStyle>"#);
        xml
    }
}

impl Default for CellStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// Table style
#[derive(Clone, Debug)]
pub struct TableStyle {
    /// Style ID
    id: u32,
    /// Style type
    style_type: TableStyleType,
    /// Name
    name: String,
    /// Header style
    header_style: CellStyle,
    /// Body style
    body_style: CellStyle,
    /// Footer style
    footer_style: CellStyle,
}

impl TableStyle {
    /// Create a new table style
    pub fn new(id: u32, style_type: TableStyleType, name: String) -> Self {
        Self {
            id,
            style_type,
            name,
            header_style: CellStyle::new(),
            body_style: CellStyle::new(),
            footer_style: CellStyle::new(),
        }
    }

    /// Get style ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get style type
    pub fn style_type(&self) -> &TableStyleType {
        &self.style_type
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get mutable header style
    pub fn header_style_mut(&mut self) -> &mut CellStyle {
        &mut self.header_style
    }

    /// Get header style
    pub fn header_style(&self) -> &CellStyle {
        &self.header_style
    }

    /// Get mutable body style
    pub fn body_style_mut(&mut self) -> &mut CellStyle {
        &mut self.body_style
    }

    /// Get body style
    pub fn body_style(&self) -> &CellStyle {
        &self.body_style
    }

    /// Get mutable footer style
    pub fn footer_style_mut(&mut self) -> &mut CellStyle {
        &mut self.footer_style
    }

    /// Get footer style
    pub fn footer_style(&self) -> &CellStyle {
        &self.footer_style
    }

    /// Generate XML for table style
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(&format!(
            r#"<a:tableStyle id="{}" name="{}" type="{}">"#,
            self.id,
            self.name,
            self.style_type.name()
        ));
        xml.push('\n');

        xml.push_str(r#"<a:headerStyle>"#);
        xml.push_str(&self.header_style.to_xml());
        xml.push_str(r#"</a:headerStyle>"#);
        xml.push('\n');

        xml.push_str(r#"<a:bodyStyle>"#);
        xml.push_str(&self.body_style.to_xml());
        xml.push_str(r#"</a:bodyStyle>"#);
        xml.push('\n');

        xml.push_str(r#"<a:footerStyle>"#);
        xml.push_str(&self.footer_style.to_xml());
        xml.push_str(r#"</a:footerStyle>"#);
        xml.push('\n');

        xml.push_str(r#"</a:tableStyle>"#);
        xml
    }
}

/// Table style manager
#[derive(Clone, Debug)]
pub struct TableStyleManager {
    /// Table styles
    styles: Vec<TableStyle>,
}

impl TableStyleManager {
    /// Create a new table style manager
    pub fn new() -> Self {
        Self {
            styles: vec![],
        }
    }

    /// Add a table style
    pub fn add_style(&mut self, style: TableStyle) -> usize {
        self.styles.push(style);
        self.styles.len() - 1
    }

    /// Create and add a new style
    pub fn create_style(&mut self, style_type: TableStyleType, name: String) -> usize {
        let id = self.styles.len() as u32;
        self.add_style(TableStyle::new(id, style_type, name))
    }

    /// Get style by index
    pub fn get(&self, index: usize) -> Option<&TableStyle> {
        self.styles.get(index)
    }

    /// Get mutable style by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut TableStyle> {
        self.styles.get_mut(index)
    }

    /// Get all styles
    pub fn all(&self) -> &[TableStyle] {
        &self.styles
    }

    /// Get number of styles
    pub fn len(&self) -> usize {
        self.styles.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.styles.is_empty()
    }

    /// Generate XML for all styles
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(r#"<a:tableStyleList>"#);
        xml.push('\n');

        for style in &self.styles {
            xml.push_str(&style.to_xml());
            xml.push('\n');
        }

        xml.push_str(r#"</a:tableStyleList>"#);
        xml
    }
}

impl Default for TableStyleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_style_type_name() {
        assert_eq!(TableStyleType::Light.name(), "Light");
        assert_eq!(TableStyleType::Medium.name(), "Medium");
        assert_eq!(TableStyleType::Dark.name(), "Dark");
    }

    #[test]
    fn test_cell_style_creation() {
        let style = CellStyle::new();
        assert!(style.fill_color().is_none());
        assert!(style.text_color().is_none());
        assert_eq!(style.border_width(), 0);
    }

    #[test]
    fn test_cell_style_colors() {
        let mut style = CellStyle::new();
        let color = RGBColor::new(255, 0, 0);
        
        style.set_fill_color(color);
        style.set_text_color(color);
        style.set_border_color(color);
        
        assert!(style.fill_color().is_some());
        assert!(style.text_color().is_some());
        assert!(style.border_color().is_some());
    }

    #[test]
    fn test_cell_style_border_width() {
        let mut style = CellStyle::new();
        style.set_border_width(10);
        assert_eq!(style.border_width(), 10);
    }

    #[test]
    fn test_cell_style_to_xml() {
        let style = CellStyle::new();
        let xml = style.to_xml();
        assert!(xml.contains(r#"<a:tcStyle>"#));
        assert!(xml.contains(r#"</a:tcStyle>"#));
    }

    #[test]
    fn test_table_style_creation() {
        let style = TableStyle::new(1, TableStyleType::Light, "Light Style".to_string());
        assert_eq!(style.id(), 1);
        assert_eq!(style.name(), "Light Style");
        assert_eq!(style.style_type(), &TableStyleType::Light);
    }

    #[test]
    fn test_table_style_cell_styles() {
        let mut style = TableStyle::new(1, TableStyleType::Light, "Light Style".to_string());
        let color = RGBColor::new(200, 200, 200);
        
        style.header_style_mut().set_fill_color(color);
        style.body_style_mut().set_fill_color(color);
        
        assert!(style.header_style().fill_color().is_some());
        assert!(style.body_style().fill_color().is_some());
    }

    #[test]
    fn test_table_style_to_xml() {
        let style = TableStyle::new(1, TableStyleType::Light, "Light Style".to_string());
        let xml = style.to_xml();
        assert!(xml.contains(r#"<a:tableStyle"#));
        assert!(xml.contains(r#"name="Light Style""#));
        assert!(xml.contains(r#"<a:headerStyle>"#));
        assert!(xml.contains(r#"<a:bodyStyle>"#));
        assert!(xml.contains(r#"<a:footerStyle>"#));
    }

    #[test]
    fn test_table_style_manager_creation() {
        let manager = TableStyleManager::new();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_table_style_manager_add() {
        let mut manager = TableStyleManager::new();
        let style = TableStyle::new(1, TableStyleType::Light, "Light".to_string());
        manager.add_style(style);
        
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_table_style_manager_create() {
        let mut manager = TableStyleManager::new();
        manager.create_style(TableStyleType::Light, "Light".to_string());
        manager.create_style(TableStyleType::Dark, "Dark".to_string());
        
        assert_eq!(manager.len(), 2);
    }

    #[test]
    fn test_table_style_manager_get() {
        let mut manager = TableStyleManager::new();
        manager.create_style(TableStyleType::Light, "Light".to_string());
        
        let style = manager.get(0);
        assert!(style.is_some());
        assert_eq!(style.unwrap().name(), "Light");
    }

    #[test]
    fn test_table_style_manager_get_mut() {
        let mut manager = TableStyleManager::new();
        manager.create_style(TableStyleType::Light, "Light".to_string());
        
        if let Some(style) = manager.get_mut(0) {
            let color = RGBColor::new(100, 100, 100);
            style.header_style_mut().set_fill_color(color);
        }
        
        assert!(manager.get(0).unwrap().header_style().fill_color().is_some());
    }

    #[test]
    fn test_table_style_manager_to_xml() {
        let mut manager = TableStyleManager::new();
        manager.create_style(TableStyleType::Light, "Light".to_string());
        
        let xml = manager.to_xml();
        assert!(xml.contains(r#"<a:tableStyleList>"#));
        assert!(xml.contains(r#"<a:tableStyle"#));
        assert!(xml.contains(r#"</a:tableStyleList>"#));
    }

    #[test]
    fn test_table_style_manager_default() {
        let manager = TableStyleManager::default();
        assert!(manager.is_empty());
    }
}
