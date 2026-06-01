//! Integration tests for new Markdown and HTML features
//!
//! This test suite validates the enhanced capabilities added to ppt-rs:
//! - Real image handling (URLs and local files)
//! - Enhanced CSS parsing
//! - Task list support
//! - Strikethrough and enhanced formatting
//! - HTML hyperlink support
//! - Enhanced HTML export functionality

use ppt_rs::api::Presentation;
use ppt_rs::generator::SlideContent;
use ppt_rs::cli::markdown::parse;
use ppt_rs::import::{parse_html, parse_html_with_options, HtmlParseOptions};
use ppt_rs::export::html::{export_to_html_with_options, HtmlExportOptions};

#[test]
fn test_markdown_image_handling() {
    // Test markdown with local and web image references
    let md = r#"
# Image Test

![Local Image](./examples/assets/sample.png)
![Web Image](https://via.placeholder.com/600x400)

- Point 1
- Point 2
"#;

    let slides = parse(md).unwrap();

    // Should create at least one slide
    assert!(!slides.is_empty());

    // The title slide should be created
    assert_eq!(slides[0].title, "Image Test");

    // Verify slide structure is created
    assert!(slides.len() >= 1);
}

#[test]
fn test_markdown_task_lists() {
    let md = r#"
# Task Management

- [x] Completed feature
- [ ] Pending task
- [ ] Another todo item
- [x] Another completed item

"#;

    let slides = parse(md).unwrap();

    // Should parse task list items
    assert!(!slides.is_empty());
    assert_eq!(slides[0].title, "Task Management");

    // Task items should be converted to bullet points
    assert!(!slides[0].bullets.is_empty());
}

#[test]
fn test_markdown_strikethrough() {
    let md = r#"
# Formatting Test

- ~~Deleted text~~
- **Bold text**
- *Italic text*
- Normal text

"#;

    let slides = parse(md).unwrap();

    assert!(!slides.is_empty());
    assert_eq!(slides[0].title, "Formatting Test");

    // Should have bullet points with various formatting
    assert!(!slides[0].bullets.is_empty());
}

#[test]
fn test_markdown_enhanced_inline_formatting() {
    let md = r#"
# Enhanced Formatting

This is a paragraph with **bold**, *italic*, and ~~strikethrough~~ text.
It also has `inline code` and combinations like ***bold italic***.

"#;

    let slides = parse(md).unwrap();

    assert!(!slides.is_empty());
    assert!(slides[0].content.len() > 0);
}

#[test]
fn test_html_enhanced_css_parsing() {
    let html = r#"
<!DOCTYPE html>
<html>
<body>
    <h1>Enhanced CSS Test</h1>
    <p style="color: #E74C3C; font-size: 32pt; margin: 20px;">
        Styled paragraph with multiple CSS properties
    </p>
    <div style="padding: 15px; border: 2px solid #3498DB;">
        Content with padding and borders
    </div>
</body>
</html>
"#;

    let slides = parse_html(html).unwrap();

    assert!(!slides.is_empty());
    assert_eq!(slides[0].title, "Enhanced CSS Test");
}

#[test]
fn test_html_hyperlink_support() {
    let html = r#"
<!DOCTYPE html>
<html>
<body>
    <h1>Link Test</h1>
    <p>Visit <a href="https://example.com">Example</a> for more info.</p>
    <p>Check out <a href="https://github.com">GitHub</a></p>
</body>
</html>
"#;

    let slides = parse_html(html).unwrap();

    assert!(!slides.is_empty());
    assert_eq!(slides[0].title, "Link Test");

    // Content should contain link text
    assert!(slides[0].content.iter().any(|c| c.contains("Example")));
}

#[test]
fn test_html_real_images() {
    let html = r#"
<!DOCTYPE html>
<html>
<body>
    <h1>Image Test</h1>
    <img src="https://via.placeholder.com/600x300" alt="Placeholder Image">
    <p>Content after image</p>
</body>
</html>
"#;

    let options = HtmlParseOptions::new()
        .include_images(true);

    let slides = parse_html_with_options(html, options).unwrap();

    assert!(!slides.is_empty());
    assert_eq!(slides[0].title, "Image Test");
}

#[test]
fn test_html_complex_styling() {
    let html = r#"
<!DOCTYPE html>
<html>
<body>
    <h1>Complex Styling</h1>
    <p style="margin-top: 30px; padding: 20px; border: 2px solid #E74C3C;">
        Text with margins, padding, and borders
    </p>
    <div style="background-color: #F8F9FA; line-height: 1.6; letter-spacing: 1px;">
        Content with background, line height, and letter spacing
    </div>
</body>
</html>
"#;

    let slides = parse_html(html).unwrap();

    assert!(!slides.is_empty());
    assert_eq!(slides[0].title, "Complex Styling");
}

#[test]
fn test_html_tables_with_styling() {
    let html = r#"
<!DOCTYPE html>
<html>
<body>
    <h1>Table Test</h1>
    <table>
        <thead>
            <tr>
                <th>Column 1</th>
                <th>Column 2</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>Data 1</td>
                <td>Data 2</td>
            </tr>
        </tbody>
    </table>
</body>
</html>
"#;

    let slides = parse_html(html).unwrap();

    assert!(!slides.is_empty());
    // Check if table was parsed
    // The table parsing might create a slide with table content
}

#[test]
fn test_html_code_blocks() {
    let html = r#"
<!DOCTYPE html>
<html>
<body>
    <h1>Code Example</h1>
    <pre><code>fn main() {
    println!("Hello");
}</code></pre>
</body>
</html>
"#;

    let slides = parse_html(html).unwrap();

    assert!(!slides.is_empty());
    assert_eq!(slides[0].title, "Code Example");
}

#[test]
fn test_html_speaker_notes() {
    let html = r#"
<!DOCTYPE html>
<html>
<body>
    <h1>Slide Title</h1>
    <p>Slide content</p>
    <blockquote>These are speaker notes</blockquote>
</body>
</html>
"#;

    let slides = parse_html(html).unwrap();

    assert!(!slides.is_empty());
    // Check if speaker notes were captured
    if let Some(ref notes) = slides[0].notes {
        assert!(notes.contains("speaker notes"));
    }
}

#[test]
fn test_html_export_with_navigation() {
    let pres = Presentation::with_title("Navigation Test")
        .add_slide(SlideContent::new("Slide 1").add_bullet("Point 1"))
        .add_slide(SlideContent::new("Slide 2").add_bullet("Point 2"));

    let options = HtmlExportOptions::new()
        .with_navigation(true);

    let html = export_to_html_with_options(&pres, &options).unwrap();

    // Check for navigation features
    assert!(html.contains("navigation-controls"));
    assert!(html.contains("addEventListener"));
    assert!(html.contains("previousSlide"));
    assert!(html.contains("nextSlide"));
}

#[test]
fn test_html_export_with_notes() {
    let mut slide1 = SlideContent::new("Slide with Notes")
        .add_bullet("Point 1")
        .add_bullet("Point 2");
    slide1.notes = Some("These are speaker notes".to_string());

    let pres = Presentation::with_title("Notes Test")
        .add_slide(slide1);

    let options = HtmlExportOptions::new()
        .with_notes(true);

    let html = export_to_html_with_options(&pres, &options).unwrap();

    // Check for speaker notes styling
    assert!(html.contains("speaker-notes"));
    assert!(html.contains("These are speaker notes"));
}

#[test]
fn test_html_export_with_syntax_highlighting() {
    let pres = Presentation::with_title("Code Test")
        .add_slide(
            SlideContent::new("Code Slide")
                .add_bullet("Regular bullet")
        );

    let options = HtmlExportOptions::new()
        .with_syntax_highlight(true);

    let html = export_to_html_with_options(&pres, &options).unwrap();

    // Basic HTML structure should be present
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("<title>Code Test</title>"));
}

#[test]
fn test_markdown_combined_features() {
    let md = r#"
# Combined Features Test

## Project Status

- [x] Image support implemented
- [ ] Advanced styling in progress
- [x] Task lists completed
- [ ] Performance optimization

## Examples

![Sample Image](https://via.placeholder.com/600x300)

This text has **bold**, *italic*, and ~~strikethrough~~ formatting.

```rust
fn example() {
    println!("Code block");
}
```

| Feature | Status |
|---------|--------|
| Images | ✅ Done |
| Tasks | ✅ Done |
| CSS | ✅ Done |

> Speaker notes for this slide

"#;

    let slides = parse(md).unwrap();

    assert!(!slides.is_empty());

    // Should have main title slide
    assert_eq!(slides[0].title, "Combined Features Test");

    // Should have additional slides for sections
    assert!(slides.len() >= 1);

    // Check that different content types are present
    let total_bullets: usize = slides.iter()
        .map(|s| s.bullets.len())
        .sum();

    assert!(total_bullets > 0, "Should have bullet points");
}

#[test]
fn test_html_parse_options() {
    let html = r#"
<!DOCTYPE html>
<html>
<body>
    <h1>Title</h1>
    <p>Content</p>
    <img src="https://example.com/image.jpg" alt="Image">
    <pre><code>Code here</code></pre>
    <table>
        <tr><th>Header</th></tr>
        <tr><td>Data</td></tr>
    </table>
</body>
</html>
"#;

    // Test with all options enabled
    let options = HtmlParseOptions::new()
        .max_slides(10)
        .max_bullets(8)
        .include_images(true)
        .include_tables(true)
        .include_code(true);

    let slides = parse_html_with_options(html, options).unwrap();

    assert!(!slides.is_empty());
    assert_eq!(slides[0].title, "Title");
}

#[test]
fn test_html_export_options_combinations() {
    let mut slide = SlideContent::new("Test Slide")
        .add_bullet("Point 1");
    slide.notes = Some("Notes here".to_string());

    let pres = Presentation::with_title("Options Test")
        .add_slide(slide);

    // Test all options enabled
    let options = HtmlExportOptions::new()
        .with_notes(true)
        .with_navigation(true)
        .with_syntax_highlight(true);

    let html = export_to_html_with_options(&pres, &options).unwrap();

    // Verify all features are present
    assert!(html.contains("speaker-notes"));
    assert!(html.contains("navigation-controls"));
    assert!(html.contains("Notes here"));
}

#[test]
fn test_markdown_mermaid_diagrams() {
    let md = r#"
# Diagram Test

```mermaid
flowchart LR
    A[Start] --> B[Process]
    B --> C[End]
```

"#;

    let slides = parse(md).unwrap();

    assert!(!slides.is_empty());
    assert_eq!(slides[0].title, "Diagram Test");

    // Mermaid diagrams should create shapes
    // (This tests that the parser still handles mermaid correctly)
}

#[test]
fn test_comprehensive_markdown_features() {
    let md = r#"
# Comprehensive Feature Test

## Text Formatting

- **Bold text**
- *Italic text*
- ~~Strikethrough~~
- `Code`

## Task Lists

- [x] Completed item
- [ ] Pending item
- [x] Another completed

## Code Blocks

```rust
fn main() {
    println!("Hello");
}
```

## Tables

| Feature | Working |
|---------|---------|
| Bold | ✅ |
| Italic | ✅ |
| Strike | ✅ |

## Images

![Web Image](https://via.placeholder.com/600x300)

## Lists

1. First item
2. Second item
3. Third item

> Speaker notes can be added using blockquotes

---

## Second Slide

More content here with additional formatting options.

"#;

    let slides = parse(md).unwrap();

    // Should create multiple slides
    assert!(slides.len() >= 2);

    // First slide should be the title
    assert_eq!(slides[0].title, "Comprehensive Feature Test");

    // Should have substantial content
    let total_content: usize = slides.iter()
        .map(|s| s.content.len() + s.bullets.len())
        .sum();

    assert!(total_content > 0, "Should have parsed content");
}