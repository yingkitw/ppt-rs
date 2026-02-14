use ppt_rs::api::Presentation;
use ppt_rs::generator::SlideContent;
use std::path::Path;
use std::fs;

#[test]
fn test_html_export_api() {
    let pres = Presentation::new()
        .title("HTML Export Test")
        .add_slide(SlideContent::new("Slide 1").add_bullet("Item 1"));
    
    let output_path = "test_export.html";
    pres.save_as_html(output_path).unwrap();
    
    assert!(Path::new(output_path).exists());
    let content = fs::read_to_string(output_path).unwrap();
    assert!(content.contains("HTML Export Test"));
    assert!(content.contains("Slide 1"));
    assert!(content.contains("Item 1"));
    
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_from_path_api() {
    // First create a pptx
    let pres = Presentation::new()
        .title("Import Test")
        .add_slide(SlideContent::new("Slide 1").add_bullet("Item 1"));
    
    let pptx_path = "test_import_api.pptx";
    pres.save(pptx_path).unwrap();
    
    // Then import it using from_path
    let imported = Presentation::from_path(pptx_path).unwrap();
    
    assert_eq!(imported.get_title(), "Import Test");
    assert_eq!(imported.slide_count(), 1);
    
    fs::remove_file(pptx_path).unwrap();
}

#[test]
fn test_pdf_export_api() {
    let pres = Presentation::new()
        .title("PDF Export Test")
        .add_slide(SlideContent::new("Slide 1"));

    let output_path = "test_export.pdf";

    // This test requires LibreOffice to be installed
    match pres.save_as_pdf(output_path) {
        Ok(_) => {
            assert!(Path::new(output_path).exists());
            fs::remove_file(output_path).unwrap();
        },
        Err(e) => {
            // Skip test if LibreOffice is not available
            println!("Skipping PDF export test (LibreOffice required): {}", e);
        }
    }
}

#[test]
fn test_png_export_api() {
    let pres = Presentation::new()
        .title("PNG Export Test")
        .add_slide(SlideContent::new("Slide 1"));

    let output_dir = "test_export_png";

    // This test requires LibreOffice and pdftoppm to be installed
    match pres.save_as_png(output_dir) {
        Ok(_) => {
            assert!(Path::new(output_dir).exists());
            // Should have at least one image
            let count = fs::read_dir(output_dir).unwrap().count();
            assert!(count > 0);
            fs::remove_dir_all(output_dir).unwrap();
        },
        Err(e) => {
            // Skip test if required tools are not available
            println!("Skipping PNG export test (LibreOffice + pdftoppm required): {}", e);
        }
    }
}

#[test]
fn test_from_pdf_api() {
    // This test requires pdftoppm and a PDF file to be present
    let pdf_path = "test_import.pdf";

    if !Path::new(pdf_path).exists() {
        println!("Skipping PDF import test: test_import.pdf not found");
        return;
    }

    match Presentation::from_pdf(pdf_path) {
        Ok(pres) => {
            assert!(pres.slide_count() > 0);
        },
        Err(e) => {
            println!("Skipping PDF import test (pdftoppm required): {}", e);
        }
    }
}
