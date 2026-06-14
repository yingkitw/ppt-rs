//! Integration tests for v0.2.16+ capabilities:
//! - Advanced theme customization
//! - Table cell formatting consolidation
//! - Image effects modularization
//! - Package XML generation
//! - Presentation API optimizations (`into_bytes`, borrow-based build)

use ppt_rs::api::Presentation;
use ppt_rs::generator::image_effects::{
    generate_blip_fill_xml, generate_effect_list_xml, generate_effect_xml,
};
use ppt_rs::generator::images::{Crop, Image, ImageEffect};
use ppt_rs::generator::memory_profile::estimate_slide_payload;
use ppt_rs::generator::package_xml::{
    create_content_types_xml, create_content_types_xml_with_notes_and_charts,
    create_presentation_rels_xml, create_presentation_xml,
};
use ppt_rs::generator::table::{
    generate_cell_xml, header_cell, table_from_string_rows, CellAlign, CellVAlign, IMPORT_HEADER_BG,
};
use ppt_rs::generator::{
    create_pptx_with_content, create_pptx_with_settings, generate_image_xml, PresentationSettings,
    PresentationTheme, SlideContent, Table, TableCell, ThemeColorScheme,
};
use ppt_rs::prelude::themes;
use std::io::Cursor;

fn read_zip_entry(pptx: &[u8], path: &str) -> String {
    let cursor = Cursor::new(pptx);
    let mut archive = zip::ZipArchive::new(cursor).expect("valid pptx zip");
    let mut file = archive.by_name(path).expect("zip entry");
    let mut content = String::new();
    std::io::Read::read_to_string(&mut file, &mut content).unwrap();
    content
}

fn theme_xml_from_pptx(data: &[u8]) -> String {
    read_zip_entry(data, "ppt/theme/theme1.xml")
}

fn slide_xml_from_pptx(data: &[u8], slide_num: usize) -> String {
    read_zip_entry(data, &format!("ppt/slides/slide{slide_num}.xml"))
}

// ---------------------------------------------------------------------------
// Theme customization (v0.2.16)
// ---------------------------------------------------------------------------

#[test]
fn test_all_theme_presets_embed_in_pptx() {
    let presets: Vec<(&str, PresentationTheme, &str)> = vec![
        ("Corporate", PresentationTheme::corporate(), "1565C0"),
        ("Modern", PresentationTheme::modern(), "212121"),
        ("Vibrant", PresentationTheme::vibrant(), "E91E63"),
        ("Dark", PresentationTheme::dark(), "BB86FC"),
        ("Nature", PresentationTheme::nature(), "2E7D32"),
        ("Tech", PresentationTheme::tech(), "0D47A1"),
        ("Carbon", PresentationTheme::carbon(), "0043CE"),
    ];

    for (name, theme, accent) in presets {
        let slides = vec![SlideContent::new("Themed")];
        let settings = PresentationSettings::new().theme(theme);
        let pptx = create_pptx_with_settings("Preset Test", &slides, Some(settings)).unwrap();
        let xml = theme_xml_from_pptx(&pptx);

        assert!(xml.contains(accent), "{name} theme should contain accent {accent}");
        assert!(xml.contains(&format!(r#"name="{name}""#)), "{name} theme name missing");
    }
}

#[test]
fn test_default_office_theme_without_settings() {
    let slides = vec![SlideContent::new("Default")];
    let pptx = create_pptx_with_content("Default Theme", slides).unwrap();
    let xml = theme_xml_from_pptx(&pptx);

    assert!(xml.contains(r#"name="Office Theme""#));
    assert!(xml.contains("4F81BD"), "office default accent1");
}

#[test]
fn test_custom_theme_color_scheme() {
    let colors = ThemeColorScheme::from_palette(
        "AA0000", "BB0000", "CC0000", "FFFFFF", "111111", "EEEEEE", "000000",
    )
    .accent1("DD1122")
    .hyperlink("00AAFF");

    let theme = PresentationTheme::new("Brand").colors(colors);
    let slides = vec![SlideContent::new("Brand")];
    let pptx = create_pptx_with_settings(
        "Brand",
        &slides,
        Some(PresentationSettings::new().theme(theme)),
    )
    .unwrap();
    let xml = theme_xml_from_pptx(&pptx);

    assert!(xml.contains("DD1122"));
    assert!(xml.contains("00AAFF"));
    assert!(xml.contains(r#"name="Brand""#));
}

#[test]
fn test_prelude_themes_bridge_to_presentation_theme() {
    for theme in themes::all() {
        let embedded = theme.to_presentation_theme();
        assert_eq!(embedded.name, theme.name);
        assert!(embedded.to_theme_xml().contains(theme.primary));
    }
}

#[test]
fn test_presentation_into_bytes_with_theme() {
    let pres = Presentation::with_title("Into Bytes")
        .add_slide(SlideContent::new("Slide"))
        .with_theme(PresentationTheme::tech());

    let via_build = pres.build().unwrap();
    let pres2 = Presentation::with_title("Into Bytes")
        .add_slide(SlideContent::new("Slide"))
        .with_theme(PresentationTheme::tech());
    let via_into = pres2.into_bytes().unwrap();

    assert_eq!(via_build.len(), via_into.len());
    assert!(theme_xml_from_pptx(&via_into).contains("0D47A1"));
}

// ---------------------------------------------------------------------------
// Table cell formatting (v0.2.17)
// ---------------------------------------------------------------------------

#[test]
fn test_cell_horizontal_alignment_in_xml() {
    let cell = TableCell::new("Left").align(CellAlign::Left);
    let xml = generate_cell_xml(&cell);
    assert!(xml.contains(r#"algn="l""#));
}

#[test]
fn test_cell_vertical_alignment_in_xml() {
    let cell = TableCell::new("Top").valign(CellVAlign::Top);
    let xml = generate_cell_xml(&cell);
    assert!(xml.contains(r#"anchor="t""#));
}

#[test]
fn test_cell_wrap_disabled_in_xml() {
    let cell = TableCell::new("No wrap").wrap(false);
    let xml = generate_cell_xml(&cell);
    assert!(xml.contains(r#"wrap="none""#));
}

#[test]
fn test_cell_merge_attributes_in_xml() {
    let anchor = TableCell::new("Merged").grid_span(2);
    let covered = TableCell::new("").h_merge();

    let anchor_xml = generate_cell_xml(&anchor);
    let covered_xml = generate_cell_xml(&covered);

    assert!(anchor_xml.contains(r#"gridSpan="2""#));
    assert!(covered_xml.contains(r#"hMerge="1""#));
    assert!(covered_xml.contains("<a:p/>"));
}

#[test]
fn test_header_cell_preset_colors() {
    let cell = header_cell("Column");
    let xml = generate_cell_xml(&cell);

    assert!(xml.contains(IMPORT_HEADER_BG));
    assert!(xml.contains("FFFFFF"));
    assert!(xml.contains(r#"b="1""#));
}

#[test]
fn test_table_from_string_rows_embedded_in_pptx() {
    let table = table_from_string_rows(
        vec![
            vec!["Name".into(), "Score".into()],
            vec!["Alice".into(), "95".into()],
        ],
        true,
    );

    let slide = SlideContent::new("Data").table(table);

    let pptx = create_pptx_with_content("Table Deck", vec![slide]).unwrap();
    let slide_xml = slide_xml_from_pptx(&pptx, 1);

    assert!(slide_xml.contains(IMPORT_HEADER_BG));
    assert!(slide_xml.contains("<a:t>Name</a:t>"));
    assert!(slide_xml.contains("<a:t>Alice</a:t>"));
}

#[test]
fn test_table_position_defaults_from_style_module() {
    let table: Table = table_from_string_rows(vec![vec!["A".into()]], false);
    assert_eq!(table.x, 500_000);
    assert_eq!(table.y, 1_800_000);
}

// ---------------------------------------------------------------------------
// Image effects module (v0.2.17)
// ---------------------------------------------------------------------------

#[test]
fn test_all_image_effect_variants_generate_xml() {
    let effects = [
        (ImageEffect::Shadow, "outerShdw"),
        (ImageEffect::Reflection, "reflection"),
        (ImageEffect::Glow, "glow"),
        (ImageEffect::SoftEdges, "softEdge"),
        (ImageEffect::InnerShadow, "innerShdw"),
        (ImageEffect::Blur, "blur"),
    ];

    for (effect, token) in effects {
        let xml = generate_effect_xml(&effect);
        assert!(xml.contains(token), "{token} missing for effect variant");
    }
}

#[test]
fn test_effect_list_module_matches_integrated_image_xml() {
    let effects = vec![
        ImageEffect::Glow,
        ImageEffect::SoftEdges,
        ImageEffect::Blur,
    ];
    let list_xml = generate_effect_list_xml(&effects);

    let img = Image::new("fx.png", 1000, 1000, "PNG")
        .with_effect(ImageEffect::Glow)
        .with_effect(ImageEffect::SoftEdges)
        .with_effect(ImageEffect::Blur);
    let integrated = generate_image_xml(&img, 1, 1);

    for token in ["glow", "softEdge", "blur"] {
        assert!(list_xml.contains(token));
        assert!(integrated.contains(token));
    }
}

#[test]
fn test_blip_fill_crop_module_matches_integrated_xml() {
    let crop = Crop {
        left: 0.05,
        top: 0.10,
        right: 0.15,
        bottom: 0.20,
    };
    let module_xml = generate_blip_fill_xml("rId3", Some(&crop));
    let img = Image::new("crop.png", 1000, 1000, "PNG").with_crop(0.05, 0.10, 0.15, 0.20);
    let integrated = generate_image_xml(&img, 1, 3);

    assert!(module_xml.contains(r#"l="5000""#));
    assert!(integrated.contains(r#"l="5000""#));
}

// ---------------------------------------------------------------------------
// Package XML generation (v0.2.17)
// ---------------------------------------------------------------------------

#[test]
fn test_content_types_xml_slide_overrides() {
    let xml = create_content_types_xml(3);
    assert!(xml.contains("/ppt/slides/slide1.xml"));
    assert!(xml.contains("/ppt/slides/slide2.xml"));
    assert!(xml.contains("/ppt/slides/slide3.xml"));
    assert!(!xml.contains("/ppt/slides/slide4.xml"));
}

#[test]
fn test_presentation_rels_xml_slide_relationships() {
    let xml = create_presentation_rels_xml(2);
    assert!(xml.contains(r#"Target="slides/slide1.xml""#));
    assert!(xml.contains(r#"Target="slides/slide2.xml""#));
    assert!(xml.contains(r#"Id="rId3""#));
    assert!(xml.contains(r#"Id="rId4""#));
}

#[test]
fn test_presentation_xml_slide_id_list() {
    let xml = create_presentation_xml("Title", 2);
    assert!(xml.contains(r#"id="257""#));
    assert!(xml.contains(r#"id="258""#));
    assert!(xml.contains(r#"r:id="rId3""#));
    assert!(xml.contains(r#"r:id="rId4""#));
}

#[test]
fn test_content_types_with_notes_and_charts() {
    let mut slide_with_notes = SlideContent::new("Notes");
    slide_with_notes.notes = Some("Speaker note".into());

    let slides = vec![SlideContent::new("Plain"), slide_with_notes];
    let xml = create_content_types_xml_with_notes_and_charts(2, Some(&slides), 1);

    assert!(xml.contains("/ppt/notesSlides/notesSlide2.xml"));
    assert!(!xml.contains("/ppt/notesSlides/notesSlide1.xml"));
    assert!(xml.contains("/ppt/charts/chart1.xml"));
    assert!(xml.contains("notesMaster1.xml"));
}

// ---------------------------------------------------------------------------
// API & memory helpers (v0.2.17)
// ---------------------------------------------------------------------------

#[test]
fn test_build_and_into_bytes_produce_same_pptx() {
    let pres = Presentation::with_title("Compare APIs")
        .add_slide(SlideContent::new("One").add_bullet("A"))
        .add_slide(SlideContent::new("Two").add_bullet("B"));

    let from_build = pres.build().unwrap();

    let pres2 = Presentation::with_title("Compare APIs")
        .add_slide(SlideContent::new("One").add_bullet("A"))
        .add_slide(SlideContent::new("Two").add_bullet("B"));
    let from_into = pres2.into_bytes().unwrap();

    assert_eq!(from_build, from_into);
}

#[test]
fn test_estimate_slide_payload_counts_text() {
    let slides = vec![
        SlideContent::new("Hello").add_bullet("World"),
        SlideContent::new("Second"),
    ];
    let bytes = estimate_slide_payload(&slides);
    assert!(bytes >= "Hello".len() + "World".len() + "Second".len());
}

#[test]
fn test_borrow_based_settings_api() {
    let slides = vec![SlideContent::new("Borrowed").add_bullet("Point")];
    let settings = PresentationSettings::new().theme(PresentationTheme::nature());
    let pptx = create_pptx_with_settings("Borrow Test", &slides, Some(settings)).unwrap();

    assert!(theme_xml_from_pptx(&pptx).contains("2E7D32"));
    read_zip_entry(&pptx, "[Content_Types].xml");
    read_zip_entry(&pptx, "ppt/slides/slide1.xml");
}
