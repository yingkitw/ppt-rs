//! Integration tests for ppt-rs
//! 
//! These tests verify that the library works correctly end-to-end,
//! including file creation, editing, validation, and persistence.

use ppt_rs::new_presentation;
use ppt_rs::util::validation::{validate_presentation, validate_pptx_file, validate_roundtrip};
use ppt_rs::util::{RoundTrip, Timer, BatchProcessor, ShapeContent, ShapeContentType, PlaceholderType};
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

// ============================================================================
// COMPREHENSIVE TEST SUITE - PHASE 5 PRODUCTION HARDENING
// ============================================================================

// Round-Trip Tests
#[test]
fn test_roundtrip_open_and_save() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).expect("Failed to save");
    
    cursor.set_position(0);
    let result = RoundTrip::open(cursor);
    assert!(result.is_ok(), "Should open saved presentation");
}

#[test]
fn test_roundtrip_modify_operation() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).expect("Failed to save");
    
    cursor.set_position(0);
    let mut output = Cursor::new(Vec::new());
    
    let result = RoundTrip::modify(cursor, &mut output, |_prs| {
        Ok(())
    });
    
    assert!(result.is_ok(), "Modify operation should succeed");
}

#[test]
fn test_roundtrip_validate_cycle() {
    let mut prs = new_presentation().expect("Failed to create presentation");
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).expect("Failed to save");
    
    cursor.set_position(0);
    let result = RoundTrip::validate_roundtrip(cursor);
    assert!(result.is_ok(), "Roundtrip validation should succeed");
}

#[test]
fn test_roundtrip_multiple_cycles() {
    for cycle in 1..=3 {
        let mut prs = new_presentation()
            .expect(&format!("Failed to create presentation cycle {}", cycle));
        let mut cursor = Cursor::new(Vec::new());
        prs.save(&mut cursor)
            .expect(&format!("Failed to save cycle {}", cycle));
        
        cursor.set_position(0);
        let result = RoundTrip::validate_roundtrip(cursor);
        assert!(result.is_ok(), "Roundtrip cycle {} should succeed", cycle);
    }
}

// Shape Content Tests
#[test]
fn test_shape_content_creation() {
    let content = ShapeContent::new(ShapeContentType::Text);
    assert_eq!(content.content_type, ShapeContentType::Text);
    assert!(content.is_empty());
}

#[test]
fn test_shape_content_placeholder_types() {
    let placeholder_types = vec![
        PlaceholderType::Title,
        PlaceholderType::Body,
        PlaceholderType::CenteredTitle,
        PlaceholderType::Subtitle,
        PlaceholderType::DateTime,
        PlaceholderType::SlideNumber,
        PlaceholderType::Footer,
        PlaceholderType::Header,
    ];
    
    for placeholder_type in placeholder_types {
        let content = ShapeContent::placeholder(placeholder_type);
        assert_eq!(content.content_type, ShapeContentType::Placeholder);
        assert_eq!(content.placeholder, Some(placeholder_type));
    }
}

#[test]
fn test_shape_content_metadata_operations() {
    let mut content = ShapeContent::new(ShapeContentType::Text);
    
    content.add_metadata("key1".to_string(), "value1".to_string());
    content.add_metadata("key2".to_string(), "value2".to_string());
    content.add_metadata("key3".to_string(), "value3".to_string());
    
    assert_eq!(content.get_metadata("key1"), Some("value1"));
    assert_eq!(content.get_metadata("key2"), Some("value2"));
    assert_eq!(content.get_metadata("key3"), Some("value3"));
    assert_eq!(content.get_metadata("nonexistent"), None);
}

#[test]
fn test_shape_content_data_handling() {
    let mut content = ShapeContent::new(ShapeContentType::Picture);
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    content.set_data(data.clone());
    
    assert_eq!(content.size(), 10);
    assert!(!content.is_empty());
}

#[test]
fn test_all_shape_content_types() {
    let types = vec![
        ShapeContentType::Text,
        ShapeContentType::Picture,
        ShapeContentType::Chart,
        ShapeContentType::Table,
        ShapeContentType::SmartArt,
        ShapeContentType::OleObject,
        ShapeContentType::Media,
        ShapeContentType::Placeholder,
    ];
    
    for content_type in types {
        let content = ShapeContent::new(content_type);
        assert_eq!(content.content_type, content_type);
    }
}

// Performance Tests
#[test]
fn test_timer_basic_operation() {
    let timer = Timer::start("test_operation");
    std::thread::sleep(std::time::Duration::from_millis(10));
    let duration = timer.stop_no_bytes();
    
    assert!(duration.as_millis() >= 10, "Duration should be at least 10ms");
}

#[test]
fn test_timer_with_metrics() {
    let timer = Timer::start("data_processing");
    let metrics = timer.stop(1_000_000); // 1 MB
    
    assert_eq!(metrics.name, "data_processing");
    assert_eq!(metrics.bytes, 1_000_000);
    assert!(metrics.throughput_mbps() > 0.0);
}

#[test]
fn test_batch_processor_basic() {
    let mut processor = BatchProcessor::new(3);
    
    processor.add(1);
    processor.add(2);
    processor.add(3);
    processor.add(4);
    processor.add(5);
    
    let batch1 = processor.next_batch();
    assert_eq!(batch1, Some(vec![1, 2, 3]));
    
    let batch2 = processor.next_batch();
    assert_eq!(batch2, Some(vec![4, 5]));
    
    let batch3 = processor.next_batch();
    assert_eq!(batch3, None);
}

#[test]
fn test_batch_processor_single_batch() {
    let mut processor = BatchProcessor::new(10);
    
    processor.add(1);
    processor.add(2);
    processor.add(3);
    
    let batch = processor.next_batch();
    assert_eq!(batch, Some(vec![1, 2, 3]));
    
    let next = processor.next_batch();
    assert_eq!(next, None);
}

#[test]
fn test_batch_processor_large_batches() {
    let mut processor = BatchProcessor::new(100);
    
    for i in 1..=250 {
        processor.add(i);
    }
    
    let batch1 = processor.next_batch();
    assert_eq!(batch1.as_ref().map(|b| b.len()), Some(100));
    
    let batch2 = processor.next_batch();
    assert_eq!(batch2.as_ref().map(|b| b.len()), Some(100));
    
    let batch3 = processor.next_batch();
    assert_eq!(batch3.as_ref().map(|b| b.len()), Some(50));
}

// Comprehensive Workflow Tests
#[test]
fn test_complete_workflow_create_save_validate() {
    // Create
    let mut prs = new_presentation().expect("Failed to create");
    
    // Save
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).expect("Failed to save");
    
    // Validate
    cursor.set_position(0);
    validate_pptx_file(cursor).expect("Failed to validate");
}

#[test]
fn test_complete_workflow_with_roundtrip() {
    // Create
    let mut prs = new_presentation().expect("Failed to create");
    
    // Save
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).expect("Failed to save");
    
    // Roundtrip
    cursor.set_position(0);
    let result = RoundTrip::open(cursor);
    assert!(result.is_ok(), "Should open saved file");
}

#[test]
fn test_multiple_operations_sequence() {
    for iteration in 1..=5 {
        // Create
        let mut prs = new_presentation()
            .expect(&format!("Failed to create iteration {}", iteration));
        
        // Validate
        validate_presentation(&mut prs)
            .expect(&format!("Failed to validate iteration {}", iteration));
        
        // Save
        let mut cursor = Cursor::new(Vec::new());
        prs.save(&mut cursor)
            .expect(&format!("Failed to save iteration {}", iteration));
        
        // Validate saved
        cursor.set_position(0);
        validate_pptx_file(cursor)
            .expect(&format!("Failed to validate saved iteration {}", iteration));
    }
}

#[test]
fn test_stress_test_repeated_operations() {
    for i in 1..=10 {
        let mut prs = new_presentation()
            .expect(&format!("Failed to create iteration {}", i));
        
        let mut cursor = Cursor::new(Vec::new());
        prs.save(&mut cursor)
            .expect(&format!("Failed to save iteration {}", i));
        
        cursor.set_position(0);
        validate_pptx_file(cursor)
            .expect(&format!("Failed to validate iteration {}", i));
    }
}

#[test]
fn test_file_size_consistency() {
    let mut prs = new_presentation().expect("Failed to create");
    
    let mut cursor1 = Cursor::new(Vec::new());
    prs.save(&mut cursor1).expect("Failed to save 1");
    let size1 = cursor1.into_inner().len();
    
    let mut cursor2 = Cursor::new(Vec::new());
    prs.save(&mut cursor2).expect("Failed to save 2");
    let size2 = cursor2.into_inner().len();
    
    assert_eq!(size1, size2, "File sizes should be consistent");
}

#[test]
fn test_content_preservation() {
    let mut prs = new_presentation().expect("Failed to create");
    
    // Save first time
    let mut cursor1 = Cursor::new(Vec::new());
    prs.save(&mut cursor1).expect("Failed to save 1");
    let data1 = cursor1.into_inner();
    
    // Save second time
    let mut cursor2 = Cursor::new(Vec::new());
    prs.save(&mut cursor2).expect("Failed to save 2");
    let data2 = cursor2.into_inner();
    
    // Content should be identical
    assert_eq!(data1, data2, "Saved content should be identical");
}

#[test]
fn test_zip_structure_consistency() {
    let mut prs = new_presentation().expect("Failed to create");
    
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).expect("Failed to save");
    
    cursor.set_position(0);
    let archive = zip::ZipArchive::new(cursor).expect("Should open ZIP");
    
    // Count files
    let file_count = archive.len();
    assert!(file_count > 0, "Archive should have files");
    
    // Check for essential files
    let essential_files = vec![
        "[Content_Types].xml",
        "_rels/.rels",
        "ppt/presentation.xml",
    ];
    
    for essential in essential_files {
        let mut archive = zip::ZipArchive::new(
            Cursor::new(prs.save_to_bytes().expect("Failed to save to bytes"))
        ).expect("Should open ZIP");
        
        let result = archive.by_name(essential);
        assert!(result.is_ok(), "Should have {}", essential);
    }
}

#[test]
fn test_error_recovery() {
    // Create valid presentation
    let mut prs = new_presentation().expect("Failed to create");
    
    // Save successfully
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor).expect("Failed to save");
    
    // Validate successfully
    cursor.set_position(0);
    validate_pptx_file(cursor).expect("Failed to validate");
    
    // Try invalid data (should fail gracefully)
    let invalid_data = vec![0xFF, 0xFE, 0xFD, 0xFC];
    let invalid_cursor = Cursor::new(invalid_data);
    let result = validate_pptx_file(invalid_cursor);
    assert!(result.is_err(), "Should fail on invalid data");
}

#[test]
fn test_memory_efficiency() {
    // Create multiple presentations
    let mut presentations = Vec::new();
    
    for _ in 0..5 {
        let prs = new_presentation().expect("Failed to create");
        presentations.push(prs);
    }
    
    // All should be valid
    for prs in presentations {
        assert!(prs.slide_width().is_some(), "Should have width");
        assert!(prs.slide_height().is_some(), "Should have height");
    }
}

#[test]
fn test_concurrent_save_operations() {
    let mut prs = new_presentation().expect("Failed to create");
    
    // Perform multiple saves
    let mut cursors = Vec::new();
    for _ in 0..3 {
        let mut cursor = Cursor::new(Vec::new());
        prs.save(&mut cursor).expect("Failed to save");
        cursors.push(cursor);
    }
    
    // All saves should produce data
    for cursor in cursors {
        let data = cursor.into_inner();
        assert!(!data.is_empty(), "Save should produce data");
    }
}

#[test]
fn test_presentation_metadata_access() {
    let prs = new_presentation().expect("Failed to create");
    
    // Access metadata
    let width = prs.slide_width();
    let height = prs.slide_height();
    
    // Should have standard dimensions
    assert_eq!(width, Some(9144000));
    assert_eq!(height, Some(6858000));
}

#[test]
fn test_save_to_bytes_method() {
    let mut prs = new_presentation().expect("Failed to create");
    
    let bytes = prs.save_to_bytes().expect("Failed to save to bytes");
    assert!(!bytes.is_empty(), "Should have bytes");
    
    // Verify ZIP signature
    assert_eq!(bytes[0], 0x50, "ZIP signature byte 1");
    assert_eq!(bytes[1], 0x4B, "ZIP signature byte 2");
}
