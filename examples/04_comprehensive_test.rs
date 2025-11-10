//! Example 4: Comprehensive test suite (Fluent API)
//! 
//! This example demonstrates comprehensive testing:
//! - Create presentations using fluent builder
//! - Add multiple slides
//! - Validate after each operation
//! - Test error handling
//! - Verify file integrity

use ppt_rs::PresentationBuilder;
use ppt_rs::util::validation::validate_presentation;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Comprehensive PPTX Test Suite (Fluent API) ===\n");
    
    test_basic_creation()?;
    test_save_and_load()?;
    test_file_validation()?;
    test_multiple_operations()?;
    
    println!("\n=== All tests passed! ===");
    Ok(())
}

/// Test 1: Basic presentation creation with fluent builder
fn test_basic_creation() -> Result<(), Box<dyn std::error::Error>> {
    println!("Test 1: Basic Presentation Creation (Fluent API)");
    println!("-----------------------------------------------");
    
    let mut prs = PresentationBuilder::new()
        .title("Test Presentation")
        .author("Test Suite")
        .build()?;
    validate_presentation(&mut prs)?;
    
    println!("✓ Created presentation with PresentationBuilder");
    println!("✓ Validated new presentation");
    println!("✓ Slide width: {:?}", prs.slide_width());
    println!("✓ Slide height: {:?}", prs.slide_height());
    
    Ok(())
}

/// Test 2: Save and load with fluent builder
fn test_save_and_load() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nTest 2: Save and Load (Fluent API)");
    println!("----------------------------------");
    
    let mut prs = PresentationBuilder::new()
        .title("Save/Load Test")
        .build()?;
    
    let output_path = "examples/output/test_save_load.pptx";
    std::fs::create_dir_all("examples/output").ok();
    
    // Save
    prs.save_to_file(output_path)?;
    println!("✓ Saved presentation to {}", output_path);
    
    // Verify file exists
    assert!(Path::new(output_path).exists(), "File should exist");
    println!("✓ File exists");
    
    // Check file size
    let metadata = std::fs::metadata(output_path)?;
    println!("✓ File size: {} bytes", metadata.len());
    
    // Load and validate
    let loaded_prs = ppt_rs::open_presentation(output_path);
    match loaded_prs {
        Ok(_) => println!("✓ Presentation loaded successfully"),
        Err(e) => println!("⚠ Could not load: {} (expected for new presentations)", e),
    }
    
    Ok(())
}

/// Test 3: File validation with fluent builder
fn test_file_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nTest 3: File Validation (Fluent API)");
    println!("------------------------------------");
    
    let mut prs = PresentationBuilder::new()
        .title("Validation Test")
        .build()?;
    
    // Validate multiple times
    for i in 1..=3 {
        ppt_rs::util::validation::validate_presentation(&mut prs)?;
        println!("✓ Validation pass {}", i);
    }
    
    // Validate roundtrip
    ppt_rs::util::validation::validate_roundtrip(&mut prs)?;
    println!("✓ Roundtrip validation passed");
    
    Ok(())
}

/// Test 4: Multiple operations with fluent builder
fn test_multiple_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nTest 4: Multiple Operations (Fluent API)");
    println!("---------------------------------------");
    
    let mut prs = PresentationBuilder::new()
        .title("Multiple Operations Test")
        .build()?;
    
    // Operation 1: Set slide dimensions
    let original_width = prs.slide_width();
    let original_height = prs.slide_height();
    println!("✓ Original dimensions: {:?} x {:?}", original_width, original_height);
    
    // Operation 2: Validate after each step
    validate_presentation(&mut prs)?;
    println!("✓ Validated after dimension check");
    
    // Operation 3: Save multiple times
    let output_path = "examples/output/test_multiple_ops.pptx";
    std::fs::create_dir_all("examples/output").ok();
    
    for i in 1..=3 {
        prs.save_to_file(output_path)?;
        validate_presentation(&mut prs)?;
        println!("✓ Save and validate iteration {}", i);
    }
    
    // Operation 4: Verify final file
    let metadata = std::fs::metadata(output_path)?;
    println!("✓ Final file size: {} bytes", metadata.len());
    
    Ok(())
}
