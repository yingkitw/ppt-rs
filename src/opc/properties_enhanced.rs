//! Enhanced properties management for OOXML documents
//!
//! Provides support for core properties, app properties, and custom properties
//! following the OOXML specification.

use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Core properties of an OOXML document
///
/// Corresponds to docProps/core.xml in the package
#[derive(Debug, Clone, Default)]
pub struct CoreProperties {
    /// Document title
    pub title: Option<String>,
    /// Document subject
    pub subject: Option<String>,
    /// Document creator/author
    pub creator: Option<String>,
    /// Document keywords
    pub keywords: Option<String>,
    /// Document description
    pub description: Option<String>,
    /// Last modified by
    pub last_modified_by: Option<String>,
    /// Creation date/time
    pub created: Option<DateTime<Utc>>,
    /// Last modification date/time
    pub modified: Option<DateTime<Utc>>,
}

impl CoreProperties {
    /// Create new core properties
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the title
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Set the subject
    pub fn with_subject(mut self, subject: String) -> Self {
        self.subject = Some(subject);
        self
    }

    /// Set the creator
    pub fn with_creator(mut self, creator: String) -> Self {
        self.creator = Some(creator);
        self
    }

    /// Set the keywords
    pub fn with_keywords(mut self, keywords: String) -> Self {
        self.keywords = Some(keywords);
        self
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set the last modified by
    pub fn with_last_modified_by(mut self, last_modified_by: String) -> Self {
        self.last_modified_by = Some(last_modified_by);
        self
    }

    /// Set the creation date
    pub fn with_created(mut self, created: DateTime<Utc>) -> Self {
        self.created = Some(created);
        self
    }

    /// Set the modification date
    pub fn with_modified(mut self, modified: DateTime<Utc>) -> Self {
        self.modified = Some(modified);
        self
    }
}

/// Application properties of an OOXML document
///
/// Corresponds to docProps/app.xml in the package
#[derive(Debug, Clone, Default)]
pub struct AppProperties {
    /// Application name (e.g., "Microsoft Office PowerPoint")
    pub application: Option<String>,
    /// Application version
    pub app_version: Option<String>,
    /// Total editing time in minutes
    pub total_time: Option<i32>,
    /// Number of slides (for presentations)
    pub slides: Option<i32>,
    /// Number of notes (for presentations)
    pub notes: Option<i32>,
    /// Number of words
    pub words: Option<i32>,
    /// Number of characters
    pub characters: Option<i32>,
    /// Number of paragraphs
    pub paragraphs: Option<i32>,
}

impl AppProperties {
    /// Create new app properties
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the application name
    pub fn with_application(mut self, application: String) -> Self {
        self.application = Some(application);
        self
    }

    /// Set the application version
    pub fn with_app_version(mut self, app_version: String) -> Self {
        self.app_version = Some(app_version);
        self
    }

    /// Set the total editing time
    pub fn with_total_time(mut self, total_time: i32) -> Self {
        self.total_time = Some(total_time);
        self
    }

    /// Set the number of slides
    pub fn with_slides(mut self, slides: i32) -> Self {
        self.slides = Some(slides);
        self
    }

    /// Set the number of notes
    pub fn with_notes(mut self, notes: i32) -> Self {
        self.notes = Some(notes);
        self
    }

    /// Set the number of words
    pub fn with_words(mut self, words: i32) -> Self {
        self.words = Some(words);
        self
    }

    /// Set the number of characters
    pub fn with_characters(mut self, characters: i32) -> Self {
        self.characters = Some(characters);
        self
    }

    /// Set the number of paragraphs
    pub fn with_paragraphs(mut self, paragraphs: i32) -> Self {
        self.paragraphs = Some(paragraphs);
        self
    }
}

/// Custom properties of an OOXML document
///
/// Corresponds to docProps/custom.xml in the package
/// Allows user-defined properties
#[derive(Debug, Clone, Default)]
pub struct CustomProperties {
    properties: HashMap<String, String>,
}

impl CustomProperties {
    /// Create new custom properties
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a custom property
    pub fn set(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }

    /// Get a custom property
    pub fn get(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }

    /// Check if a custom property exists
    pub fn contains(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    /// Remove a custom property
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.properties.remove(key)
    }

    /// Get all custom properties
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.properties.iter()
    }

    /// Get the number of custom properties
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Check if there are any custom properties
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_properties_new() {
        let props = CoreProperties::new();
        assert!(props.title.is_none());
        assert!(props.creator.is_none());
    }

    #[test]
    fn test_core_properties_builder() {
        let props = CoreProperties::new()
            .with_title("Test Title".to_string())
            .with_creator("Test Creator".to_string());

        assert_eq!(props.title, Some("Test Title".to_string()));
        assert_eq!(props.creator, Some("Test Creator".to_string()));
    }

    #[test]
    fn test_app_properties_new() {
        let props = AppProperties::new();
        assert!(props.application.is_none());
        assert!(props.slides.is_none());
    }

    #[test]
    fn test_app_properties_builder() {
        let props = AppProperties::new()
            .with_application("PowerPoint".to_string())
            .with_slides(5);

        assert_eq!(props.application, Some("PowerPoint".to_string()));
        assert_eq!(props.slides, Some(5));
    }

    #[test]
    fn test_custom_properties_new() {
        let props = CustomProperties::new();
        assert!(props.is_empty());
    }

    #[test]
    fn test_custom_properties_set_get() {
        let mut props = CustomProperties::new();
        props.set("key1".to_string(), "value1".to_string());
        assert_eq!(props.get("key1"), Some("value1"));
    }

    #[test]
    fn test_custom_properties_contains() {
        let mut props = CustomProperties::new();
        props.set("key1".to_string(), "value1".to_string());
        assert!(props.contains("key1"));
        assert!(!props.contains("key2"));
    }

    #[test]
    fn test_custom_properties_remove() {
        let mut props = CustomProperties::new();
        props.set("key1".to_string(), "value1".to_string());
        let removed = props.remove("key1");
        assert_eq!(removed, Some("value1".to_string()));
        assert!(!props.contains("key1"));
    }

    #[test]
    fn test_custom_properties_len() {
        let mut props = CustomProperties::new();
        props.set("key1".to_string(), "value1".to_string());
        props.set("key2".to_string(), "value2".to_string());
        assert_eq!(props.len(), 2);
    }
}
