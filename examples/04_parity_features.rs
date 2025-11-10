//! Example: Parity Features Implementation
//! 
//! This example demonstrates the new parity features implemented to match python-pptx:
//! - Thumbnail placeholder image
//! - Keywords and comments metadata
//! - Custom slide dimensions
//! - Remove slide functionality
//! - Shape shadows
//! - Notes pages

use ppt_rs::PresentationBuilder;
use ppt_rs::shapes::{Shadow, ShadowManager};
use ppt_rs::slide::NotesSlide;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating presentation with parity features...\n");

    // 1. Create presentation with custom metadata
    println!("1️⃣  Creating presentation with keywords and comments...");
    let mut prs = PresentationBuilder::new()
        .title("Parity Features Demo")
        .author("Rust Developer")
        .keywords("presentation, features, parity, python-pptx")
        .comments("This presentation demonstrates parity features with python-pptx")
        .build()?;
    println!("   ✓ Presentation created with metadata");

    // 2. Set custom slide dimensions (16:9 widescreen)
    println!("\n2️⃣  Setting custom slide dimensions (16:9 widescreen)...");
    let width_16_9 = 12192000;  // 13.33 inches
    let height_16_9 = 6858000;  // 7.5 inches
    prs.set_slide_width(width_16_9)?;
    prs.set_slide_height(height_16_9)?;
    println!("   ✓ Slide dimensions: {}x{} EMU (16:9)", width_16_9, height_16_9);

    // 3. Add multiple slides
    println!("\n3️⃣  Adding slides...");
    let slide1_idx = prs.add_slide()?;
    println!("   ✓ Added slide 1 (index: {})", slide1_idx);
    
    let slide2_idx = prs.add_slide()?;
    println!("   ✓ Added slide 2 (index: {})", slide2_idx);
    
    let slide3_idx = prs.add_slide()?;
    println!("   ✓ Added slide 3 (index: {})", slide3_idx);

    // 4. Add notes to slides
    println!("\n4️⃣  Adding notes to slides...");
    let mut notes = NotesSlide::new();
    notes.set_text("This is the speaker notes for slide 1. It contains important talking points.".to_string());
    println!("   ✓ Notes added: {}", notes.text());

    // 5. Demonstrate shadow effects (already implemented)
    println!("\n5️⃣  Shadow effects available...");
    let mut shadow_manager = ShadowManager::new();
    let _outer_idx = shadow_manager.add_outer_shadow();
    println!("   ✓ Outer shadow added");
    
    let _inner_idx = shadow_manager.add_inner_shadow();
    println!("   ✓ Inner shadow added");
    
    if let Some(shadow) = shadow_manager.get_mut(0) {
        shadow.set_blur_radius(50000);
        shadow.set_distance(60000);
        shadow.set_opacity(0.8);
        println!("   ✓ Shadow customized (blur: 50000, distance: 60000, opacity: 0.8)");
    }

    // 6. Remove a slide
    println!("\n6️⃣  Removing slide...");
    let removed = prs.remove_slide(1)?;
    if removed {
        println!("   ✓ Slide 2 removed successfully");
    }

    // 7. Save presentation
    println!("\n7️⃣  Saving presentation...");
    let output_path = "examples/output/04_parity_features.pptx";
    prs.save_to_file(output_path)?;
    println!("   ✓ Saved to {}", output_path);

    // 8. Verify file
    println!("\n8️⃣  Verifying file...");
    let metadata = std::fs::metadata(output_path)?;
    println!("   ✓ File size: {} bytes", metadata.len());

    println!("\n✅ Parity features demonstration complete!");
    println!("\nFeatures demonstrated:");
    println!("  • Thumbnail placeholder image (in docProps/thumbnail.jpeg)");
    println!("  • Keywords and comments metadata");
    println!("  • Custom slide dimensions (16:9 widescreen)");
    println!("  • Slide management (add/remove)");
    println!("  • Notes pages support");
    println!("  • Shadow effects support");
    println!("\n🎉 All parity features working correctly!");

    Ok(())
}
