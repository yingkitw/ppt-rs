//! Slide Layouts Collection - Manages all 11 predefined layouts

use super::layout::{SlideLayout, LayoutType};

/// Collection of all 11 predefined slide layouts
#[derive(Clone, Debug)]
pub struct SlideLayouts {
    layouts: Vec<SlideLayout>,
}

impl SlideLayouts {
    /// Create a new collection with all 11 predefined layouts
    pub fn new() -> Self {
        let layouts = vec![
            SlideLayout::new(LayoutType::TitleSlide),
            SlideLayout::new(LayoutType::TitleAndContent),
            SlideLayout::new(LayoutType::TitleOnly),
            SlideLayout::new(LayoutType::CenteredTitle),
            SlideLayout::new(LayoutType::TitleAndTwoContent),
            SlideLayout::new(LayoutType::Blank),
            SlideLayout::new(LayoutType::Comparison),
            SlideLayout::new(LayoutType::TitleContentCaption),
            SlideLayout::new(LayoutType::PictureCaption),
            SlideLayout::new(LayoutType::BlankWithTitle),
            SlideLayout::new(LayoutType::TitleAndVerticalContent),
        ];

        Self { layouts }
    }

    /// Get layout by index (1-11)
    pub fn get(&self, index: u32) -> Option<&SlideLayout> {
        if index >= 1 && index <= 11 {
            self.layouts.get((index - 1) as usize)
        } else {
            None
        }
    }

    /// Get layout by type
    pub fn get_by_type(&self, layout_type: &LayoutType) -> Option<&SlideLayout> {
        self.layouts.iter().find(|l| l.layout_type() == layout_type)
    }

    /// Get all layouts
    pub fn all(&self) -> &[SlideLayout] {
        &self.layouts
    }

    /// Get number of layouts
    pub fn len(&self) -> usize {
        self.layouts.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.layouts.is_empty()
    }

    /// Get layout IDs for master
    pub fn layout_ids(&self) -> Vec<u32> {
        self.layouts.iter().map(|l| 256 + l.index() - 1).collect()
    }
}

impl Default for SlideLayouts {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_layouts_creation() {
        let layouts = SlideLayouts::new();
        assert_eq!(layouts.len(), 11);
    }

    #[test]
    fn test_slide_layouts_get_by_index() {
        let layouts = SlideLayouts::new();
        
        let layout1 = layouts.get(1);
        assert!(layout1.is_some());
        assert_eq!(layout1.unwrap().name(), "Title Slide");
        
        let layout2 = layouts.get(2);
        assert!(layout2.is_some());
        assert_eq!(layout2.unwrap().name(), "Title and Content");
        
        let layout_invalid = layouts.get(12);
        assert!(layout_invalid.is_none());
    }

    #[test]
    fn test_slide_layouts_get_by_type() {
        let layouts = SlideLayouts::new();
        
        let layout = layouts.get_by_type(&LayoutType::TitleAndContent);
        assert!(layout.is_some());
        assert_eq!(layout.unwrap().name(), "Title and Content");
    }

    #[test]
    fn test_slide_layouts_all() {
        let layouts = SlideLayouts::new();
        let all = layouts.all();
        
        assert_eq!(all.len(), 11);
        assert_eq!(all[0].name(), "Title Slide");
        assert_eq!(all[5].name(), "Blank");
    }

    #[test]
    fn test_slide_layouts_ids() {
        let layouts = SlideLayouts::new();
        let ids = layouts.layout_ids();
        
        assert_eq!(ids.len(), 11);
        // IDs should be sequential starting from 256
        for (i, &id) in ids.iter().enumerate() {
            assert_eq!(id, 256 + i as u32);
        }
    }

    #[test]
    fn test_slide_layouts_default() {
        let layouts = SlideLayouts::default();
        assert_eq!(layouts.len(), 11);
    }

    #[test]
    fn test_all_layouts_have_xml() {
        let layouts = SlideLayouts::new();
        
        for layout in layouts.all() {
            let xml = layout.to_xml();
            assert!(xml.contains(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#));
            assert!(xml.contains(r#"<p:sldLayout"#));
            assert!(xml.contains(r#"</p:sldLayout>"#));
        }
    }
}
