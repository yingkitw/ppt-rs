//! Integration tests for ppt-rs
//!
//! These tests verify end-to-end functionality including:
//! - PPTX generation and validation
//! - Constants usage
//! - File structure validation
//! - Alignment with standards

use ppt_rs::generator::{
    SlideContent, create_pptx_with_content, create_pptx, create_pptx_with_settings,
    PresentationSettings, SlideSize,
};
use ppt_rs::generator::constants::{
    SLIDE_WIDTH, SLIDE_HEIGHT, TITLE_X, TITLE_Y, TITLE_WIDTH, TITLE_HEIGHT,
    CONTENT_X, CONTENT_Y_START, CONTENT_WIDTH,
    TITLE_FONT_SIZE, CONTENT_FONT_SIZE,
};
use std::fs;
use std::io::{Cursor, Read};
use zip::ZipArchive;

// ============================================================================
// VALIDATION COMMAND TESTS
// ============================================================================

#[test]
fn test_generated_pptx_passes_validation() {
    let slides = vec![
        SlideContent::new("Test Slide")
            .add_bullet("Content"),
    ];

    let result = create_pptx_with_content("Test Presentation", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    
    // Write to temporary file
    let test_file = "target/test_validation.pptx";
    fs::create_dir_all("target").ok();
    fs::write(test_file, &pptx_data).expect("Failed to write test file");
    
    // Validate using the validation logic (simulating validate command)
    let validation_result = validate_pptx_structure(&pptx_data);
    assert!(validation_result.is_ok(), "Generated PPTX should pass validation");
    
    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_validation_detects_invalid_zip() {
    let invalid_data = b"Not a valid ZIP file";
    let result = validate_pptx_structure(invalid_data);
    assert!(result.is_err(), "Invalid ZIP should fail validation");
}

#[test]
fn test_validation_checks_required_files() {
    let slides = vec![
        SlideContent::new("Test")
            .add_bullet("Content"),
    ];

    let pptx_data = create_pptx_with_content("Test", slides).unwrap();
    
    // Verify required files exist
    let cursor = Cursor::new(&pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();
    
    let required_files = vec![
        "[Content_Types].xml",
        "_rels/.rels",
        "ppt/presentation.xml",
        "docProps/core.xml",
    ];
    
    for file in required_files {
        assert!(
            archive.by_name(file).is_ok(),
            "Required file {} should exist",
            file
        );
    }
}

// ============================================================================
// CONSTANTS USAGE TESTS
// ============================================================================

#[test]
fn test_constants_are_used_in_generated_xml() {
    let slides = vec![
        SlideContent::new("Test Title")
            .add_bullet("Test Content"),
    ];

    let pptx_data = create_pptx_with_content("Test", slides).unwrap();
    
    // Check that slide dimensions match constants
    let cursor = Cursor::new(&pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();
    
    // Read presentation.xml to check dimensions
    if let Ok(mut file) = archive.by_name("ppt/presentation.xml") {
        let mut content = String::new();
        file.read_to_string(&mut content).ok();
        
        // Check for slide dimensions in XML
        assert!(
            content.contains(&format!("cx=\"{}\"", SLIDE_WIDTH)) ||
            content.contains(&SLIDE_WIDTH.to_string()),
            "Slide width constant should be used"
        );
    }
}

#[test]
fn test_widescreen_slide_size_is_written_to_package_xml() {
    let slides = vec![
        SlideContent::new("Widescreen"),
    ];

    let settings = PresentationSettings::new()
        .slide_size(SlideSize::Widescreen16x9);

    let pptx_data = create_pptx_with_settings("Test", slides, Some(settings)).unwrap();

    let cursor = Cursor::new(&pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    let mut presentation_xml = String::new();
    archive.by_name("ppt/presentation.xml").unwrap()
        .read_to_string(&mut presentation_xml).unwrap();

    assert!(presentation_xml.contains("cx=\"12192000\""));
    assert!(presentation_xml.contains("cy=\"6858000\""));
    assert!(presentation_xml.contains("type=\"screen16x9\""));

    let mut app_xml = String::new();
    archive.by_name("docProps/app.xml").unwrap()
        .read_to_string(&mut app_xml).unwrap();

    assert!(app_xml.contains("<PresentationFormat>Widescreen</PresentationFormat>"));
}

#[test]
fn test_constants_have_valid_emu_values() {
    // Verify EMU conversions are correct
    // 1 inch = 914400 EMU
    assert_eq!(SLIDE_WIDTH, 9144000, "SLIDE_WIDTH should be 10 inches (9144000 EMU)");
    assert_eq!(SLIDE_HEIGHT, 6858000, "SLIDE_HEIGHT should be 7.5 inches (6858000 EMU)");
    
    // Title positioning
    assert_eq!(TITLE_X, 457200, "TITLE_X should be 0.5 inches (457200 EMU)");
    assert!(TITLE_Y > 0, "TITLE_Y should be positive");
    
    // Content positioning
    assert_eq!(CONTENT_X, TITLE_X, "CONTENT_X should match TITLE_X");
    assert!(CONTENT_Y_START > TITLE_Y, "CONTENT_Y_START should be below TITLE_Y");
    
    // Font sizes (in 100ths of points)
    assert!(TITLE_FONT_SIZE > 0, "TITLE_FONT_SIZE should be positive");
    assert!(CONTENT_FONT_SIZE > 0, "CONTENT_FONT_SIZE should be positive");
    assert!(TITLE_FONT_SIZE > CONTENT_FONT_SIZE, "Title font should be larger than content");
}

#[test]
fn test_constants_positioning_consistency() {
    // Verify positioning makes sense
    assert!(TITLE_X + TITLE_WIDTH <= SLIDE_WIDTH, "Title should fit within slide width");
    assert!(CONTENT_X + CONTENT_WIDTH <= SLIDE_WIDTH, "Content should fit within slide width");
    assert!(TITLE_Y + TITLE_HEIGHT < CONTENT_Y_START, "Content should start below title");
}

// ============================================================================
// ALIGNMENT TESTING SUPPORT
// ============================================================================

#[test]
fn test_alignment_test_example_generates_valid_pptx() {
    // Simulate the alignment test example
    let slides = vec![
        SlideContent::new("Alignment Test Presentation")
            .title_size(54)
            .title_bold(true)
            .title_color("003366"),
        SlideContent::new("Shapes and Formatting")
            .title_size(44)
            .title_bold(true)
            .title_color("003366")
            .add_bullet("Text formatting (bold, colors, sizes)")
            .add_bullet("Shape creation and positioning")
            .add_bullet("Multiple slides and layouts"),
    ];
    
    let result = create_pptx_with_content("Alignment Test Presentation", slides);
    assert!(result.is_ok());
    
    let pptx_data = result.unwrap();
    
    // Verify it's a valid ZIP
    let cursor = Cursor::new(&pptx_data);
    let zip_result = ZipArchive::new(cursor);
    assert!(zip_result.is_ok(), "Alignment test PPTX should be valid ZIP");
    
    // Verify structure
    let validation_result = validate_pptx_structure(&pptx_data);
    assert!(validation_result.is_ok(), "Alignment test PPTX should pass validation");
}

#[test]
fn test_generated_pptx_has_correct_metadata() {
    let slides = vec![
        SlideContent::new("Test Slide"),
    ];
    
    let title = "Test Presentation";
    let pptx_data = create_pptx_with_content(title, slides).unwrap();
    
    // Check metadata in core.xml
    let cursor = Cursor::new(&pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();
    
    if let Ok(mut file) = archive.by_name("docProps/core.xml") {
        let mut content = String::new();
        file.read_to_string(&mut content).ok();
        
        // Should contain title
        assert!(
            content.contains(title) || content.contains("dc:title"),
            "Core properties should contain title"
        );
    }
}

// ============================================================================
// END-TO-END GENERATION TESTS
// ============================================================================

#[test]
fn test_create_and_validate_multiple_slides() {
    let slides = vec![
        SlideContent::new("Slide 1"),
        SlideContent::new("Slide 2").add_bullet("Content"),
        SlideContent::new("Slide 3").add_bullet("Point 1").add_bullet("Point 2"),
    ];
    
    let result = create_pptx_with_content("Multi-Slide Test", slides);
    assert!(result.is_ok());
    
    let pptx_data = result.unwrap();
    let cursor = Cursor::new(&pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();
    
    // Verify all slides exist
    for i in 1..=3 {
        let slide_path = format!("ppt/slides/slide{}.xml", i);
        assert!(
            archive.by_name(&slide_path).is_ok(),
            "Slide {} should exist",
            i
        );
    }
}

#[test]
fn test_create_empty_presentation() {
    let result = create_pptx("Empty Presentation", 0);
    assert!(result.is_ok());
    
    let pptx_data = result.unwrap();
    let validation_result = validate_pptx_structure(&pptx_data);
    assert!(validation_result.is_ok(), "Empty presentation should be valid");
}

#[test]
fn test_create_presentation_with_all_layouts() {
    use ppt_rs::generator::SlideLayout;
    
    let layouts = vec![
        SlideLayout::TitleOnly,
        SlideLayout::CenteredTitle,
        SlideLayout::TitleAndContent,
        SlideLayout::TitleAndBigContent,
        SlideLayout::TwoColumn,
        SlideLayout::Blank,
    ];
    
    let mut slides = Vec::new();
    for layout in layouts {
        slides.push(
            SlideContent::new("Test")
                .add_bullet("Content")
                .layout(layout)
        );
    }
    
    let result = create_pptx_with_content("All Layouts", slides);
    assert!(result.is_ok());
    
    let pptx_data = result.unwrap();
    let validation_result = validate_pptx_structure(&pptx_data);
    assert!(validation_result.is_ok(), "All layouts should generate valid PPTX");
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Validate PPTX structure (simulates validation command logic)
fn validate_pptx_structure(data: &[u8]) -> Result<(), String> {
    // Check ZIP validity
    let cursor = Cursor::new(data);
    let mut archive = ZipArchive::new(cursor)
        .map_err(|e| format!("Invalid ZIP archive: {}", e))?;
    
    // Check required files
    let required_files = vec![
        "[Content_Types].xml",
        "_rels/.rels",
        "ppt/presentation.xml",
        "docProps/core.xml",
    ];
    
    for file in required_files {
        archive.by_name(file)
            .map_err(|_| format!("Missing required file: {}", file))?;
    }
    
    // Check XML validity for key files
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to read archive entry: {}", e))?;
        
        let name = file.name().to_string();
        if name.ends_with(".xml") || name.ends_with(".rels") {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| format!("Failed to read XML file {}: {}", name, e))?;
            
            // Basic XML validation
            if content.trim().is_empty() {
                return Err(format!("Empty XML file: {}", name));
            }
        }
    }
    
    Ok(())
}
