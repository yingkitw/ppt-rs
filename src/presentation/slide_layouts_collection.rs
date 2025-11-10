//! Slide layouts collection
//!
//! Provides access to all available slide layouts in a presentation.

use std::collections::HashMap;

/// Slide layout information
#[derive(Debug, Clone)]
pub struct SlideLayoutInfo {
    name: String,
    layout_id: usize,
    layout_type: String,
}

impl SlideLayoutInfo {
    /// Create a new slide layout info
    pub fn new(name: String, layout_id: usize, layout_type: String) -> Self {
        Self {
            name,
            layout_id,
            layout_type,
        }
    }

    /// Get the layout name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the layout ID
    pub fn layout_id(&self) -> usize {
        self.layout_id
    }

    /// Get the layout type
    pub fn layout_type(&self) -> &str {
        &self.layout_type
    }
}

/// Slide layouts collection
#[derive(Debug, Clone)]
pub struct SlideLayoutsCollection {
    layouts: HashMap<usize, SlideLayoutInfo>,
}

impl SlideLayoutsCollection {
    /// Create a new slide layouts collection with default layouts
    pub fn new() -> Self {
        let mut layouts = HashMap::new();

        // Default PowerPoint layouts
        let default_layouts = vec![
            ("Blank", "blank"),
            ("Title Slide", "titleSlide"),
            ("Title and Content", "titleAndContent"),
            ("Section Header", "sectionHeader"),
            ("Two Content", "twoContent"),
            ("Comparison", "comparison"),
            ("Title Only", "titleOnly"),
            ("Blank", "blank"),
            ("Title and Vertical Content", "titleAndVerticalContent"),
            ("Vertical Title and Content", "verticalTitleAndContent"),
            ("Title Picture", "titlePicture"),
        ];

        for (idx, (name, layout_type)) in default_layouts.iter().enumerate() {
            layouts.insert(
                idx,
                SlideLayoutInfo::new(name.to_string(), idx, layout_type.to_string()),
            );
        }

        Self { layouts }
    }

    /// Get a layout by index
    pub fn get(&self, idx: usize) -> Option<&SlideLayoutInfo> {
        self.layouts.get(&idx)
    }

    /// Get a layout by name
    pub fn get_by_name(&self, name: &str) -> Option<&SlideLayoutInfo> {
        self.layouts.values().find(|l| l.name == name)
    }

    /// Get all layouts
    pub fn all(&self) -> Vec<&SlideLayoutInfo> {
        let mut layouts: Vec<_> = self.layouts.values().collect();
        layouts.sort_by_key(|l| l.layout_id);
        layouts
    }

    /// Get layout count
    pub fn len(&self) -> usize {
        self.layouts.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.layouts.is_empty()
    }

    /// Add a custom layout
    pub fn add(&mut self, layout: SlideLayoutInfo) {
        let id = self.layouts.len();
        self.layouts.insert(id, layout);
    }
}

impl Default for SlideLayoutsCollection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_layout_info() {
        let layout = SlideLayoutInfo::new("Test".to_string(), 0, "test".to_string());
        assert_eq!(layout.name(), "Test");
        assert_eq!(layout.layout_id(), 0);
        assert_eq!(layout.layout_type(), "test");
    }

    #[test]
    fn test_slide_layouts_collection() {
        let layouts = SlideLayoutsCollection::new();
        assert_eq!(layouts.len(), 11);
        assert!(layouts.get(0).is_some());
        assert!(layouts.get_by_name("Blank").is_some());
    }

    #[test]
    fn test_slide_layouts_all() {
        let layouts = SlideLayoutsCollection::new();
        let all = layouts.all();
        assert_eq!(all.len(), 11);
    }

    #[test]
    fn test_slide_layouts_add() {
        let mut layouts = SlideLayoutsCollection::new();
        let custom = SlideLayoutInfo::new("Custom".to_string(), 11, "custom".to_string());
        layouts.add(custom);
        assert_eq!(layouts.len(), 12);
    }
}
