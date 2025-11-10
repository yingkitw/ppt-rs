//! Example 2: Create a presentation with slides (Fluent API)
//! 
//! This example demonstrates how to:
//! - Create a presentation using PresentationBuilder
//! - Add multiple slides
//! - Configure slide properties
//! - Save and validate the file

use ppt_rs::PresentationBuilder;
use ppt_rs::util::validation::validate_presentation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating a presentation with multiple slides using Fluent API...\n");
    
    // Step 1: Create a presentation with fluent builder
    let mut prs = PresentationBuilder::new()
        .title("Multi-Slide Presentation")
        .author("Rust Developer")
        .build()?;
    println!("✓ Created presentation with PresentationBuilder");
    println!("  - Title: Multi-Slide Presentation");
    println!("  - Author: Rust Developer");
    
    // Step 2: Get and display presentation properties
    println!("\n--- Presentation Properties ---");
    println!("✓ Slide width: {:?} EMU", prs.slide_width());
    println!("✓ Slide height: {:?} EMU", prs.slide_height());
    
    // Step 3: Add multiple slides
    println!("\n--- Adding Slides ---");
    for i in 1..=4 {
        let slide_idx = prs.add_slide()?;
        println!("✓ Added slide {} (index: {})", i, slide_idx);
    }
    
    // Step 4: Check final slide count
    let final_slide_count = prs.part().slide_id_manager().all().len();
    println!("\n--- Summary ---");
    println!("✓ Total slides created: {}", final_slide_count);
    
    // Step 5: Validate the presentation
    validate_presentation(&mut prs)?;
    println!("✓ Presentation is valid");
    
    // Step 6: Save the presentation
    let output_path = "examples/output/02_with_slides.pptx";
    std::fs::create_dir_all("examples/output").ok();
    prs.save_to_file(output_path)?;
    println!("✓ Saved to {}", output_path);
    
    // Step 7: Verify file
    if std::path::Path::new(output_path).exists() {
        let file_size = std::fs::metadata(output_path)?.len();
        println!("✓ File size: {} bytes", file_size);
    }
    
    println!("\n✅ Multi-slide presentation created successfully!");
    println!("\nUsing Fluent API:");
    println!("  • PresentationBuilder for configuration");
    println!("  • Fluent method chaining");
    println!("  • Type-safe slide management");
    println!("  • Easy validation and saving");
    
    Ok(())
}
