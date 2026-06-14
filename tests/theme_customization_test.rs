//! Integration tests for custom presentation themes

use ppt_rs::api::Presentation;
use ppt_rs::generator::{PresentationTheme, SlideContent, create_pptx_with_settings, PresentationSettings};
use ppt_rs::prelude::themes;
use std::io::Cursor;

fn theme_xml_from_pptx(data: &[u8]) -> String {
    let cursor = Cursor::new(data);
    let mut archive = zip::ZipArchive::new(cursor).expect("valid pptx zip");
    let mut file = archive.by_name("ppt/theme/theme1.xml").expect("theme part");
    let mut xml = String::new();
    std::io::Read::read_to_string(&mut file, &mut xml).unwrap();
    xml
}

#[test]
fn test_corporate_theme_embedded_in_pptx() {
    let slides = vec![SlideContent::new("Themed").add_bullet("Point")];
    let settings = PresentationSettings::new().theme(PresentationTheme::corporate());
    let pptx = create_pptx_with_settings("Themed Deck", slides, Some(settings)).unwrap();
    let xml = theme_xml_from_pptx(&pptx);

    assert!(xml.contains("1565C0"), "corporate primary accent");
    assert!(xml.contains(r#"name="Corporate""#));
}

#[test]
fn test_presentation_api_with_theme() {
    let pres = Presentation::with_title("API Theme Test")
        .add_slide(SlideContent::new("Slide").add_bullet("A"))
        .with_theme(themes::CARBON.to_presentation_theme());

    let pptx = pres.build().unwrap();
    let xml = theme_xml_from_pptx(&pptx);

    assert!(xml.contains("0043CE"));
    assert!(xml.contains(r#"name="Carbon""#));
}

#[test]
fn test_custom_fonts_in_theme() {
    let theme = PresentationTheme::modern()
        .major_font("Georgia")
        .minor_font("Verdana");
    let slides = vec![SlideContent::new("Fonts")];
    let settings = PresentationSettings::new().theme(theme);
    let pptx = create_pptx_with_settings("Font Theme", slides, Some(settings)).unwrap();
    let xml = theme_xml_from_pptx(&pptx);

    assert!(xml.contains(r#"typeface="Georgia""#));
    assert!(xml.contains(r#"typeface="Verdana""#));
}
