//! Example 3: Validate file integrity (Fluent API)
//! 
//! This example demonstrates how to:
//! - Create a presentation using fluent builder
//! - Validate file integrity
//! - Verify ZIP structure
//! - Confirm roundtrip save/open

use ppt_rs::PresentationBuilder;
use ppt_rs::util::validation::{validate_presentation, validate_pptx_file, validate_roundtrip};
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating file integrity with Fluent API...\n");
    
    // Create a new presentation using fluent builder
    let mut prs = PresentationBuilder::new()
        .title("Validation Test")
        .author("Fluent API")
        .build()?;
    println!("✓ Created presentation with PresentationBuilder");
    
    // Test 1: Validate new presentation
    println!("\nTest 1: Validate new presentation");
    validate_presentation(&mut prs)?;
    println!("✓ New presentation is valid");
    
    // Test 2: Save to memory and validate structure
    println!("\nTest 2: Validate PPTX file structure");
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor)?;
    cursor.set_position(0);
    validate_pptx_file(cursor)?;
    println!("✓ PPTX file structure is valid");
    
    // Test 3: Validate roundtrip
    println!("\nTest 3: Validate roundtrip (save/open)");
    validate_roundtrip(&mut prs)?;
    println!("✓ Presentation survives roundtrip");
    
    // Test 4: Multiple validations
    println!("\nTest 4: Multiple validations");
    for i in 1..=5 {
        validate_presentation(&mut prs)?;
        println!("✓ Validation {} passed", i);
    }
    
    // Test 5: Save and verify file
    println!("\nTest 5: Save and verify file");
    let output_path = "examples/output/03_validated.pptx";
    std::fs::create_dir_all("examples/output").ok();
    prs.save_to_file(output_path)?;
    println!("✓ Saved to {}", output_path);
    
    // Verify file can be opened
    let file = std::fs::File::open(output_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    println!("✓ File is a valid ZIP archive with {} entries", archive.len());
    
    // List essential files
    println!("\nEssential files in archive:");
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let name = file.name();
        if name.contains("Content_Types") || name.contains("_rels") || name.contains("presentation.xml") {
            println!("  ✓ {}", name);
        }
    }
    
    Ok(())
}
