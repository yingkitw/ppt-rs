//! Builder types for presentations and slides

use crate::generator;
use crate::exc::Result;
use crate::config::Config;
use crate::constants;
use std::fs;

/// Complete PPTX presentation builder
pub struct PresentationBuilder {
    pub title: String,
    pub slides: usize,
    pub config: Config,
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
            .map_err(crate::exc::PptxError::Io)
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
    pub title: String,
    pub content: String,
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
