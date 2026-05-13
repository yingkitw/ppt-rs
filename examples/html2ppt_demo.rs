//! HTML to PowerPoint (html2ppt) Example
//!
//! Demonstrates two ways to convert HTML to PowerPoint:
//!   1. Using the programmatic API (parse_html → create_pptx_with_content)
//!   2. Using the Html2Ppt converter struct with options
//!
//! Run with: cargo run --example html2ppt_demo
//!
//! CLI alternative:
//!   cargo run --bin pptcli -- html2ppt examples/rust_intro.html examples/output/rust_intro.pptx

use std::fs;
use std::path::Path;

use ppt_rs::generator::create_pptx_with_content;
use ppt_rs::import::parse_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("examples/output")?;
    println!("HTML to PowerPoint Demo\n");

    // ---------------------------------------------------------------
    // Example 1: Basic HTML → PPTX with the simple parse_html() API
    // ---------------------------------------------------------------
    println!("--- Example 1: Quick parse_html() ---");

    let html = r#"
        <h1>Quick Demo</h1>
        <p>This slide was created from an inline HTML string.</p>
        <ul>
            <li>Fast and easy</li>
            <li>No external files needed</li>
        </ul>
        <h1>Features</h1>
        <p>HTML elements map to PowerPoint elements:</p>
        <table>
            <tr><th>HTML</th><th>PPT</th></tr>
            <tr><td>&lt;h1&gt;</td><td>Slide title</td></tr>
            <tr><td>&lt;p&gt;</td><td>Bullet point</td></tr>
            <tr><td>&lt;ul&gt;/&lt;ol&gt;</td><td>Lists</td></tr>
            <tr><td>&lt;table&gt;</td><td>Table with styled header</td></tr>
        </table>
    "#;

    let slides = parse_html(html)?;
    let pptx_data = create_pptx_with_content("Quick Example", slides)?;
    let path = "examples/output/html2ppt_quick.pptx";
    fs::write(path, &pptx_data)?;
    println!("  ✓ Created {} ({} bytes)\n", path, pptx_data.len());

    // ---------------------------------------------------------------
    // Example 2: HTML from a file with Html2Ppt converter
    // ---------------------------------------------------------------
    println!("--- Example 2: Html2Ppt converter from file ---");

    use ppt_rs::import::{Html2Ppt, HtmlParseOptions};

    let options = HtmlParseOptions::new()
        .max_slides(20)
        .max_bullets(8);

    let converter = Html2Ppt::with_options(options);

    // Parse the example HTML file
    let sample_path = "examples/rust_intro.html";
    if Path::new(sample_path).exists() {
        let slides = converter.parse_file(sample_path)?;
        let pptx_data = create_pptx_with_content("Rust Programming Language", slides)?;
        let path = "examples/output/html2ppt_rust_intro.pptx";
        fs::write(path, &pptx_data)?;
        println!("  ✓ Created {} ({} bytes)", path, pptx_data.len());
        println!("  Source: {sample_path}");
    } else {
        println!("  ⚠ Sample file {sample_path} not found, skipping");
    }

    // ---------------------------------------------------------------
    // Example 3: Advanced HTML with code, notes, and images
    // ---------------------------------------------------------------
    println!("\n--- Example 3: Advanced features ---");

    let html = r#"
        <h1>Advanced Features</h1>
        <p>HTML2PPT supports speaker notes via &lt;blockquote&gt;.</p>
        <pre><code>// Code blocks are preserved
fn calculate(x: i32) -> i32 {
    x * 2
}</code></pre>
        <blockquote>This text becomes a speaker note in the presentation.</blockquote>

        <h1>Break Slides with &lt;hr&gt;</h1>
        <p>Content before the break.</p>
        <hr>
        <p>Content after the break appears on a new slide.</p>
    "#;

    let slides = parse_html(html)?;
    let pptx_data = create_pptx_with_content("Advanced Demo", slides)?;
    let path = "examples/output/html2ppt_advanced.pptx";
    fs::write(path, &pptx_data)?;
    println!("  ✓ Created {} ({} bytes)\n", path, pptx_data.len());

    println!("Done! Open the .pptx files in PowerPoint, LibreOffice, or Google Slides.");
    Ok(())
}
