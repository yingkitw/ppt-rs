/// Alignment Test Example
/// 
/// This example generates a PPTX file that can be compared with python-pptx output.
/// It demonstrates:
/// - Presentation metadata (title, author, subject, keywords, comments)
/// - Multiple slides
/// - Text formatting
/// - Basic slide structure

use ppt_rs::generator::{SlideContent, create_pptx_with_content};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Alignment Test: ppt-rs vs python-pptx ===\n");
    
    // Create output directory
    fs::create_dir_all("examples/output")?;
    
    // Create slides matching the reference presentation structure
    let slides = vec![
        // Slide 1: Title Slide
        SlideContent::new("Alignment Test Presentation")
            .title_size(54)
            .title_bold(true)
            .title_color("003366"),  // RGB(0, 51, 102)
        
        // Slide 2: Content Slide
        SlideContent::new("Shapes and Formatting")
            .title_size(44)
            .title_bold(true)
            .title_color("003366")
            .add_bullet("Text formatting (bold, colors, sizes)")
            .add_bullet("Shape creation and positioning")
            .add_bullet("Multiple slides and layouts"),
    ];
    
    // Generate PPTX
    let pptx_data = create_pptx_with_content(
        "Alignment Test Presentation",
        slides,
    )?;
    
    // Write to file
    let output_path = "examples/output/alignment_test_ppt_rs.pptx";
    fs::write(output_path, pptx_data)?;
    
    println!("✓ Created presentation: {output_path}");
    println!("  - Title: Alignment Test Presentation");
    println!("  - Slides: 2");
    println!("\nNext steps:");
    println!("  1. Open the generated file in PowerPoint to verify");
    println!("  2. Compare with a python-pptx reference if available");
    
    Ok(())
}

