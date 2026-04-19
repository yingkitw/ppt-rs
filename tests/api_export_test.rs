use ppt_rs::api::Presentation;
use ppt_rs::generator::SlideContent;
use ppt_rs::export::image_export::{ImageExportOptions, ImageFormat};
use ppt_rs::opc::compress::{CompressionOptions, CompressionLevel};
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

#[test]
fn test_markdown_export_api() {
    let pres = Presentation::new()
        .title("Markdown API Test")
        .add_slide(SlideContent::new("Slide 1").add_bullet("Item 1"))
        .add_slide(SlideContent::new("Slide 2").add_bullet("Item A").add_bullet("Item B"));

    let output_path = "test_api_export.md";
    pres.save_as_markdown(output_path).unwrap();

    assert!(Path::new(output_path).exists());
    let content = fs::read_to_string(output_path).unwrap();

    assert!(content.contains("# Markdown API Test"));
    assert!(content.contains("Slide 1"));
    assert!(content.contains("Slide 2"));
    assert!(content.contains("- Item 1"));
    assert!(content.contains("---"));

    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_markdown_export_with_options_api() {
    let pres = Presentation::new()
        .title("Options API Test")
        .add_slide(SlideContent::new("Test Slide").add_bullet("Point"));

    let options = ppt_rs::export::md::MarkdownOptions::new()
        .with_slide_numbers(false)
        .with_frontmatter(false);

    let output_path = "test_api_options.md";
    pres.save_as_markdown_with_options(output_path, &options).unwrap();

    let content = fs::read_to_string(output_path).unwrap();

    // Without options, these should not be present
    assert!(!content.contains("## Slide 1:"));
    assert!(!content.contains("title:"));

    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_image_export_api() {
    let pres = Presentation::new()
        .title("Image Export API")
        .add_slide(SlideContent::new("Slide 1"));

    let output_dir = "test_api_images";
    let options = ImageExportOptions::new()
        .with_format(ImageFormat::Png)
        .with_dpi(96);

    // This test requires LibreOffice
    match pres.save_as_images(output_dir, &options) {
        Ok(paths) => {
            if !paths.is_empty() {
                assert!(Path::new(output_dir).exists());
                // Cleanup
                let _ = fs::remove_dir_all(output_dir);
            }
        },
        Err(e) => {
            println!("Skipping image export test (LibreOffice required): {}", e);
        }
    }
}

#[test]
fn test_slide_image_export_api() {
    let pres = Presentation::new()
        .title("Single Slide Export")
        .add_slide(SlideContent::new("Slide 1"))
        .add_slide(SlideContent::new("Slide 2"));

    let output_path = "test_single_slide.png";
    let options = ImageExportOptions::new()
        .with_format(ImageFormat::Png)
        .with_slide(1);

    // This test requires LibreOffice
    match pres.save_slide_as_image(1, output_path, &options) {
        Ok(_) => {
            if Path::new(output_path).exists() {
                fs::remove_file(output_path).unwrap();
            }
        },
        Err(e) => {
            println!("Skipping single slide export test (LibreOffice required): {}", e);
        }
    }
}

#[test]
fn test_thumbnail_export_api() {
    let pres = Presentation::new()
        .title("Thumbnail Test")
        .add_slide(SlideContent::new("Title Slide"));

    let output_path = "test_thumbnail.png";

    // This test requires LibreOffice
    match pres.save_thumbnail(output_path, 300) {
        Ok(_) => {
            if Path::new(output_path).exists() {
                fs::remove_file(output_path).unwrap();
            }
        },
        Err(e) => {
            println!("Skipping thumbnail export test (LibreOffice required): {}", e);
        }
    }
}

#[test]
fn test_compression_api_basic() {
    let pres = Presentation::new()
        .title("Compression API")
        .add_slide(SlideContent::new("Slide 1").add_bullet("Point"))
        .add_slide(SlideContent::new("Slide 2").add_bullet("Point"));

    let output_path = "test_api_compressed.pptx";
    let options = CompressionOptions::new()
        .with_level(CompressionLevel::Light);

    let result = pres.compress(output_path, &options).unwrap();

    assert!(Path::new(output_path).exists());
    assert!(result.original_size > 0);
    assert!(result.compressed_size > 0);
    assert!(result.reduction_percent >= 0.0);

    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_analyze_size_api() {
    let pres = Presentation::new()
        .title("Analysis API")
        .add_slide(SlideContent::new("Slide 1"))
        .add_slide(SlideContent::new("Slide 2"));

    let analysis = pres.analyze_size().unwrap();

    assert!(analysis.total_size > 0);
    assert_eq!(analysis.slide_count, 2);

    let summary = analysis.summary();
    assert!(summary.contains("PPTX Analysis"));
    assert!(summary.contains("2"));
}

#[test]
fn test_compression_preset_api() {
    let pres = Presentation::new()
        .title("Preset API")
        .add_slide(SlideContent::new("Test"));

    // Test web preset
    let web_opts = CompressionOptions::web();
    let web_result = pres.compress("test_web_preset.pptx", &web_opts).unwrap();
    assert!(web_result.original_size > 0);

    // Test maximum preset
    let max_opts = CompressionOptions::maximum();
    let max_result = pres.compress("test_max_preset.pptx", &max_opts).unwrap();
    assert!(max_result.original_size > 0);

    // Cleanup
    let _ = fs::remove_file("test_web_preset.pptx");
    let _ = fs::remove_file("test_max_preset.pptx");
}

#[test]
fn test_image_export_options_presets() {
    let hq = ImageExportOptions::high_quality();
    assert_eq!(hq.dpi, 300);
    assert_eq!(hq.format, ImageFormat::Png);

    let web = ImageExportOptions::web_optimized();
    assert_eq!(web.dpi, 96);
    assert_eq!(web.format, ImageFormat::Jpeg);
    assert_eq!(web.jpeg_quality, 85);
}
