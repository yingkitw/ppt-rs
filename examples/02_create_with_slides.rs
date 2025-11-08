//! Example 2: Create a presentation with slides
//! 
//! This example demonstrates how to:
//! - Create a new presentation
//! - Validate the presentation
//! - Save and verify the file

use ppt_rs::new_presentation;
use ppt_rs::util::validation::validate_presentation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating a presentation with slides...");
    
    // Create a new presentation
    let mut prs = new_presentation()?;
    println!("✓ Created new presentation");
    
    // Validate the presentation
    validate_presentation(&mut prs)?;
    println!("✓ Presentation is valid");
    
    // Check slide count
    let slide_count = prs.slides().len();
    println!("✓ Total slides: {}", slide_count);
    
    // Check slide dimensions
    println!("✓ Slide width: {:?}", prs.slide_width());
    println!("✓ Slide height: {:?}", prs.slide_height());
    
    // Save the presentation
    let output_path = "examples/output/02_with_slides.pptx";
    std::fs::create_dir_all("examples/output").ok();
    prs.save_to_file(output_path)?;
    println!("✓ Saved to {}", output_path);
    
    // Verify file
    if std::path::Path::new(output_path).exists() {
        let file_size = std::fs::metadata(output_path)?.len();
        println!("✓ File size: {} bytes", file_size);
    }
    
    Ok(())
}
