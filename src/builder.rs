//! Fluent builder API for creating presentations
//! 
//! This module provides a simple, intuitive API inspired by PptxGenJS
//! that makes it easy to create PowerPoint presentations in Rust.
//!
//! # Example
//!
//! ```ignore
//! use ppt_rs::builder::PresentationBuilder;
//!
//! let mut prs = PresentationBuilder::new()
//!     .title("My Presentation")
//!     .author("John Doe")
//!     .build()?;
//!
//! let mut slide = prs.add_slide();
//! slide.add_text("Hello, World!", Default::default())?;
//!
//! prs.save_to_file("output.pptx")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::error::Result;
use crate::presentation::Presentation;

/// Builder for creating presentations with a fluent API
pub struct PresentationBuilder {
    title: Option<String>,
    author: Option<String>,
    subject: Option<String>,
    company: Option<String>,
    keywords: Option<String>,
    comments: Option<String>,
    slide_width: Option<u32>,
    slide_height: Option<u32>,
    custom_properties: Vec<(String, String)>,
}

impl PresentationBuilder {
    /// Create a new presentation builder
    pub fn new() -> Self {
        Self {
            title: None,
            author: None,
            subject: None,
            company: None,
            keywords: None,
            comments: None,
            slide_width: None,
            slide_height: None,
            custom_properties: Vec::new(),
        }
    }

    /// Set the presentation title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the presentation author
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set the presentation subject
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Set the presentation company
    pub fn company(mut self, company: impl Into<String>) -> Self {
        self.company = Some(company.into());
        self
    }

    /// Set the presentation keywords
    pub fn keywords(mut self, keywords: impl Into<String>) -> Self {
        self.keywords = Some(keywords.into());
        self
    }

    /// Set the presentation comments/description
    pub fn comments(mut self, comments: impl Into<String>) -> Self {
        self.comments = Some(comments.into());
        self
    }

    /// Set custom slide width in EMU (English Metric Units)
    /// 
    /// Default is 9144000 EMU (10 inches)
    /// 1 inch = 914400 EMU
    pub fn slide_width(mut self, width: u32) -> Self {
        self.slide_width = Some(width);
        self
    }

    /// Set custom slide height in EMU (English Metric Units)
    /// 
    /// Default is 6858000 EMU (7.5 inches)
    /// 1 inch = 914400 EMU
    pub fn slide_height(mut self, height: u32) -> Self {
        self.slide_height = Some(height);
        self
    }

    /// Add a custom property (user-defined metadata)
    pub fn custom_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_properties.push((key.into(), value.into()));
        self
    }

    /// Build the presentation
    pub fn build(self) -> Result<Presentation> {
        let mut prs = Presentation::new()?;
        
        // Set custom dimensions if provided
        if let Some(width) = self.slide_width {
            prs.set_slide_width(width)?;
        }
        if let Some(height) = self.slide_height {
            prs.set_slide_height(height)?;
        }
        
        // Set custom properties if provided
        for (key, value) in self.custom_properties {
            prs.custom_props_mut().set(key, value);
        }
        
        // Note: Properties are set via CoreProperties in the presentation
        // This is a simplified builder that creates a valid presentation
        // Additional properties can be set after building if needed
        
        Ok(prs)
    }
}

impl Default for PresentationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_new() {
        let builder = PresentationBuilder::new();
        assert!(builder.title.is_none());
        assert!(builder.author.is_none());
    }

    #[test]
    fn test_builder_with_title() {
        let builder = PresentationBuilder::new()
            .title("Test Presentation");
        assert_eq!(builder.title, Some("Test Presentation".to_string()));
    }

    #[test]
    fn test_builder_with_multiple_properties() {
        let builder = PresentationBuilder::new()
            .title("Test")
            .author("John")
            .subject("Testing")
            .company("Acme");
        
        assert_eq!(builder.title, Some("Test".to_string()));
        assert_eq!(builder.author, Some("John".to_string()));
        assert_eq!(builder.subject, Some("Testing".to_string()));
        assert_eq!(builder.company, Some("Acme".to_string()));
    }

    #[test]
    fn test_builder_build() {
        let prs = PresentationBuilder::new()
            .title("Test")
            .build();
        assert!(prs.is_ok());
    }
}
