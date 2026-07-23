//! Example demonstrating reading and inspecting PPTX files

use ppt_rs::opc::package::Package;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First, create a sample PPTX file to read
    use ppt_rs::generator::{create_pptx_with_content, SlideContent};

    let slides = vec![
        SlideContent::new("Sample Presentation")
            .add_bullet("This is a sample slide")
            .add_bullet("Created for reading demonstration"),
        SlideContent::new("Second Slide")
            .add_bullet("More content here")
            .add_bullet("Ready to be read"),
    ];

    let pptx_data = create_pptx_with_content("Sample", slides)?;
    fs::write("sample_to_read.pptx", pptx_data)?;

    // Now read the PPTX file
    println!("Reading PPTX file: sample_to_read.pptx\n");

    let package = Package::open("sample_to_read.pptx")?;

    println!("Package Information:");
    println!("  Total parts: {}", package.part_count());
    println!("\nPackage Contents:");

    let paths = package.part_paths();
    for path in &paths {
        if let Some(content) = package.get_part(path) {
            println!("  {} ({} bytes)", path, content.len());
        }
    }

    // Display key parts
    println!("\n--- Key Parts Analysis ---");

    // Check for presentation.xml
    if let Some(pres_content) = package.get_part("ppt/presentation.xml") {
        println!("\n✓ presentation.xml found ({} bytes)", pres_content.len());
        if let Ok(content_str) = std::str::from_utf8(pres_content)
            && content_str.contains("<p:sldIdLst>") {
                println!("  - Contains slide list");
            }
    }

    // Check for slides
    let mut slide_count = 0;
    for path in &paths {
        if path.starts_with("ppt/slides/slide") && path.ends_with(".xml") {
            slide_count += 1;
            if let Some(content) = package.get_part(path) {
                println!("\n✓ {} found ({} bytes)", path, content.len());
            }
        }
    }
    println!("\nTotal slides: {}", slide_count);

    // Check for core properties
    if let Some(core_content) = package.get_part("docProps/core.xml") {
        println!("\n✓ docProps/core.xml found ({} bytes)", core_content.len());
        if let Ok(content_str) = std::str::from_utf8(core_content)
            && content_str.contains("<dc:title>") {
                println!("  - Contains title metadata");
            }
    }

    // Check for relationships
    if let Some(rels_content) = package.get_part("_rels/.rels") {
        println!("\n✓ _rels/.rels found ({} bytes)", rels_content.len());
        if let Ok(content_str) = std::str::from_utf8(rels_content) {
            let rel_count = content_str.matches("<Relationship").count();
            println!("  - Contains {} relationships", rel_count);
        }
    }

    // Check for content types
    if let Some(ct_content) = package.get_part("[Content_Types].xml") {
        println!("\n✓ [Content_Types].xml found ({} bytes)", ct_content.len());
        if let Ok(content_str) = std::str::from_utf8(ct_content) {
            let override_count = content_str.matches("<Override").count();
            let default_count = content_str.matches("<Default").count();
            println!("  - Contains {} Override entries", override_count);
            println!("  - Contains {} Default entries", default_count);
        }
    }

    // Summary
    println!("\n--- Summary ---");
    println!("Successfully read PPTX package with {} parts", package.part_count());
    println!("Package structure is valid and can be modified");

    Ok(())
}
