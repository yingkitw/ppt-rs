//! Comprehensive integrated example using all PPTX modules
//!
//! Run with: cargo run --example integrated_example

use std::fs;
use pptx_rs::integration::{PresentationBuilder, SlideBuilder, PresentationMetadata};
use pptx_rs::integration::utils;
use pptx_rs::enums;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     Integrated PPTX Generation Example                    ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Create output directory
    fs::create_dir_all("examples/output")?;

    // Example 1: Using PresentationBuilder
    println!("1. Creating presentation with PresentationBuilder...");
    let builder = PresentationBuilder::new("Integrated Example")
        .with_slides(5);
    builder.save_to_file("examples/output/integrated_example.pptx")?;
    println!("   ✓ Created: examples/output/integrated_example.pptx\n");

    // Example 2: Using metadata
    println!("2. Creating presentation with metadata...");
    let metadata = PresentationMetadata::new("Business Report", 8);
    println!("   Title: {}", metadata.title);
    println!("   Slides: {}", metadata.slides);
    println!("   Created: {}", metadata.created);
    println!("   Modified: {}\n", metadata.modified);

    let builder = PresentationBuilder::new(&metadata.title)
        .with_slides(metadata.slides);
    builder.save_to_file("examples/output/business_report.pptx")?;
    println!("   ✓ Created: examples/output/business_report.pptx\n");

    // Example 3: Using slide builder
    println!("3. Creating slides with SlideBuilder...");
    let slide1 = SlideBuilder::new("Introduction")
        .with_content("Welcome to the presentation");
    let (title1, content1) = slide1.build();
    println!("   Slide 1: {} - {}", title1, content1);

    let slide2 = SlideBuilder::new("Main Content")
        .with_content("Key points and details");
    let (title2, content2) = slide2.build();
    println!("   Slide 2: {} - {}\n", title2, content2);

    // Example 4: Using utility functions
    println!("4. Using utility functions...");
    let inches = utils::inches_to_emu(1.0);
    println!("   1 inch = {} EMU", inches);

    let cm = utils::cm_to_emu(2.54);
    println!("   2.54 cm = {} EMU", cm);

    let pt = utils::pt_to_emu(12.0);
    println!("   12 pt = {} EMU\n", pt);

    // Example 5: File size formatting
    println!("5. File size formatting...");
    println!("   {} bytes = {}", 512, utils::format_size(512));
    println!("   {} bytes = {}", 5637, utils::format_size(5637));
    println!("   {} bytes = {}\n", 1024 * 1024, utils::format_size(1024 * 1024));

    // Example 6: Using enumerations
    println!("6. Using enumeration types...");
    println!("   Action: {}", enums::action::PpActionType::HYPERLINK);
    println!("   Chart: {}", enums::chart::XlChartType::COLUMN_CLUSTERED);
    println!("   Shape: {}", enums::shapes::MsoShapeType::AUTO_SHAPE);
    println!("   Text: {}", enums::text::PpParagraphAlignment::CENTER);
    println!("   Language: {}", enums::lang::MsoLanguageID::ENGLISH_US);

    // Example 7: Verify generated files
    println!("\n7. Verifying generated files...");
    for file in &[
        "examples/output/integrated_example.pptx",
        "examples/output/business_report.pptx",
    ] {
        if let Ok(metadata) = fs::metadata(file) {
            let size = utils::format_size(metadata.len() as usize);
            println!("   ✓ {} ({})", file, size);
        }
    }

    println!("\n✅ Integrated example completed successfully!");
    println!("\nKey features demonstrated:");
    println!("  • PresentationBuilder for easy creation");
    println!("  • SlideBuilder for slide management");
    println!("  • PresentationMetadata for tracking");
    println!("  • Utility functions for conversions");
    println!("  • Enumeration types for type safety");
    println!("  • File size formatting");
    println!("  • Error handling with Result types");

    Ok(())
}
