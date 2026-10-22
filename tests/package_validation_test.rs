//! Systematic package validation integration tests.
//!
//! Every generated deck variant should pass [`common::assert_package_valid`].
//! Add new rules in `src/core/package_validation/rules.rs` and cover them here.

mod common;

use std::io::{Cursor, Read, Write};
use std::path::PathBuf;

use ppt_rs::core::{
    PackageValidationIssue, ValidationCategory, ValidationSeverity, validate_package_bytes,
};
use ppt_rs::generator::layout_parts::STANDARD_LAYOUT_COUNT;
use ppt_rs::generator::package_xml::{HANDOUT_MASTER_ID, NOTES_MASTER_ID, SLIDE_LAYOUT_ID};
use ppt_rs::generator::slide_content::print_settings::HandoutLayout;
use ppt_rs::generator::{
    ChartBuilder, ChartSeries, ChartType, Image, PresentationSettings, PrintSettings, PrintWhat,
    SlideContent, create_pptx, create_pptx_with_content, create_pptx_with_settings,
};
use ppt_rs::opc::Package;
use zip::ZipArchive;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

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
        .categories(vec!["A", "B", "C"])
        .position(1_000_000, 1_000_000)
        .size(3_000_000, 2_000_000)
        .add_series(ChartSeries::new("Q1", vec![1.0, 2.0, 3.0]))
        .build()
        .expect("valid chart");
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

#[test]
fn generated_presentation_preserves_media_and_relationships_in_zip() {
    let image_bytes = include_bytes!("../examples/assets/diagram.png").to_vec();
    let slides = vec![
        SlideContent::new("Cover").add_image(Image::from_bytes(
            image_bytes.clone(),
            1_000_000,
            1_000_000,
            "png",
        )),
        SlideContent::new("Details").add_bullet("Second slide content"),
    ];

    let bytes = create_pptx_with_content("ZIP migration", slides).unwrap();
    assert_package_valid(&bytes, "ZIP migration deck");

    let mut archive = ZipArchive::new(Cursor::new(bytes)).expect("generated PPTX is a ZIP");
    let slide_one = {
        let mut part = archive
            .by_name("ppt/slides/slide1.xml")
            .expect("first slide is present");
        let mut xml = String::new();
        part.read_to_string(&mut xml).unwrap();
        xml
    };
    let slide_two = {
        let mut part = archive
            .by_name("ppt/slides/slide2.xml")
            .expect("second slide is present");
        let mut xml = String::new();
        part.read_to_string(&mut xml).unwrap();
        xml
    };
    assert!(slide_one.contains("r:embed=\"rId2\""));
    assert!(slide_two.contains("Second slide content"));

    let rels = {
        let mut part = archive
            .by_name("ppt/slides/_rels/slide1.xml.rels")
            .expect("image relationship part is present");
        let mut xml = String::new();
        part.read_to_string(&mut xml).unwrap();
        xml
    };
    assert!(rels.contains("../media/image1.png"));

    let mut image = Vec::new();
    archive
        .by_name("ppt/media/image1.png")
        .expect("embedded image part is present")
        .read_to_end(&mut image)
        .unwrap();
    assert_eq!(image, image_bytes);
}

#[test]
fn opc_package_round_trip_preserves_nested_paths_and_binary_parts() {
    let mut package = Package::new();
    package.add_part(
        "ppt/embeddings/workbook.bin".to_string(),
        vec![0, 1, 2, 0xff, 0x80, 42],
    );
    package.add_part(
        "ppt/slides/_rels/slide1.xml.rels".to_string(),
        b"<Relationships/>".to_vec(),
    );

    let mut cursor = Cursor::new(Vec::new());
    package
        .save_writer(&mut cursor)
        .expect("package serializes to ZIP");
    let reopened = Package::open_reader(Cursor::new(cursor.into_inner()))
        .expect("serialized package opens again");

    assert_eq!(
        reopened.get_part("ppt/embeddings/workbook.bin"),
        Some(&[0, 1, 2, 0xff, 0x80, 42][..])
    );
    assert_eq!(
        reopened.get_part("ppt/slides/_rels/slide1.xml.rels"),
        Some(b"<Relationships/>".as_slice())
    );
}

#[test]
fn opc_package_rejects_truncated_zip_input() {
    let mut package = Package::new();
    package.add_part(
        "ppt/presentation.xml".to_string(),
        b"<p:presentation/>".to_vec(),
    );
    let mut cursor = Cursor::new(Vec::new());
    package
        .save_writer(&mut cursor)
        .expect("package serializes to ZIP");
    let mut truncated = cursor.into_inner();
    truncated.truncate(truncated.len().saturating_sub(12));

    assert!(
        Package::open_reader(Cursor::new(truncated)).is_err(),
        "a missing ZIP end-of-central-directory record must return an error"
    );
}

#[test]
fn opc_package_decodes_legacy_zip_filenames() {
    let mut package = Package::new();
    package.add_part("x".to_string(), b"part".to_vec());
    let mut cursor = Cursor::new(Vec::new());
    package.save_writer(&mut cursor).unwrap();
    let mut bytes = cursor.into_inner();

    let local_header = bytes
        .windows(4)
        .position(|signature| signature == b"PK\x03\x04")
        .expect("local file header exists");
    let local_name_len =
        u16::from_le_bytes([bytes[local_header + 26], bytes[local_header + 27]]) as usize;
    assert_eq!(local_name_len, 1);
    bytes[local_header + 30] = 0x82; // CP437 e-acute.

    let central_header = bytes
        .windows(4)
        .position(|signature| signature == b"PK\x01\x02")
        .expect("central directory header exists");
    let central_name_len =
        u16::from_le_bytes([bytes[central_header + 28], bytes[central_header + 29]]) as usize;
    assert_eq!(central_name_len, 1);
    bytes[central_header + 46] = 0x82;

    let reopened = Package::open_reader(Cursor::new(bytes)).unwrap();
    assert_eq!(reopened.get_part("é"), Some(&b"part"[..]));
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
        let options = SimpleFileOptions::default();
        let mut archive = ZipArchive::new(Cursor::new(&bytes)).unwrap();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let name = file.name().unwrap().to_string();
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
