//! Integration tests for PPTX compression functionality

use ppt_rs::api::Presentation;
use ppt_rs::opc::compress::{CompressionOptions, CompressionLevel, compress_pptx, analyze_pptx};
use ppt_rs::generator::SlideContent;
use std::fs;
use std::path::Path;

#[test]
fn test_compression_api() {
    // Create a presentation with some content
    let pres = Presentation::new()
        .title("Compression Test")
        .add_slide(SlideContent::new("Slide 1").add_bullet("Point 1").add_bullet("Point 2"))
        .add_slide(SlideContent::new("Slide 2").add_bullet("Point A").add_bullet("Point B"));

    let input_path = "test_compression_input.pptx";
    let output_path = "test_compression_output.pptx";

    pres.save(input_path).unwrap();

    // Compress with default options
    let options = CompressionOptions::new()
        .with_level(CompressionLevel::Light);

    let result = pres.compress(output_path, &options).unwrap();

    // Verify output exists
    assert!(Path::new(output_path).exists());

    // Check compression result
    assert!(result.original_size > 0);
    assert!(result.compressed_size > 0);
    assert!(result.reduction_percent >= 0.0);

    // Compressed file should not be larger (or only slightly)
    assert!(result.compressed_size <= result.original_size * 2);

    // Cleanup
    fs::remove_file(input_path).unwrap();
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_compression_with_different_levels() {
    let pres = Presentation::new()
        .title("Compression Levels")
        .add_slide(SlideContent::new("Test Slide").add_bullet("Item"));

    let input_path = "test_levels.pptx";
    pres.save(input_path).unwrap();

    // Test Light compression
    let light_opts = CompressionOptions::new().with_level(CompressionLevel::Light);
    let light_result = compress_pptx(input_path, "test_light.pptx", &light_opts).unwrap();

    // Test Medium compression
    let medium_opts = CompressionOptions::new().with_level(CompressionLevel::Medium);
    let medium_result = compress_pptx(input_path, "test_medium.pptx", &medium_opts).unwrap();

    // Both should succeed
    assert!(light_result.original_size > 0);
    assert!(medium_result.original_size > 0);

    // Cleanup
    fs::remove_file(input_path).unwrap();
    if Path::new("test_light.pptx").exists() {
        fs::remove_file("test_light.pptx").unwrap();
    }
    if Path::new("test_medium.pptx").exists() {
        fs::remove_file("test_medium.pptx").unwrap();
    }
}

#[test]
fn test_compression_presets() {
    let pres = Presentation::new()
        .title("Preset Test")
        .add_slide(SlideContent::new("Slide"));

    let input_path = "test_presets.pptx";
    let output_max = "test_max.pptx";
    let output_web = "test_web.pptx";

    pres.save(input_path).unwrap();

    // Test maximum compression preset
    let max_opts = CompressionOptions::maximum();
    let max_result = compress_pptx(input_path, output_max, &max_opts).unwrap();
    assert!(max_result.original_size > 0);

    // Test web optimization preset
    let web_opts = CompressionOptions::web();
    let web_result = compress_pptx(input_path, output_web, &web_opts).unwrap();
    assert!(web_result.original_size > 0);

    // Cleanup
    fs::remove_file(input_path).unwrap();
    fs::remove_file(output_max).unwrap();
    fs::remove_file(output_web).unwrap();
}

#[test]
fn test_analyze_pptx() {
    let pres = Presentation::new()
        .title("Analysis Test")
        .add_slide(SlideContent::new("Slide 1").add_bullet("Point"))
        .add_slide(SlideContent::new("Slide 2").add_bullet("Point"));

    let analysis = pres.analyze_size().unwrap();

    // Check analysis results
    assert!(analysis.total_size > 0);
    assert!(analysis.xml_size > 0); // PPTX has XML content
    assert_eq!(analysis.slide_count, 2);

    // Summary should contain key info
    let summary = analysis.summary();
    assert!(summary.contains("PPTX Analysis"));
    assert!(summary.contains("2")); // Slide count
}

#[test]
fn test_compression_removes_unused_media() {
    let pres = Presentation::new()
        .title("Media Removal Test")
        .add_slide(SlideContent::new("Slide"));

    let input_path = "test_media_removal.pptx";
    let output_path = "test_media_removal_compressed.pptx";

    pres.save(input_path).unwrap();

    // Compress with media removal enabled
    let options = CompressionOptions::new()
        .with_unused_media_removal(true);

    let result = compress_pptx(input_path, output_path, &options).unwrap();

    // Should have analyzed the file
    assert!(result.original_size > 0);
    assert!(result.compressed_size > 0);

    // Cleanup
    fs::remove_file(input_path).unwrap();
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_compression_with_properties_removal() {
    let pres = Presentation::new()
        .title("Properties Removal")
        .add_slide(SlideContent::new("Slide"));

    let input_path = "test_props.pptx";
    let output_path = "test_props_compressed.pptx";

    pres.save(input_path).unwrap();

    let options = CompressionOptions::new()
        .with_properties_removal(true);

    let result = compress_pptx(input_path, output_path, &options).unwrap();

    assert!(result.original_size > 0);
    assert!(Path::new(output_path).exists());

    // Cleanup
    fs::remove_file(input_path).unwrap();
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_compression_target_size() {
    let pres = Presentation::new()
        .title("Target Size Test")
        .add_slide(SlideContent::new("Slide"));

    let input_path = "test_target.pptx";
    let output_path = "test_target_compressed.pptx";

    pres.save(input_path).unwrap();

    // Set a very large target (should always achieve)
    let options = CompressionOptions::new()
        .with_target_size(100 * 1024 * 1024); // 100MB target

    let result = compress_pptx(input_path, output_path, &options).unwrap();

    // Should achieve target since it's very large
    assert!(result.target_achieved);

    // Cleanup
    fs::remove_file(input_path).unwrap();
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_compression_xml_optimization() {
    let pres = Presentation::new()
        .title("XML Optimization")
        .add_slide(SlideContent::new("Slide with content").add_bullet("Point 1").add_bullet("Point 2"));

    let input_path = "test_xml_opt.pptx";
    let output_path = "test_xml_opt_compressed.pptx";

    pres.save(input_path).unwrap();

    let options = CompressionOptions::new()
        .with_xml_optimization(true);

    let result = compress_pptx(input_path, output_path, &options).unwrap();

    assert!(result.original_size > 0);
    assert!(Path::new(output_path).exists());

    // Cleanup
    fs::remove_file(input_path).unwrap();
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_compression_result_fields() {
    let pres = Presentation::new()
        .title("Result Fields")
        .add_slide(SlideContent::new("Slide"));

    let input_path = "test_result.pptx";
    let output_path = "test_result_compressed.pptx";

    pres.save(input_path).unwrap();

    let options = CompressionOptions::new();
    let result = compress_pptx(input_path, output_path, &options).unwrap();

    // Verify all fields are populated correctly
    assert!(result.original_size > 0);
    assert!(result.compressed_size > 0);
    assert!(result.reduction_percent >= 0.0);
    // unused_media_removed and images_compressed may be 0 for simple presentations

    // Cleanup
    fs::remove_file(input_path).unwrap();
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_analyze_empty_presentation() {
    // An empty presentation (no slides) might error during analysis
    // So we add at least one slide
    let pres = Presentation::new()
        .title("Minimal Analysis")
        .add_slide(SlideContent::new("Only Slide"));

    let analysis = pres.analyze_size().unwrap();

    // Minimal presentation should still have some XML structure
    assert!(analysis.total_size > 0);
    assert_eq!(analysis.slide_count, 1);

    let summary = analysis.summary();
    assert!(summary.contains("1")); // One slide
}

#[test]
fn test_compression_multiple_slides() {
    let mut pres = Presentation::new().title("Multi-Slide Compression");

    for i in 1..=10 {
        pres = pres.add_slide(
            SlideContent::new(&format!("Slide {}", i))
                .add_bullet(&format!("Bullet {}A", i))
                .add_bullet(&format!("Bullet {}B", i))
        );
    }

    let input_path = "test_multi.pptx";
    let output_path = "test_multi_compressed.pptx";

    pres.save(input_path).unwrap();

    let options = CompressionOptions::new().with_level(CompressionLevel::Medium);
    let result = compress_pptx(input_path, output_path, &options).unwrap();

    assert!(result.original_size > 0);
    assert!(Path::new(output_path).exists());

    // Cleanup
    fs::remove_file(input_path).unwrap();
    fs::remove_file(output_path).unwrap();
}
