//! Example 4: Batch generate multiple presentations
//!
//! Run with: cargo run --example batch_generator

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Batch generating presentations...\n");

    // Create output directory
    fs::create_dir_all("examples/output")?;

    // Define batch jobs
    let jobs = vec![
        ("Sales Report", 8),
        ("Marketing Overview", 10),
        ("Engineering Update", 12),
        ("HR Initiatives", 6),
        ("Financial Summary", 9),
    ];

    println!("Generating {} presentations:\n", jobs.len());

    for (title, slides) in jobs {
        let filename = format!(
            "examples/output/{}.pptx",
            title.to_lowercase().replace(" ", "_")
        );

        let presentation = create_presentation(title, slides);
        fs::write(&filename, presentation)?;

        let size = fs::metadata(&filename)?.len();
        println!("✓ {} ({} slides, {} bytes)", title, slides, size);
    }

    println!("\n✓ All presentations generated successfully!");

    Ok(())
}

fn create_presentation(title: &str, slides: usize) -> String {
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<presentation>\n");
    xml.push_str(&format!("  <title>{}</title>\n", escape_xml(title)));
    xml.push_str(&format!("  <slides count=\"{}\">\n", slides));

    for i in 1..=slides {
        xml.push_str(&format!("    <slide number=\"{}\">\n", i));
        if i == 1 {
            xml.push_str(&format!("      <title>{}</title>\n", escape_xml(title)));
            xml.push_str("      <content>Presentation Overview</content>\n");
        } else if i == slides {
            xml.push_str("      <title>Thank You</title>\n");
            xml.push_str("      <content>Questions?</content>\n");
        } else {
            xml.push_str(&format!("      <title>Section {}</title>\n", i - 1));
            xml.push_str("      <content>Content goes here</content>\n");
        }
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
