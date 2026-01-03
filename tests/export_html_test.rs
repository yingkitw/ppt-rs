use ppt_rs::api::Presentation;
use ppt_rs::generator::SlideContent;
use ppt_rs::export::html::export_to_html;

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
