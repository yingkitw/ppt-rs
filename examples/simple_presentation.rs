//! Example 1: Create a simple presentation using the prelude API
//!
//! Run with: cargo run --example simple_presentation

use std::fs;
use ppt_rs::pptx;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Creating a simple presentation...\n");

    // Create output directory
    fs::create_dir_all("examples/output")?;

    // Create presentation using the pptx! macro
    pptx!("My First Presentation")
        .slide("Welcome", &[
            "This is my first presentation",
            "Created with ppt-rs",
            "Using the simplified prelude API",
        ])
        .slide("Features", &[
            "Easy to use macros",
            "Fluent builder pattern",
            "Type-safe API",
        ])
        .save("examples/output/simple.pptx")?;

    println!("✓ Presentation created: examples/output/simple.pptx");
    println!("  Title: My First Presentation");
    println!("  Slides: 2");
    println!("  Size: {} bytes", fs::metadata("examples/output/simple.pptx")?.len());

    Ok(())
}
