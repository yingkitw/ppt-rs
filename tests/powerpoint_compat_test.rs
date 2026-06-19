//! PowerPoint structural compatibility validation suite.

mod common;

use std::fs::{self, File};
use std::io::{Cursor, Read, Write};

use ppt_rs::core::REQUIRED_PACKAGE_PARTS;
use ppt_rs::generator::package_xml::{
    first_slide_rel_id, handout_master_rel_id, notes_master_rel_id, slide_id_value,
    slide_rel_id,
};
use ppt_rs::generator::{
    create_pptx, create_pptx_lazy_to_writer, create_pptx_to_writer,
    create_pptx_with_content, create_pptx_with_settings, ChartBuilder, ChartSeries,
    ChartType, LazySlideSource, PresentationSettings, PresentationTheme, PrintSettings,
    PrintWhat, SlideContent,
};
use ppt_rs::generator::slide_content::print_settings::HandoutLayout;
use zip::ZipArchive;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

struct ArchiveParts {
    names: Vec<String>,
}

impl ArchiveParts {
    fn from_bytes(bytes: &[u8]) -> Self {
        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).expect("valid zip");
        let mut names = Vec::new();
        for i in 0..archive.len() {
            if let Ok(f) = archive.by_index(i) {
                if !f.is_dir() {
                    names.push(f.name().to_string());
                }
            }
        }
        Self { names }
    }

    fn has(&self, path: &str) -> bool {
        self.names.iter().any(|n| n == path)
    }

    fn read_part(bytes: &[u8], path: &str) -> String {
        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).expect("valid zip");
        let mut content = String::new();
        archive
            .by_name(path)
            .unwrap_or_else(|_| panic!("missing part: {path}"))
            .read_to_string(&mut content)
            .expect("read part");
        content
    }

    fn part_len(bytes: &[u8], path: &str) -> usize {
        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).expect("valid zip");
        let mut buf = Vec::new();
        archive
            .by_name(path)
            .unwrap_or_else(|_| panic!("missing part: {path}"))
            .read_to_end(&mut buf)
            .expect("read part");
        buf.len()
    }
}

fn assert_compat(bytes: &[u8], label: &str) {
    common::assert_package_valid(bytes, label);
}

fn bar_chart(title: &str) -> ppt_rs::generator::Chart {
    ChartBuilder::new(title, ChartType::Bar)
        .position(1_000_000, 1_000_000)
        .size(3_000_000, 2_000_000)
        .add_series(ChartSeries::new("S1", vec![1.0, 2.0, 3.0]))
        .build()
}

// ---------------------------------------------------------------------------
// Compat gate — baseline decks
// ---------------------------------------------------------------------------

#[test]
fn minimal_deck_passes_compat_gate() {
    assert_compat(&create_pptx("Minimal", 1).unwrap(), "minimal");
}

#[test]
fn multi_slide_deck_passes_compat_gate() {
    let slides: Vec<_> = (1..=5)
        .map(|i| SlideContent::new(&format!("Slide {i}")).add_bullet("Point"))
        .collect();
    let bytes = create_pptx_with_content("Multi", slides).unwrap();
    assert_compat(&bytes, "multi-slide");

    let pres = ArchiveParts::read_part(&bytes, "ppt/presentation.xml");
    assert!(pres.contains(r#"id="256""#));
    assert!(pres.contains(r#"id="260""#));
    assert!(pres.contains(r#"r:id="rId2""#));
    assert!(pres.contains(r#"r:id="rId6""#));
}

#[test]
fn streaming_api_passes_compat_gate() {
    let cursor = Cursor::new(Vec::new());
    let cursor = create_pptx_to_writer(cursor, "Stream", 4).unwrap();
    assert_compat(&cursor.into_inner(), "streaming");
}

#[test]
fn lazy_api_passes_compat_gate() {
    struct Src {
        count: usize,
    }
    impl LazySlideSource for Src {
        fn slide_count(&self) -> usize {
            self.count
        }
        fn generate_slide(&self, index: usize) -> Option<SlideContent> {
            (index < self.count).then(|| {
                SlideContent::new(&format!("Lazy {}", index + 1)).add_bullet("item")
            })
        }
    }

    let cursor = Cursor::new(Vec::new());
    let cursor = create_pptx_lazy_to_writer(cursor, "Lazy", Box::new(Src { count: 6 }), None)
        .unwrap();
    assert_compat(&cursor.into_inner(), "lazy");
}

#[test]
fn custom_theme_deck_passes_compat_gate() {
    let settings = PresentationSettings::new().theme(PresentationTheme::corporate());
    let bytes = create_pptx_with_settings(
        "Branded",
        &[SlideContent::new("Title")],
        Some(settings),
    )
    .unwrap();
    assert_compat(&bytes, "custom-theme");
    assert!(
        ArchiveParts::part_len(&bytes, "ppt/theme/theme1.xml") >= 4000,
        "custom theme should patch full Office template, not a stub"
    );
}

// ---------------------------------------------------------------------------
// Standard package parts (always emitted)
// ---------------------------------------------------------------------------

#[test]
fn standard_package_parts_always_present() {
    let bytes = create_pptx("Parts", 1).unwrap();
    let parts = ArchiveParts::from_bytes(&bytes);

    for required in REQUIRED_PACKAGE_PARTS {
        assert!(parts.has(required), "missing {required}");
    }
}

#[test]
fn presentation_includes_default_text_style() {
    let bytes = create_pptx("TextStyle", 1).unwrap();
    let pres = ArchiveParts::read_part(&bytes, "ppt/presentation.xml");
    assert!(pres.contains("<p:defaultTextStyle>"));
    assert!(pres.contains("<a:lvl1pPr"));
}

#[test]
fn pres_props_view_props_table_styles_are_well_formed() {
    let bytes = create_pptx("Props", 1).unwrap();
    let pres_props = ArchiveParts::read_part(&bytes, "ppt/presProps.xml");
    let view_props = ArchiveParts::read_part(&bytes, "ppt/viewProps.xml");
    let table_styles = ArchiveParts::read_part(&bytes, "ppt/tableStyles.xml");

    assert!(pres_props.contains("<p:presentationPr"));
    assert!(pres_props.contains("<p:extLst>"));
    assert!(view_props.contains("<p:viewPr"));
    assert!(table_styles.contains("<a:tblStyleLst"));
}

// ---------------------------------------------------------------------------
// Package relationship ordering
// ---------------------------------------------------------------------------

#[test]
fn presentation_rel_order_matches_powerpoint() {
    let bytes = create_pptx("Rels", 1).unwrap();
    let rels = ArchiveParts::read_part(&bytes, "ppt/_rels/presentation.xml.rels");

    let master_pos = rels.find("slideMaster").unwrap();
    let slide_pos = rels.find("slides/slide1").unwrap();
    let pres_props_pos = rels.find("presProps").unwrap();
    let view_props_pos = rels.find("viewProps").unwrap();
    let theme_pos = rels.find("theme/theme1").unwrap();
    let table_pos = rels.find("tableStyles").unwrap();

    assert!(master_pos < slide_pos);
    assert!(slide_pos < pres_props_pos);
    assert!(pres_props_pos < view_props_pos);
    assert!(view_props_pos < theme_pos);
    assert!(theme_pos < table_pos);
}

#[test]
fn presentation_rels_use_exact_rid_mapping() {
    let bytes = create_pptx("RidMap", 1).unwrap();
    let rels = ArchiveParts::read_part(&bytes, "ppt/_rels/presentation.xml.rels");

    assert!(rels.contains(r#"Id="rId1""#) && rels.contains("slideMaster"));
    assert!(rels.contains(r#"Id="rId2""#) && rels.contains("slides/slide1"));
    assert!(rels.contains(r#"Id="rId3""#) && rels.contains("presProps"));
    assert!(rels.contains(r#"Id="rId4""#) && rels.contains("viewProps"));
    assert!(rels.contains(r#"Id="rId5""#) && rels.contains("theme/theme1"));
    assert!(rels.contains(r#"Id="rId6""#) && rels.contains("tableStyles"));
}

#[test]
fn slide_ids_and_rids_match_package_xml_helpers() {
    let bytes = create_pptx("Ids", 3).unwrap();
    let pres = ArchiveParts::read_part(&bytes, "ppt/presentation.xml");

    for slide_num in 1..=3 {
        let id = slide_id_value(slide_num);
        let rid = slide_rel_id(slide_num, false, false);
        assert!(pres.contains(&format!(r#"id="{id}""#)));
        assert!(pres.contains(&format!(r#"r:id="rId{rid}""#)));
    }
    assert_eq!(first_slide_rel_id(false, false), 2);
}

// ---------------------------------------------------------------------------
// Slide master & theme completeness
// ---------------------------------------------------------------------------

#[test]
fn slide_master_has_tx_styles() {
    let bytes = create_pptx("TxStyles", 1).unwrap();
    let master = ArchiveParts::read_part(&bytes, "ppt/slideMasters/slideMaster1.xml");
    assert!(master.contains("<p:txStyles>"));
    assert!(master.contains("<p:titleStyle>"));
    assert!(master.contains("<p:bodyStyle>"));
    assert!(master.contains("<p:otherStyle>"));
}

#[test]
fn theme_is_full_office_template() {
    let bytes = create_pptx("Theme", 1).unwrap();
    let len = ArchiveParts::part_len(&bytes, "ppt/theme/theme1.xml");
    assert!(
        len >= 7000,
        "theme should be full Office template, got {len} bytes"
    );

    let theme = ArchiveParts::read_part(&bytes, "ppt/theme/theme1.xml");
    assert!(theme.contains("<a:fmtScheme"));
    assert!(theme.contains("<a:fontScheme"));
}

// ---------------------------------------------------------------------------
// Chart Excel workbook embedding
// ---------------------------------------------------------------------------

#[test]
fn chart_deck_has_embedding_and_external_data() {
    let slide = SlideContent::new("Chart").add_chart(bar_chart("Sales"));
    let bytes = create_pptx_with_content("Chart", vec![slide]).unwrap();
    assert_compat(&bytes, "chart");

    let chart_xml = ArchiveParts::read_part(&bytes, "ppt/charts/chart1.xml");
    assert!(chart_xml.contains("<c:externalData"));
    assert!(chart_xml.contains(r#"r:id="rId1""#));

    let chart_rels = ArchiveParts::read_part(&bytes, "ppt/charts/_rels/chart1.xml.rels");
    assert!(chart_rels.contains("relationships/package"));
    assert!(chart_rels.contains("Microsoft_Excel_Sheet1.xlsx"));

    let parts = ArchiveParts::from_bytes(&bytes);
    assert!(parts.has("ppt/embeddings/Microsoft_Excel_Sheet1.xlsx"));

    let ct = ArchiveParts::read_part(&bytes, "[Content_Types].xml");
    assert!(ct.contains("Microsoft_Excel_Sheet1.xlsx"));
    assert!(ct.contains("spreadsheetml.sheet"));
}

#[test]
fn multiple_charts_get_distinct_embeddings() {
    let slide = SlideContent::new("Charts")
        .add_chart(bar_chart("A"))
        .add_chart(bar_chart("B"));
    let bytes = create_pptx_with_content("MultiChart", vec![slide]).unwrap();
    assert_compat(&bytes, "multi-chart");

    let parts = ArchiveParts::from_bytes(&bytes);
    assert!(parts.has("ppt/charts/chart1.xml"));
    assert!(parts.has("ppt/charts/chart2.xml"));
    assert!(parts.has("ppt/embeddings/Microsoft_Excel_Sheet1.xlsx"));
    assert!(parts.has("ppt/embeddings/Microsoft_Excel_Sheet2.xlsx"));

    let rels2 = ArchiveParts::read_part(&bytes, "ppt/charts/_rels/chart2.xml.rels");
    assert!(rels2.contains("Microsoft_Excel_Sheet2.xlsx"));
}

#[test]
fn charts_on_multiple_slides_each_embed_workbook() {
    let slides = vec![
        SlideContent::new("S1").add_chart(bar_chart("C1")),
        SlideContent::new("S2").add_chart(bar_chart("C2")),
    ];
    let bytes = create_pptx_with_content("ChartSlides", slides).unwrap();
    assert_compat(&bytes, "charts-across-slides");

    let parts = ArchiveParts::from_bytes(&bytes);
    assert!(parts.has("ppt/charts/chart1.xml"));
    assert!(parts.has("ppt/charts/chart2.xml"));
}

#[test]
fn embedding_xlsx_is_non_empty_zip() {
    let slide = SlideContent::new("Chart").add_chart(bar_chart("Data"));
    let bytes = create_pptx_with_content("Xlsx", vec![slide]).unwrap();

    let cursor = Cursor::new(bytes);
    let mut outer = ZipArchive::new(cursor).unwrap();
    let mut xlsx = Vec::new();
    outer
        .by_name("ppt/embeddings/Microsoft_Excel_Sheet1.xlsx")
        .unwrap()
        .read_to_end(&mut xlsx)
        .unwrap();
    assert!(xlsx.len() > 100, "embedding should be a real xlsx blob");

    let inner = ZipArchive::new(Cursor::new(xlsx)).expect("xlsx is zip");
    assert!(inner.len() > 0, "xlsx should contain workbook parts");
}

// ---------------------------------------------------------------------------
// Handout master packaging
// ---------------------------------------------------------------------------

#[test]
fn handout_deck_includes_handout_master() {
    let print = PrintSettings::default()
        .print_what(PrintWhat::Handouts)
        .handout_layout(HandoutLayout::SlidesPerPage6);
    let settings = PresentationSettings::new().print(print);
    let bytes = create_pptx_with_settings(
        "Handouts",
        &[SlideContent::new("Slide 1")],
        Some(settings),
    )
    .unwrap();
    assert_compat(&bytes, "handouts");

    let parts = ArchiveParts::from_bytes(&bytes);
    assert!(parts.has("ppt/handoutMasters/handoutMaster1.xml"));
    assert!(parts.has("ppt/handoutMasters/_rels/handoutMaster1.xml.rels"));
    assert!(parts.has("ppt/theme/theme3.xml"));

    let pres = ArchiveParts::read_part(&bytes, "ppt/presentation.xml");
    assert!(pres.contains("<p:handoutMasterIdLst>"));
    assert!(!pres.contains("<p:handoutMasterId id="));

    let rels = ArchiveParts::read_part(&bytes, "ppt/_rels/presentation.xml.rels");
    let handout_rid = handout_master_rel_id(1, false);
    assert!(rels.contains(&format!(r#"Id="rId{handout_rid}""#)));
    assert!(rels.contains("handoutMaster"));

    let pres_props = ArchiveParts::read_part(&bytes, "ppt/presProps.xml");
    assert!(
        !pres_props.contains("<p:prnPr"),
        "handout master packaging should not pair with presProps prnPr"
    );

    let handout = ArchiveParts::read_part(&bytes, "ppt/handoutMasters/handoutMaster1.xml");
    assert!(handout.contains("<p:handoutMaster"));
    assert!(handout.contains("<p:bg>"));

    let handout_rels =
        ArchiveParts::read_part(&bytes, "ppt/handoutMasters/_rels/handoutMaster1.xml.rels");
    assert!(handout_rels.contains("theme/theme3.xml"));
}

#[test]
fn slides_print_mode_does_not_package_handout_master() {
    let print = PrintSettings::default().print_what(PrintWhat::Slides);
    let settings = PresentationSettings::new().print(print);
    let bytes = create_pptx_with_settings(
        "SlidesOnly",
        &[SlideContent::new("Slide 1")],
        Some(settings),
    )
    .unwrap();
    assert_compat(&bytes, "slides-print");

    let parts = ArchiveParts::from_bytes(&bytes);
    assert!(!parts.has("ppt/handoutMasters/handoutMaster1.xml"));

    let pres = ArchiveParts::read_part(&bytes, "ppt/presentation.xml");
    assert!(!pres.contains("<p:handoutMasterIdLst>"));

    let pres_props = ArchiveParts::read_part(&bytes, "ppt/presProps.xml");
    assert!(!pres_props.contains("prnWhat"));
}

// ---------------------------------------------------------------------------
// Notes master + combined optional masters
// ---------------------------------------------------------------------------

#[test]
fn notes_deck_wires_notes_master_and_shifts_slide_rids() {
    let mut slide = SlideContent::new("Talk");
    slide.notes = Some("Remember the demo".into());
    let bytes = create_pptx_with_content("Notes", vec![slide]).unwrap();
    assert_compat(&bytes, "notes");

    let parts = ArchiveParts::from_bytes(&bytes);
    assert!(parts.has("ppt/notesMasters/notesMaster1.xml"));
    assert!(parts.has("ppt/notesSlides/notesSlide1.xml"));
    assert!(parts.has("ppt/theme/theme2.xml"));

    let pres = ArchiveParts::read_part(&bytes, "ppt/presentation.xml");
    assert!(pres.contains("<p:notesMasterIdLst>"));
    assert!(!pres.contains("<p:notesMasterId id="));
    assert!(pres.contains(&format!(
        r#"r:id="rId{}""#,
        notes_master_rel_id(1)
    )));

    let first_rid = slide_rel_id(1, true, false);
    assert!(pres.contains(&format!(r#"r:id="rId{first_rid}""#)));
    assert_eq!(first_rid, 2);

    let notes_rels =
        ArchiveParts::read_part(&bytes, "ppt/notesMasters/_rels/notesMaster1.xml.rels");
    assert!(notes_rels.contains("theme/theme2.xml"), "notes master uses dedicated theme2");
}

#[test]
fn notes_and_handouts_both_shift_slide_rids() {
    let print = PrintSettings::default()
        .print_what(PrintWhat::Handouts)
        .handout_layout(HandoutLayout::SlidesPerPage4);
    let settings = PresentationSettings::new().print(print);

    let mut slide = SlideContent::new("Combined");
    slide.notes = Some("Note".into());

    let bytes = create_pptx_with_settings("Both", &[slide], Some(settings)).unwrap();
    assert_compat(&bytes, "notes+handouts");

    let pres = ArchiveParts::read_part(&bytes, "ppt/presentation.xml");
    assert!(pres.contains("<p:notesMasterIdLst>"));
    assert!(pres.contains("<p:handoutMasterIdLst>"));

    let first_rid = slide_rel_id(1, true, true);
    assert_eq!(first_rid, 2);
    assert!(pres.contains(&format!(r#"r:id="rId{first_rid}""#)));

    let rels = ArchiveParts::read_part(&bytes, "ppt/_rels/presentation.xml.rels");
    assert!(rels.contains(&format!(r#"Id="rId{}""#, notes_master_rel_id(1))));
    assert!(rels.contains(&format!(r#"Id="rId{}""#, handout_master_rel_id(1, true))));
}

// ---------------------------------------------------------------------------
// Bisect matrix — golden variants for manual PowerPoint open + CI gate
// ---------------------------------------------------------------------------

#[test]
fn bisect_matrix_all_variants_pass_compat_gate() {
    let out_dir = std::path::Path::new("target/powerpoint_bisect");
    fs::create_dir_all(out_dir).ok();

    let handout_settings = PresentationSettings::new().print(
        PrintSettings::default()
            .print_what(PrintWhat::Handouts)
            .handout_layout(HandoutLayout::SlidesPerPage6),
    );

    let mut slide_with_notes = SlideContent::new("Notes");
    slide_with_notes.notes = Some("Speaker note".into());

    let variants: Vec<(&str, Vec<u8>)> = vec![
        ("01_minimal.pptx", create_pptx("Minimal", 1).unwrap()),
        (
            "02_multi_slide.pptx",
            create_pptx_with_content(
                "Multi",
                vec![
                    SlideContent::new("A"),
                    SlideContent::new("B"),
                    SlideContent::new("C"),
                ],
            )
            .unwrap(),
        ),
        (
            "03_with_chart.pptx",
            create_pptx_with_content(
                "Chart",
                vec![SlideContent::new("C").add_chart(bar_chart("Data"))],
            )
            .unwrap(),
        ),
        (
            "04_with_notes.pptx",
            create_pptx_with_content("Notes", vec![slide_with_notes.clone()]).unwrap(),
        ),
        (
            "05_handouts.pptx",
            create_pptx_with_settings(
                "Handouts",
                &[SlideContent::new("H")],
                Some(handout_settings.clone()),
            )
            .unwrap(),
        ),
        (
            "06_notes_and_handouts.pptx",
            create_pptx_with_settings("Both", &[slide_with_notes], Some(handout_settings)).unwrap(),
        ),
        (
            "07_custom_theme.pptx",
            create_pptx_with_settings(
                "Theme",
                &[SlideContent::new("T")],
                Some(PresentationSettings::new().theme(PresentationTheme::modern())),
            )
            .unwrap(),
        ),
    ];

    for (name, bytes) in variants {
        assert_compat(&bytes, name);
        let path = out_dir.join(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(&bytes).unwrap();
    }
}
