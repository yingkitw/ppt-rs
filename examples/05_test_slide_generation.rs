//! Example 5: Create presentation with multiple slides (Fluent API)
//! 
//! This example creates a presentation with multiple slides using
//! the fluent API to demonstrate slide generation.

use ppt_rs::PresentationBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating presentation with multiple slides using Fluent API...\n");
    
    // Create presentation with fluent builder
    let mut prs = PresentationBuilder::new()
        .title("Multi-Slide Test")
        .author("Fluent API Example")
        .build()?;
    println!("✓ Created presentation with PresentationBuilder");
    
    // Add 5 slides
    println!("\n--- Adding Slides ---");
    for i in 1..=5 {
        let slide_idx = prs.add_slide()?;
        println!("✓ Added slide {} (index: {})", i, slide_idx);
    }
    
    // Get final slide count
    let final_count = prs.part().slide_id_manager().all().len();
    println!("\n--- Summary ---");
    println!("✓ Total slides created: {}", final_count);
    
    // Save
    let output_path = "examples/output/05_test_slides.pptx";
    std::fs::create_dir_all("examples/output").ok();
    prs.save_to_file(output_path)?;
    println!("✓ Saved to {}", output_path);
    
    // Verify file
    if std::path::Path::new(output_path).exists() {
        let file_size = std::fs::metadata(output_path)?.len();
        println!("✓ File size: {} bytes", file_size);
    }
    
    println!("\n✅ Multi-slide presentation created successfully!");
    println!("\nUsing Fluent API:");
    println!("  • PresentationBuilder for configuration");
    println!("  • Simple add_slide() method");
    println!("  • Type-safe and ergonomic");
    
    Ok(())
}
