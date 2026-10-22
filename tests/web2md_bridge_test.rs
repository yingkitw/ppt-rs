//! Integration tests for the web2md bridge pipeline.
//!
//! These tests exercise the HTML → Markdown → Slides → PPTX pipeline
//! provided by the `web2md` feature.

#![cfg(feature = "web2md")]

use ppt_rs::{ConversionOptions, Web2PptConfig, html_to_pptx_via_web2md};
use std::io::Cursor;
use zip::ZipArchive;

fn assert_valid_pptx(data: &[u8]) {
    let cursor = Cursor::new(data.to_vec());
    let zip = ZipArchive::new(cursor).expect("Should be valid ZIP");
    let mut names: Vec<_> = zip
        .file_names()
        .map(|name| name.unwrap().to_string())
        .collect();
    names.sort();
    assert!(
        names.iter().any(|n| n.starts_with("ppt/slides/slide")),
        "Should contain slide parts, got: {:?}",
        names
    );
}

#[test]
fn test_html_to_pptx_via_web2md_basic() {
    let html = r#"<!DOCTYPE html>
<html>
<head><title>Test Page</title></head>
<body>
<article>
<h1>Main Title</h1>
<p>This is a paragraph with enough text to be included in the presentation.</p>
<h2>Section 1</h2>
<p>Section 1 content with enough text to be included in the presentation.</p>
<ul>
<li>Item 1</li>
<li>Item 2</li>
</ul>
</article>
</body>
</html>"#;

    let config = Web2PptConfig::default();
    let options = ConversionOptions::default();
    let result = html_to_pptx_via_web2md(html, "https://example.com", config, &options);
    assert!(result.is_ok(), "Expected Ok, got: {:?}", result.err());
    let pptx = result.unwrap();
    assert!(!pptx.is_empty());
    assert_valid_pptx(&pptx);
}

#[test]
fn test_html_to_pptx_via_web2md_custom_title() {
    let html = r#"<html><body><article>
<h1>Original Title</h1>
<p>Some content here that is long enough to be included in the output.</p>
</article></body></html>"#;

    let config = Web2PptConfig::default();
    let options = ConversionOptions::new().title("Custom Title");
    let result = html_to_pptx_via_web2md(html, "https://example.com", config, &options);
    assert!(result.is_ok());
    let pptx = result.unwrap();
    assert_valid_pptx(&pptx);
}

#[test]
fn test_html_to_pptx_via_web2md_empty_content() {
    let html = "<html><body></body></html>";
    let config = Web2PptConfig::default();
    let options = ConversionOptions::default();
    let result = html_to_pptx_via_web2md(html, "https://example.com", config, &options);
    assert!(result.is_err(), "Expected error for empty content");
}

#[test]
fn test_html_to_pptx_via_web2md_max_slides_limit() {
    let mut html = String::from("<html><body><article>");
    for i in 1..=50 {
        html.push_str(&format!(
            "<h1>Slide {i}</h1>\n<p>Content for slide {i} with enough text.</p>\n"
        ));
    }
    html.push_str("</article></body></html>");

    let config = Web2PptConfig::default().max_slides(5);
    let options = ConversionOptions::default();
    let result = html_to_pptx_via_web2md(&html, "https://example.com", config, &options);
    assert!(result.is_ok());
    let pptx = result.unwrap();
    assert_valid_pptx(&pptx);
}

#[test]
fn test_html_to_pptx_via_web2md_with_tables() {
    let html = r#"<html><body><article>
<h1>Data Report</h1>
<p>Some introductory text that is long enough to be included.</p>
<table>
<tr><th>Name</th><th>Value</th></tr>
<tr><td>A</td><td>100</td></tr>
<tr><td>B</td><td>200</td></tr>
</table>
</article></body></html>"#;

    let config = Web2PptConfig::default();
    let options = ConversionOptions::default();
    let result = html_to_pptx_via_web2md(html, "https://example.com", config, &options);
    assert!(result.is_ok());
    let pptx = result.unwrap();
    assert_valid_pptx(&pptx);
}

#[test]
fn test_html_to_pptx_via_web2md_no_images() {
    let html = r#"<html><body><article>
<h1>Article Title</h1>
<p>Content paragraph that is long enough to pass the threshold for inclusion.</p>
<img src="https://example.com/image.png" alt="Test Image">
</article></body></html>"#;

    let config = Web2PptConfig::default().with_images(false);
    let options = ConversionOptions::default();
    let result = html_to_pptx_via_web2md(html, "https://example.com", config, &options);
    assert!(result.is_ok());
    let pptx = result.unwrap();
    assert_valid_pptx(&pptx);
}
