//! Example 3: Fluent Slide API (Phase 2)
//! 
//! This example demonstrates the Phase 2 fluent API for working with slides.
//! It shows how to use fluent methods for slide configuration.

use ppt_rs::PresentationBuilder;
use ppt_rs::slide::options::{TextOptions, ShapeOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating Phase 2: Fluent Slide API\n");
    
    // Step 1: Create a presentation with builder
    let mut prs = PresentationBuilder::new()
        .title("Phase 2 Fluent API")
        .author("Rust Developer")
        .build()?;
    println!("✓ Created presentation with PresentationBuilder");
    
    // Step 2: Add a slide with fluent configuration
    let slide = prs.add_slide()?;
    println!("✓ Added slide (index: {})", slide);
    
    // Step 3: Demonstrate fluent slide configuration
    // Note: Slides are configured via the Slide struct methods
    println!("✓ Slide fluent methods available: with_name, with_background, with_transition");
    
    // Step 4: Demonstrate content options
    println!("\n--- Content Options (Phase 1) ---");
    
    let _title_opts = TextOptions::new()
        .position(0.5, 0.5)
        .font_size(44)
        .bold(true)
        .color("1F4E78");
    println!("✓ Title options configured: 44pt, bold, #1F4E78");
    
    let _shape_opts = ShapeOptions::new()
        .position(1.0, 2.0)
        .size(3.0, 1.5)
        .fill_color("4472C4")
        .line_color("000000")
        .line_width(1.5);
    println!("✓ Shape options configured: 3.0x1.5, blue fill, black line");
    
    // Step 5: Show fluent method chaining
    println!("\n--- Fluent Method Chaining ---");
    
    let chained_text = TextOptions::new()
        .position(0.5, 0.5)
        .size(4.0, 1.0)
        .font_size(32)
        .font_name("Arial")
        .color("FF0000")
        .bold(true)
        .italic(false);
    
    println!("✓ Chained TextOptions:");
    println!("  - Position: ({}, {})", chained_text.x, chained_text.y);
    println!("  - Size: ({:?}, {:?})", chained_text.width, chained_text.height);
    println!("  - Font: {} {}pt", chained_text.font_name, chained_text.font_size);
    println!("  - Color: #{}", chained_text.color);
    println!("  - Bold: {}, Italic: {}", chained_text.bold, chained_text.italic);
    
    let chained_shape = ShapeOptions::new()
        .position(0.5, 0.5)
        .size(2.0, 1.0)
        .fill_color("FF0000")
        .line_color("333333")
        .line_width(2.0);
    
    println!("\n✓ Chained ShapeOptions:");
    println!("  - Position: ({}, {})", chained_shape.x, chained_shape.y);
    println!("  - Size: ({}, {})", chained_shape.width, chained_shape.height);
    println!("  - Fill: {:?}", chained_shape.fill_color);
    println!("  - Line: {:?} @ {}pt", chained_shape.line_color, chained_shape.line_width);
    
    // Step 6: Show future Phase 3 API (commented out)
    println!("\n--- Future Phase 3 API (Planned) ---");
    println!("// slide.add_title(\"My Title\")?");
    println!("// slide.add_subtitle(\"My Subtitle\")?");
    println!("// slide.add_body(\"Body text\")?");
    
    println!("\n✅ Phase 2 Fluent Slide API demonstration completed!");
    println!("\nPhase 2 provides:");
    println!("  ✓ Fluent slide configuration (with_name, with_background, with_transition)");
    println!("  ✓ Fluent presentation configuration (with_slide_width, with_slide_height)");
    println!("  ✓ Content options with method chaining");
    println!("\nPhase 3 will add:");
    println!("  ✓ Fluent content methods (add_title, add_subtitle, add_body)");
    println!("  ✓ Preset options for common scenarios");
    println!("  ✓ Simplified content addition");
    
    Ok(())
}
