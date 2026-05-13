//! Comprehensive html2ppt Example
//!
//! Demonstrates all HTML-to-PPTX APIs and features:
//!   1. Quick inline HTML with parse_html()
//!   2. Html2Ppt struct with custom options
//!   3. Html2Ppt::parse_file() with the comprehensive fixture
//!   4. Presentation API builder pattern from parsed slides
//!   5. Error handling
//!   6. PPTX validation (showing file sizes)
//!
//! Run with: cargo run --example html2ppt_comprehensive

use std::fs;
use std::path::Path;

use ppt_rs::api::Presentation;
use ppt_rs::generator::create_pptx_with_content;
use ppt_rs::import::{parse_html, parse_html_with_options, Html2Ppt, HtmlParseOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("examples/output")?;
    println!("=== Comprehensive html2ppt Demo ===\n");

    // ---------------------------------------------------------------
    // 1. Quick inline HTML with parse_html()
    // ---------------------------------------------------------------
    println!("--- 1. Quick parse_html() from inline string ---");

    let html = r#"
        <h1>Quick Start</h1>
        <p>Parse HTML directly from a string.</p>
        <ul>
            <li>No file I/O needed</li>
            <li>Great for testing</li>
        </ul>
    "#;

    let slides = parse_html(html)?;
    let pptx_data = create_pptx_with_content("Quick Start", slides)?;
    let path = "examples/output/comprehensive_quick.pptx";
    fs::write(path, &pptx_data)?;
    println!("  ✓ {} ({} bytes, {} slides)", path, pptx_data.len(), 1);

    // ---------------------------------------------------------------
    // 2. Html2Ppt struct with custom options
    // ---------------------------------------------------------------
    println!("\n--- 2. Html2Ppt struct with custom options ---");

    let opts = HtmlParseOptions::new()
        .max_slides(5)
        .max_bullets(6)
        .include_code(true)
        .include_tables(true)
        .include_images(true);

    let converter = Html2Ppt::with_options(opts);

    let html = r#"
        <h1>Options Demo</h1>
        <p>This demo limits slides and bullets.</p>
        <p>Only 6 bullets will appear per slide.</p>
        <p>Item 1</p><p>Item 2</p><p>Item 3</p>
        <p>Item 4</p><p>Item 5</p><p>Item 6</p><p>Item 7</p>
        <h1>Slide Two</h1>
        <p>Content on second slide.</p>
    "#;

    let slides = converter.parse(html)?;
    let pptx_data = create_pptx_with_content("Options Demo", slides)?;
    let path = "examples/output/comprehensive_options.pptx";
    fs::write(path, &pptx_data)?;
    println!("  ✓ {} ({} bytes, {} slides)", path, pptx_data.len(), 2);
    println!("     (first slide limited to 6 bullets by max_bullets)");

    // ---------------------------------------------------------------
    // 3. Parse the comprehensive fixture from file
    // ---------------------------------------------------------------
    println!("\n--- 3. Parse comprehensive fixture from file ---");

    let fixture_path = "tests/fixtures/html/comprehensive.html";
    if Path::new(fixture_path).exists() {
        let slides = Html2Ppt::new().parse_file(fixture_path)?;
        println!("  Parsed {} slides from {fixture_path}", slides.len());

        for (i, slide) in slides.iter().enumerate() {
            println!("  Slide {}: \"{}\"", i + 1, slide.title);
            if !slide.content.is_empty() {
                println!("         bullets: {}", slide.content.len());
            }
            if slide.table.is_some() {
                println!("         has table ✓");
            }
            if !slide.code_blocks.is_empty() {
                println!("         has code ✓");
            }
            if slide.notes.is_some() {
                println!("         has speaker notes ✓");
            }
            if slide.title_color.is_some() {
                println!("         title color: {} ✓", slide.title_color.as_ref().unwrap());
            }
            // Count bullets with inline styles
            let styled = slide.bullets.iter().filter(|b| b.format.is_some()).count();
            if styled > 0 {
                println!("         bullets with inline styles: {}/{}", styled, slide.bullets.len());
            }
        }

        let pptx_data = create_pptx_with_content("Comprehensive Demo", slides)?;
        let path = "examples/output/comprehensive_fixture.pptx";
        fs::write(path, &pptx_data)?;
        println!("  ✓ {} ({} bytes)", path, pptx_data.len());
    } else {
        println!("  ⚠ Fixture {fixture_path} not found, skipping");
    }

    // ---------------------------------------------------------------
    // 4. Presentation API builder pattern
    // ---------------------------------------------------------------
    println!("\n--- 4. Presentation API builder pattern ---");

    let html = r#"
        <h1>API Builder</h1>
        <p>Using the Presentation API to build slides.</p>
        <h1>Second Slide</h1>
        <ol>
            <li>Builder pattern</li>
            <li>Add slides individually</li>
            <li>Control ordering</li>
        </ol>
        <h1>Third Slide</h1>
        <table>
            <tr><th>Feature</th><th>Status</th></tr>
            <tr><td>parse_html</td><td>Ready</td></tr>
            <tr><td>Presentation API</td><td>Ready</td></tr>
        </table>
    "#;

    let slides = parse_html(html)?;

    let mut pres = Presentation::with_title("Presentation API Demo");
    for slide in slides {
        pres = pres.add_slide(slide);
    }

    println!("  Slides: {}", pres.slide_count());
    println!("  Title: \"{}\"", pres.get_title());

    let pptx_data = pres.build()?;
    let path = "examples/output/comprehensive_api.pptx";
    pres.save(path)?;
    println!("  ✓ {} ({} bytes)", path, pptx_data.len());

    // Re-import and verify round-trip
    let imported = Presentation::from_path(path)?;
    assert_eq!(imported.slide_count(), pres.slide_count());
    assert_eq!(imported.slides()[0].title, "API Builder");
    println!("  ✓ Round-trip verified: {} slides preserved", imported.slide_count());

    // ---------------------------------------------------------------
    // 5. Error handling
    // ---------------------------------------------------------------
    println!("\n--- 5. Error handling ---");

    // Empty HTML should error
    match parse_html("<html></html>") {
        Err(e) => println!("  ✓ Empty HTML correctly errors: {e}"),
        Ok(_) => println!("  ⚠ Empty HTML should have errored"),
    }

    // Non-existent file
    match Html2Ppt::new().parse_file("/nonexistent/path.html") {
        Err(e) => println!("  ✓ Missing file correctly errors: {e}"),
        Ok(_) => println!("  ⚠ Missing file should have errored"),
    }

    // ---------------------------------------------------------------
    // 6. Disable features with options
    // ---------------------------------------------------------------
    println!("\n--- 6. Feature toggling with HtmlParseOptions ---");

    let html_with_features = r#"
        <h1>Feature Toggles</h1>
        <p>Text content</p>
        <img src="chart.png" alt="Chart">
        <table>
            <tr><td>Hidden</td></tr>
        </table>
        <pre><code>Hidden code</code></pre>
    "#;

    // All features disabled
    let opts = HtmlParseOptions::new()
        .include_images(false)
        .include_tables(false)
        .include_code(false);
    let slides = parse_html_with_options(html_with_features, opts)?;
    let has_image = slides[0].content.iter().any(|c| c.contains("[Image:"));
    let has_table = slides[0].table.is_some();
    let has_code = !slides[0].code_blocks.is_empty();
    println!("  Images disabled: {} (should be false)", has_image);
    println!("  Tables disabled: {} (should be false)", has_table);
    println!("  Code disabled:   {} (should be false)", has_code);
    assert!(!has_image, "Images should be disabled");
    assert!(!has_table, "Tables should be disabled");
    assert!(!has_code, "Code should be disabled");
    println!("  ✓ All feature toggles verified");

    // ---------------------------------------------------------------
    // 7. Html2Ppt default vs with_options
    // ---------------------------------------------------------------
    println!("\n--- 7. Html2Ppt default vs with_options ---");

    let default_converter = Html2Ppt::new();
    let slides_default = default_converter.parse("<h1>Test</h1><p>A</p>")?;
    println!("  Html2Ppt::new(): {} slides", slides_default.len());

    let custom_opts = HtmlParseOptions::new().max_slides(1);
    let custom_converter = Html2Ppt::with_options(custom_opts);
    let slides_custom = custom_converter.parse("<h1>A</h1><h1>B</h1>")?;
    println!("  Html2Ppt::with_options(max_slides=1): {} slides (limited to 1)", slides_custom.len());

    println!("\n=== All examples completed ===");
    println!("Output files in examples/output/:");
    for entry in fs::read_dir("examples/output")? {
        let entry = entry?;
        if entry.path().extension().map_or(false, |e| e == "pptx") {
            let name = entry.file_name().into_string().unwrap_or_default();
            if name.starts_with("comprehensive_") {
                let size = entry.metadata()?.len();
                println!("  {name} ({} bytes)", size);
            }
        }
    }

    Ok(())
}
