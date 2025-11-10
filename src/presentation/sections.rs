//! Slide sections for organizing presentations
//!
//! Sections allow grouping slides together for better organization.
//! Based on PptxGenJS sections support.

use std::collections::HashMap;

/// A section containing slides
#[derive(Debug, Clone)]
pub struct Section {
    /// Section ID (unique within presentation)
    id: u32,
    /// Section title/name
    title: String,
    /// Starting slide index (0-based)
    start_slide: u32,
    /// Ending slide index (0-based, inclusive)
    end_slide: u32,
}

impl Section {
    /// Create a new section
    pub fn new(id: u32, title: String, start_slide: u32, end_slide: u32) -> Self {
        Self {
            id,
            title,
            start_slide,
            end_slide,
        }
    }

    /// Get section ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get section title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set section title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Get starting slide index
    pub fn start_slide(&self) -> u32 {
        self.start_slide
    }

    /// Set starting slide index
    pub fn set_start_slide(&mut self, start: u32) {
        self.start_slide = start;
    }

    /// Get ending slide index
    pub fn end_slide(&self) -> u32 {
        self.end_slide
    }

    /// Set ending slide index
    pub fn set_end_slide(&mut self, end: u32) {
        self.end_slide = end;
    }

    /// Get number of slides in section
    pub fn slide_count(&self) -> u32 {
        self.end_slide - self.start_slide + 1
    }

    /// Check if slide index is in this section
    pub fn contains_slide(&self, slide_index: u32) -> bool {
        slide_index >= self.start_slide && slide_index <= self.end_slide
    }

    /// Validate section
    pub fn validate(&self) -> Result<(), String> {
        if self.title.is_empty() {
            return Err("Section title cannot be empty".to_string());
        }

        if self.start_slide > self.end_slide {
            return Err(format!(
                "Start slide ({}) cannot be greater than end slide ({})",
                self.start_slide, self.end_slide
            ));
        }

        Ok(())
    }
}

/// Collection of sections
#[derive(Debug, Clone)]
pub struct SectionCollection {
    sections: Vec<Section>,
    next_id: u32,
}

impl SectionCollection {
    /// Create a new section collection
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            next_id: 1,
        }
    }

    /// Add a section
    pub fn add_section(&mut self, title: String, start_slide: u32, end_slide: u32) -> Result<&mut Section, String> {
        // Validate
        if title.is_empty() {
            return Err("Section title cannot be empty".to_string());
        }

        if start_slide > end_slide {
            return Err("Start slide cannot be greater than end slide".to_string());
        }

        // Check for overlaps
        for section in &self.sections {
            if (start_slide >= section.start_slide && start_slide <= section.end_slide)
                || (end_slide >= section.start_slide && end_slide <= section.end_slide)
                || (start_slide <= section.start_slide && end_slide >= section.end_slide)
            {
                return Err(format!(
                    "Section overlaps with existing section: {}",
                    section.title
                ));
            }
        }

        let section = Section::new(self.next_id, title, start_slide, end_slide);
        self.next_id += 1;
        self.sections.push(section);

        Ok(self.sections.last_mut().unwrap())
    }

    /// Get section by index
    pub fn get(&self, index: usize) -> Option<&Section> {
        self.sections.get(index)
    }

    /// Get mutable section by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Section> {
        self.sections.get_mut(index)
    }

    /// Get section by ID
    pub fn get_by_id(&self, id: u32) -> Option<&Section> {
        self.sections.iter().find(|s| s.id == id)
    }

    /// Get mutable section by ID
    pub fn get_by_id_mut(&mut self, id: u32) -> Option<&mut Section> {
        self.sections.iter_mut().find(|s| s.id == id)
    }

    /// Get section containing slide
    pub fn get_section_for_slide(&self, slide_index: u32) -> Option<&Section> {
        self.sections.iter().find(|s| s.contains_slide(slide_index))
    }

    /// Get all sections
    pub fn sections(&self) -> &[Section] {
        &self.sections
    }

    /// Get number of sections
    pub fn len(&self) -> usize {
        self.sections.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }

    /// Remove section by index
    pub fn remove(&mut self, index: usize) -> Option<Section> {
        if index < self.sections.len() {
            Some(self.sections.remove(index))
        } else {
            None
        }
    }

    /// Clear all sections
    pub fn clear(&mut self) {
        self.sections.clear();
    }

    /// Validate all sections
    pub fn validate(&self) -> Result<(), String> {
        for section in &self.sections {
            section.validate()?;
        }
        Ok(())
    }
}

impl Default for SectionCollection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_new() {
        let section = Section::new(1, "Introduction".to_string(), 0, 2);
        assert_eq!(section.id(), 1);
        assert_eq!(section.title(), "Introduction");
        assert_eq!(section.start_slide(), 0);
        assert_eq!(section.end_slide(), 2);
    }

    #[test]
    fn test_section_slide_count() {
        let section = Section::new(1, "Section".to_string(), 0, 4);
        assert_eq!(section.slide_count(), 5);
    }

    #[test]
    fn test_section_contains_slide() {
        let section = Section::new(1, "Section".to_string(), 2, 5);
        assert!(!section.contains_slide(1));
        assert!(section.contains_slide(2));
        assert!(section.contains_slide(3));
        assert!(section.contains_slide(5));
        assert!(!section.contains_slide(6));
    }

    #[test]
    fn test_section_set_title() {
        let mut section = Section::new(1, "Old Title".to_string(), 0, 2);
        section.set_title("New Title".to_string());
        assert_eq!(section.title(), "New Title");
    }

    #[test]
    fn test_section_set_slides() {
        let mut section = Section::new(1, "Section".to_string(), 0, 2);
        section.set_start_slide(1);
        section.set_end_slide(5);
        assert_eq!(section.start_slide(), 1);
        assert_eq!(section.end_slide(), 5);
    }

    #[test]
    fn test_section_validate_empty_title() {
        let section = Section::new(1, "".to_string(), 0, 2);
        assert!(section.validate().is_err());
    }

    #[test]
    fn test_section_validate_invalid_range() {
        let section = Section::new(1, "Section".to_string(), 5, 2);
        assert!(section.validate().is_err());
    }

    #[test]
    fn test_section_validate_valid() {
        let section = Section::new(1, "Section".to_string(), 0, 2);
        assert!(section.validate().is_ok());
    }

    #[test]
    fn test_section_collection_new() {
        let collection = SectionCollection::new();
        assert!(collection.is_empty());
        assert_eq!(collection.len(), 0);
    }

    #[test]
    fn test_section_collection_add() {
        let mut collection = SectionCollection::new();
        let result = collection.add_section("Section 1".to_string(), 0, 2);
        assert!(result.is_ok());
        assert_eq!(collection.len(), 1);
    }

    #[test]
    fn test_section_collection_add_multiple() {
        let mut collection = SectionCollection::new();
        collection.add_section("Section 1".to_string(), 0, 2).unwrap();
        collection.add_section("Section 2".to_string(), 3, 5).unwrap();
        collection.add_section("Section 3".to_string(), 6, 8).unwrap();
        assert_eq!(collection.len(), 3);
    }

    #[test]
    fn test_section_collection_add_empty_title() {
        let mut collection = SectionCollection::new();
        let result = collection.add_section("".to_string(), 0, 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_section_collection_add_invalid_range() {
        let mut collection = SectionCollection::new();
        let result = collection.add_section("Section".to_string(), 5, 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_section_collection_add_overlap() {
        let mut collection = SectionCollection::new();
        collection.add_section("Section 1".to_string(), 0, 3).unwrap();
        let result = collection.add_section("Section 2".to_string(), 2, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_section_collection_get() {
        let mut collection = SectionCollection::new();
        collection.add_section("Section 1".to_string(), 0, 2).unwrap();
        collection.add_section("Section 2".to_string(), 3, 5).unwrap();

        let section = collection.get(0).unwrap();
        assert_eq!(section.title(), "Section 1");

        let section = collection.get(1).unwrap();
        assert_eq!(section.title(), "Section 2");
    }

    #[test]
    fn test_section_collection_get_by_id() {
        let mut collection = SectionCollection::new();
        collection.add_section("Section 1".to_string(), 0, 2).unwrap();
        collection.add_section("Section 2".to_string(), 3, 5).unwrap();

        let section = collection.get_by_id(1).unwrap();
        assert_eq!(section.title(), "Section 1");

        let section = collection.get_by_id(2).unwrap();
        assert_eq!(section.title(), "Section 2");
    }

    #[test]
    fn test_section_collection_get_section_for_slide() {
        let mut collection = SectionCollection::new();
        collection.add_section("Section 1".to_string(), 0, 2).unwrap();
        collection.add_section("Section 2".to_string(), 3, 5).unwrap();

        let section = collection.get_section_for_slide(1).unwrap();
        assert_eq!(section.title(), "Section 1");

        let section = collection.get_section_for_slide(4).unwrap();
        assert_eq!(section.title(), "Section 2");

        assert!(collection.get_section_for_slide(10).is_none());
    }

    #[test]
    fn test_section_collection_remove() {
        let mut collection = SectionCollection::new();
        collection.add_section("Section 1".to_string(), 0, 2).unwrap();
        collection.add_section("Section 2".to_string(), 3, 5).unwrap();

        let removed = collection.remove(0);
        assert!(removed.is_some());
        assert_eq!(collection.len(), 1);
    }

    #[test]
    fn test_section_collection_clear() {
        let mut collection = SectionCollection::new();
        collection.add_section("Section 1".to_string(), 0, 2).unwrap();
        collection.add_section("Section 2".to_string(), 3, 5).unwrap();

        collection.clear();
        assert!(collection.is_empty());
    }

    #[test]
    fn test_section_collection_validate() {
        let mut collection = SectionCollection::new();
        collection.add_section("Section 1".to_string(), 0, 2).unwrap();
        collection.add_section("Section 2".to_string(), 3, 5).unwrap();

        assert!(collection.validate().is_ok());
    }

    #[test]
    fn test_section_collection_default() {
        let collection = SectionCollection::default();
        assert!(collection.is_empty());
    }

    #[test]
    fn test_section_collection_sequential_sections() {
        let mut collection = SectionCollection::new();
        collection.add_section("Intro".to_string(), 0, 0).unwrap();
        collection.add_section("Content".to_string(), 1, 8).unwrap();
        collection.add_section("Conclusion".to_string(), 9, 10).unwrap();

        assert_eq!(collection.len(), 3);
        assert_eq!(collection.get_section_for_slide(0).unwrap().title(), "Intro");
        assert_eq!(collection.get_section_for_slide(5).unwrap().title(), "Content");
        assert_eq!(collection.get_section_for_slide(10).unwrap().title(), "Conclusion");
    }
}
