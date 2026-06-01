use ppt_rs::api::Presentation;
use ppt_rs::generator::SlideContent;
use ppt_rs::export::html::{export_to_html, export_to_html_with_options, HtmlExportOptions};

#[test]
fn test_html_export() {
    let pres = Presentation::with_title("HTML Export Test")
        .add_slide(SlideContent::new("First Slide").add_bullet("Bullet 1").add_bullet("Bullet 2"))
        .add_slide(SlideContent::new("Second Slide").add_bullet("Another point"));

    let html = export_to_html(&pres).expect("Failed to export HTML");

    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("<title>HTML Export Test</title>"));
    assert!(html.contains("<h1>HTML Export Test</h1>"));
    assert!(html.contains("<h2>First Slide</h2>"));
    assert!(html.contains("Bullet 1"));
    assert!(html.contains("<h2>Second Slide</h2>"));
    assert!(html.contains(".slide {")); // CSS check
}

#[test]
fn test_html_export_with_options() {
    let pres = Presentation::with_title("Enhanced Export Test")
        .add_slide(
            SlideContent::new("Slide with Notes")
                .add_bullet("Point 1")
                .add_bullet("Point 2")
        )
        .add_slide(
            SlideContent::new("Another Slide")
                .add_bullet("More content")
        );

    // Add notes to first slide
    let mut slide_with_notes = SlideContent::new("Slide with Notes")
        .add_bullet("Point 1")
        .add_bullet("Point 2");
    slide_with_notes.notes = Some("Speaker notes here".to_string());

    let options = HtmlExportOptions::new()
        .with_notes(true)
        .with_navigation(true)
        .with_syntax_highlight(false);

    let html = export_to_html_with_options(&pres, &options).expect("Failed to export HTML with options");

    // Check for enhanced features
    assert!(html.contains("navigation-controls")); // Navigation should be included
    assert!(html.contains("slideCounter"));       // Counter functionality (JavaScript variable)
    assert!(html.contains("addEventListener"));   // Event listeners for navigation
    assert!(html.contains("speaker-notes"));      // Notes styling
}
