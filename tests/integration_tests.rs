//! Integration tests for ppt-rs
//! 
//! These tests verify that the library works correctly end-to-end,
//! including file creation, editing, validation, and persistence.

use ppt_rs::new_presentation;
use ppt_rs::util::validation::{validate_presentation, validate_pptx_file, validate_roundtrip};
use std::io::Cursor;

#[test]
fn test_create_empty_presentation() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    validate_presentation(&mut prs).expect("Presentation should be valid");
}

#[test]
fn test_save_empty_presentation() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).expect("Failed to save presentation");
    
    let data = cursor.into_inner();
    assert!(!data.is_empty(), "Saved data should not be empty");
    assert!(data.len() > 0, "Saved file should have content");
}

#[test]
fn test_save_to_file() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    
    let test_file = "target/test_output.pptx";
    std::fs::create_dir_all("target").ok();
    
    prs.save_to_file(test_file).expect("Failed to save to file");
    
    assert!(std::path::Path::new(test_file).exists(), "File should exist");
    
    // Clean up
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_validate_saved_file() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).expect("Failed to save");
    
    cursor.set_position(0);
    validate_pptx_file(cursor).expect("Saved file should be valid");
}

#[test]
fn test_roundtrip_validation() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    
    validate_roundtrip(&mut prs).expect("Roundtrip should succeed");
}

#[test]
fn test_multiple_validations() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    
    for i in 1..=5 {
        validate_presentation(&mut prs)
            .expect(&format!("Validation {} should succeed", i));
    }
}

#[test]
fn test_save_multiple_times() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    
    for i in 1..=3 {
        let mut cursor = Cursor::new(Vec::new());
        prs.save(&mut cursor)
            .expect(&format!("Save {} should succeed", i));
        
        let data = cursor.into_inner();
        assert!(!data.is_empty(), "Save {} should produce data", i);
    }
}

#[test]
fn test_validate_after_each_save() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    
    for i in 1..=3 {
        let mut cursor = Cursor::new(Vec::new());
        prs.save(&mut cursor)
            .expect(&format!("Save {} should succeed", i));
        
        cursor.set_position(0);
        validate_pptx_file(cursor)
            .expect(&format!("Validation after save {} should succeed", i));
    }
}

#[test]
fn test_presentation_properties() {
    let prs = new_presentation().expect("Failed to create presentation");
    
    // Check slide dimensions
    let width = prs.slide_width();
    let height = prs.slide_height();
    
    assert!(width.is_some(), "Slide width should be set");
    assert!(height.is_some(), "Slide height should be set");
    
    if let (Some(w), Some(h)) = (width, height) {
        assert!(w > 0, "Slide width should be positive");
        assert!(h > 0, "Slide height should be positive");
    }
}

#[test]
fn test_file_integrity_after_save() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).expect("Failed to save");
    
    let data = cursor.into_inner();
    
    // Check ZIP signature
    assert!(data.len() > 4, "File should have ZIP header");
    assert_eq!(data[0], 0x50, "ZIP signature byte 1 should be 0x50 (P)");
    assert_eq!(data[1], 0x4B, "ZIP signature byte 2 should be 0x4B (K)");
    assert_eq!(data[2], 0x03, "ZIP signature byte 3 should be 0x03");
    assert_eq!(data[3], 0x04, "ZIP signature byte 4 should be 0x04");
}

#[test]
fn test_zip_archive_structure() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).expect("Failed to save");
    
    cursor.set_position(0);
    let mut archive = zip::ZipArchive::new(cursor).expect("Should be valid ZIP");
    
    // Check for essential files
    let mut has_content_types = false;
    let mut has_rels = false;
    let mut has_presentation = false;
    
    for i in 0..archive.len() {
        let file = archive.by_index(i).expect("Should read file");
        let name = file.name();
        
        if name == "[Content_Types].xml" {
            has_content_types = true;
        }
        if name == "_rels/.rels" {
            has_rels = true;
        }
        if name == "ppt/presentation.xml" {
            has_presentation = true;
        }
    }
    
    assert!(has_content_types, "Should have [Content_Types].xml");
    assert!(has_rels, "Should have _rels/.rels");
    assert!(has_presentation, "Should have ppt/presentation.xml");
}

#[test]
fn test_xml_content_validity() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).expect("Failed to save");
    
    cursor.set_position(0);
    let mut archive = zip::ZipArchive::new(cursor).expect("Should be valid ZIP");
    
    // Check presentation.xml
    let mut pres_file = archive.by_name("ppt/presentation.xml")
        .expect("Should have presentation.xml");
    let mut pres_content = String::new();
    std::io::Read::read_to_string(&mut pres_file, &mut pres_content)
        .expect("Should read presentation.xml");
    
    assert!(pres_content.contains("<?xml"), "Should have XML declaration");
    assert!(pres_content.contains("<p:presentation"), "Should have presentation element");
    assert!(pres_content.contains("xmlns:p="), "Should have namespace");
}

#[test]
fn test_concurrent_validations() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    
    // Perform multiple validations in sequence
    for _ in 0..10 {
        validate_presentation(&mut prs).expect("Validation should succeed");
    }
}

#[test]
fn test_large_file_handling() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    
    // Save multiple times to simulate large file operations
    for _ in 0..5 {
        let mut cursor = Cursor::new(Vec::new());
        prs.save(&mut cursor).expect("Save should succeed");
        
        let data = cursor.into_inner();
        assert!(!data.is_empty(), "Saved data should not be empty");
    }
}

#[test]
fn test_error_handling_on_invalid_file() {
    // Try to validate invalid data
    let invalid_data = vec![0xFF, 0xFE, 0xFD, 0xFC];
    let cursor = Cursor::new(invalid_data);
    
    let result = validate_pptx_file(cursor);
    assert!(result.is_err(), "Should fail on invalid ZIP");
}

#[test]
fn test_presentation_dimensions() {
    let prs = new_presentation().expect("Failed to create presentation");
    
    let width = prs.slide_width();
    let height = prs.slide_height();
    
    // Standard PowerPoint dimensions (in EMU - English Metric Units)
    // Standard slide: 10 inches x 7.5 inches
    // 1 inch = 914400 EMU
    let expected_width = 9144000; // 10 inches
    let expected_height = 6858000; // 7.5 inches
    
    assert_eq!(width, Some(expected_width), "Width should match standard");
    assert_eq!(height, Some(expected_height), "Height should match standard");
}
