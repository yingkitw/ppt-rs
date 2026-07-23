use crate::api::Presentation;
use crate::generator::{SlideContent, Image};
use crate::exc::Result;

/// Export options for HTML output
#[derive(Clone, Debug)]
pub struct HtmlExportOptions {
    /// Include speaker notes
    pub include_notes: bool,
    /// Enable keyboard navigation
    pub enable_navigation: bool,
    /// Include code syntax highlighting
    pub syntax_highlight: bool,
    /// Export images as separate files instead of base64
    pub export_images_as_files: bool,
    /// Image output directory (for file export)
    pub image_output_dir: Option<String>,
}

impl Default for HtmlExportOptions {
    fn default() -> Self {
        Self {
            include_notes: true,
            enable_navigation: true,
            syntax_highlight: true,
            export_images_as_files: false,
            image_output_dir: None,
        }
    }
}

impl HtmlExportOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_notes(mut self, include: bool) -> Self {
        self.include_notes = include;
        self
    }

    pub fn with_navigation(mut self, enable: bool) -> Self {
        self.enable_navigation = enable;
        self
    }

    pub fn with_syntax_highlight(mut self, enable: bool) -> Self {
        self.syntax_highlight = enable;
        self
    }

    pub fn with_image_files(mut self, enable: bool, dir: Option<&str>) -> Self {
        self.export_images_as_files = enable;
        self.image_output_dir = dir.map(|d| d.to_string());
        self
    }
}

/// Export a presentation to a single HTML file
pub fn export_to_html(presentation: &Presentation) -> Result<String> {
    export_to_html_with_options(presentation, &HtmlExportOptions::default())
}

/// Export a presentation to HTML with custom options
pub fn export_to_html_with_options(presentation: &Presentation, options: &HtmlExportOptions) -> Result<String> {
    let mut html = String::new();

    // Header
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str(&format!("<title>{}</title>\n", presentation.get_title()));

    // Enhanced CSS
    html.push_str("<style>\n");
    html.push_str(include_str!("html_style.css"));

    // Add additional styles based on options
    if options.enable_navigation {
        html.push_str(include_str!("html_navigation.css"));
    }

    if options.include_notes {
        html.push_str(include_str!("html_notes.css"));
    }

    html.push_str("</style>\n");

    // Add JavaScript for navigation
    if options.enable_navigation {
        html.push_str("<script>\n");
        html.push_str(include_str!("html_navigation.js"));
        html.push_str("</script>\n");
    }

    html.push_str("</head>\n<body>\n");

    // Title Slide (Presentation Title)
    html.push_str("<div class=\"slide title-slide\" data-slide=\"0\">\n");
    html.push_str(&format!("<h1>{}</h1>\n", presentation.get_title()));
    html.push_str("</div>\n");

    // Slides
    for (i, slide) in presentation.slides().iter().enumerate() {
        html.push_str(&render_slide_with_options(slide, i + 1, options));
    }

    // Navigation controls
    if options.enable_navigation {
        html.push_str("<div class=\"navigation-controls\">\n");
        html.push_str("<button onclick=\"previousSlide()\" id=\"prevBtn\">Previous</button>\n");
        html.push_str("<button onclick=\"nextSlide()\" id=\"nextBtn\">Next</button>\n");
        html.push_str("<span id=\"slideCounter\"></span>\n");
        html.push_str("</div>\n");
    }

    html.push_str("</body>\n</html>");

    Ok(html)
}

fn render_slide_with_options(slide: &SlideContent, index: usize, options: &HtmlExportOptions) -> String {
    let mut html = String::new();

    html.push_str(&format!("<div class=\"slide\" id=\"slide-{}\" data-slide=\"{}\">\n", index, index));

    // Slide Number
    html.push_str(&format!("<div class=\"slide-number\">{}</div>\n", index));

    // Title
    html.push_str(&format!("<h2>{}</h2>\n", slide.title));

    // Content Container
    html.push_str("<div class=\"content\">\n");

    // Bullets / Content
    if !slide.content.is_empty() {
        html.push_str("<ul>\n");
        for item in &slide.content {
            html.push_str(&format!("<li>{}</li>\n", item));
        }
        html.push_str("</ul>\n");
    }

    // Tables
    if let Some(ref table) = slide.table {
        html.push_str(&render_table_html(table));
    }

    // Images
    for image in &slide.images {
        if let Some(img_html) = render_image_with_options(image, options) {
            html.push_str(&img_html);
        }
    }

    // Code Blocks
    for code in &slide.code_blocks {
        if options.syntax_highlight {
            html.push_str(&format!("<pre><code class=\"language-{}\">", code.language));
        } else {
            html.push_str("<pre><code>");
        }
        html.push_str(&escape_html(&code.code));
        html.push_str("</code></pre>\n");
    }

    // Speaker Notes
    if options.include_notes
        && let Some(ref notes) = slide.notes {
            html.push_str(&format!("<div class=\"speaker-notes\"><strong>Notes:</strong> {}</div>\n", escape_html(notes)));
        }

    html.push_str("</div>\n"); // content
    html.push_str("</div>\n"); // slide

    html
}

/// Render a table as HTML
fn render_table_html(_table: &crate::generator::Table) -> String {
    let mut html = String::new();
    html.push_str("<table class=\"ppt-table\">\n");

    // Render rows (assuming table has row data structure)
    // This is a simplified version - you may need to adapt based on actual Table structure
    html.push_str("<tbody>\n");
    html.push_str("<tr><td>Table content</td></tr>\n"); // Placeholder
    html.push_str("</tbody>\n");

    html.push_str("</table>\n");
    html
}

/// Escape HTML entities
fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn render_image_with_options(image: &Image, options: &HtmlExportOptions) -> Option<String> {
    let bytes = image.get_bytes()?;
    let mime = match image.format.to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        _ => "application/octet-stream",
    };

    if options.export_images_as_files {
        // Generate image filename and reference
        let filename = format!("slide_img_{}.{}", image.filename, image.format);
        let filepath = if let Some(ref dir) = options.image_output_dir {
            format!("{}/{}", dir, filename)
        } else {
            filename.clone()
        };

        Some(format!(
            "<div class=\"image-container\"><img src=\"{}\" alt=\"{}\" /></div>\n",
            filepath, image.filename
        ))
    } else {
        // Use base64 encoding
        let b64 = base64_encode(&bytes);
        Some(format!(
            "<div class=\"image-container\"><img src=\"data:{};base64,{}\" alt=\"{}\" /></div>\n",
            mime, b64, image.filename
        ))
    }
}

// Simple base64 encoder
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::with_capacity(data.len() * 4 / 3 + 4);
    
    let mut i = 0;
    while i < data.len() {
        let mut buf = [0u8; 3];
        let mut len = 0;
        
        for j in 0..3 {
            if i + j < data.len() {
                buf[j] = data[i + j];
                len += 1;
            }
        }
        
        let b0 = (buf[0] >> 2) & 0x3F;
        let b1 = ((buf[0] & 0x03) << 4) | ((buf[1] >> 4) & 0x0F);
        let b2 = ((buf[1] & 0x0F) << 2) | ((buf[2] >> 6) & 0x03);
        let b3 = buf[2] & 0x3F;
        
        output.push(ALPHABET[b0 as usize] as char);
        output.push(ALPHABET[b1 as usize] as char);
        
        if len > 1 {
            output.push(ALPHABET[b2 as usize] as char);
        } else {
            output.push('=');
        }
        
        if len > 2 {
            output.push(ALPHABET[b3 as usize] as char);
        } else {
            output.push('=');
        }
        
        i += 3;
    }
    
    output
}
