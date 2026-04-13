//! Compatibility tests for PPTX files
//!
//! This module validates that generated PPTX files are compatible with:
//! - Microsoft Office 2007+
//! - LibreOffice Impress
//! - Google Slides
//!
//! Note: Full integration testing requires manual verification in the target applications.
//! These tests focus on structural validation and schema compliance.

use ppt_rs::generator::{SlideContent, create_pptx_with_content, create_pptx_to_writer, create_pptx_lazy_to_writer, LazySlideSource};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

/// Output directory for test files
const TEST_OUTPUT_DIR: &str = "test_output/compatibility";

/// Validate a PPTX file structure
pub struct PptxValidator {
    path: PathBuf,
}

impl PptxValidator {
    /// Create a new validator for a PPTX file
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Validate the ZIP structure
    pub fn validate_zip_structure(&self) -> Result<Vec<String>, String> {
        let file = File::open(&self.path)
            .map_err(|e| format!("Cannot open file: {}", e))?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| format!("Not a valid ZIP file: {}", e))?;

        let mut issues = Vec::new();
        let mut found_files = Vec::new();

        // Required root files
        let required_root = [
            "[Content_Types].xml",
            "_rels/.rels",
        ];

        // Required ppt files
        let required_ppt = [
            "ppt/presentation.xml",
            "ppt/_rels/presentation.xml.rels",
            "ppt/theme/theme1.xml",
            "ppt/slideLayouts/slideLayout1.xml",
            "ppt/slideLayouts/_rels/slideLayout1.xml.rels",
            "ppt/slideMasters/slideMaster1.xml",
            "ppt/slideMasters/_rels/slideMaster1.xml.rels",
        ];

        // Collect all files
        for i in 0..archive.len() {
            let file = archive.by_index(i)
                .map_err(|e| format!("Cannot read file index {}: {}", i, e))?;
            if !file.is_dir() {
                found_files.push(file.name().to_string());
            }
        }

        // Check required files
        for req in required_root.iter().chain(required_ppt.iter()) {
            if !found_files.iter().any(|f| f == req) {
                issues.push(format!("Missing required file: {}", req));
            }
        }

        // Check for at least one slide
        let has_slides = found_files.iter().any(|f| f.starts_with("ppt/slides/slide"));
        if !has_slides {
            issues.push("No slide files found".to_string());
        }

        if issues.is_empty() {
            Ok(found_files)
        } else {
            Err(issues.join("; "))
        }
    }

    /// Validate Content_Types.xml structure
    pub fn validate_content_types(&self) -> Result<(), String> {
        let file = File::open(&self.path)
            .map_err(|e| format!("Cannot open file: {}", e))?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| format!("Not a valid ZIP file: {}", e))?;

        let mut content_types = archive.by_name("[Content_Types].xml")
            .map_err(|_| "Content_Types.xml not found".to_string())?;

        let mut content = String::new();
        content_types.read_to_string(&mut content)
            .map_err(|e| format!("Cannot read Content_Types.xml: {}", e))?;

        // Basic XML validation
        if !content.contains("<Types") || !content.contains("</Types>") {
            return Err("Content_Types.xml missing Types element".to_string());
        }

        // Check for required content types
        let required_types = [
            "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml",
            "application/vnd.openxmlformats-officedocument.presentationml.slide+xml",
            "application/vnd.openxmlformats-officedocument.theme+xml",
        ];

        for content_type in required_types.iter() {
            if !content.contains(content_type) {
                return Err(format!("Missing content type: {}", content_type));
            }
        }

        Ok(())
    }

    /// Validate presentation XML structure
    pub fn validate_presentation(&self) -> Result<(), String> {
        let file = File::open(&self.path)
            .map_err(|e| format!("Cannot open file: {}", e))?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| format!("Not a valid ZIP file: {}", e))?;

        let mut presentation = archive.by_name("ppt/presentation.xml")
            .map_err(|_| "presentation.xml not found".to_string())?;

        let mut content = String::new();
        presentation.read_to_string(&mut content)
            .map_err(|e| format!("Cannot read presentation.xml: {}", e))?;

        // Check for required namespaces
        let required_namespaces = [
            "http://schemas.openxmlformats.org/presentationml/2006/main",
            "http://schemas.openxmlformats.org/drawingml/2006/main",
        ];

        for ns in required_namespaces.iter() {
            if !content.contains(ns) {
                return Err(format!("Missing namespace: {}", ns));
            }
        }

        // Check for presentation structure
        if !content.contains("<p:presentation") {
            return Err("Missing presentation element".to_string());
        }

        Ok(())
    }

    /// Get all slide file names
    pub fn get_slide_files(&self) -> Result<Vec<String>, String> {
        let file = File::open(&self.path)
            .map_err(|e| format!("Cannot open file: {}", e))?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| format!("Not a valid ZIP file: {}", e))?;

        let mut slides = Vec::new();
        for i in 0..archive.len() {
            let file = archive.by_index(i)
                .map_err(|e| format!("Cannot read file index {}: {}", i, e))?;
            let name = file.name().to_string();
            if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
                slides.push(name);
            }
        }

        slides.sort();
        Ok(slides)
    }

    /// Validate slide XML structure
    pub fn validate_slide(&self, slide_path: &str) -> Result<(), String> {
        let file = File::open(&self.path)
            .map_err(|e| format!("Cannot open file: {}", e))?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| format!("Not a valid ZIP file: {}", e))?;

        let mut slide = archive.by_name(slide_path)
            .map_err(|_| format!("Slide not found: {}", slide_path))?;

        let mut content = String::new();
        slide.read_to_string(&mut content)
            .map_err(|e| format!("Cannot read {}: {}", slide_path, e))?;

        // Check for required namespaces
        let required_namespaces = [
            "http://schemas.openxmlformats.org/presentationml/2006/main",
            "http://schemas.openxmlformats.org/drawingml/2006/main",
        ];

        for ns in required_namespaces.iter() {
            if !content.contains(ns) {
                return Err(format!("Slide {} missing namespace: {}", slide_path, ns));
            }
        }

        // Check for slide structure
        if !content.contains("<p:slide") && !content.contains("<p:spTree") {
            return Err(format!("Slide {} missing required elements", slide_path));
        }

        Ok(())
    }
}

/// Generate test PPTX files for manual compatibility testing
pub struct CompatibilityTestSuite {
    output_dir: PathBuf,
}

impl CompatibilityTestSuite {
    /// Create a new test suite
    pub fn new() -> Self {
        // Use a unique per-suite directory to avoid test parallelism races where one
        // test is validating a file while another test is still writing it.
        let output_dir = PathBuf::from(TEST_OUTPUT_DIR).join(uuid::Uuid::new_v4().to_string());
        fs::create_dir_all(&output_dir).ok();
        Self { output_dir }
    }

    /// Get the output directory
    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }

    /// Generate a basic presentation test file
    pub fn generate_basic_presentation(&self) -> Result<PathBuf, String> {
        let slides = vec![
            SlideContent::new("Title Slide")
                .add_bullet("First point")
                .add_bullet("Second point")
                .add_bullet("Third point"),
            SlideContent::new("Content Slide")
                .add_bullet("Item 1")
                .add_sub_bullet("Sub-item 1")
                .add_sub_bullet("Sub-item 2")
                .add_bullet("Item 2")
                .add_bullet("Item 3"),
        ];

        let data = create_pptx_with_content("Basic Compatibility Test", slides)
            .map_err(|e| format!("Failed to create presentation: {}", e))?;

        let path = self.output_dir.join("01_basic.pptx");
        let mut file = File::create(&path)
            .map_err(|e| format!("Cannot create file: {}", e))?;
        file.write_all(&data)
            .map_err(|e| format!("Cannot write file: {}", e))?;
        file.flush()
            .map_err(|e| format!("Cannot flush file: {}", e))?;

        Ok(path)
    }

    /// Generate a presentation with various shapes
    pub fn generate_shapes_presentation(&self) -> Result<PathBuf, String> {
        use ppt_rs::generator::{Shape, ShapeType, ShapeFill, ShapeLine};

        let mut slides = vec![
            SlideContent::new("Shapes Demo")
                .add_bullet("Rectangle shape")
                .add_bullet("Circle shape")
                .add_bullet("Triangle shape"),
        ];

        let shape1 = Shape::new(ShapeType::Rectangle, 1000000, 1000000, 2000000, 1000000)
            .with_fill(ShapeFill::new("FF0000"))
            .with_line(ShapeLine::new("000000", 12700));

        let shape2 = Shape::new(ShapeType::Circle, 4000000, 2000000, 1500000, 1500000)
            .with_fill(ShapeFill::new("00FF00"));

        let shape3 = Shape::new(ShapeType::Triangle, 1000000, 2500000, 1500000, 1500000)
            .with_fill(ShapeFill::new("0000FF"));

        slides[0] = slides[0].clone().with_shapes(vec![shape1, shape2, shape3]);

        let data = create_pptx_with_content("Shapes Compatibility Test", slides)
            .map_err(|e| format!("Failed to create presentation: {}", e))?;

        let path = self.output_dir.join("02_shapes.pptx");
        let mut file = File::create(&path)
            .map_err(|e| format!("Cannot create file: {}", e))?;
        file.write_all(&data)
            .map_err(|e| format!("Cannot write file: {}", e))?;

        Ok(path)
    }

    /// Generate a presentation with charts
    pub fn generate_chart_presentation(&self) -> Result<PathBuf, String> {
        use ppt_rs::generator::{ChartType, ChartSeries, ChartBuilder};

        let mut slides = vec![
            SlideContent::new("Chart Demo")
                .add_bullet("Bar chart visualization")
                .add_bullet("Multiple data series")
                .add_bullet("Labeled axes"),
        ];

        let chart = ChartBuilder::new("Sales Data", ChartType::Bar)
            .add_series(ChartSeries::new("Q1", vec![10.0, 20.0, 30.0]))
            .add_series(ChartSeries::new("Q2", vec![15.0, 25.0, 35.0]))
            .add_series(ChartSeries::new("Q3", vec![20.0, 30.0, 40.0]))
            .position(1000000, 1000000)
            .size(6000000, 4000000)
            .build();

        slides[0] = slides[0].clone().add_chart(chart);

        let data = create_pptx_with_content("Chart Compatibility Test", slides)
            .map_err(|e| format!("Failed to create presentation: {}", e))?;

        let path = self.output_dir.join("03_charts.pptx");
        let mut file = File::create(&path)
            .map_err(|e| format!("Cannot create file: {}", e))?;
        file.write_all(&data)
            .map_err(|e| format!("Cannot write file: {}", e))?;

        Ok(path)
    }

    /// Generate a presentation with images (placeholder - actual image data needed)
    pub fn generate_image_presentation(&self) -> Result<PathBuf, String> {
        let slides = vec![
            SlideContent::new("Image Demo")
                .add_bullet("Image support can be verified by rendering")
                .add_bullet("Insert actual images for full testing")
                .add_bullet("Supports PNG, JPG, and other formats"),
        ];

        let data = create_pptx_with_content("Image Compatibility Test", slides)
            .map_err(|e| format!("Failed to create presentation: {}", e))?;

        let path = self.output_dir.join("04_images.pptx");
        let mut file = File::create(&path)
            .map_err(|e| format!("Cannot create file: {}", e))?;
        file.write_all(&data)
            .map_err(|e| format!("Cannot write file: {}", e))?;

        Ok(path)
    }

    /// Generate a large presentation (for performance testing)
    pub fn generate_large_presentation(&self, slide_count: usize) -> Result<PathBuf, String> {
        let slides = (0..slide_count)
            .map(|i| SlideContent::new(&format!("Slide {}", i + 1))
                .add_bullet(&format!("Point {}", i + 1))
                .add_bullet(&format!("Point {}", i + 2))
                .add_bullet(&format!("Point {}", i + 3))
                .add_sub_bullet(&format!("Sub-point {}", i + 1)))
            .collect();

        let data = create_pptx_with_content(&format!("Large Presentation ({} slides)", slide_count), slides)
            .map_err(|e| format!("Failed to create presentation: {}", e))?;

        let path = self.output_dir.join(format!("05_large_{}slides.pptx", slide_count));
        let mut file = File::create(&path)
            .map_err(|e| format!("Cannot create file: {}", e))?;
        file.write_all(&data)
            .map_err(|e| format!("Cannot write file: {}", e))?;

        Ok(path)
    }

    /// Generate a presentation using streaming API
    pub fn generate_streaming_presentation(&self) -> Result<PathBuf, String> {
        let path = self.output_dir.join("06_streaming.pptx");
        let file = File::create(&path)
            .map_err(|e| format!("Cannot create file: {}", e))?;

        create_pptx_to_writer(file, "Streaming API Test", 10)
            .map_err(|e| format!("Failed to create presentation: {}", e))?;

        Ok(path)
    }

    /// Generate a presentation using lazy slide generation
    pub fn generate_lazy_presentation(&self) -> Result<PathBuf, String> {
        struct LazyTestSource { count: usize }
        impl LazySlideSource for LazyTestSource {
            fn slide_count(&self) -> usize { self.count }
            fn generate_slide(&self, index: usize) -> Option<SlideContent> {
                if index < self.count {
                    Some(SlideContent::new(&format!("Lazy Slide {}", index + 1))
                        .add_bullet(&format!("Content {}", index + 1))
                        .add_bullet(&format!("More content {}", index + 2)))
                } else {
                    None
                }
            }
        }

        let path = self.output_dir.join("07_lazy.pptx");
        let file = File::create(&path)
            .map_err(|e| format!("Cannot create file: {}", e))?;

        create_pptx_lazy_to_writer(file, "Lazy Loading Test", Box::new(LazyTestSource { count: 20 }), None)
            .map_err(|e| format!("Failed to create presentation: {}", e))?;

        Ok(path)
    }

    /// Generate all test files
    pub fn generate_all(&self) -> Result<Vec<PathBuf>, String> {
        let mut results = Vec::new();

        println!("Generating compatibility test files...");

        results.push(self.generate_basic_presentation()?);
        println!("✓ Basic presentation (3 slides)");

        results.push(self.generate_shapes_presentation()?);
        println!("✓ Shapes presentation");

        results.push(self.generate_chart_presentation()?);
        println!("✓ Chart presentation");

        results.push(self.generate_image_presentation()?);
        println!("✓ Image presentation (placeholder)");

        results.push(self.generate_large_presentation(100)?);
        println!("✓ Large presentation (100 slides)");

        results.push(self.generate_streaming_presentation()?);
        println!("✓ Streaming API presentation");

        results.push(self.generate_lazy_presentation()?);
        println!("✓ Lazy loading presentation (20 slides)");

        println!("\nGenerated {} test files in: {}", results.len(), self.output_dir.display());
        println!("\nTo verify compatibility:");
        println!("1. Microsoft PowerPoint 2007+ - Open each .pptx file and verify rendering");
        println!("2. LibreOffice Impress - Open each file and check for compatibility issues");
        println!("3. Google Slides - Upload each file to Google Drive and verify display");
        println!("4. Key areas to verify:");
        println!("   - Text rendering and fonts");
        println!("   - Shape colors and positions");
        println!("   - Chart data and labels");
        println!("   - Slide transitions");
        println!("   - File size and load time");

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_zip_structure() {
        let suite = CompatibilityTestSuite::new();
        let path = suite.generate_basic_presentation().unwrap();

        let validator = PptxValidator::new(&path);
        match validator.validate_zip_structure() {
            Ok(files) => println!("ZIP validation passed, found {} files", files.len()),
            Err(e) => panic!("ZIP validation failed: {}", e),
        }
        assert!(validator.validate_zip_structure().is_ok());
    }

    #[test]
    fn test_validate_content_types() {
        let suite = CompatibilityTestSuite::new();
        let path = suite.generate_streaming_presentation().unwrap();

        let validator = PptxValidator::new(&path);
        assert!(validator.validate_content_types().is_ok(),
            "Content types validation failed for: {:?}", path);
    }

    #[test]
    fn test_validate_presentation() {
        let suite = CompatibilityTestSuite::new();
        let path = suite.generate_chart_presentation().unwrap();

        let validator = PptxValidator::new(&path);
        assert!(validator.validate_presentation().is_ok(),
            "Presentation validation failed for: {:?}", path);
    }

    #[test]
    fn test_get_slide_files() {
        let suite = CompatibilityTestSuite::new();
        let path = suite.generate_lazy_presentation().unwrap();

        let validator = PptxValidator::new(&path);
        validator.validate_zip_structure().unwrap();

        let slides = validator.get_slide_files().unwrap();

        assert_eq!(slides.len(), 20);

        // Check first and last slides exist (note: slides are sorted alphabetically)
        assert!(slides.iter().any(|s| s.contains("slide1.xml")));
        assert!(slides.iter().any(|s| s.contains("slide20.xml")));
    }

    #[test]
    fn test_validate_slides() {
        let suite = CompatibilityTestSuite::new();
        let path = suite.generate_shapes_presentation().unwrap();

        let validator = PptxValidator::new(&path);
        let slides = validator.get_slide_files().unwrap();

        assert_eq!(slides.len(), 1);
        for slide_path in slides {
            assert!(validator.validate_slide(&slide_path).is_ok(),
                "Failed to validate: {}", slide_path);
        }
    }

    #[test]
    fn test_compatibility_suite() {
        let suite = CompatibilityTestSuite::new();
        let paths = suite.generate_all().unwrap();

        assert_eq!(paths.len(), 7);

        // Validate each generated file
        for path in &paths {
            let validator = PptxValidator::new(path);
            assert!(validator.validate_zip_structure().is_ok(),
                "ZIP structure validation failed for: {:?}", path);
            assert!(validator.validate_content_types().is_ok(),
                "Content Types validation failed for: {:?}", path);
            assert!(validator.validate_presentation().is_ok(),
                "Presentation validation failed for: {:?}", path);
        }
    }
}
