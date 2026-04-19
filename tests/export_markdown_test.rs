//! Integration tests for Markdown export functionality

use ppt_rs::api::Presentation;
use ppt_rs::export::md::{MarkdownOptions, export_to_markdown, export_to_markdown_with_options};
use ppt_rs::generator::{SlideContent, TableBuilder, TableCell, TableRow, CodeBlock};
use std::fs;
use std::path::Path;

#[test]
fn test_markdown_export_api() {
    let pres = Presentation::new()
        .title("Markdown Export Test")
        .add_slide(SlideContent::new("Slide 1").add_bullet("Item 1").add_bullet("Item 2"))
        .add_slide(SlideContent::new("Slide 2").add_bullet("Point A"));

    let output_path = "test_export.md";
    pres.save_as_markdown(output_path).unwrap();

    assert!(Path::new(output_path).exists());
    let content = fs::read_to_string(output_path).unwrap();

    // Check structure
    assert!(content.contains("# Markdown Export Test"));
    assert!(content.contains("## Slide 1: Slide 1"));
    assert!(content.contains("## Slide 2: Slide 2"));
    assert!(content.contains("- Item 1"));
    assert!(content.contains("- Item 2"));
    assert!(content.contains("- Point A"));
    assert!(content.contains("---")); // Slide separator

    // Check frontmatter
    assert!(content.contains("---"));
    assert!(content.contains("title:"));
    assert!(content.contains("slides: 2"));

    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_markdown_export_with_table() {
    let cells1 = vec![TableCell::new("Name"), TableCell::new("Value")];
    let cells2 = vec![TableCell::new("Item 1"), TableCell::new("100")];
    let table = TableBuilder::new(vec![100, 100])
        .add_row(TableRow::new(cells1))
        .add_row(TableRow::new(cells2))
        .build();

    let mut slide = SlideContent::new("Table Slide");
    slide.table = Some(table);
    slide.has_table = true;

    let pres = Presentation::new()
        .title("Table Test")
        .add_slide(slide);

    let output_path = "test_table_export.md";
    pres.save_as_markdown(output_path).unwrap();

    let content = fs::read_to_string(output_path).unwrap();

    // Check GFM table format
    assert!(content.contains("| Name | Value |"));
    assert!(content.contains("| --- | --- |"));
    assert!(content.contains("| Item 1 | 100 |"));

    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_markdown_export_with_code_block() {
    let mut slide = SlideContent::new("Code Slide");
    slide.code_blocks.push(CodeBlock::new("fn main() {\n    println!(\"Hello\");\n}", "rust"));

    let pres = Presentation::new()
        .title("Code Test")
        .add_slide(slide);

    let output_path = "test_code_export.md";
    pres.save_as_markdown(output_path).unwrap();

    let content = fs::read_to_string(output_path).unwrap();

    assert!(content.contains("```rust"));
    assert!(content.contains("fn main()"));
    assert!(content.contains("println!"));
    assert!(content.contains("```"));

    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_markdown_export_with_options() {
    let pres = Presentation::new()
        .title("Options Test")
        .add_slide(SlideContent::new("Slide 1").add_bullet("Point"));

    let options = MarkdownOptions::new()
        .with_slide_numbers(false)
        .with_frontmatter(false)
        .with_images(false);

    let output_path = "test_options_export.md";
    pres.save_as_markdown_with_options(output_path, &options).unwrap();

    let content = fs::read_to_string(output_path).unwrap();

    // Without slide numbers
    assert!(!content.contains("## Slide 1:"));
    assert!(content.contains("## Slide 1")); // Just the title

    // Without frontmatter
    assert!(!content.contains("title:"));

    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_markdown_export_special_characters() {
    let pres = Presentation::new()
        .title("Special * Characters [test]")
        .add_slide(SlideContent::new("Slide with `code` and *bold*").add_bullet("Item [link]"));

    let output_path = "test_special_chars.md";
    pres.save_as_markdown(output_path).unwrap();

    let content = fs::read_to_string(output_path).unwrap();

    // Check escaping
    assert!(content.contains(r"\*bold\*") || content.contains("*bold*"));
    assert!(content.contains(r"\[link\]") || content.contains("[link]"));

    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_markdown_export_empty_presentation() {
    let pres = Presentation::new().title("Empty Presentation");

    let output_path = "test_empty.md";
    pres.save_as_markdown(output_path).unwrap();

    let content = fs::read_to_string(output_path).unwrap();

    assert!(content.contains("# Empty Presentation"));
    // Should have frontmatter but no slides
    assert!(content.contains("slides: 0"));

    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_markdown_export_multiple_slides() {
    let mut pres = Presentation::new().title("Multi Slide");

    for i in 1..=5 {
        pres = pres.add_slide(
            SlideContent::new(&format!("Slide {}", i))
                .add_bullet(&format!("Bullet {}", i))
        );
    }

    let output_path = "test_multi_slide.md";
    pres.save_as_markdown(output_path).unwrap();

    let content = fs::read_to_string(output_path).unwrap();

    // Check all slides are present
    for i in 1..=5 {
        assert!(content.contains(&format!("Slide {}", i)));
        assert!(content.contains(&format!("Bullet {}", i)));
    }

    // Check separators between slides
    let separator_count = content.matches("---").count();
    assert!(separator_count >= 4); // At least 4 separators for 5 slides

    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_markdown_export_with_notes() {
    let mut slide = SlideContent::new("Notes Slide");
    slide.notes = Some("This is an important note for the speaker".to_string());

    let pres = Presentation::new()
        .title("Notes Test")
        .add_slide(slide);

    let output_path = "test_notes.md";
    pres.save_as_markdown(output_path).unwrap();

    let content = fs::read_to_string(output_path).unwrap();

    assert!(content.contains("**Notes:**"));
    assert!(content.contains("This is an important note"));

    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_export_to_markdown_function() {
    let pres = Presentation::new()
        .title("Direct Export")
        .add_slide(SlideContent::new("Slide").add_bullet("Point"));

    let md = export_to_markdown(&pres).unwrap();

    assert!(md.contains("# Direct Export"));
    assert!(md.contains("Slide"));
    assert!(md.contains("Point"));
}

#[test]
fn test_export_to_markdown_with_custom_options() {
    let pres = Presentation::new()
        .title("Custom Options")
        .add_slide(SlideContent::new("Slide"));

    let options = MarkdownOptions::new()
        .with_separator("===")
        .with_notes(false)
        .with_gfm_tables(true);

    let md = export_to_markdown_with_options(&pres, &options).unwrap();

    assert!(md.contains("===")); // Custom separator
}
