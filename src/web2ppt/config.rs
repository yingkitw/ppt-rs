//! Configuration for Web2PPT conversion

/// Configuration options for web2ppt conversion
#[derive(Clone, Debug)]
pub struct Web2PptConfig {
    /// Maximum number of slides to generate
    pub max_slides: usize,
    /// Maximum bullets per slide
    pub max_bullets_per_slide: usize,
    /// Include images from the webpage
    pub include_images: bool,
    /// Include tables from the webpage
    pub include_tables: bool,
    /// Include code blocks
    pub include_code: bool,
    /// User agent for HTTP requests
    pub user_agent: String,
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// Title font size
    pub title_font_size: u32,
    /// Content font size
    pub content_font_size: u32,
    /// Extract links as hyperlinks
    pub extract_links: bool,
    /// Group content by headings
    pub group_by_headings: bool,
}

impl Default for Web2PptConfig {
    fn default() -> Self {
        Web2PptConfig {
            max_slides: 20,
            max_bullets_per_slide: 6,
            include_images: true,
            include_tables: true,
            include_code: true,
            // Use a realistic browser user agent
            user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            timeout_secs: 30,
            title_font_size: 44,
            content_font_size: 24,
            extract_links: true,
            group_by_headings: true,
        }
    }
}

impl Web2PptConfig {
    /// Create a new config with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum slides
    pub fn max_slides(mut self, max: usize) -> Self {
        self.max_slides = max;
        self
    }

    /// Set maximum bullets per slide
    pub fn max_bullets(mut self, max: usize) -> Self {
        self.max_bullets_per_slide = max;
        self
    }

    /// Enable/disable images
    pub fn with_images(mut self, include: bool) -> Self {
        self.include_images = include;
        self
    }

    /// Enable/disable tables
    pub fn with_tables(mut self, include: bool) -> Self {
        self.include_tables = include;
        self
    }

    /// Enable/disable code blocks
    pub fn with_code(mut self, include: bool) -> Self {
        self.include_code = include;
        self
    }

    /// Set custom user agent
    pub fn user_agent(mut self, ua: &str) -> Self {
        self.user_agent = ua.to_string();
        self
    }

    /// Set request timeout
    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Set title font size
    pub fn title_size(mut self, size: u32) -> Self {
        self.title_font_size = size;
        self
    }

    /// Set content font size
    pub fn content_size(mut self, size: u32) -> Self {
        self.content_font_size = size;
        self
    }

    /// Enable/disable link extraction
    pub fn with_links(mut self, extract: bool) -> Self {
        self.extract_links = extract;
        self
    }

    /// Enable/disable grouping by headings
    pub fn group_by_headings(mut self, group: bool) -> Self {
        self.group_by_headings = group;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Web2PptConfig::default();
        assert_eq!(config.max_slides, 20);
        assert_eq!(config.max_bullets_per_slide, 6);
        assert!(config.include_images);
    }

    #[test]
    fn test_config_builder() {
        let config = Web2PptConfig::new()
            .max_slides(10)
            .max_bullets(4)
            .with_images(false);
        
        assert_eq!(config.max_slides, 10);
        assert_eq!(config.max_bullets_per_slide, 4);
        assert!(!config.include_images);
    }
}
