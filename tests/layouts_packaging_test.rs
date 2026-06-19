//! Tests for multiple slide layouts, per-slide layout rels, footers, and templates.

mod common;

use std::fs;
use std::io::{Cursor, Read};

use ppt_rs::generator::{
    create_pptx, create_pptx_with_content, create_pptx_with_settings, create_pptx_with_template,
    PresentationSettings, PrintSettings, PptxTemplate, SlideContent, SlideLayout,
    STANDARD_LAYOUT_COUNT,
};
use zip::ZipArchive;

const ALL_LAYOUTS: [(SlideLayout, &str); 7] = [
    (SlideLayout::CenteredTitle, "Centered Title"),
    (SlideLayout::TitleAndContent, "Title and Content"),
    (SlideLayout::TwoColumn, "Two Column"),
    (SlideLayout::SectionHeader, "Section Header"),
    (SlideLayout::Blank, "Blank"),
    (SlideLayout::TitleOnly, "Title Only"),
    (SlideLayout::TitleAndBigContent, "Title and Big Content"),
];

fn read_part(bytes: &[u8], path: &str) -> String {
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).unwrap();
    let mut s = String::new();
    archive.by_name(path).unwrap().read_to_string(&mut s).unwrap();
    s
}

fn part_exists(bytes: &[u8], path: &str) -> bool {
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).unwrap();
    archive.by_name(path).is_ok()
}

#[test]
fn all_standard_layout_parts_are_packaged() {
    let bytes = create_pptx("Layouts", 1).unwrap();
    for n in 1..=STANDARD_LAYOUT_COUNT {
        assert!(part_exists(&bytes, &format!("ppt/slideLayouts/slideLayout{n}.xml")));
        assert!(part_exists(
            &bytes,
            &format!("ppt/slideLayouts/_rels/slideLayout{n}.xml.rels")
        ));
    }
}

#[test]
fn slide_master_lists_all_layouts() {
    let bytes = create_pptx("Master", 1).unwrap();
    let master = read_part(&bytes, "ppt/slideMasters/slideMaster1.xml");
    assert!(master.contains("<p:sldLayoutIdLst>"));
    for n in 1..=STANDARD_LAYOUT_COUNT {
        assert!(
            master.contains(&format!("r:id=\"rId{n}\"")),
            "master missing layout rId{n}"
        );
    }
}

#[test]
fn master_rels_include_all_layouts_before_theme() {
    let bytes = create_pptx("Rels", 1).unwrap();
    let rels = read_part(&bytes, "ppt/slideMasters/_rels/slideMaster1.xml.rels");
    assert!(rels.find("slideLayout1.xml").unwrap() < rels.find("theme/theme1.xml").unwrap());
    assert!(rels.contains("slideLayout7.xml"));
    assert!(rels.contains(r#"Id="rId8""#));
}

#[test]
fn per_slide_layout_rels_match_with_layout() {
    let slides = vec![
        SlideContent::new("Title").with_layout(SlideLayout::CenteredTitle),
        SlideContent::new("Body").with_layout(SlideLayout::TitleAndContent),
        SlideContent::new("Section").with_layout(SlideLayout::SectionHeader),
        SlideContent::new("Two").with_layout(SlideLayout::TwoColumn),
        SlideContent::new("Empty").with_layout(SlideLayout::Blank),
    ];
    let bytes = create_pptx_with_content("Mixed", slides).unwrap();

    let expected = [1, 2, 4, 3, 5];
    for (i, layout_num) in expected.iter().enumerate() {
        let rels = read_part(&bytes, &format!("ppt/slides/_rels/slide{}.xml.rels", i + 1));
        assert!(
            rels.contains(&format!("slideLayout{layout_num}.xml")),
            "slide {} rels: {rels}",
            i + 1
        );
    }
}

#[test]
fn layout_parts_have_placeholders() {
    let bytes = create_pptx("Ph", 1).unwrap();
    let title_content = read_part(&bytes, "ppt/slideLayouts/slideLayout2.xml");
    assert!(title_content.contains(r#"type="title""#));
    assert!(title_content.contains(r#"type="body""#));

    let section = read_part(&bytes, "ppt/slideLayouts/slideLayout4.xml");
    assert!(section.contains("secHead"));

    let two_col = read_part(&bytes, "ppt/slideLayouts/slideLayout3.xml");
    assert!(two_col.contains("twoObj"));
}

#[test]
fn slide_master_footer_placeholders_when_configured() {
    let print = PrintSettings::default()
        .footer("Confidential")
        .print_date(true)
        .print_page_numbers(true);
    let settings = PresentationSettings::new().print(print);
    let bytes = create_pptx_with_settings("Footers", &[SlideContent::new("A")], Some(settings)).unwrap();

    let master = read_part(&bytes, "ppt/slideMasters/slideMaster1.xml");
    assert!(master.contains(r#"type="ftr""#));
    assert!(master.contains(r#"type="dt""#));
    assert!(master.contains(r#"type="sldNum""#));
    assert!(master.contains("Confidential"));

    let layout = read_part(&bytes, "ppt/slideLayouts/slideLayout2.xml");
    assert!(layout.contains("<p:hf/>"));
}

#[test]
fn template_generation_clones_master_and_layouts() {
    let base = create_pptx("Base", 1).unwrap();
    let base_path = std::env::temp_dir().join("ppt_rs_layout_template_base.pptx");
    fs::write(&base_path, &base).unwrap();

    let tpl = PptxTemplate::load(&base_path).unwrap();
    assert!(tpl.layout_count() >= STANDARD_LAYOUT_COUNT);

    let bytes = create_pptx_with_template(
        "FromTemplate",
        &[SlideContent::new("New").with_layout(SlideLayout::TitleAndContent)],
        base_path.to_str().unwrap(),
        None,
    )
    .unwrap();

    assert!(part_exists(&bytes, "ppt/slideMasters/slideMaster1.xml"));
    assert!(part_exists(&bytes, "ppt/slideLayouts/slideLayout2.xml"));
    assert!(part_exists(&bytes, "ppt/slides/slide1.xml"));

    fs::remove_file(base_path).ok();
}

#[test]
fn with_layout_is_alias_for_layout() {
    let slide = SlideContent::new("T").with_layout(SlideLayout::TwoColumn);
    assert_eq!(slide.layout, SlideLayout::TwoColumn);
}

#[test]
fn slide_layout_number_mapping_is_stable() {
    assert_eq!(SlideLayout::CenteredTitle.layout_number(), 1);
    assert_eq!(SlideLayout::TitleAndContent.layout_number(), 2);
    assert_eq!(SlideLayout::TwoColumn.layout_number(), 3);
    assert_eq!(SlideLayout::SectionHeader.layout_number(), 4);
    assert_eq!(SlideLayout::Blank.layout_number(), 5);
    assert_eq!(SlideLayout::TitleOnly.layout_number(), 6);
    assert_eq!(SlideLayout::TitleAndBigContent.layout_number(), 7);
}

#[test]
fn all_seven_layouts_wired_in_slide_rels() {
    let slides: Vec<SlideContent> = ALL_LAYOUTS
        .iter()
        .map(|(layout, title)| {
            SlideContent::new(*title)
                .add_bullet("Sample body")
                .with_layout(*layout)
        })
        .collect();

    let bytes = create_pptx_with_content("All Seven", slides).unwrap();

    for (i, (layout, _)) in ALL_LAYOUTS.iter().enumerate() {
        let rels = read_part(&bytes, &format!("ppt/slides/_rels/slide{}.xml.rels", i + 1));
        let n = layout.layout_number();
        assert!(
            rels.contains(&format!("slideLayout{n}.xml")),
            "slide {} expected layout {n}, rels: {rels}",
            i + 1
        );
    }
}

#[test]
fn layout_part_types_match_ooxml_names() {
    let bytes = create_pptx("Types", 1).unwrap();
    let checks = [
        (1, "title"),
        (2, "obj"),
        (3, "twoObj"),
        (4, "secHead"),
        (5, "blank"),
        (6, "titleOnly"),
        (7, "obj"),
    ];
    for (n, type_attr) in checks {
        let xml = read_part(&bytes, &format!("ppt/slideLayouts/slideLayout{n}.xml"));
        assert!(
            xml.contains(&format!(r#"type="{type_attr}""#)),
            "slideLayout{n} missing type={type_attr}"
        );
    }
}

#[test]
fn mixed_layouts_pass_powerpoint_compat_gate() {
    let slides = vec![
        SlideContent::new("Cover").with_layout(SlideLayout::CenteredTitle),
        SlideContent::new("Agenda")
            .add_bullet("Layouts")
            .add_bullet("Templates")
            .with_layout(SlideLayout::TitleAndContent),
        SlideContent::new("Part II").with_layout(SlideLayout::SectionHeader),
        SlideContent::new("Compare")
            .add_bullet("Left")
            .add_bullet("Right")
            .with_layout(SlideLayout::TwoColumn),
    ];
    let print = PrintSettings::default()
        .footer("Internal")
        .print_page_numbers(true);
    let settings = PresentationSettings::new().print(print);
    let bytes =
        create_pptx_with_settings("Compat Layouts", &slides, Some(settings)).unwrap();

    common::assert_package_valid(&bytes, "mixed layouts compat");
}

#[test]
fn slide_master_header_placeholder_when_configured() {
    let print = PrintSettings::default().header("Quarterly Review");
    let settings = PresentationSettings::new().print(print);
    let bytes =
        create_pptx_with_settings("Header", &[SlideContent::new("A")], Some(settings)).unwrap();

    let master = read_part(&bytes, "ppt/slideMasters/slideMaster1.xml");
    assert!(master.contains(r#"type="hdr""#));
    assert!(master.contains("Header Placeholder"));
}

#[test]
fn footer_placeholders_omitted_when_print_settings_empty() {
    let bytes = create_pptx("No Footers", 1).unwrap();
    let master = read_part(&bytes, "ppt/slideMasters/slideMaster1.xml");
    assert!(!master.contains(r#"type="ftr""#));
    assert!(!master.contains(r#"type="dt""#));
    assert!(!master.contains(r#"type="sldNum""#));

    let layout = read_part(&bytes, "ppt/slideLayouts/slideLayout2.xml");
    assert!(layout.contains("<p:hf/>"));
}

#[test]
fn template_with_footers_and_mixed_layouts() {
    let base = create_pptx_with_settings(
        "Template Base",
        &[SlideContent::new("Seed").with_layout(SlideLayout::TitleAndContent)],
        None,
    )
    .unwrap();
    let base_path = std::env::temp_dir().join("ppt_rs_layout_template_full.pptx");
    fs::write(&base_path, &base).unwrap();

    let print = PrintSettings::default()
        .footer("From Template")
        .print_date(true)
        .print_page_numbers(true);
    let settings = PresentationSettings::new().print(print);

    let slides = vec![
        SlideContent::new("New Cover").with_layout(SlideLayout::CenteredTitle),
        SlideContent::new("Section")
            .add_bullet("Point")
            .with_layout(SlideLayout::SectionHeader),
    ];

    let bytes = create_pptx_with_template(
        "Template Mix",
        &slides,
        base_path.to_str().unwrap(),
        Some(settings),
    )
    .unwrap();

    assert!(part_exists(&bytes, "ppt/slides/slide1.xml"));
    assert!(part_exists(&bytes, "ppt/slides/slide2.xml"));

    let rels1 = read_part(&bytes, "ppt/slides/_rels/slide1.xml.rels");
    assert!(rels1.contains("slideLayout1.xml"));

    let rels2 = read_part(&bytes, "ppt/slides/_rels/slide2.xml.rels");
    assert!(rels2.contains("slideLayout4.xml"));

    // Template path clones master/layout parts from the base deck (footer text lives there).
    assert!(part_exists(&bytes, "ppt/slideMasters/slideMaster1.xml"));
    assert!(part_exists(&bytes, "ppt/slideLayouts/slideLayout4.xml"));

    common::assert_package_valid(&bytes, "template deck");

    fs::remove_file(base_path).ok();
}

#[test]
fn default_slide_uses_title_and_content_layout() {
    let bytes = create_pptx_with_content("Default", vec![SlideContent::new("Only Title")]).unwrap();
    let rels = read_part(&bytes, "ppt/slides/_rels/slide1.xml.rels");
    assert!(rels.contains("slideLayout2.xml"));
}
