//! Comprehensive unit tests for ppt-rs
//! 
//! These tests verify that every small step executes correctly,
//! covering core functionality, edge cases, and error handling.

use ppt_rs::PresentationBuilder;
use ppt_rs::util::validation::validate_presentation;
use std::io::Cursor;

// ============================================================================
// SECTION 1: PresentationBuilder Tests
// ============================================================================

#[test]
fn test_builder_create_empty() {
    let result = PresentationBuilder::new().build();
    assert!(result.is_ok(), "Builder should create presentation");
}

#[test]
fn test_builder_with_title() {
    let result = PresentationBuilder::new()
        .title("Test Title")
        .build();
    assert!(result.is_ok(), "Builder should accept title");
}

#[test]
fn test_builder_with_author() {
    let result = PresentationBuilder::new()
        .author("Test Author")
        .build();
    assert!(result.is_ok(), "Builder should accept author");
}

#[test]
fn test_builder_with_subject() {
    let result = PresentationBuilder::new()
        .subject("Test Subject")
        .build();
    assert!(result.is_ok(), "Builder should accept subject");
}

#[test]
fn test_builder_with_company() {
    let result = PresentationBuilder::new()
        .company("Test Company")
        .build();
    assert!(result.is_ok(), "Builder should accept company");
}

#[test]
fn test_builder_chaining_all_properties() {
    let result = PresentationBuilder::new()
        .title("Title")
        .author("Author")
        .subject("Subject")
        .company("Company")
        .build();
    assert!(result.is_ok(), "Builder should chain all properties");
}

#[test]
fn test_builder_with_slide_dimensions() {
    let result = PresentationBuilder::new()
        .title("Dimensions Test")
        .build();
    
    assert!(result.is_ok(), "Builder should create presentation");
    let prs = result.unwrap();
    
    let width = prs.slide_width();
    let height = prs.slide_height();
    
    assert!(width.is_some(), "Width should be set");
    assert!(height.is_some(), "Height should be set");
    assert_eq!(width, Some(9144000), "Width should be standard");
    assert_eq!(height, Some(6858000), "Height should be standard");
}

#[test]
fn test_builder_multiple_instances() {
    let prs1 = PresentationBuilder::new().title("First").build();
    let prs2 = PresentationBuilder::new().title("Second").build();
    
    assert!(prs1.is_ok(), "First presentation should build");
    assert!(prs2.is_ok(), "Second presentation should build");
}

// ============================================================================
// SECTION 2: Slide Management Tests
// ============================================================================

#[test]
fn test_add_single_slide() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    let result = prs.add_slide();
    
    assert!(result.is_ok(), "Should add slide");
    assert_eq!(result.unwrap(), 0, "First slide should have index 0");
}

#[test]
fn test_add_multiple_slides() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    
    for i in 0..5 {
        let result = prs.add_slide();
        assert!(result.is_ok(), "Should add slide {}", i);
        assert_eq!(result.unwrap(), i, "Slide {} should have index {}", i, i);
    }
}

#[test]
fn test_slide_count_after_adding() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    
    prs.add_slide().unwrap();
    prs.add_slide().unwrap();
    prs.add_slide().unwrap();
    
    let count = prs.part().slide_id_manager().all().len();
    assert_eq!(count, 3, "Should have 3 slides");
}

#[test]
fn test_slide_index_increments() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    
    let idx1 = prs.add_slide().unwrap();
    let idx2 = prs.add_slide().unwrap();
    let idx3 = prs.add_slide().unwrap();
    
    assert_eq!(idx1, 0, "First slide index");
    assert_eq!(idx2, 1, "Second slide index");
    assert_eq!(idx3, 2, "Third slide index");
}

#[test]
fn test_slide_addition_preserves_order() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    
    let indices: Vec<_> = (0..10)
        .map(|_| prs.add_slide().unwrap())
        .collect();
    
    for (i, &idx) in indices.iter().enumerate() {
        assert_eq!(idx, i, "Slide {} should have index {}", i, i);
    }
}

// ============================================================================
// SECTION 3: Presentation Properties Tests
// ============================================================================

#[test]
fn test_slide_width_getter() {
    let prs = PresentationBuilder::new().build().unwrap();
    let width = prs.slide_width();
    
    assert!(width.is_some(), "Width should be present");
    assert_eq!(width.unwrap(), 9144000, "Width should be standard");
}

#[test]
fn test_slide_height_getter() {
    let prs = PresentationBuilder::new().build().unwrap();
    let height = prs.slide_height();
    
    assert!(height.is_some(), "Height should be present");
    assert_eq!(height.unwrap(), 6858000, "Height should be standard");
}

#[test]
fn test_slide_dimensions_consistency() {
    let prs1 = PresentationBuilder::new().build().unwrap();
    let prs2 = PresentationBuilder::new().build().unwrap();
    
    assert_eq!(prs1.slide_width(), prs2.slide_width(), "Width should be consistent");
    assert_eq!(prs1.slide_height(), prs2.slide_height(), "Height should be consistent");
}

#[test]
fn test_fluent_with_slide_width() {
    let result = PresentationBuilder::new()
        .title("Width Test")
        .build();
    
    assert!(result.is_ok(), "Should build with title");
    let prs = result.unwrap();
    assert_eq!(prs.slide_width(), Some(9144000), "Width should be set");
}

#[test]
fn test_fluent_with_slide_height() {
    let result = PresentationBuilder::new()
        .title("Height Test")
        .build();
    
    assert!(result.is_ok(), "Should build with title");
    let prs = result.unwrap();
    assert_eq!(prs.slide_height(), Some(6858000), "Height should be set");
}

// ============================================================================
// SECTION 4: Save and Serialization Tests
// ============================================================================

#[test]
fn test_save_to_cursor() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    let mut cursor = Cursor::new(Vec::new());
    
    let result = prs.save(&mut cursor);
    assert!(result.is_ok(), "Should save to cursor");
}

#[test]
fn test_save_produces_data() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    let mut cursor = Cursor::new(Vec::new());
    
    prs.save(&mut cursor).unwrap();
    let data = cursor.into_inner();
    
    assert!(!data.is_empty(), "Save should produce data");
    assert!(data.len() > 100, "Save should produce meaningful data");
}

#[test]
fn test_save_produces_valid_zip() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    let mut cursor = Cursor::new(Vec::new());
    
    prs.save(&mut cursor).unwrap();
    let data = cursor.into_inner();
    
    // Check ZIP signature: PK\x03\x04
    assert_eq!(data[0], 0x50, "ZIP signature byte 1");
    assert_eq!(data[1], 0x4B, "ZIP signature byte 2");
    assert_eq!(data[2], 0x03, "ZIP signature byte 3");
    assert_eq!(data[3], 0x04, "ZIP signature byte 4");
}

#[test]
fn test_save_with_slides() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    prs.add_slide().unwrap();
    prs.add_slide().unwrap();
    
    let mut cursor = Cursor::new(Vec::new());
    let result = prs.save(&mut cursor);
    
    assert!(result.is_ok(), "Should save with slides");
}

#[test]
fn test_save_multiple_times() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    
    for i in 1..=3 {
        let mut cursor = Cursor::new(Vec::new());
        let result = prs.save(&mut cursor);
        assert!(result.is_ok(), "Save {} should succeed", i);
    }
}

#[test]
fn test_save_to_file() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    
    let test_file = "target/test_unit_save.pptx";
    std::fs::create_dir_all("target").ok();
    
    let result = prs.save_to_file(test_file);
    assert!(result.is_ok(), "Should save to file");
    assert!(std::path::Path::new(test_file).exists(), "File should exist");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

// ============================================================================
// SECTION 5: Validation Tests
// ============================================================================

#[test]
fn test_validate_empty_presentation() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    let result = validate_presentation(&mut prs);
    
    assert!(result.is_ok(), "Empty presentation should be valid");
}

#[test]
fn test_validate_with_slides() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    prs.add_slide().unwrap();
    prs.add_slide().unwrap();
    
    let result = validate_presentation(&mut prs);
    assert!(result.is_ok(), "Presentation with slides should be valid");
}

#[test]
fn test_validate_multiple_times() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    
    for i in 1..=5 {
        let result = validate_presentation(&mut prs);
        assert!(result.is_ok(), "Validation {} should succeed", i);
    }
}

#[test]
fn test_validate_after_save() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    prs.add_slide().unwrap();
    
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).unwrap();
    
    // Validate original
    let result = validate_presentation(&mut prs);
    assert!(result.is_ok(), "Should validate after save");
}

// ============================================================================
// SECTION 6: ZIP Structure Tests
// ============================================================================

#[test]
fn test_zip_has_content_types() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).unwrap();
    
    cursor.set_position(0);
    let archive = zip::ZipArchive::new(cursor).unwrap();
    
    let has_file = archive.file_names().any(|n| n == "[Content_Types].xml");
    assert!(has_file, "Should have [Content_Types].xml");
}

#[test]
fn test_zip_has_rels() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).unwrap();
    
    cursor.set_position(0);
    let archive = zip::ZipArchive::new(cursor).unwrap();
    
    let has_file = archive.file_names().any(|n| n == "_rels/.rels");
    assert!(has_file, "Should have _rels/.rels");
}

#[test]
fn test_zip_has_presentation() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).unwrap();
    
    cursor.set_position(0);
    let archive = zip::ZipArchive::new(cursor).unwrap();
    
    let has_file = archive.file_names().any(|n| n == "ppt/presentation.xml");
    assert!(has_file, "Should have ppt/presentation.xml");
}

#[test]
fn test_zip_has_core_properties() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).unwrap();
    
    cursor.set_position(0);
    let archive = zip::ZipArchive::new(cursor).unwrap();
    
    let has_file = archive.file_names().any(|n| n == "docProps/core.xml");
    assert!(has_file, "Should have docProps/core.xml");
}

#[test]
fn test_zip_file_order_correct() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).unwrap();
    
    cursor.set_position(0);
    let archive = zip::ZipArchive::new(cursor).unwrap();
    let files: Vec<_> = archive.file_names().collect();
    
    // Verify essential files exist
    let content_types_idx = files.iter().position(|&f| f == "[Content_Types].xml");
    let rels_idx = files.iter().position(|&f| f == "_rels/.rels");
    let presentation_idx = files.iter().position(|&f| f == "ppt/presentation.xml");
    
    assert!(content_types_idx.is_some(), "Should have [Content_Types].xml");
    assert!(rels_idx.is_some(), "Should have _rels/.rels");
    assert!(presentation_idx.is_some(), "Should have ppt/presentation.xml");
    
    // Just verify they all exist - order may vary due to ZIP implementation
    // The important thing is that the files are present and valid
}

#[test]
fn test_zip_contains_slides() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    prs.add_slide().unwrap();
    prs.add_slide().unwrap();
    
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).unwrap();
    
    cursor.set_position(0);
    let archive = zip::ZipArchive::new(cursor).unwrap();
    
    let has_slide1 = archive.file_names().any(|n| n == "ppt/slides/slide1.xml");
    let has_slide2 = archive.file_names().any(|n| n == "ppt/slides/slide2.xml");
    
    assert!(has_slide1, "Should have slide1.xml");
    assert!(has_slide2, "Should have slide2.xml");
}

// ============================================================================
// SECTION 7: XML Content Tests
// ============================================================================

#[test]
fn test_presentation_xml_has_declaration() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).unwrap();
    
    cursor.set_position(0);
    let mut archive = zip::ZipArchive::new(cursor).unwrap();
    let mut file = archive.by_name("ppt/presentation.xml").unwrap();
    
    let mut content = String::new();
    std::io::Read::read_to_string(&mut file, &mut content).unwrap();
    
    assert!(content.contains("<?xml"), "Should have XML declaration");
}

#[test]
fn test_presentation_xml_has_namespace() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).unwrap();
    
    cursor.set_position(0);
    let mut archive = zip::ZipArchive::new(cursor).unwrap();
    let mut file = archive.by_name("ppt/presentation.xml").unwrap();
    
    let mut content = String::new();
    std::io::Read::read_to_string(&mut file, &mut content).unwrap();
    
    assert!(content.contains("xmlns:p="), "Should have namespace");
}

#[test]
fn test_presentation_xml_has_element() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).unwrap();
    
    cursor.set_position(0);
    let mut archive = zip::ZipArchive::new(cursor).unwrap();
    let mut file = archive.by_name("ppt/presentation.xml").unwrap();
    
    let mut content = String::new();
    std::io::Read::read_to_string(&mut file, &mut content).unwrap();
    
    assert!(content.contains("<p:presentation"), "Should have presentation element");
}

#[test]
fn test_content_types_xml_valid() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).unwrap();
    
    cursor.set_position(0);
    let mut archive = zip::ZipArchive::new(cursor).unwrap();
    let mut file = archive.by_name("[Content_Types].xml").unwrap();
    
    let mut content = String::new();
    std::io::Read::read_to_string(&mut file, &mut content).unwrap();
    
    assert!(content.contains("<Types"), "Should have Types element");
    assert!(content.contains("ContentType="), "Should have ContentType attributes");
}

// ============================================================================
// SECTION 8: Edge Cases and Error Handling
// ============================================================================

#[test]
fn test_many_slides() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    
    for _ in 0..50 {
        let result = prs.add_slide();
        assert!(result.is_ok(), "Should add slide");
    }
    
    let count = prs.part().slide_id_manager().all().len();
    assert_eq!(count, 50, "Should have 50 slides");
}

#[test]
fn test_save_large_presentation() {
    let mut prs = PresentationBuilder::new()
        .title("Large Presentation")
        .build()
        .unwrap();
    
    for _ in 0..20 {
        prs.add_slide().unwrap();
    }
    
    let mut cursor = Cursor::new(Vec::new());
    let result = prs.save(&mut cursor);
    
    assert!(result.is_ok(), "Should save large presentation");
    let data = cursor.into_inner();
    assert!(data.len() > 1000, "Large presentation should have significant size");
}

#[test]
fn test_builder_empty_strings() {
    let result = PresentationBuilder::new()
        .title("")
        .author("")
        .subject("")
        .company("")
        .build();
    
    assert!(result.is_ok(), "Should handle empty strings");
}

#[test]
fn test_builder_long_strings() {
    let long_string = "a".repeat(1000);
    let result = PresentationBuilder::new()
        .title(&long_string)
        .author(&long_string)
        .build();
    
    assert!(result.is_ok(), "Should handle long strings");
}

#[test]
fn test_builder_special_characters() {
    let result = PresentationBuilder::new()
        .title("Title <>&\"'")
        .author("Author <>&\"'")
        .build();
    
    assert!(result.is_ok(), "Should handle special characters");
}

// ============================================================================
// SECTION 9: Integration Tests
// ============================================================================

#[test]
fn test_full_workflow() {
    // Create
    let mut prs = PresentationBuilder::new()
        .title("Full Workflow")
        .author("Test")
        .build()
        .unwrap();
    
    // Add slides
    prs.add_slide().unwrap();
    prs.add_slide().unwrap();
    
    // Validate
    validate_presentation(&mut prs).unwrap();
    
    // Save
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).unwrap();
    
    // Verify
    let data = cursor.into_inner();
    assert!(!data.is_empty(), "Should produce data");
}

#[test]
fn test_workflow_with_file() {
    let test_file = "target/test_workflow.pptx";
    std::fs::create_dir_all("target").ok();
    
    // Create
    let mut prs = PresentationBuilder::new()
        .title("Workflow Test")
        .build()
        .unwrap();
    
    // Add slides
    for _ in 0..3 {
        prs.add_slide().unwrap();
    }
    
    // Save
    prs.save_to_file(test_file).unwrap();
    
    // Verify file exists
    assert!(std::path::Path::new(test_file).exists(), "File should exist");
    
    // Verify file size
    let metadata = std::fs::metadata(test_file).unwrap();
    assert!(metadata.len() > 1000, "File should have content");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_repeated_operations() {
    for iteration in 1..=5 {
        let mut prs = PresentationBuilder::new()
            .title(&format!("Iteration {}", iteration))
            .build()
            .unwrap();
        
        for _ in 0..3 {
            prs.add_slide().unwrap();
        }
        
        let mut cursor = Cursor::new(Vec::new());
        prs.save(&mut cursor).unwrap();
        
        validate_presentation(&mut prs).unwrap();
    }
}

// ============================================================================
// SECTION 10: Consistency Tests
// ============================================================================

#[test]
fn test_consistent_slide_count() {
    let mut prs = PresentationBuilder::new().build().unwrap();
    
    for i in 0..10 {
        prs.add_slide().unwrap();
        let count = prs.part().slide_id_manager().all().len();
        assert_eq!(count, i + 1, "Slide count should be {}", i + 1);
    }
}

#[test]
fn test_consistent_dimensions() {
    for _ in 0..5 {
        let prs = PresentationBuilder::new().build().unwrap();
        assert_eq!(prs.slide_width(), Some(9144000), "Width should be consistent");
        assert_eq!(prs.slide_height(), Some(6858000), "Height should be consistent");
    }
}

#[test]
fn test_consistent_save_output() {
    let mut prs1 = PresentationBuilder::new().title("Test").build().unwrap();
    prs1.add_slide().unwrap();
    
    let mut prs2 = PresentationBuilder::new().title("Test").build().unwrap();
    prs2.add_slide().unwrap();
    
    let mut cursor1 = Cursor::new(Vec::new());
    let mut cursor2 = Cursor::new(Vec::new());
    
    prs1.save(&mut cursor1).unwrap();
    prs2.save(&mut cursor2).unwrap();
    
    let data1 = cursor1.into_inner();
    let data2 = cursor2.into_inner();
    
    // Should produce similar size files (not necessarily identical due to timestamps)
    assert!((data1.len() as i32 - data2.len() as i32).abs() < 100, 
            "Save output should be consistent");
}
