use crate::api::Presentation;
use crate::generator::{SlideContent, Image};
use crate::exc::Result;

/// Export a presentation to a single HTML file
pub fn export_to_html(presentation: &Presentation) -> Result<String> {
    let mut html = String::new();
    
    // Header
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str(&format!("<title>{}</title>\n", presentation.get_title()));
    
    // CSS
    html.push_str("<style>\n");
    html.push_str(include_str!("html_style.css"));
    html.push_str("</style>\n");
    
    html.push_str("</head>\n<body>\n");
    
    // Title Slide (Presentation Title)
    html.push_str("<div class=\"slide title-slide\">\n");
    html.push_str(&format!("<h1>{}</h1>\n", presentation.get_title()));
    html.push_str("</div>\n");
    
    // Slides
    for (i, slide) in presentation.slides().iter().enumerate() {
        html.push_str(&render_slide(slide, i + 1));
    }
    
    html.push_str("</body>\n</html>");
    
    Ok(html)
}

fn render_slide(slide: &SlideContent, index: usize) -> String {
    let mut html = String::new();
    
    html.push_str(&format!("<div class=\"slide\" id=\"slide-{}\">\n", index));
    
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
    
    // Images
    for image in &slide.images {
        if let Some(img_html) = render_image(image) {
            html.push_str(&img_html);
        }
    }
    
    // Code Blocks
    for code in &slide.code_blocks {
        html.push_str("<pre><code>");
        html.push_str(&code.code);
        html.push_str("</code></pre>\n");
    }
    
    html.push_str("</div>\n"); // content
    html.push_str("</div>\n"); // slide
    
    html
}

fn render_image(image: &Image) -> Option<String> {
    let bytes = image.get_bytes()?;
    let b64 = base64_encode(&bytes);
    let mime = match image.format.to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        _ => "application/octet-stream",
    };
    
    // Calculate style for positioning
    // Converting EMUs to percentage or px is tricky without context of slide size.
    // For simple HTML export, we might just display images inline or block.
    // Or we can try to use absolute positioning if we assume a fixed slide size (e.g. 16:9 aspect ratio).
    // Let's use simple block display for now to be safe.
    
    Some(format!(
        "<div class=\"image-container\"><img src=\"data:{};base64,{}\" alt=\"{}\" /></div>\n",
        mime, b64, image.filename
    ))
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
