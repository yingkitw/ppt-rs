//! Example 1: Create a simple presentation (Fluent API)
//! 
//! This example demonstrates how to:
//! - Create a new presentation using PresentationBuilder
//! - Add a slide
//! - Save the presentation
//! - Validate the file integrity

use ppt_rs::PresentationBuilder;
use ppt_rs::util::validation::validate_presentation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating a simple presentation with Fluent API...\n");
    
    // Step 1: Create a presentation using the fluent builder
    let mut prs = PresentationBuilder::new()
        .title("Simple Presentation")
        .author("Rust Developer")
        .build()?;
    println!("✓ Created presentation with PresentationBuilder");
    println!("  - Title: Simple Presentation");
    println!("  - Author: Rust Developer");
    
    // Step 2: Add a slide
    let slide_idx = prs.add_slide()?;
    println!("✓ Added slide (index: {})", slide_idx);
    
    // Step 3: Validate the presentation
    validate_presentation(&mut prs)?;
    println!("✓ Presentation is valid");
    
    // Step 4: Save the presentation
    let output_path = "examples/output/01_simple.pptx";
    std::fs::create_dir_all("examples/output").ok();
    prs.save_to_file(output_path)?;
    println!("✓ Saved to {}", output_path);
    
    // Step 5: Verify file
    if std::path::Path::new(output_path).exists() {
        let file_size = std::fs::metadata(output_path)?.len();
        println!("✓ File size: {} bytes", file_size);
    }
    
    println!("\n✅ Simple presentation created successfully!");
    println!("\nUsing Fluent API:");
    println!("  • PresentationBuilder for configuration");
    println!("  • Method chaining for intuitive setup");
    println!("  • Type-safe and ergonomic");
    
    Ok(())
}
