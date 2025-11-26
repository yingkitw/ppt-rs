//! Example 2: Create a multi-slide presentation with content
//!
//! Run with: cargo run --example multi_slide_presentation

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating a multi-slide presentation...\n");

    let output_file = "examples/output/multi_slide.pptx";
    
    // Create output directory
    fs::create_dir_all("examples/output")?;

    // Define slides with content
    let slides = vec![
        Slide {
            title: "Title Slide".to_string(),
            content: "Welcome to Rust PowerPoint Generation".to_string(),
        },
        Slide {
            title: "Overview".to_string(),
            content: "This presentation demonstrates PPTX generation in Rust".to_string(),
        },
        Slide {
            title: "Features".to_string(),
            content: "• Create presentations\n• Add slides\n• Generate XML".to_string(),
        },
        Slide {
            title: "Benefits".to_string(),
            content: "• Type-safe\n• Fast\n• Memory-efficient".to_string(),
        },
        Slide {
            title: "Conclusion".to_string(),
            content: "Thank you!".to_string(),
        },
    ];

    // Create presentation
    let presentation = create_presentation("Rust PowerPoint Demo", &slides);
    
    // Write to file
    fs::write(output_file, presentation)?;

    println!("✓ Presentation created: {}", output_file);
    println!("  Title: Rust PowerPoint Demo");
    println!("  Slides: {}", slides.len());
    println!("  Size: {} bytes", fs::metadata(output_file)?.len());
    println!("\nSlides:");
    for (i, slide) in slides.iter().enumerate() {
        println!("  {}. {}", i + 1, slide.title);
    }

    Ok(())
}

struct Slide {
    title: String,
    content: String,
}

fn create_presentation(title: &str, slides: &[Slide]) -> String {
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<presentation>\n");
    xml.push_str(&format!("  <title>{}</title>\n", escape_xml(title)));
    xml.push_str(&format!("  <slides count=\"{}\">\n", slides.len()));

    for (i, slide) in slides.iter().enumerate() {
        xml.push_str(&format!("    <slide number=\"{}\">\n", i + 1));
        xml.push_str(&format!("      <title>{}</title>\n", escape_xml(&slide.title)));
        xml.push_str(&format!("      <content>{}</content>\n", escape_xml(&slide.content)));
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
