//! Converter from web content to PowerPoint

use super::{Web2PptError, Result, Web2PptConfig, WebContent, ContentType};
use crate::{create_pptx_with_content, SlideContent, SlideLayout};

/// Safely truncate text at char boundary
fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        return text.to_string();
    }
    
    // Find a safe truncation point
    let mut end = max_len;
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }
    
    // Try to break at word boundary
    if let Some(last_space) = text[..end].rfind(' ') {
        if last_space > max_len / 2 {
            end = last_space;
        }
    }
    
    format!("{}...", &text[..end].trim_end())
}

/// Options for conversion
#[derive(Clone, Debug)]
pub struct ConversionOptions {
    /// Presentation title (overrides page title)
    pub title: Option<String>,
    /// Author name
    pub author: Option<String>,
    /// Add source URL to slides
    pub include_source_url: bool,
    /// Add page numbers
    pub add_page_numbers: bool,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        ConversionOptions {
            title: None,
            author: None,
            include_source_url: true,
            add_page_numbers: false,
        }
    }
}

impl ConversionOptions {
    /// Create new options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set custom title
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set author
    pub fn author(mut self, author: &str) -> Self {
        self.author = Some(author.to_string());
        self
    }

    /// Include source URL
    pub fn with_source_url(mut self, include: bool) -> Self {
        self.include_source_url = include;
        self
    }

    /// Add page numbers
    pub fn with_page_numbers(mut self, add: bool) -> Self {
        self.add_page_numbers = add;
        self
    }
}

/// Web to PowerPoint converter
pub struct Web2Ppt {
    config: Web2PptConfig,
}

impl Web2Ppt {
    /// Create a new converter with default config
    pub fn new() -> Self {
        Self::with_config(Web2PptConfig::default())
    }

    /// Create a new converter with custom config
    pub fn with_config(config: Web2PptConfig) -> Self {
        Web2Ppt { config }
    }

    /// Convert web content to PowerPoint bytes
    pub fn convert(&self, content: &WebContent, options: &ConversionOptions) -> Result<Vec<u8>> {
        let slides = self.build_slides(content, options)?;
        let title = options.title.as_ref().unwrap_or(&content.title);

        create_pptx_with_content(title, slides)
            .map_err(|e| Web2PptError::GenerationError(e.to_string()))
    }

    /// Build slides from web content
    fn build_slides(&self, content: &WebContent, options: &ConversionOptions) -> Result<Vec<SlideContent>> {
        let mut slides = Vec::new();

        // Title slide
        let title = options.title.as_ref().unwrap_or(&content.title);
        let mut title_slide = SlideContent::new(title)
            .layout(SlideLayout::CenteredTitle);

        if let Some(desc) = &content.description {
            title_slide = title_slide.add_bullet(desc);
        }

        if options.include_source_url {
            title_slide = title_slide.add_bullet(&format!("Source: {}", content.url));
        }

        slides.push(title_slide);

        // Content slides
        if self.config.group_by_headings {
            self.build_grouped_slides(content, &mut slides)?;
        } else {
            self.build_linear_slides(content, &mut slides)?;
        }

        // Limit slides
        if slides.len() > self.config.max_slides {
            slides.truncate(self.config.max_slides);
        }

        Ok(slides)
    }

    /// Build slides grouped by headings
    fn build_grouped_slides(&self, content: &WebContent, slides: &mut Vec<SlideContent>) -> Result<()> {
        let groups = content.grouped_by_headings();

        // If no groups found, fall back to linear mode
        if groups.is_empty() {
            return self.build_linear_slides(content, slides);
        }

        for (heading, blocks) in groups {
            if slides.len() >= self.config.max_slides {
                break;
            }

            let mut slide = SlideContent::new(&heading.text)
                .layout(SlideLayout::TitleAndContent);

            let mut bullet_count = 0;

            for block in blocks {
                if bullet_count >= self.config.max_bullets_per_slide {
                    // Start a new slide for overflow
                    slides.push(slide);
                    slide = SlideContent::new(&format!("{} (cont.)", heading.text))
                        .layout(SlideLayout::TitleAndContent);
                    bullet_count = 0;

                    if slides.len() >= self.config.max_slides {
                        break;
                    }
                }

                match &block.content_type {
                    ContentType::Paragraph => {
                        // Truncate long paragraphs - use char_indices for safe slicing
                        let text = truncate_text(&block.text, 200);
                        slide = slide.add_bullet(&text);
                        bullet_count += 1;
                    }
                    ContentType::ListItem => {
                        let text = truncate_text(&block.text, 180);
                        slide = slide.add_bullet(&format!("• {}", text));
                        bullet_count += 1;
                    }
                    ContentType::Quote => {
                        let text = truncate_text(&block.text, 180);
                        slide = slide.add_bullet(&format!("\"{}\"", text));
                        bullet_count += 1;
                    }
                    ContentType::Code => {
                        if self.config.include_code {
                            let code = truncate_text(&block.text, 150);
                            slide = slide.add_bullet(&format!("[Code] {}", code));
                            bullet_count += 1;
                        }
                    }
                    ContentType::Table(rows) => {
                        if self.config.include_tables && !rows.is_empty() {
                            let summary = format!("[Table: {} rows × {} cols]", 
                                rows.len(), 
                                rows.first().map(|r| r.len()).unwrap_or(0)
                            );
                            slide = slide.add_bullet(&summary);
                            bullet_count += 1;
                        }
                    }
                    ContentType::Image { alt, .. } => {
                        if self.config.include_images && !alt.is_empty() {
                            slide = slide.add_bullet(&format!("[Image: {}]", alt));
                            bullet_count += 1;
                        }
                    }
                    _ => {}
                }
            }

            // Only add slide if it has content
            if bullet_count > 0 {
                slides.push(slide);
            }
        }

        Ok(())
    }

    /// Build slides linearly (not grouped)
    fn build_linear_slides(&self, content: &WebContent, slides: &mut Vec<SlideContent>) -> Result<()> {
        let mut current_slide: Option<SlideContent> = None;
        let mut bullet_count = 0;

        // If no content blocks, create a slide with description
        if content.blocks.is_empty() {
            if let Some(desc) = &content.description {
                let slide = SlideContent::new("Content")
                    .layout(SlideLayout::TitleAndContent)
                    .add_bullet(desc);
                slides.push(slide);
            }
            return Ok(());
        }

        for block in &content.blocks {
            if slides.len() >= self.config.max_slides {
                break;
            }

            match &block.content_type {
                ContentType::Title | ContentType::Heading(_) => {
                    // Save current slide if it has content
                    if let Some(slide) = current_slide.take() {
                        if bullet_count > 0 {
                            slides.push(slide);
                        }
                    }

                    // Start new slide
                    current_slide = Some(
                        SlideContent::new(&block.text)
                            .layout(SlideLayout::TitleAndContent)
                    );
                    bullet_count = 0;
                }
                ContentType::Paragraph => {
                    // If no current slide, create one
                    if current_slide.is_none() {
                        current_slide = Some(
                            SlideContent::new("Overview")
                                .layout(SlideLayout::TitleAndContent)
                        );
                    }
                    
                    if let Some(ref mut slide) = current_slide {
                        if bullet_count < self.config.max_bullets_per_slide {
                            let text = truncate_text(&block.text, 200);
                            *slide = slide.clone().add_bullet(&text);
                            bullet_count += 1;
                        } else {
                            // Start new continuation slide
                            slides.push(slide.clone());
                            let title = slide.title.clone();
                            *slide = SlideContent::new(&format!("{} (cont.)", title))
                                .layout(SlideLayout::TitleAndContent);
                            let text = truncate_text(&block.text, 200);
                            *slide = slide.clone().add_bullet(&text);
                            bullet_count = 1;
                        }
                    }
                }
                ContentType::ListItem => {
                    if current_slide.is_none() {
                        current_slide = Some(
                            SlideContent::new("Key Points")
                                .layout(SlideLayout::TitleAndContent)
                        );
                    }
                    
                    if let Some(ref mut slide) = current_slide {
                        if bullet_count < self.config.max_bullets_per_slide {
                            let text = truncate_text(&block.text, 180);
                            *slide = slide.clone().add_bullet(&format!("• {}", text));
                            bullet_count += 1;
                        }
                    }
                }
                ContentType::Quote => {
                    if let Some(ref mut slide) = current_slide {
                        if bullet_count < self.config.max_bullets_per_slide {
                            let text = truncate_text(&block.text, 180);
                            *slide = slide.clone().add_bullet(&format!("\"{}\"", text));
                            bullet_count += 1;
                        }
                    }
                }
                _ => {}
            }
        }

        // Save last slide
        if let Some(slide) = current_slide {
            if bullet_count > 0 {
                slides.push(slide);
            }
        }

        Ok(())
    }

    /// Get config
    pub fn config(&self) -> &Web2PptConfig {
        &self.config
    }
}

impl Default for Web2Ppt {
    fn default() -> Self {
        Self::new()
    }
}

/// High-level function to convert a URL to PPTX bytes
#[cfg(feature = "web2ppt")]
pub fn url_to_pptx(url: &str) -> Result<Vec<u8>> {
    url_to_pptx_with_options(url, Web2PptConfig::default(), ConversionOptions::default())
}

/// High-level function to convert a URL to PPTX bytes with options
#[cfg(feature = "web2ppt")]
pub fn url_to_pptx_with_options(
    url: &str,
    config: Web2PptConfig,
    options: ConversionOptions,
) -> Result<Vec<u8>> {
    use super::{WebFetcher, WebParser};

    // Fetch
    let fetcher = WebFetcher::with_config(config.clone())?;
    let html = fetcher.fetch(url)?;

    // Parse
    let parser = WebParser::with_config(config.clone());
    let content = parser.parse(&html, url)?;

    // Convert
    let converter = Web2Ppt::with_config(config);
    converter.convert(&content, &options)
}

/// Convert HTML string to PPTX bytes
pub fn html_to_pptx(html: &str, url: &str) -> Result<Vec<u8>> {
    html_to_pptx_with_options(html, url, Web2PptConfig::default(), ConversionOptions::default())
}

/// Convert HTML string to PPTX bytes with options
pub fn html_to_pptx_with_options(
    html: &str,
    url: &str,
    config: Web2PptConfig,
    options: ConversionOptions,
) -> Result<Vec<u8>> {
    use super::WebParser;

    // Parse
    let parser = WebParser::with_config(config.clone());
    let content = parser.parse(html, url)?;

    // Convert
    let converter = Web2Ppt::with_config(config);
    converter.convert(&content, &options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_options() {
        let options = ConversionOptions::new()
            .title("Custom Title")
            .author("Test Author")
            .with_source_url(false);

        assert_eq!(options.title, Some("Custom Title".to_string()));
        assert_eq!(options.author, Some("Test Author".to_string()));
        assert!(!options.include_source_url);
    }

    #[test]
    fn test_html_to_pptx() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head><title>Test Page</title></head>
            <body>
                <h1>Main Title</h1>
                <p>This is a paragraph with enough text to be included in the presentation.</p>
                <h2>Section 1</h2>
                <p>Section 1 content with enough text to be included in the presentation.</p>
                <ul>
                    <li>Item 1</li>
                    <li>Item 2</li>
                </ul>
            </body>
            </html>
        "#;

        let result = html_to_pptx(html, "https://example.com");
        assert!(result.is_ok());

        let pptx = result.unwrap();
        assert!(!pptx.is_empty());
    }

    #[test]
    fn test_web2ppt_config() {
        let config = Web2PptConfig::new()
            .max_slides(5)
            .max_bullets(3);

        let converter = Web2Ppt::with_config(config);
        assert_eq!(converter.config().max_slides, 5);
        assert_eq!(converter.config().max_bullets_per_slide, 3);
    }
}
