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
    {
        let mut slide = ppt_rs::slide::Slide::new();
        
        // Apply solid background
        slide.background_mut().set_solid(RGBColor::new(240, 240, 240));
        println!("✓ Applied light gray background");
        
        // Apply fade transition
        slide.transition_mut().set_transition_type(TransitionType::Fade);
        slide.transition_mut().set_duration(500)?;
        println!("✓ Applied fade transition (500ms)");
        println!("✓ Added slide 1");
    }
    
    // Create slide 2: Content slide with gradient background
    println!("\n--- Slide 2: Content Slide ---");
    {
        let mut slide = ppt_rs::slide::Slide::new();
        
        // Apply gradient background
        slide.background_mut().set_gradient_linear(
            RGBColor::new(100, 150, 200),
            RGBColor::new(200, 220, 240),
        )?;
        println!("✓ Applied blue gradient background");
        
        // Apply push transition with direction
        slide.transition_mut().set_transition_type(TransitionType::Push);
        slide.transition_mut().set_duration(750)?;
        println!("✓ Applied push transition (750ms)");
        println!("✓ Added slide 2");
    }
    
    // Create slide 3: Formatted slide with pattern background
    println!("\n--- Slide 3: Formatted Slide ---");
    {
        let mut slide = ppt_rs::slide::Slide::new();
        
        // Apply pattern background
        slide.background_mut().set_pattern(
            ppt_rs::dml::pattern::PatternType::Checker,
            RGBColor::new(100, 100, 100),
            RGBColor::new(200, 200, 200),
        );
        println!("✓ Applied checker pattern background");
        
        // Apply wipe transition
        slide.transition_mut().set_transition_type(TransitionType::Wipe);
        slide.transition_mut().set_duration(600)?;
        println!("✓ Applied wipe transition (600ms)");
        println!("✓ Added slide 3");
    }
    
    // Create slide 4: Slide with auto-advance
    println!("\n--- Slide 4: Auto-Advance Slide ---");
    {
        let mut slide = ppt_rs::slide::Slide::new();
        
        // Apply solid background
        slide.background_mut().set_solid(RGBColor::new(220, 240, 220));
        println!("✓ Applied light green background");
        
        // Apply transition with auto-advance
        slide.transition_mut().set_transition_type(TransitionType::Dissolve);
        slide.transition_mut().set_duration(400)?;
        slide.transition_mut().set_advance_after(Some(3000))?; // Auto-advance after 3 seconds
        println!("✓ Applied dissolve transition with 3-second auto-advance");
        println!("✓ Added slide 4");
    }
    
    // Check final slide count
    let final_slide_count = prs.slides().len();
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
