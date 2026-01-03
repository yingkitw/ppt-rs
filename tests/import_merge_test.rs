use ppt_rs::api::Presentation;
use ppt_rs::generator::SlideContent;
use ppt_rs::import::import_pptx;
use std::path::Path;

#[test]
fn test_import_and_merge() {
    let source_path = "tests/test_source.pptx";
    let merged_path = "tests/test_merged.pptx";
    
    // 1. Create source PPTX
    let pres = Presentation::with_title("Source Presentation")
        .add_slide(SlideContent::new("Slide 1").add_bullet("Point 1"))
        .add_slide(SlideContent::new("Slide 2").add_bullet("Point 2"));
        
    pres.save(source_path).expect("Failed to save source PPTX");
    assert!(Path::new(source_path).exists());
    
    // 2. Import it
    let mut imported_pres = import_pptx(source_path).expect("Failed to import PPTX");
    
    // 3. Verify content
    assert_eq!(imported_pres.get_title(), "Source Presentation");
    assert_eq!(imported_pres.slide_count(), 2);
    
    let slides = imported_pres.slides();
    assert_eq!(slides[0].title, "Slide 1");
    assert_eq!(slides[0].content[0], "Point 1");
    assert_eq!(slides[1].title, "Slide 2");
    
    // 4. Merge (add new slides)
    imported_pres = imported_pres.add_slide(SlideContent::new("Slide 3 (Merged)").add_bullet("Point 3"));
    
    // 5. Save merged
    imported_pres.save(merged_path).expect("Failed to save merged PPTX");
    assert!(Path::new(merged_path).exists());
    
    // 6. Verify merged (by importing again)
    let reimported = import_pptx(merged_path).expect("Failed to import merged PPTX");
    assert_eq!(reimported.slide_count(), 3);
    assert_eq!(reimported.slides()[2].title, "Slide 3 (Merged)");
    
    // Cleanup
    std::fs::remove_file(source_path).ok();
    std::fs::remove_file(merged_path).ok();
}
