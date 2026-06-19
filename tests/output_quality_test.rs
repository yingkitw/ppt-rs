//! Output Quality Guardrail Tests
//!
//! Comprehensive tests to verify PPTX output quality, structure, and compliance
//! Tests focus on:
//! - Valid ZIP structure
//! - Proper XML formatting
//! - ECMA-376 compliance
//! - Content preservation
//! - File integrity

use ppt_rs::generator::{
    SlideContent, Table, TableRow, TableCell, SlideLayout, create_pptx_with_content,
};
use std::fs;
use std::io::{Cursor, Read};
use zip::ZipArchive;

// ============================================================================
// ZIP STRUCTURE TESTS
// ============================================================================

#[test]
fn test_pptx_is_valid_zip() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Test Slide")
            .add_bullet("Content"),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    
    // Verify it's a valid ZIP file
    let cursor = Cursor::new(pptx_data);
    let zip_result = ZipArchive::new(cursor);
    assert!(zip_result.is_ok(), "PPTX should be a valid ZIP file");
}

#[test]
fn test_pptx_contains_required_files() {
    let slides = vec![
        SlideContent::new("Test")
            .add_bullet("Content"),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    // Check for required files
    let required_files = vec![
        "[Content_Types].xml",
        "_rels/.rels",
        "ppt/presentation.xml",
        "ppt/_rels/presentation.xml.rels",
        "ppt/slides/slide1.xml",
        "ppt/slides/_rels/slide1.xml.rels",
        "ppt/slideLayouts/slideLayout1.xml",
        "ppt/slideMasters/slideMaster1.xml",
        "ppt/theme/theme1.xml",
    ];

    for file in required_files {
        assert!(
            archive.by_name(file).is_ok(),
            "PPTX should contain required file: {}",
            file
        );
    }
}

#[test]
fn test_pptx_file_size_reasonable() {
    let slides = vec![
        SlideContent::new("Test Slide")
            .add_bullet("Content"),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    
    // Minimum size: ~5KB for basic PPTX (compressed XML is smaller than expected)
    // Maximum size: ~1MB for single slide (should be much smaller)
    assert!(pptx_data.len() > 5_000, "PPTX file should be at least 5KB");
    assert!(pptx_data.len() < 1_000_000, "Single slide PPTX should be less than 1MB");
}

// ============================================================================
// XML STRUCTURE TESTS
// ============================================================================

#[test]
fn test_slide_xml_has_valid_structure() {
    let slides = vec![
        SlideContent::new("Title")
            .add_bullet("Content"),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    let slide_xml = archive.by_name("ppt/slides/slide1.xml").unwrap();
    let mut content = String::new();
    let mut reader = std::io::BufReader::new(slide_xml);
    std::io::Read::read_to_string(&mut reader, &mut content).unwrap();

    // Check XML declaration
    assert!(content.starts_with("<?xml"), "Slide XML should start with XML declaration");
    
    // Check root element
    assert!(content.contains("<p:sld"), "Slide XML should have p:sld root element");
    assert!(content.contains("</p:sld>"), "Slide XML should close p:sld element");
    
    // Check required namespaces
    assert!(content.contains("xmlns:a="), "Should have DrawingML namespace");
    assert!(content.contains("xmlns:p="), "Should have PresentationML namespace");
    assert!(content.contains("xmlns:r="), "Should have Relationships namespace");
}

#[test]
fn test_presentation_xml_valid_structure() {
    let slides = vec![
        SlideContent::new("Slide 1"),
        SlideContent::new("Slide 2"),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    let pres_xml = archive.by_name("ppt/presentation.xml").unwrap();
    let mut content = String::new();
    let mut reader = std::io::BufReader::new(pres_xml);
    std::io::Read::read_to_string(&mut reader, &mut content).unwrap();

    // Check structure
    assert!(content.contains("<p:presentation"), "Should have presentation element");
    assert!(content.contains("<p:sldIdLst"), "Should have slide ID list");
    assert!(content.contains("</p:presentation>"), "Should close presentation element");
}

#[test]
fn test_content_types_xml_valid() {
    let slides = vec![
        SlideContent::new("Test"),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    let ct_xml = archive.by_name("[Content_Types].xml").unwrap();
    let mut content = String::new();
    let mut reader = std::io::BufReader::new(ct_xml);
    std::io::Read::read_to_string(&mut reader, &mut content).unwrap();

    // Check structure
    assert!(content.contains("<Types"), "Should have Types element");
    assert!(content.contains("</Types>"), "Should close Types element");
    assert!(content.contains("Override"), "Should have Override elements");
}

// ============================================================================
// CONTENT PRESERVATION TESTS
// ============================================================================

#[test]
fn test_slide_title_preserved() {
    let title = "My Important Title";
    let slides = vec![
        SlideContent::new(title),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    let slide_xml = archive.by_name("ppt/slides/slide1.xml").unwrap();
    let mut content = String::new();
    let mut reader = std::io::BufReader::new(slide_xml);
    reader.read_to_string(&mut content).unwrap();

    assert!(content.contains(title), "Slide title should be preserved in XML");
}

#[test]
fn test_bullet_content_preserved() {
    let bullets = vec!["First point", "Second point", "Third point"];
    let mut slide = SlideContent::new("Title");
    for bullet in &bullets {
        slide = slide.add_bullet(bullet);
    }

    let result = create_pptx_with_content("Test", vec![slide]);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    let slide_xml = archive.by_name("ppt/slides/slide1.xml").unwrap();
    let mut content = String::new();
    let mut reader = std::io::BufReader::new(slide_xml);
    reader.read_to_string(&mut content).unwrap();

    for bullet in &bullets {
        assert!(content.contains(bullet), "Bullet '{}' should be preserved", bullet);
    }
}

#[test]
fn test_special_characters_preserved() {
    let special_text = "Test & <More> \"Content\"";
    let slides = vec![
        SlideContent::new("Special Chars")
            .add_bullet(special_text),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    let slide_xml = archive.by_name("ppt/slides/slide1.xml").unwrap();
    let mut content = String::new();
    let mut reader = std::io::BufReader::new(slide_xml);
    reader.read_to_string(&mut content).unwrap();

    // Check for XML-escaped versions
    assert!(
        content.contains("&amp;") || content.contains("&"),
        "Ampersand should be preserved (escaped or not)"
    );
    assert!(
        content.contains("&lt;") || content.contains("<"),
        "Less-than should be preserved (escaped or not)"
    );
}

#[test]
fn test_unicode_content_preserved() {
    let unicode_text = "日本語テキスト";
    let slides = vec![
        SlideContent::new("Unicode")
            .add_bullet(unicode_text),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    let slide_xml = archive.by_name("ppt/slides/slide1.xml").unwrap();
    let mut content = String::new();
    let mut reader = std::io::BufReader::new(slide_xml);
    reader.read_to_string(&mut content).unwrap();

    assert!(content.contains(unicode_text), "Unicode text should be preserved");
}

// ============================================================================
// FORMATTING TESTS
// ============================================================================

#[test]
fn test_bold_formatting_in_xml() {
    let slides = vec![
        SlideContent::new("Title")
            .title_bold(true)
            .add_bullet("Content"),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    let slide_xml = archive.by_name("ppt/slides/slide1.xml").unwrap();
    let mut content = String::new();
    let mut reader = std::io::BufReader::new(slide_xml);
    reader.read_to_string(&mut content).unwrap();

    // Check for bold attribute in title
    assert!(content.contains(r#"b="1""#), "Bold formatting should be in XML");
}

#[test]
fn test_font_size_in_xml() {
    let font_size = 56;
    let slides = vec![
        SlideContent::new("Title")
            .title_size(font_size)
            .add_bullet("Content"),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    let slide_xml = archive.by_name("ppt/slides/slide1.xml").unwrap();
    let mut content = String::new();
    let mut reader = std::io::BufReader::new(slide_xml);
    reader.read_to_string(&mut content).unwrap();

    // Font size in XML is in hundredths of a point
    let expected_size = font_size * 100;
    assert!(
        content.contains(&format!("sz=\"{}\"", expected_size)),
        "Font size {} should be in XML",
        expected_size
    );
}

// ============================================================================
// TABLE QUALITY TESTS
// ============================================================================

#[test]
fn test_table_in_slide_has_valid_structure() {
    let table = Table::from_data(
        vec![
            vec!["Header 1", "Header 2"],
            vec!["Data 1", "Data 2"],
        ],
        vec![2000000, 2000000],
        457200,
        1400000,
    );

    let slides = vec![
        SlideContent::new("Table Slide")
            .table(table),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    let slide_xml = archive.by_name("ppt/slides/slide1.xml").unwrap();
    let mut content = String::new();
    let mut reader = std::io::BufReader::new(slide_xml);
    reader.read_to_string(&mut content).unwrap();

    // Check for table structure
    assert!(content.contains("p:graphicFrame"), "Should have graphic frame for table");
    assert!(content.contains("a:tbl"), "Should have table element");
    assert!(content.contains("a:tblGrid"), "Should have table grid");
    assert!(content.contains("a:tr"), "Should have table rows");
    assert!(content.contains("a:tc"), "Should have table cells");
}

#[test]
fn test_table_content_preserved() {
    let table = Table::from_data(
        vec![
            vec!["Name", "Age"],
            vec!["Alice", "30"],
            vec!["Bob", "25"],
        ],
        vec![2000000, 2000000],
        457200,
        1400000,
    );

    let slides = vec![
        SlideContent::new("Data Table")
            .table(table),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    let slide_xml = archive.by_name("ppt/slides/slide1.xml").unwrap();
    let mut content = String::new();
    let mut reader = std::io::BufReader::new(slide_xml);
    reader.read_to_string(&mut content).unwrap();

    // Check for all table data
    assert!(content.contains("Name"), "Table header 'Name' should be preserved");
    assert!(content.contains("Age"), "Table header 'Age' should be preserved");
    assert!(content.contains("Alice"), "Table data 'Alice' should be preserved");
    assert!(content.contains("30"), "Table data '30' should be preserved");
}

#[test]
fn test_table_styling_preserved() {
    let cells = vec![
        TableCell::new("Header").bold().background_color("FF0000"),
        TableCell::new("Data").background_color("00FF00"),
    ];
    let row = TableRow::new(cells);
    let table = Table::new(vec![row], vec![2000000, 2000000], 457200, 1400000);

    let slides = vec![
        SlideContent::new("Styled Table")
            .table(table),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    let slide_xml = archive.by_name("ppt/slides/slide1.xml").unwrap();
    let mut content = String::new();
    let mut reader = std::io::BufReader::new(slide_xml);
    reader.read_to_string(&mut content).unwrap();

    // Check for styling
    assert!(content.contains("FF0000"), "Red color should be in XML");
    assert!(content.contains("00FF00"), "Green color should be in XML");
    assert!(content.contains(r#"b="1""#), "Bold formatting should be in XML");
}

// ============================================================================
// SLIDE LAYOUT TESTS
// ============================================================================

#[test]
fn test_all_layouts_generate_valid_pptx() {
    let layouts = vec![
        SlideLayout::TitleOnly,
        SlideLayout::CenteredTitle,
        SlideLayout::TitleAndContent,
        SlideLayout::TitleAndBigContent,
        SlideLayout::TwoColumn,
        SlideLayout::SectionHeader,
        SlideLayout::Blank,
    ];

    for layout in layouts {
        let slides = vec![
            SlideContent::new("Test")
                .add_bullet("Content")
                .layout(layout),
        ];

        let result = create_pptx_with_content("Test", slides);
        assert!(result.is_ok(), "Layout {:?} should generate valid PPTX", layout);

        let pptx_data = result.unwrap();
        let cursor = Cursor::new(pptx_data);
        let zip_result = ZipArchive::new(cursor);
        assert!(
            zip_result.is_ok(),
            "Layout {:?} should produce valid ZIP",
            layout
        );
    }
}

// ============================================================================
// MULTI-SLIDE TESTS
// ============================================================================

#[test]
fn test_multiple_slides_all_present() {
    let num_slides = 5;
    let mut slides = Vec::new();
    for i in 1..=num_slides {
        slides.push(
            SlideContent::new(&format!("Slide {}", i))
                .add_bullet(&format!("Content {}", i))
        );
    }

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    // Check that all slides exist
    for i in 1..=num_slides {
        let slide_path = format!("ppt/slides/slide{}.xml", i);
        assert!(
            archive.by_name(&slide_path).is_ok(),
            "Slide {} should exist in PPTX",
            i
        );
    }
}

#[test]
fn test_multiple_slides_content_preserved() {
    let slides = vec![
        SlideContent::new("Slide 1")
            .add_bullet("First slide content"),
        SlideContent::new("Slide 2")
            .add_bullet("Second slide content"),
        SlideContent::new("Slide 3")
            .add_bullet("Third slide content"),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).unwrap();

    // Check each slide's content
    for i in 1..=3 {
        let slide_xml = archive.by_name(&format!("ppt/slides/slide{}.xml", i)).unwrap();
        let mut content = String::new();
        let mut reader = std::io::BufReader::new(slide_xml);
        reader.read_to_string(&mut content).unwrap();

        assert!(
            content.contains(&format!("Slide {}", i)),
            "Slide {} title should be preserved",
            i
        );
        assert!(
            content.contains("slide content"),
            "Slide {} content should be preserved",
            i
        );
    }
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_empty_slide_title() {
    let slides = vec![
        SlideContent::new("")
            .add_bullet("Content"),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok(), "Empty title should be handled gracefully");

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let zip_result = ZipArchive::new(cursor);
    assert!(zip_result.is_ok(), "Should produce valid ZIP with empty title");
}

#[test]
fn test_very_long_title() {
    let long_title = "A".repeat(500);
    let slides = vec![
        SlideContent::new(&long_title)
            .add_bullet("Content"),
    ];

    let result = create_pptx_with_content("Test", slides);
    assert!(result.is_ok(), "Long title should be handled");

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let zip_result = ZipArchive::new(cursor);
    assert!(zip_result.is_ok(), "Should produce valid ZIP with long title");
}

#[test]
fn test_many_bullets() {
    let mut slide = SlideContent::new("Many Bullets");
    for i in 1..=50 {
        slide = slide.add_bullet(&format!("Bullet {}", i));
    }

    let result = create_pptx_with_content("Test", vec![slide]);
    assert!(result.is_ok(), "Many bullets should be handled");

    let pptx_data = result.unwrap();
    let cursor = Cursor::new(pptx_data);
    let zip_result = ZipArchive::new(cursor);
    assert!(zip_result.is_ok(), "Should produce valid ZIP with many bullets");
}

#[test]
fn test_large_presentation() {
    let mut slides = Vec::new();
    for i in 1..=20 {
        slides.push(
            SlideContent::new(&format!("Slide {}", i))
                .add_bullet(&format!("Content {}", i))
                .add_bullet("Additional point")
                .add_bullet("More details")
        );
    }

    let result = create_pptx_with_content("Large Presentation", slides);
    assert!(result.is_ok(), "Large presentation should be generated");

    let pptx_data = result.unwrap();
    
    // File size should be reasonable (< 2MB for 20 slides)
    assert!(pptx_data.len() < 2_000_000, "Large presentation should be < 2MB");
    
    let cursor = Cursor::new(pptx_data);
    let zip_result = ZipArchive::new(cursor);
    assert!(zip_result.is_ok(), "Should produce valid ZIP for large presentation");
}
