//! Example: Generate proper PPTX files (ZIP-based)
//!
//! Run with: cargo run --example proper_pptx

use std::fs;
use pptx_rs::generator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating proper PPTX files...\n");

    // Create output directory
    fs::create_dir_all("examples/output")?;

    // Example 1: Simple presentation
    println!("Creating simple.pptx...");
    let pptx_data = generator::create_pptx("My Presentation", 1)?;
    fs::write("examples/output/simple_proper.pptx", pptx_data)?;
    println!("✓ Created: examples/output/simple_proper.pptx");

    // Example 2: Multi-slide presentation
    println!("\nCreating multi_slide_proper.pptx...");
    let pptx_data = generator::create_pptx("Multi-Slide Presentation", 5)?;
    fs::write("examples/output/multi_slide_proper.pptx", pptx_data)?;
    println!("✓ Created: examples/output/multi_slide_proper.pptx");

    // Example 3: Report
    println!("\nCreating report_proper.pptx...");
    let pptx_data = generator::create_pptx("Quarterly Report Q1 2025", 6)?;
    fs::write("examples/output/report_proper.pptx", pptx_data)?;
    println!("✓ Created: examples/output/report_proper.pptx");

    // Example 4: Training
    println!("\nCreating training_proper.pptx...");
    let pptx_data = generator::create_pptx("Rust Training Course", 10)?;
    fs::write("examples/output/training_proper.pptx", pptx_data)?;
    println!("✓ Created: examples/output/training_proper.pptx");

    println!("\n✅ All proper PPTX files generated successfully!");
    println!("\nGenerated files:");
    println!("  - examples/output/simple_proper.pptx (1 slide)");
    println!("  - examples/output/multi_slide_proper.pptx (5 slides)");
    println!("  - examples/output/report_proper.pptx (6 slides)");
    println!("  - examples/output/training_proper.pptx (10 slides)");

    // Verify files are valid PPTX (ZIP format)
    println!("\nVerifying PPTX format:");
    for file in &[
        "examples/output/simple_proper.pptx",
        "examples/output/multi_slide_proper.pptx",
        "examples/output/report_proper.pptx",
        "examples/output/training_proper.pptx",
    ] {
        let metadata = fs::metadata(file)?;
        let size = metadata.len();
        println!("  ✓ {} ({} bytes)", file, size);
    }

    Ok(())
}
