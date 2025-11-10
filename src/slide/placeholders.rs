//! Placeholder shapes support
//!
//! Placeholders are special shapes that hold content like titles, subtitles, and body text.
//! They are defined in the slide layout and can be accessed and modified on slides.

use crate::shapes::Shape;
use std::collections::HashMap;

/// Placeholder types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceholderType {
    Title,
    Body,
    CenteredTitle,
    Subtitle,
    DateTime,
    SlideNumber,
    Footer,
    Header,
    Object,
    Chart,
    Table,
    ClipArt,
    Diagram,
    Media,
    SlideImage,
}

impl PlaceholderType {
    /// Get the placeholder type ID
    pub fn type_id(&self) -> u32 {
        match self {
            PlaceholderType::Title => 1,
            PlaceholderType::Body => 2,
            PlaceholderType::CenteredTitle => 3,
            PlaceholderType::Subtitle => 4,
            PlaceholderType::DateTime => 5,
            PlaceholderType::SlideNumber => 6,
            PlaceholderType::Footer => 7,
            PlaceholderType::Header => 8,
            PlaceholderType::Object => 9,
            PlaceholderType::Chart => 10,
            PlaceholderType::Table => 11,
            PlaceholderType::ClipArt => 12,
            PlaceholderType::Diagram => 13,
            PlaceholderType::Media => 14,
            PlaceholderType::SlideImage => 15,
        }
    }

    /// Get placeholder type from ID
    pub fn from_type_id(id: u32) -> Option<Self> {
        match id {
            1 => Some(PlaceholderType::Title),
            2 => Some(PlaceholderType::Body),
            3 => Some(PlaceholderType::CenteredTitle),
            4 => Some(PlaceholderType::Subtitle),
            5 => Some(PlaceholderType::DateTime),
            6 => Some(PlaceholderType::SlideNumber),
            7 => Some(PlaceholderType::Footer),
            8 => Some(PlaceholderType::Header),
            9 => Some(PlaceholderType::Object),
            10 => Some(PlaceholderType::Chart),
            11 => Some(PlaceholderType::Table),
            12 => Some(PlaceholderType::ClipArt),
            13 => Some(PlaceholderType::Diagram),
            14 => Some(PlaceholderType::Media),
            15 => Some(PlaceholderType::SlideImage),
            _ => None,
        }
    }
}

/// Placeholder shape wrapper
pub struct Placeholder {
    idx: usize,
    placeholder_type: PlaceholderType,
    shape: Option<Box<dyn Shape>>,
}

impl std::fmt::Debug for Placeholder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Placeholder")
            .field("idx", &self.idx)
            .field("placeholder_type", &self.placeholder_type)
            .field("shape", &"<dyn Shape>")
            .finish()
    }
}

impl Clone for Placeholder {
    fn clone(&self) -> Self {
        Self {
            idx: self.idx,
            placeholder_type: self.placeholder_type,
            shape: None, // Cannot clone trait object
        }
    }
}

impl Placeholder {
    /// Create a new placeholder
    pub fn new(idx: usize, placeholder_type: PlaceholderType) -> Self {
        Self {
            idx,
            placeholder_type,
            shape: None,
        }
    }

    /// Get the placeholder index
    pub fn index(&self) -> usize {
        self.idx
    }

    /// Get the placeholder type
    pub fn placeholder_type(&self) -> PlaceholderType {
        self.placeholder_type
    }

    /// Get the associated shape
    pub fn shape(&self) -> Option<&dyn Shape> {
        self.shape.as_ref().map(|s| s.as_ref())
    }

    /// Get mutable reference to the associated shape
    pub fn shape_mut(&mut self) -> Option<&mut dyn Shape> {
        self.shape.as_mut().map(|s| s.as_mut() as &mut dyn Shape)
    }

    /// Set the associated shape
    pub fn set_shape(&mut self, shape: Box<dyn Shape>) {
        self.shape = Some(shape);
    }
}

/// Placeholders collection
#[derive(Clone)]
pub struct Placeholders {
    placeholders: HashMap<usize, Placeholder>,
}

impl std::fmt::Debug for Placeholders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Placeholders")
            .field("count", &self.placeholders.len())
            .finish()
    }
}

impl Placeholders {
    /// Create a new placeholders collection
    pub fn new() -> Self {
        Self {
            placeholders: HashMap::new(),
        }
    }

    /// Add a placeholder
    pub fn add(&mut self, placeholder: Placeholder) {
        self.placeholders.insert(placeholder.index(), placeholder);
    }

    /// Get a placeholder by index
    pub fn get(&self, idx: usize) -> Option<&Placeholder> {
        self.placeholders.get(&idx)
    }

    /// Get mutable reference to a placeholder by index
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Placeholder> {
        self.placeholders.get_mut(&idx)
    }

    /// Get title placeholder (usually index 0)
    pub fn title(&self) -> Option<&Placeholder> {
        self.placeholders.values().find(|p| p.placeholder_type == PlaceholderType::Title)
    }

    /// Get mutable reference to title placeholder
    pub fn title_mut(&mut self) -> Option<&mut Placeholder> {
        let title_idx = self.placeholders.values()
            .find(|p| p.placeholder_type == PlaceholderType::Title)
            .map(|p| p.index());
        
        if let Some(idx) = title_idx {
            self.placeholders.get_mut(&idx)
        } else {
            None
        }
    }

    /// Get all placeholders
    pub fn all(&self) -> Vec<&Placeholder> {
        let mut placeholders: Vec<_> = self.placeholders.values().collect();
        placeholders.sort_by_key(|p| p.index());
        placeholders
    }

    /// Get count of placeholders
    pub fn len(&self) -> usize {
        self.placeholders.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.placeholders.is_empty()
    }
}

impl Default for Placeholders {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_type_id() {
        assert_eq!(PlaceholderType::Title.type_id(), 1);
        assert_eq!(PlaceholderType::Body.type_id(), 2);
        assert_eq!(PlaceholderType::Subtitle.type_id(), 4);
    }

    #[test]
    fn test_placeholder_from_type_id() {
        assert_eq!(PlaceholderType::from_type_id(1), Some(PlaceholderType::Title));
        assert_eq!(PlaceholderType::from_type_id(2), Some(PlaceholderType::Body));
        assert_eq!(PlaceholderType::from_type_id(999), None);
    }

    #[test]
    fn test_placeholders_collection() {
        let mut placeholders = Placeholders::new();
        let placeholder = Placeholder::new(0, PlaceholderType::Title);
        placeholders.add(placeholder);
        
        assert_eq!(placeholders.len(), 1);
        assert!(placeholders.get(0).is_some());
        assert!(placeholders.title().is_some());
    }
}
