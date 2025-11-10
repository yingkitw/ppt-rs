//! Example 2: Using the Fluent API
//! 
//! This example demonstrates the new fluent API inspired by PptxGenJS
//! that makes creating presentations more intuitive.
//!
//! Note: The fluent API is designed to be simple and intuitive.
//! This example shows how to create content options with sensible defaults.
//! 
//! Currently, adding slides requires using the underlying Slides API.
//! Future versions will provide a fully fluent interface for adding slides and content.

use ppt_rs::PresentationBuilder;
use ppt_rs::slide::options::{TextOptions, ShapeOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating Fluent API for content options...\n");
    
    // Step 1: Create a presentation using the builder
    let _prs = PresentationBuilder::new()
        .title("Fluent API Example")
        .author("Rust Developer")
        .build()?;
    println!("✓ Created presentation with builder pattern");
    
    // Step 2: Demonstrate TextOptions with fluent API
    println!("\n--- TextOptions Examples ---");
    
    let title_opts = TextOptions::new()
        .position(0.5, 0.5)
        .font_size(44)
        .bold(true)
        .color("1F4E78");
    println!("✓ Title options: position=(0.5, 0.5), size=44pt, bold, color=#1F4E78");
    
    let subtitle_opts = TextOptions::new()
        .position(0.5, 1.5)
        .font_size(24)
        .color("595959");
    println!("✓ Subtitle options: position=(0.5, 1.5), size=24pt, color=#595959");
    
    let body_opts = TextOptions::new()
        .position(1.0, 2.5)
        .font_size(18)
        .italic(true);
    println!("✓ Body options: position=(1.0, 2.5), size=18pt, italic");
    
    // Step 3: Demonstrate ShapeOptions with fluent API
    println!("\n--- ShapeOptions Examples ---");
    
    let blue_shape = ShapeOptions::new()
        .position(1.0, 3.0)
        .size(2.0, 1.0)
        .fill_color("4472C4")
        .line_color("000000");
    println!("✓ Blue shape: position=(1.0, 3.0), size=(2.0, 1.0), fill=#4472C4");
    
    let red_shape = ShapeOptions::new()
        .position(3.5, 3.0)
        .size(2.0, 1.0)
        .fill_color("FF0000")
        .line_color("333333")
        .line_width(2.0);
    println!("✓ Red shape: position=(3.5, 3.0), size=(2.0, 1.0), fill=#FF0000, line=2pt");
    
    // Step 4: Demonstrate sensible defaults
    println!("\n--- Sensible Defaults ---");
    
    let default_text = TextOptions::default();
    println!("✓ Default TextOptions:");
    println!("  - Font: {}", default_text.font_name);
    println!("  - Size: {}pt", default_text.font_size);
    println!("  - Color: #{}", default_text.color);
    println!("  - Bold: {}", default_text.bold);
    println!("  - Italic: {}", default_text.italic);
    
    let default_shape = ShapeOptions::default();
    println!("✓ Default ShapeOptions:");
    println!("  - Position: ({}, {})", default_shape.x, default_shape.y);
    println!("  - Size: ({}, {})", default_shape.width, default_shape.height);
    println!("  - Fill: {:?}", default_shape.fill_color);
    println!("  - Line: {:?}", default_shape.line_color);
    println!("  - Line Width: {}pt", default_shape.line_width);
    
    // Step 5: Show method chaining
    println!("\n--- Method Chaining Example ---");
    let complex_opts = TextOptions::new()
        .position(0.5, 0.5)
        .size(4.0, 1.0)
        .font_size(32)
        .font_name("Arial")
        .color("1F4E78")
        .bold(true)
        .italic(false);
    println!("✓ Complex options created with method chaining");
    println!("  - Position: ({}, {})", complex_opts.x, complex_opts.y);
    println!("  - Size: ({:?}, {:?})", complex_opts.width, complex_opts.height);
    println!("  - Font: {} {}pt", complex_opts.font_name, complex_opts.font_size);
    println!("  - Color: #{}", complex_opts.color);
    println!("  - Bold: {}, Italic: {}", complex_opts.bold, complex_opts.italic);
    
    println!("\n✅ Fluent API demonstration completed successfully!");
    println!("\nNote: To create presentations with actual slides, use the underlying Slides API.");
    println!("See example 01_create_simple_presentation.rs for slide creation.");
    
    Ok(())
}
