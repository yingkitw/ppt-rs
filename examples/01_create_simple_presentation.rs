//! Example 1: Create a simple presentation
//! 
//! This example demonstrates how to:
//! - Create a new presentation
//! - Add a slide
//! - Save the presentation
//! - Validate the file integrity

use ppt_rs::new_presentation;
use ppt_rs::util::validation::validate_presentation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating a simple presentation...");
    
    // Create a new presentation
    let mut prs = new_presentation()?;
    println!("✓ Created new presentation");
    
    // Validate the presentation
    validate_presentation(&mut prs)?;
    println!("✓ Presentation is valid");
    
    // Save the presentation
    let output_path = "examples/output/01_simple.pptx";
    std::fs::create_dir_all("examples/output").ok();
    prs.save_to_file(output_path)?;
    println!("✓ Saved to {}", output_path);
    
    // Verify file exists
    if std::path::Path::new(output_path).exists() {
        let file_size = std::fs::metadata(output_path)?.len();
        println!("✓ File size: {} bytes", file_size);
    }
    
    Ok(())
}
