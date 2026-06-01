//! Example: Using Enhanced HTML Features
//!
//! This example demonstrates how to use the new HTML to PowerPoint capabilities:
//! - Enhanced CSS parsing (margins, padding, borders, etc.)
//! - Real image downloading from web pages
//! - Hyperlink support and preservation
//! - Advanced HTML export with navigation and notes

use ppt_rs::generator::create_pptx_with_content;
use ppt_rs::import::{parse_html, parse_html_with_options, HtmlParseOptions};
use ppt_rs::export::html::{export_to_html_with_options, HtmlExportOptions};
use ppt_rs::api::Presentation;
use ppt_rs::generator::SlideContent;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: HTML with enhanced CSS styling
    let html_with_css = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Enhanced CSS Features</title>
</head>
<body>
    <h1>Advanced Styling Capabilities</h1>

    <p style="color: #E74C3C; font-size: 36pt; font-weight: bold; margin: 20px;">
        This text demonstrates custom color, size, weight, and margins.
    </p>

    <div style="padding: 25px; border: 3px solid #3498DB; background-color: #EBF5FB; margin: 30px 0;">
        <p style="margin: 0; line-height: 1.8;">
            This container combines padding, borders, background color, margins, and line-height.
        </p>
    </div>

    <p style="font-family: 'Georgia', serif; letter-spacing: 2px; font-size: 24pt;">
        Elegant typography with custom font and letter spacing.
    </p>
</body>
</html>
"#;

    // Parse with enhanced CSS support
    let options = HtmlParseOptions::new()
        .max_slides(20)
        .include_images(true)
        .include_tables(true);

    let slides = parse_html_with_options(html_with_css, options)?;
    let slide_count = slides.len();
    let pptx = create_pptx_with_content("CSS Features Demo", slides)?;
    fs::write("css_features.pptx", pptx)?;
    println!("Created css_features.pptx with {} slides", slide_count);

    // Example 2: HTML with real images and hyperlinks
    let html_with_media = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Media and Links Demo</title>
</head>
<body>
    <h1>Enhanced Media Support</h1>

    <h2>Real Image Embedding</h2>
    <img src="https://via.placeholder.com/800x400/2ECC71/ffffff?text=Real+Image+Embedding" alt="Embedded Image">
    <p>The image above was downloaded and embedded directly into the PowerPoint.</p>

    <h2>Hyperlink Support</h2>
    <p>Visit our <a href="https://github.com/anthropics/ppt-rs">GitHub repository</a> for the source code.</p>
    <p>Check out <a href="https://crates.io/crates/ppt-rs">ppt-rs on crates.io</a> for installation instructions.</p>
    <p>Read the <a href="https://docs.rs/ppt-rs">documentation on docs.rs</a> for detailed API reference.</p>

    <h2>Combined Features</h2>
    <div style="padding: 20px; background-color: #F8F9FA; border: 2px solid #DEE2E6;">
        <p>This content is styled with CSS and contains a <a href="https://example.com">hyperlink</a>.</p>
    </div>
</body>
</html>
"#;

    let options = HtmlParseOptions::new()
        .include_images(true)
        .include_tables(true);

    let slides = parse_html_with_options(html_with_media, options)?;
    let slide_count = slides.len();
    let pptx = create_pptx_with_content("Media and Links", slides)?;
    fs::write("media_links.pptx", pptx)?;
    println!("Created media_links.pptx with {} slides", slide_count);

    // Example 3: Complex HTML with tables and code
    let html_complex = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Complex HTML Example</title>
</head>
<body>
    <h1>Complex HTML Structures</h1>

    <h2>Data Tables</h2>
    <table>
        <thead>
            <tr>
                <th>Feature</th>
                <th>Status</th>
                <th>Priority</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>Enhanced CSS</td>
                <td>✅ Complete</td>
                <td>High</td>
            </tr>
            <tr>
                <td>Image Support</td>
                <td>✅ Complete</td>
                <td>High</td>
            </tr>
            <tr>
                <td>Hyperlinks</td>
                <td>✅ Complete</td>
                <td>Medium</td>
            </tr>
        </tbody>
    </table>

    <h2>Code Examples</h2>
    <pre><code>fn main() {
    // Enhanced HTML parsing
    let html = "<h1>Hello</h1>";
    let slides = parse_html(html)?;
    create_pptx("Title", slides)?;
}</code></pre>

    <h2>Styled Lists</h2>
    <ul style="list-style-type: square; padding-left: 40px;">
        <li style="color: #E74C3C;">Enhanced CSS parsing</li>
        <li style="color: #3498DB;">Real image downloading</li>
        <li style="color: #2ECC71;">Hyperlink preservation</li>
        <li style="color: #F39C12;">Advanced HTML export</li>
    </ul>

    <blockquote>
        The enhanced HTML parser makes ppt-rs the most comprehensive solution
        for web-to-PowerPoint conversion in Rust.
    </blockquote>
</body>
</html>
"#;

    let slides = parse_html(html_complex)?;
    let slide_count = slides.len();
    let pptx = create_pptx_with_content("Complex HTML Demo", slides)?;
    fs::write("complex_html.pptx", pptx)?;
    println!("Created complex_html.pptx with {} slides", slide_count);

    // Example 4: Enhanced HTML export with all options
    println!("\n📊 Enhanced HTML Export Examples:");

    // Create a presentation with speaker notes
    let mut slide1 = SlideContent::new("Introduction")
        .add_bullet("Welcome to the presentation")
        .add_bullet("We'll cover new features");
    slide1.notes = Some("These are speaker notes for the introduction slide.".to_string());

    let mut slide2 = SlideContent::new("New Features")
        .add_bullet("Enhanced CSS parsing")
        .add_bullet("Real image support")
        .add_bullet("Hyperlink handling");
    slide2.notes = Some("Emphasize the practical applications of these features.".to_string());

    let presentation = Presentation::with_title("Enhanced Features Demo")
        .add_slide(slide1)
        .add_slide(slide2);

    // Export with navigation enabled
    let nav_options = HtmlExportOptions::new()
        .with_navigation(true)
        .with_notes(true)
        .with_syntax_highlight(true);

    let html_nav = export_to_html_with_options(&presentation, &nav_options)?;
    fs::write("presentation_with_nav.html", html_nav)?;
    println!("Created presentation_with_nav.html with navigation and notes");

    // Export with minimal options (presentation mode)
    let minimal_options = HtmlExportOptions::new()
        .with_navigation(false)
        .with_notes(false);

    let html_minimal = export_to_html_with_options(&presentation, &minimal_options)?;
    fs::write("presentation_minimal.html", html_minimal)?;
    println!("Created presentation_minimal.html (no navigation or notes)");

    // Example 5: Web scraping simulation (local HTML file)
    println!("\n🌐 Web Page Conversion:");

    // Simulate converting a complex web page
    let web_page = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Company Overview</title>
    <meta name="description" content="Introduction to our company and products">
</head>
<body>
    <h1>Welcome to Our Company</h1>

    <p style="color: #2C3E50; font-size: 28pt; line-height: 1.6;">
        We are a leading technology company specializing in innovative solutions
        for modern businesses.
    </p>

    <h2>Our Products</h2>
    <div style="display: flex; gap: 20px;">
        <div style="flex: 1; padding: 20px; background: #ECF0F1;">
            <h3 style="color: #E74C3C;">Product A</h3>
            <p>Advanced analytics platform for data-driven decisions.</p>
        </div>
        <div style="flex: 1; padding: 20px; background: #D5DBDB;">
            <h3 style="color: #3498DB;">Product B</h3>
            <p>Cloud collaboration tools for remote teams.</p>
        </div>
    </div>

    <h2>Key Statistics</h2>
    <table>
        <thead>
            <tr>
                <th>Metric</th>
                <th>Value</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>Customers</td>
                <td>10,000+</td>
            </tr>
            <tr>
                <td>Countries</td>
                <td>50+</td>
            </tr>
            <tr>
                <td>Revenue</td>
                <td>$50M+</td>
            </tr>
        </tbody>
    </table>

    <h2>Contact Us</h2>
    <p>Visit <a href="https://example.com/contact">our contact page</a> or email <a href="mailto:info@example.com">info@example.com</a></p>
</body>
</html>
"#;

    let options = HtmlParseOptions::new()
        .max_slides(15)
        .include_images(true)
        .include_tables(true);

    let slides = parse_html_with_options(web_page, options)?;
    let slide_count = slides.len();
    let pptx = create_pptx_with_content("Company Overview", slides)?;
    fs::write("company_overview.pptx", pptx)?;
    println!("Created company_overview.pptx with {} slides", slide_count);

    println!("\n✅ All examples completed successfully!");
    println!("\nNew capabilities demonstrated:");
    println!("  • Enhanced CSS property parsing (margins, padding, borders, etc.)");
    println!("  • Real image downloading from web URLs");
    println!("  • Hyperlink preservation and handling");
    println!("  • Complex HTML structure support");
    println!("  • Advanced HTML export with navigation");
    println!("  • Speaker notes export in HTML");
    println!("  • Configurable export options");
    println!("  • Web page conversion simulation");

    Ok(())
}