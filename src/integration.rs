//! Integration module - connects all PPTX components

use crate::generator;
use crate::exc::Result;
use crate::config::Config;
use crate::constants;
use std::fs;

/// Complete PPTX presentation builder
pub struct PresentationBuilder {
    title: String,
    slides: usize,
    config: Config,
}

impl PresentationBuilder {
    /// Create a new presentation builder
    pub fn new(title: &str) -> Self {
        PresentationBuilder {
            title: title.to_string(),
            slides: constants::presentation::DEFAULT_SLIDES,
            config: Config::default(),
        }
    }

    /// Create with custom config
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    /// Set number of slides
    pub fn with_slides(mut self, count: usize) -> Self {
        self.slides = count;
        self
    }

    /// Build and generate PPTX file
    pub fn build(&self) -> Result<Vec<u8>> {
        generator::create_pptx(&self.title, self.slides)
            .map_err(|e| crate::exc::PptxError::Generic(e.to_string()))
    }

    /// Save to file
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let pptx_data = self.build()?;
        fs::write(path, pptx_data)
            .map_err(|e| crate::exc::PptxError::Io(e))
    }

    /// Save to configured output directory
    pub fn save(&self, filename: &str) -> Result<()> {
        let path = self.config.output_path(filename);
        self.save_to_file(&path)
    }
}

/// Presentation metadata
pub struct PresentationMetadata {
    pub title: String,
    pub slides: usize,
    pub created: String,
    pub modified: String,
}

impl PresentationMetadata {
    /// Create new metadata
    pub fn new(title: &str, slides: usize) -> Self {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        PresentationMetadata {
            title: title.to_string(),
            slides,
            created: now.clone(),
            modified: now,
        }
    }
}

/// Slide builder
pub struct SlideBuilder {
    title: String,
    content: String,
}

impl SlideBuilder {
    /// Create new slide
    pub fn new(title: &str) -> Self {
        SlideBuilder {
            title: title.to_string(),
            content: String::new(),
        }
    }

    /// Add content
    pub fn with_content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    /// Get slide data
    pub fn build(&self) -> (String, String) {
        (self.title.clone(), self.content.clone())
    }
}

/// Utility functions for presentation generation
pub mod utils {
    use crate::util;

    /// Convert inches to EMU
    pub fn inches_to_emu(inches: f64) -> i32 {
        util::inches(inches).into()
    }

    /// Convert centimeters to EMU
    pub fn cm_to_emu(cm: f64) -> i32 {
        util::cm(cm).into()
    }

    /// Convert points to EMU
    pub fn pt_to_emu(pt: f64) -> i32 {
        util::pt(pt).into()
    }

    /// Format file size
    pub fn format_size(bytes: usize) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        }
    }
}

/// Enumeration helpers
pub mod enum_helpers {
    use crate::enums;

    /// Get action type description
    pub fn action_description(action: &enums::base::BaseEnum) -> String {
        format!("{} ({})", action.name, action.value)
    }

    /// Get chart type description
    pub fn chart_description(chart: &enums::base::BaseEnum) -> String {
        format!("{} ({})", chart.name, chart.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentation_builder() {
        let builder = PresentationBuilder::new("Test")
            .with_slides(5);
        assert_eq!(builder.slides, 5);
        assert_eq!(builder.title, "Test");
    }

    #[test]
    fn test_slide_builder() {
        let slide = SlideBuilder::new("Title")
            .with_content("Content");
        let (title, content) = slide.build();
        assert_eq!(title, "Title");
        assert_eq!(content, "Content");
    }

    #[test]
    fn test_format_size() {
        assert_eq!(utils::format_size(512), "512 B");
        assert_eq!(utils::format_size(1024), "1.0 KB");
        assert_eq!(utils::format_size(1024 * 1024), "1.0 MB");
    }
}
