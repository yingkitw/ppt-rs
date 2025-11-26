//! Example 1: Create a simple presentation
//!
//! Run with: cargo run --example simple_presentation

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating a simple presentation...\n");

    let output_file = "examples/output/simple.pptx";
    
    // Create output directory
    fs::create_dir_all("examples/output")?;

    // Create presentation XML
    let presentation = create_simple_presentation("My First Presentation", 1);
    
    // Write to file
    fs::write(output_file, presentation)?;

    println!("✓ Presentation created: {}", output_file);
    println!("  Title: My First Presentation");
    println!("  Slides: 1");
    println!("  Size: {} bytes", fs::metadata(output_file)?.len());

    Ok(())
}

fn create_simple_presentation(title: &str, slides: usize) -> String {
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<presentation>\n");
    xml.push_str(&format!("  <title>{}</title>\n", escape_xml(title)));
    xml.push_str(&format!("  <slides count=\"{}\">\n", slides));

    for i in 1..=slides {
        xml.push_str(&format!("    <slide number=\"{}\">\n", i));
        xml.push_str(&format!("      <title>Slide {}</title>\n", i));
        xml.push_str("      <content></content>\n");
        xml.push_str("    </slide>\n");
    }

    xml.push_str("  </slides>\n");
    xml.push_str("</presentation>\n");
    xml
}

fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}
