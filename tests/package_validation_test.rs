//! Systematic package validation integration tests.
//!
//! Every generated deck variant should pass [`common::assert_package_valid`].
//! Add new rules in `src/core/package_validation/rules.rs` and cover them here.

mod common;

use std::io::{Cursor, Read, Write};
use std::path::PathBuf;

use ppt_rs::core::{
    validate_package_bytes, PackageValidationIssue, ValidationCategory, ValidationSeverity,
};
use ppt_rs::generator::layout_parts::STANDARD_LAYOUT_COUNT;
use ppt_rs::generator::package_xml::{HANDOUT_MASTER_ID, NOTES_MASTER_ID, SLIDE_LAYOUT_ID};
use ppt_rs::generator::{
    create_pptx, create_pptx_with_content, create_pptx_with_settings, ChartBuilder,
    ChartSeries, ChartType, Image, PresentationSettings, PrintSettings, PrintWhat, SlideContent,
};
use ppt_rs::generator::slide_content::print_settings::HandoutLayout;
use zip::write::FileOptions;
use zip::ZipArchive;
use zip::ZipWriter;

use common::{assert_package_file_valid, assert_package_valid, issues_in_category};

fn project_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(name)
}

fn read_part(bytes: &[u8], path: &str) -> String {
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut file = archive.by_name(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}

// ---------------------------------------------------------------------------
// Golden outputs
// ---------------------------------------------------------------------------

#[test]
fn comprehensive_demo_passes_package_validation() {
    let path = project_path("comprehensive_demo.pptx");
    if !path.exists() {
        eprintln!("skip: run `cargo run --example comprehensive_demo` first");
        return;
    }
    assert_package_file_valid(&path, "comprehensive_demo.pptx");
}

#[test]
fn layout_demo_passes_package_validation() {
    let path = project_path("layout_demo.pptx");
    if !path.exists() {
        eprintln!("skip: run `cargo run --example layout_demo` first");
        return;
    }
    assert_package_file_valid(&path, "layout_demo.pptx");
}

// ---------------------------------------------------------------------------
// Generator matrix
// ---------------------------------------------------------------------------

#[test]
fn minimal_deck_passes_validation() {
    assert_package_valid(&create_pptx("Minimal", 1).unwrap(), "minimal");
}

#[test]
fn multi_slide_deck_passes_validation() {
    let slides: Vec<_> = (1..=5)
        .map(|i| SlideContent::new(&format!("Slide {i}")).add_bullet("Point"))
        .collect();
    assert_package_valid(
        &create_pptx_with_content("Multi", slides).unwrap(),
        "multi-slide",
    );
}

#[test]
fn chart_deck_passes_validation() {
    let chart = ChartBuilder::new("Sales", ChartType::Bar)
        .position(1_000_000, 1_000_000)
        .size(3_000_000, 2_000_000)
        .add_series(ChartSeries::new("Q1", vec![1.0, 2.0, 3.0]))
        .build();
    let slide = SlideContent::new("Chart").add_chart(chart);
    assert_package_valid(
        &create_pptx_with_content("Chart", vec![slide]).unwrap(),
        "chart",
    );
}

#[test]
fn notes_handouts_and_images_pass_validation() {
    let print = PrintSettings::default()
        .print_what(PrintWhat::Handouts)
        .handout_layout(HandoutLayout::SlidesPerPage6)
        .footer("Footer")
        .print_date(true);
    let settings = PresentationSettings::new().print(print);
    let mut slide = SlideContent::new("Talk");
    slide.notes = Some("Remember the demo".into());
    slide.images.push(Image::from_bytes(
        include_bytes!("../examples/assets/diagram.png").to_vec(),
        1_000_000,
        1_000_000,
        "png",
    ));

    let bytes = create_pptx_with_settings("Both", &[slide], Some(settings)).unwrap();
    assert_package_valid(&bytes, "notes+handouts+image");

    let pres_xml = read_part(&bytes, "ppt/presentation.xml");
    let expected_notes_id = SLIDE_LAYOUT_ID + STANDARD_LAYOUT_COUNT as u32;
    assert_eq!(NOTES_MASTER_ID, expected_notes_id);
    assert_eq!(HANDOUT_MASTER_ID, NOTES_MASTER_ID + 1);
    assert!(pres_xml.contains("<p:notesMasterIdLst>"));
    assert!(pres_xml.contains("<p:handoutMasterIdLst>"));
    assert!(!pres_xml.contains("<p:notesMasterId id="));
    assert!(!pres_xml.contains("<p:handoutMasterId id="));

    let report = validate_package_bytes(&bytes);
    assert_eq!(
        issues_in_category(&report, ValidationCategory::SlideMaster).len(),
        0
    );
}

// ---------------------------------------------------------------------------
// Rule-focused regressions
// ---------------------------------------------------------------------------

#[test]
fn broken_relationship_target_is_detected() {
    let bytes = create_pptx("Ok", 1).unwrap();
    let mut out = Vec::new();
    {
        let mut zip = ZipWriter::new(Cursor::new(&mut out));
        let options = FileOptions::default();
        let mut archive = ZipArchive::new(Cursor::new(&bytes)).unwrap();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let name = file.name().to_string();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            zip.start_file(&name, options).unwrap();
            if name == "ppt/_rels/presentation.xml.rels" {
                let broken = String::from_utf8_lossy(&buf)
                    .replace("slides/slide1.xml", "slides/missing.xml");
                zip.write_all(broken.as_bytes()).unwrap();
            } else {
                zip.write_all(&buf).unwrap();
            }
        }
        zip.finish().unwrap();
    }

    let report = validate_package_bytes(&out);
    assert!(!report.is_valid());
    assert!(
        issues_in_category(&report, ValidationCategory::Relationship)
            .iter()
            .any(|i| i.message.contains("missing")),
        "expected broken relationship issue, got {:?}",
        report.issues
    );
}

#[test]
fn master_id_collision_issue_has_expected_shape() {
    let issue = PackageValidationIssue::error(
        ValidationCategory::SlideMaster,
        "notes master id 2147483650 collides with a slide layout id on the slide master",
        Some("ppt/presentation.xml"),
    );
    assert_eq!(issue.severity, ValidationSeverity::Error);
    assert_eq!(issue.category, ValidationCategory::SlideMaster);
}
