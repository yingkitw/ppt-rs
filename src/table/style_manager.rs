//! Table Style Management
//!
//! This module provides comprehensive table style management including:
//! - Predefined table styles (Light, Medium, Dark, etc.)
//! - Custom style creation
//! - Style application to tables
//! - Style inheritance and composition

use crate::dml::color::RGBColor;
use std::collections::HashMap;

/// Predefined table styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableStylePreset {
    /// Light style - minimal formatting
    Light,
    /// Light Grid style - light with grid lines
    LightGrid,
    /// Light List style - light with list formatting
    LightList,
    /// Medium style - moderate formatting
    Medium,
    /// Medium Grid style - medium with grid lines
    MediumGrid,
    /// Medium List style - medium with list formatting
    MediumList,
    /// Dark style - dark formatting
    Dark,
    /// Dark Grid style - dark with grid lines
    DarkGrid,
    /// Dark List style - dark with list formatting
    DarkList,
    /// Themed style 1
    Themed1,
    /// Themed style 2
    Themed2,
    /// Themed style 3
    Themed3,
}

impl TableStylePreset {
    /// Get the style name
    pub fn name(&self) -> &str {
        match self {
            TableStylePreset::Light => "Light",
            TableStylePreset::LightGrid => "Light Grid",
            TableStylePreset::LightList => "Light List",
            TableStylePreset::Medium => "Medium",
            TableStylePreset::MediumGrid => "Medium Grid",
            TableStylePreset::MediumList => "Medium List",
            TableStylePreset::Dark => "Dark",
            TableStylePreset::DarkGrid => "Dark Grid",
            TableStylePreset::DarkList => "Dark List",
            TableStylePreset::Themed1 => "Themed Style 1",
            TableStylePreset::Themed2 => "Themed Style 2",
            TableStylePreset::Themed3 => "Themed Style 3",
        }
    }

    /// Get the header background color
    pub fn header_background(&self) -> RGBColor {
        match self {
            TableStylePreset::Light => RGBColor::new(242, 242, 242),
            TableStylePreset::LightGrid => RGBColor::new(242, 242, 242),
            TableStylePreset::LightList => RGBColor::new(242, 242, 242),
            TableStylePreset::Medium => RGBColor::new(192, 192, 192),
            TableStylePreset::MediumGrid => RGBColor::new(192, 192, 192),
            TableStylePreset::MediumList => RGBColor::new(192, 192, 192),
            TableStylePreset::Dark => RGBColor::new(64, 64, 64),
            TableStylePreset::DarkGrid => RGBColor::new(64, 64, 64),
            TableStylePreset::DarkList => RGBColor::new(64, 64, 64),
            TableStylePreset::Themed1 => RGBColor::new(79, 129, 189),
            TableStylePreset::Themed2 => RGBColor::new(155, 187, 89),
            TableStylePreset::Themed3 => RGBColor::new(192, 0, 0),
        }
    }

    /// Get the header text color
    pub fn header_text_color(&self) -> RGBColor {
        match self {
            TableStylePreset::Light => RGBColor::new(0, 0, 0),
            TableStylePreset::LightGrid => RGBColor::new(0, 0, 0),
            TableStylePreset::LightList => RGBColor::new(0, 0, 0),
            TableStylePreset::Medium => RGBColor::new(0, 0, 0),
            TableStylePreset::MediumGrid => RGBColor::new(0, 0, 0),
            TableStylePreset::MediumList => RGBColor::new(0, 0, 0),
            TableStylePreset::Dark => RGBColor::new(255, 255, 255),
            TableStylePreset::DarkGrid => RGBColor::new(255, 255, 255),
            TableStylePreset::DarkList => RGBColor::new(255, 255, 255),
            TableStylePreset::Themed1 => RGBColor::new(255, 255, 255),
            TableStylePreset::Themed2 => RGBColor::new(255, 255, 255),
            TableStylePreset::Themed3 => RGBColor::new(255, 255, 255),
        }
    }

    /// Get the row background color
    pub fn row_background(&self) -> RGBColor {
        match self {
            TableStylePreset::Light => RGBColor::new(255, 255, 255),
            TableStylePreset::LightGrid => RGBColor::new(255, 255, 255),
            TableStylePreset::LightList => RGBColor::new(242, 242, 242),
            TableStylePreset::Medium => RGBColor::new(242, 242, 242),
            TableStylePreset::MediumGrid => RGBColor::new(242, 242, 242),
            TableStylePreset::MediumList => RGBColor::new(242, 242, 242),
            TableStylePreset::Dark => RGBColor::new(128, 128, 128),
            TableStylePreset::DarkGrid => RGBColor::new(128, 128, 128),
            TableStylePreset::DarkList => RGBColor::new(128, 128, 128),
            TableStylePreset::Themed1 => RGBColor::new(217, 225, 242),
            TableStylePreset::Themed2 => RGBColor::new(235, 241, 222),
            TableStylePreset::Themed3 => RGBColor::new(242, 220, 219),
        }
    }

    /// Get the row text color
    pub fn row_text_color(&self) -> RGBColor {
        match self {
            TableStylePreset::Light => RGBColor::new(0, 0, 0),
            TableStylePreset::LightGrid => RGBColor::new(0, 0, 0),
            TableStylePreset::LightList => RGBColor::new(0, 0, 0),
            TableStylePreset::Medium => RGBColor::new(0, 0, 0),
            TableStylePreset::MediumGrid => RGBColor::new(0, 0, 0),
            TableStylePreset::MediumList => RGBColor::new(0, 0, 0),
            TableStylePreset::Dark => RGBColor::new(255, 255, 255),
            TableStylePreset::DarkGrid => RGBColor::new(255, 255, 255),
            TableStylePreset::DarkList => RGBColor::new(255, 255, 255),
            TableStylePreset::Themed1 => RGBColor::new(0, 0, 0),
            TableStylePreset::Themed2 => RGBColor::new(0, 0, 0),
            TableStylePreset::Themed3 => RGBColor::new(0, 0, 0),
        }
    }

    /// Check if style has grid lines
    pub fn has_grid_lines(&self) -> bool {
        matches!(
            self,
            TableStylePreset::LightGrid
                | TableStylePreset::MediumGrid
                | TableStylePreset::DarkGrid
        )
    }

    /// Check if style has banded rows
    pub fn has_banded_rows(&self) -> bool {
        matches!(
            self,
            TableStylePreset::LightList
                | TableStylePreset::MediumList
                | TableStylePreset::DarkList
        )
    }
}

/// Custom table style
#[derive(Debug, Clone)]
pub struct TableStyle {
    /// Style name
    name: String,
    /// Header background color
    header_background: RGBColor,
    /// Header text color
    header_text_color: RGBColor,
    /// Row background color
    row_background: RGBColor,
    /// Row text color
    row_text_color: RGBColor,
    /// Alternate row background color
    alternate_row_background: Option<RGBColor>,
    /// Has grid lines
    has_grid: bool,
    /// Has banded rows
    has_banded: bool,
}

impl TableStyle {
    /// Create a new custom table style
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            header_background: RGBColor::new(242, 242, 242),
            header_text_color: RGBColor::new(0, 0, 0),
            row_background: RGBColor::new(255, 255, 255),
            row_text_color: RGBColor::new(0, 0, 0),
            alternate_row_background: None,
            has_grid: false,
            has_banded: false,
        }
    }

    /// Create from preset style
    pub fn from_preset(preset: TableStylePreset) -> Self {
        Self {
            name: preset.name().to_string(),
            header_background: preset.header_background(),
            header_text_color: preset.header_text_color(),
            row_background: preset.row_background(),
            row_text_color: preset.row_text_color(),
            alternate_row_background: None,
            has_grid: preset.has_grid_lines(),
            has_banded: preset.has_banded_rows(),
        }
    }

    /// Get style name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set header background color
    pub fn set_header_background(mut self, color: RGBColor) -> Self {
        self.header_background = color;
        self
    }

    /// Get header background color
    pub fn header_background(&self) -> &RGBColor {
        &self.header_background
    }

    /// Set header text color
    pub fn set_header_text_color(mut self, color: RGBColor) -> Self {
        self.header_text_color = color;
        self
    }

    /// Get header text color
    pub fn header_text_color(&self) -> &RGBColor {
        &self.header_text_color
    }

    /// Set row background color
    pub fn set_row_background(mut self, color: RGBColor) -> Self {
        self.row_background = color;
        self
    }

    /// Get row background color
    pub fn row_background(&self) -> &RGBColor {
        &self.row_background
    }

    /// Set row text color
    pub fn set_row_text_color(mut self, color: RGBColor) -> Self {
        self.row_text_color = color;
        self
    }

    /// Get row text color
    pub fn row_text_color(&self) -> &RGBColor {
        &self.row_text_color
    }

    /// Set alternate row background color
    pub fn set_alternate_row_background(mut self, color: RGBColor) -> Self {
        self.alternate_row_background = Some(color);
        self
    }

    /// Get alternate row background color
    pub fn alternate_row_background(&self) -> Option<&RGBColor> {
        self.alternate_row_background.as_ref()
    }

    /// Enable grid lines
    pub fn enable_grid(mut self) -> Self {
        self.has_grid = true;
        self
    }

    /// Disable grid lines
    pub fn disable_grid(mut self) -> Self {
        self.has_grid = false;
        self
    }

    /// Check if grid lines are enabled
    pub fn has_grid(&self) -> bool {
        self.has_grid
    }

    /// Enable banded rows
    pub fn enable_banded(mut self) -> Self {
        self.has_banded = true;
        self
    }

    /// Disable banded rows
    pub fn disable_banded(mut self) -> Self {
        self.has_banded = false;
        self
    }

    /// Check if banded rows are enabled
    pub fn has_banded(&self) -> bool {
        self.has_banded
    }

    /// Generate XML for table style
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<a:tableStyleId>");
        xml.push_str(&format!(
            "{{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}}"
        ));
        xml.push_str("</a:tableStyleId>");
        xml
    }
}

/// Table style manager
#[derive(Debug, Clone)]
pub struct TableStyleManager {
    /// Predefined styles
    presets: HashMap<String, TableStylePreset>,
    /// Custom styles
    custom_styles: HashMap<String, TableStyle>,
}

impl Default for TableStyleManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TableStyleManager {
    /// Create a new table style manager
    pub fn new() -> Self {
        let mut presets = HashMap::new();
        presets.insert("Light".to_string(), TableStylePreset::Light);
        presets.insert("Light Grid".to_string(), TableStylePreset::LightGrid);
        presets.insert("Light List".to_string(), TableStylePreset::LightList);
        presets.insert("Medium".to_string(), TableStylePreset::Medium);
        presets.insert("Medium Grid".to_string(), TableStylePreset::MediumGrid);
        presets.insert("Medium List".to_string(), TableStylePreset::MediumList);
        presets.insert("Dark".to_string(), TableStylePreset::Dark);
        presets.insert("Dark Grid".to_string(), TableStylePreset::DarkGrid);
        presets.insert("Dark List".to_string(), TableStylePreset::DarkList);
        presets.insert("Themed Style 1".to_string(), TableStylePreset::Themed1);
        presets.insert("Themed Style 2".to_string(), TableStylePreset::Themed2);
        presets.insert("Themed Style 3".to_string(), TableStylePreset::Themed3);

        Self {
            presets,
            custom_styles: HashMap::new(),
        }
    }

    /// Get a preset style
    pub fn get_preset(&self, name: &str) -> Option<TableStylePreset> {
        self.presets.get(name).copied()
    }

    /// Get a custom style
    pub fn get_custom(&self, name: &str) -> Option<&TableStyle> {
        self.custom_styles.get(name)
    }

    /// Add a custom style
    pub fn add_custom_style(&mut self, style: TableStyle) {
        self.custom_styles.insert(style.name().to_string(), style);
    }

    /// Create a custom style from a preset
    pub fn create_from_preset(&mut self, preset: TableStylePreset) {
        let style = TableStyle::from_preset(preset);
        self.custom_styles.insert(style.name().to_string(), style);
    }

    /// List all available preset styles
    pub fn list_presets(&self) -> Vec<&str> {
        self.presets.keys().map(|s| s.as_str()).collect()
    }

    /// List all custom styles
    pub fn list_custom(&self) -> Vec<&str> {
        self.custom_styles.keys().map(|s| s.as_str()).collect()
    }

    /// Get total number of styles
    pub fn total_styles(&self) -> usize {
        self.presets.len() + self.custom_styles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_style_preset_light() {
        let preset = TableStylePreset::Light;
        assert_eq!(preset.name(), "Light");
        assert!(!preset.has_grid_lines());
        assert!(!preset.has_banded_rows());
    }

    #[test]
    fn test_table_style_preset_dark() {
        let preset = TableStylePreset::Dark;
        assert_eq!(preset.name(), "Dark");
        assert!(!preset.has_grid_lines());
        assert!(!preset.has_banded_rows());
    }

    #[test]
    fn test_table_style_preset_light_grid() {
        let preset = TableStylePreset::LightGrid;
        assert!(preset.has_grid_lines());
        assert!(!preset.has_banded_rows());
    }

    #[test]
    fn test_table_style_preset_medium_list() {
        let preset = TableStylePreset::MediumList;
        assert!(!preset.has_grid_lines());
        assert!(preset.has_banded_rows());
    }

    #[test]
    fn test_custom_table_style_creation() {
        let style = TableStyle::new("Custom");
        assert_eq!(style.name(), "Custom");
        assert!(!style.has_grid());
        assert!(!style.has_banded());
    }

    #[test]
    fn test_custom_table_style_from_preset() {
        let style = TableStyle::from_preset(TableStylePreset::Dark);
        assert_eq!(style.name(), "Dark");
        assert_eq!(style.header_text_color(), &RGBColor::new(255, 255, 255));
    }

    #[test]
    fn test_custom_table_style_colors() {
        let style = TableStyle::new("Custom")
            .set_header_background(RGBColor::new(100, 100, 100))
            .set_header_text_color(RGBColor::new(255, 255, 255));

        assert_eq!(style.header_background(), &RGBColor::new(100, 100, 100));
        assert_eq!(style.header_text_color(), &RGBColor::new(255, 255, 255));
    }

    #[test]
    fn test_custom_table_style_grid_and_banded() {
        let style = TableStyle::new("Custom")
            .enable_grid()
            .enable_banded();

        assert!(style.has_grid());
        assert!(style.has_banded());
    }

    #[test]
    fn test_table_style_manager_new() {
        let manager = TableStyleManager::new();
        assert_eq!(manager.total_styles(), 12);
    }

    #[test]
    fn test_table_style_manager_get_preset() {
        let manager = TableStyleManager::new();
        assert!(manager.get_preset("Light").is_some());
        assert!(manager.get_preset("Dark").is_some());
        assert!(manager.get_preset("NonExistent").is_none());
    }

    #[test]
    fn test_table_style_manager_add_custom() {
        let mut manager = TableStyleManager::new();
        let custom = TableStyle::new("MyStyle");
        manager.add_custom_style(custom);

        assert!(manager.get_custom("MyStyle").is_some());
        assert_eq!(manager.total_styles(), 13);
    }

    #[test]
    fn test_table_style_manager_list_presets() {
        let manager = TableStyleManager::new();
        let presets = manager.list_presets();
        assert_eq!(presets.len(), 12);
        assert!(presets.contains(&"Light"));
        assert!(presets.contains(&"Dark"));
    }

    #[test]
    fn test_table_style_manager_create_from_preset() {
        let mut manager = TableStyleManager::new();
        manager.create_from_preset(TableStylePreset::Dark);

        assert!(manager.get_custom("Dark").is_some());
    }

    #[test]
    fn test_table_style_alternate_row_background() {
        let style = TableStyle::new("Custom")
            .set_alternate_row_background(RGBColor::new(200, 200, 200));

        assert!(style.alternate_row_background().is_some());
    }

    #[test]
    fn test_table_style_to_xml() {
        let style = TableStyle::new("Custom");
        let xml = style.to_xml();
        assert!(xml.contains("<a:tableStyleId>"));
        assert!(xml.contains("</a:tableStyleId>"));
    }
}
