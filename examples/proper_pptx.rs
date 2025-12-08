//! Example: Generate proper PPTX files using the simplified prelude API
//!
//! Run with: cargo run --example proper_pptx

use std::fs;
use ppt_rs::prelude::{shapes, colors, ShapeFill};
use ppt_rs::pptx;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Generating PPTX files with simplified API...\n");

    // Create output directory
    fs::create_dir_all("examples/output")?;

    // Example 1: Simple presentation using pptx! macro
    println!("Creating simple.pptx...");
    pptx!("My Presentation")
        .slide("Welcome", &["This is a simple presentation", "Created with ppt-rs"])
        .save("examples/output/simple_proper.pptx")?;
    println!("✓ Created: examples/output/simple_proper.pptx");

    // Example 2: Multi-slide presentation with bullet points
    println!("\nCreating multi_slide.pptx...");
    pptx!("Multi-Slide Presentation")
        .title_slide("Introduction")
        .slide("Agenda", &["Overview", "Details", "Summary", "Q&A"])
        .slide("Overview", &["Key concepts", "Main features", "Benefits"])
        .slide("Details", &["Technical specs", "Implementation", "Best practices"])
        .slide("Summary", &["Key takeaways", "Next steps"])
        .save("examples/output/multi_slide_proper.pptx")?;
    println!("✓ Created: examples/output/multi_slide_proper.pptx");

    // Example 3: Report with shapes
    println!("\nCreating report.pptx...");
    pptx!("Quarterly Report Q1 2025")
        .title_slide("Q1 2025 Report")
        .slide("Highlights", &["Revenue up 15%", "New customers: 500+", "Product launches: 3"])
        .shapes_slide("Key Metrics", vec![
            shapes::rect(1.0, 2.0, 2.0, 1.5)
                .with_fill(ShapeFill::new(colors::CORPORATE_BLUE))
                .with_text("Revenue"),
            shapes::rect(4.0, 2.0, 2.0, 1.5)
                .with_fill(ShapeFill::new(colors::CORPORATE_GREEN))
                .with_text("Growth"),
            shapes::rect(7.0, 2.0, 2.0, 1.5)
                .with_fill(ShapeFill::new(colors::CORPORATE_ORANGE))
                .with_text("Users"),
        ])
        .slide("Next Quarter", &["Goals", "Initiatives", "Timeline"])
        .save("examples/output/report_proper.pptx")?;
    println!("✓ Created: examples/output/report_proper.pptx");

    // Example 4: Training with gradient shapes
    println!("\nCreating training.pptx...");
    pptx!("Rust Training Course")
        .title_slide("Learn Rust Programming")
        .slide("Course Overview", &[
            "Introduction to Rust",
            "Ownership and Borrowing",
            "Structs and Enums",
            "Error Handling",
            "Concurrency",
        ])
        .slide("Why Rust?", &[
            "Memory safety without garbage collection",
            "Zero-cost abstractions",
            "Fearless concurrency",
            "Great tooling (cargo, rustfmt, clippy)",
        ])
        .shapes_slide("Rust Features", vec![
            shapes::circle(2.0, 2.5, 1.5)
                .with_fill(ShapeFill::new(colors::ORANGE))
                .with_text("Safe"),
            shapes::circle(5.0, 2.5, 1.5)
                .with_fill(ShapeFill::new(colors::BLUE))
                .with_text("Fast"),
            shapes::circle(8.0, 2.5, 1.5)
                .with_fill(ShapeFill::new(colors::GREEN))
                .with_text("Concurrent"),
        ])
        .slide("Getting Started", &[
            "Install rustup",
            "cargo new my_project",
            "cargo build && cargo run",
        ])
        .save("examples/output/training_proper.pptx")?;
    println!("✓ Created: examples/output/training_proper.pptx");

    println!("\n✅ All PPTX files generated successfully!");
    println!("\nGenerated files:");

    // Verify files
    for file in &[
        "examples/output/simple_proper.pptx",
        "examples/output/multi_slide_proper.pptx",
        "examples/output/report_proper.pptx",
        "examples/output/training_proper.pptx",
    ] {
        let metadata = fs::metadata(file)?;
        println!("  ✓ {} ({} bytes)", file, metadata.len());
    }

    Ok(())
}
