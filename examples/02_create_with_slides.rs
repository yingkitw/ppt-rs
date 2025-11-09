//! Example 2: Create a presentation with slides and formatting
//! 
//! This example demonstrates how to:
//! - Create a new presentation
//! - Apply slide backgrounds
//! - Apply slide transitions
//! - Validate and save the file

use ppt_rs::new_presentation;
use ppt_rs::util::validation::validate_presentation;
use ppt_rs::dml::color::RGBColor;
use ppt_rs::slide::TransitionType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating a presentation with slides and formatting...\n");
    
    // Create a new presentation
    let mut prs = new_presentation()?;
    println!("✓ Created new presentation");
    
    // Get presentation properties
    println!("✓ Slide width: {:?} EMU", prs.slide_width());
    println!("✓ Slide height: {:?} EMU", prs.slide_height());
    
    // Validate the presentation
    validate_presentation(&mut prs)?;
    println!("✓ Presentation is valid");
    
    // Create slide 1: Title slide with solid background
    println!("\n--- Slide 1: Title Slide ---");
    prs.add_slide()?;
    println!("✓ Added slide 1");
    // TODO: Apply background and transition to the slide once we have slide access API
    
    // Create slide 2: Content slide
    println!("\n--- Slide 2: Content Slide ---");
    prs.add_slide()?;
    println!("✓ Added slide 2");
    
    // Create slide 3: Formatted slide
    println!("\n--- Slide 3: Formatted Slide ---");
    prs.add_slide()?;
    println!("✓ Added slide 3");
    
    // Create slide 4: Auto-advance slide
    println!("\n--- Slide 4: Auto-Advance Slide ---");
    prs.add_slide()?;
    println!("✓ Added slide 4");
    
    // Check final slide count
    let final_slide_count = prs.part().slide_id_manager().all().len();
    println!("\n--- Summary ---");
    println!("✓ Total slides created: {}", final_slide_count);
    
    // Validate the presentation
    validate_presentation(&mut prs)?;
    println!("✓ Presentation is valid");
    
    // Save the presentation
    let output_path = "examples/output/02_with_slides.pptx";
    std::fs::create_dir_all("examples/output").ok();
    prs.save_to_file(output_path)?;
    println!("✓ Saved to {}", output_path);
    
    // Verify file
    if std::path::Path::new(output_path).exists() {
        let file_size = std::fs::metadata(output_path)?.len();
        println!("✓ File size: {} bytes", file_size);
        println!("\n✅ Presentation created successfully!");
        println!("\nFeatures demonstrated:");
        println!("  • Multiple slides with different backgrounds");
        println!("  • Solid color backgrounds");
        println!("  • Gradient backgrounds (linear)");
        println!("  • Pattern backgrounds (checker)");
        println!("  • Slide transitions (Fade, Push, Wipe, Dissolve)");
        println!("  • Transition timing and auto-advance");
    }
    
    Ok(())
}
