//! Integration tests for pure-Rust PDF export (`pdfrs`).
//!
//! Requires the `pdf-native` Cargo feature.

#![cfg(feature = "pdf-native")]

use ppt_rs::api::Presentation;
use ppt_rs::export::pdf_export::{
    export_to_pdf, export_to_pdf_bytes, PdfExportOptions, PdfOrientation,
};
use ppt_rs::generator::SlideContent;
use std::fs;
use std::path::Path;

fn temp_pdf_path(suffix: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "ppt_rs_pdf_int_{}_{}.pdf",
        suffix,
        uuid::Uuid::new_v4()
    ))
}

fn assert_valid_pdf(path: &Path) {
    let bytes = fs::read(path).expect("pdf file readable");
    assert!(!bytes.is_empty(), "pdf must not be empty");
    assert_eq!(&bytes[..5], b"%PDF-", "magic header");
    assert!(
        pdfrs::pdf::validate_pdf_bytes(&bytes).valid,
        "pdf must be structurally valid"
    );
}

#[test]
fn test_save_as_pdf_via_pdfrs_writes_file() {
    let pres = Presentation::with_title("Native PDF Test")
        .add_slide(SlideContent::new("Slide 1").add_bullet("First point"))
        .add_slide(SlideContent::new("Slide 2").add_bullet("Second point"));

    let path = temp_pdf_path("via_api");
    pres.save_as_pdf_via_pdfrs(&path, &PdfExportOptions::default())
        .expect("save_as_pdf_via_pdfrs succeeds");
    assert_valid_pdf(&path);
    let _ = fs::remove_file(&path);
}

#[test]
fn test_to_pdf_bytes_returns_valid_buffer() {
    let pres = Presentation::with_title("Bytes Test")
        .add_slide(SlideContent::new("Slide A").add_bullet("Alpha"));

    let bytes = pres
        .to_pdf_bytes(&PdfExportOptions::default())
        .expect("to_pdf_bytes succeeds");
    assert_eq!(&bytes[..5], b"%PDF-");
    assert!(pdfrs::pdf::validate_pdf_bytes(&bytes).valid);
}

#[test]
fn test_export_module_landscape() {
    let pres = Presentation::with_title("Landscape")
        .add_slide(SlideContent::new("Only Slide").add_bullet("Wide"));

    let opts = PdfExportOptions::new().with_orientation(PdfOrientation::Landscape);
    let bytes = export_to_pdf_bytes(&pres, &opts).unwrap();
    assert_eq!(&bytes[..5], b"%PDF-");
    assert!(pdfrs::pdf::validate_pdf_bytes(&bytes).valid);
}

#[test]
fn test_export_module_minimal_no_frontmatter_or_notes() {
    let mut slide = SlideContent::new("Quiet");
    slide.notes = Some("Internal note that should be excluded".to_string());
    let pres = Presentation::with_title("Quiet")
        .add_slide(slide)
        .add_slide(SlideContent::new("Plain").add_bullet("Bullet"));

    let opts = PdfExportOptions::new()
        .with_frontmatter(false)
        .with_notes(false)
        .with_images(false);
    let path = temp_pdf_path("minimal");
    export_to_pdf(&pres, &path, &opts).expect("export_to_pdf succeeds");
    assert_valid_pdf(&path);
    let _ = fs::remove_file(&path);
}

#[test]
fn test_export_module_custom_font() {
    let pres = Presentation::with_title("Custom Font")
        .add_slide(SlideContent::new("Slide").add_bullet("Body"));

    let opts = PdfExportOptions::new()
        .with_font("Times-Roman")
        .with_font_size(10.0);
    let path = temp_pdf_path("font");
    export_to_pdf(&pres, &path, &opts).expect("export_to_pdf with custom font");
    assert_valid_pdf(&path);
    let _ = fs::remove_file(&path);
}

#[test]
fn test_export_module_multi_slide_round_trip() {
    let pres = Presentation::with_title("Multi-Slide")
        .add_slide(SlideContent::new("Title Slide").add_bullet("Sub"))
        .add_slide(SlideContent::new("Agenda").add_bullet("One").add_bullet("Two"))
        .add_slide(SlideContent::new("Closing"));

    let opts = PdfExportOptions::new().with_frontmatter(false);
    let path = temp_pdf_path("multi");
    export_to_pdf(&pres, &path, &opts).expect("multi slide export");
    assert_valid_pdf(&path);
    let _ = fs::remove_file(&path);
}

#[test]
fn test_pdf_options_re_exported_from_crate_root() {
    // Compile-time check that PdfExportOptions is reachable via ppt_rs::*
    let _opts: ppt_rs::PdfExportOptions = ppt_rs::PdfExportOptions::default();
}

#[test]
fn test_pdf_orientation_re_exported_from_crate_root() {
    let _o: ppt_rs::PdfOrientation = ppt_rs::PdfOrientation::Landscape;
}