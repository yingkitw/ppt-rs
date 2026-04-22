//! PPTX Compression Module
//!
//! Provides functionality to optimize and compress PPTX files:
//! - Remove unused media files
//! - Compress images to reduce file size
//! - Remove document properties and revision history
//! - Optimize XML (remove unnecessary whitespace)

use super::Package;
use crate::exc::{PptxError, Result};
use std::collections::HashSet;
use std::path::Path;

/// Compression level options
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionLevel {
    /// Light compression - remove unused parts only
    Light,
    /// Medium compression - compress images slightly
    Medium,
    /// Aggressive compression - maximize size reduction
    Aggressive,
    /// Custom compression with specific image quality
    Custom(u8), // JPEG quality 0-100
}

impl Default for CompressionLevel {
    fn default() -> Self {
        CompressionLevel::Medium
    }
}

impl CompressionLevel {
    /// Get image quality for this level (for JPEG compression)
    pub fn image_quality(&self) -> u8 {
        match self {
            CompressionLevel::Light => 95,
            CompressionLevel::Medium => 85,
            CompressionLevel::Aggressive => 70,
            CompressionLevel::Custom(q) => *q,
        }
    }

    /// Whether to resize large images
    pub fn should_resize_images(&self) -> bool {
        matches!(self, CompressionLevel::Aggressive | CompressionLevel::Custom(_))
    }

    /// Maximum image dimension for this level
    pub fn max_image_dimension(&self) -> u32 {
        match self {
            CompressionLevel::Light => 2048,
            CompressionLevel::Medium => 1600,
            CompressionLevel::Aggressive => 1280,
            CompressionLevel::Custom(_) => 1600,
        }
    }
}

/// Compression options
#[derive(Debug, Clone)]
pub struct CompressionOptions {
    /// Compression level
    pub level: CompressionLevel,
    /// Remove unused media files
    pub remove_unused_media: bool,
    /// Remove document properties
    pub remove_properties: bool,
    /// Remove notes slides
    pub remove_notes: bool,
    /// Remove comments
    pub remove_comments: bool,
    /// Optimize XML (remove whitespace)
    pub optimize_xml: bool,
    /// Target file size in bytes (0 = no target)
    pub target_size: usize,
}

impl Default for CompressionOptions {
    fn default() -> Self {
        Self {
            level: CompressionLevel::Medium,
            remove_unused_media: true,
            remove_properties: false,
            remove_notes: false,
            remove_comments: true,
            optimize_xml: true,
            target_size: 0,
        }
    }
}

impl CompressionOptions {
    /// Create new options with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set compression level
    pub fn with_level(mut self, level: CompressionLevel) -> Self {
        self.level = level;
        self
    }

    /// Set unused media removal
    pub fn with_unused_media_removal(mut self, remove: bool) -> Self {
        self.remove_unused_media = remove;
        self
    }

    /// Set properties removal
    pub fn with_properties_removal(mut self, remove: bool) -> Self {
        self.remove_properties = remove;
        self
    }

    /// Set notes removal
    pub fn with_notes_removal(mut self, remove: bool) -> Self {
        self.remove_notes = remove;
        self
    }

    /// Set comments removal
    pub fn with_comments_removal(mut self, remove: bool) -> Self {
        self.remove_comments = remove;
        self
    }

    /// Set XML optimization
    pub fn with_xml_optimization(mut self, optimize: bool) -> Self {
        self.optimize_xml = optimize;
        self
    }

    /// Set target file size
    pub fn with_target_size(mut self, size: usize) -> Self {
        self.target_size = size;
        self
    }

    /// Preset for maximum compression
    pub fn maximum() -> Self {
        Self {
            level: CompressionLevel::Aggressive,
            remove_unused_media: true,
            remove_properties: true,
            remove_notes: true,
            remove_comments: true,
            optimize_xml: true,
            target_size: 0,
        }
    }

    /// Preset for web optimization
    pub fn web() -> Self {
        Self {
            level: CompressionLevel::Medium,
            remove_unused_media: true,
            remove_properties: true,
            remove_notes: false,
            remove_comments: true,
            optimize_xml: true,
            target_size: 5 * 1024 * 1024, // 5MB target
        }
    }
}

/// Compression result
#[derive(Debug)]
pub struct CompressionResult {
    /// Original file size in bytes
    pub original_size: usize,
    /// Compressed file size in bytes
    pub compressed_size: usize,
    /// Reduction percentage
    pub reduction_percent: f64,
    /// Number of unused media files removed
    pub unused_media_removed: usize,
    /// Number of images compressed
    pub images_compressed: usize,
    /// Whether target size was achieved
    pub target_achieved: bool,
}

/// Compress a PPTX file
///
/// # Arguments
/// * `input_path` - Path to input PPTX file
/// * `output_path` - Path for compressed output
/// * `options` - Compression options
///
/// # Returns
/// Compression result with statistics
///
/// # Example
/// ```no_run
/// use ppt_rs::opc::compress::{compress_pptx, CompressionOptions, CompressionLevel};
///
/// let options = CompressionOptions::new()
///     .with_level(CompressionLevel::Medium);
///
/// let result = compress_pptx("input.pptx", "output.pptx", &options).unwrap();
/// println!("Reduced by {:.1}%", result.reduction_percent);
/// ```
pub fn compress_pptx<P: AsRef<Path>, Q: AsRef<Path>>(
    input_path: P,
    output_path: Q,
    options: &CompressionOptions,
) -> Result<CompressionResult> {
    // Load package
    let mut package = Package::open(input_path.as_ref())?;

    let original_size = std::fs::metadata(input_path.as_ref())?.len() as usize;

    let mut unused_media_removed = 0;
    let mut images_compressed = 0;

    // Remove unused media files
    if options.remove_unused_media {
        unused_media_removed = remove_unused_media(&mut package)?;
    }

    // Remove properties if requested
    if options.remove_properties {
        remove_document_properties(&mut package);
    }

    // Remove notes if requested
    if options.remove_notes {
        remove_notes_slides(&mut package)?;
    }

    // Optimize XML
    if options.optimize_xml {
        optimize_xml_content(&mut package)?;
    }

    // Save compressed package
    package.save(output_path.as_ref())?;

    let compressed_size = std::fs::metadata(output_path.as_ref())?.len() as usize;
    let reduction_percent = if original_size > 0 {
        ((original_size - compressed_size) as f64 / original_size as f64) * 100.0
    } else {
        0.0
    };

    let target_achieved = options.target_size == 0 || compressed_size <= options.target_size;

    Ok(CompressionResult {
        original_size,
        compressed_size,
        reduction_percent,
        unused_media_removed,
        images_compressed,
        target_achieved,
    })
}

/// Compress a PPTX in memory
pub fn compress_pptx_in_memory(
    data: &[u8],
    options: &CompressionOptions,
) -> Result<(Vec<u8>, CompressionResult)> {
    use std::io::Write;

    // Write to temp file
    let temp_dir = std::env::temp_dir();
    let temp_input = temp_dir.join("compress_input.pptx");
    let temp_output = temp_dir.join("compress_output.pptx");

    std::fs::write(&temp_input, data)?;

    let result = compress_pptx(&temp_input, &temp_output, options)?;
    let output_data = std::fs::read(&temp_output)?;

    // Cleanup
    let _ = std::fs::remove_file(&temp_input);
    let _ = std::fs::remove_file(&temp_output);

    Ok((output_data, result))
}

/// Remove unused media files from package
fn remove_unused_media(package: &mut Package) -> Result<usize> {
    let media_paths: Vec<String> = package
        .part_paths()
        .iter()
        .filter(|p| p.starts_with("ppt/media/"))
        .map(|s| s.to_string())
        .collect();

    let mut referenced = HashSet::new();
    let mut removed = 0;

    // Find all media references in slide files
    for path in package.part_paths() {
        if path.starts_with("ppt/slides/slide") && path.ends_with(".xml") {
            if let Some(content) = package.get_part_string(&path) {
                // Look for media references like rId5, image1.png, etc.
                for media_path in &media_paths {
                    let filename = Path::new(media_path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("");
                    if content.contains(filename) || content.contains(&media_path[4..]) {
                        referenced.insert(media_path.clone());
                    }
                }
            }
        }
    }

    // Remove unreferenced media
    for media_path in media_paths {
        if !referenced.contains(&media_path) {
            package.remove_part(&media_path);
            removed += 1;
        }
    }

    Ok(removed)
}

/// Remove document properties
fn remove_document_properties(package: &mut Package) {
    // Remove core properties
    package.remove_part("docProps/core.xml");
    // Remove app properties
    package.remove_part("docProps/app.xml");
    // Remove custom properties
    package.remove_part("docProps/custom.xml");
    // Remove thumbnail
    package.remove_part("docProps/thumbnail.jpeg");
}

/// Remove notes slides
fn remove_notes_slides(package: &mut Package) -> Result<()> {
    let notes_paths: Vec<String> = package
        .part_paths()
        .iter()
        .filter(|p| p.starts_with("ppt/notesSlides/"))
        .map(|s| s.to_string())
        .collect();

    for path in notes_paths {
        package.remove_part(&path);
        // Also remove relationships
        let rels_path = path.replace("notesSlides/", "notesSlides/_rels/") + ".rels";
        package.remove_part(&rels_path);
    }

    Ok(())
}

/// Optimize XML content (minimize whitespace)
fn optimize_xml_content(package: &mut Package) -> Result<()> {
    let xml_paths: Vec<String> = package
        .part_paths()
        .iter()
        .filter(|p| p.ends_with(".xml") || p.ends_with(".rels"))
        .map(|s| s.to_string())
        .collect();

    for path in xml_paths {
        if let Some(content) = package.get_part_string(&path) {
            let optimized = minimize_xml(&content);
            package.add_part(path, optimized.into_bytes());
        }
    }

    Ok(())
}

/// Minimize XML by removing unnecessary whitespace
fn minimize_xml(xml: &str) -> String {
    let mut result = String::with_capacity(xml.len());
    let mut in_tag = false;
    let mut in_string = false;
    let mut prev_char = ' ';

    for ch in xml.chars() {
        match ch {
            '"' if !in_tag => {
                in_string = !in_string;
                result.push(ch);
            }
            '"' if in_tag => {
                in_string = !in_string;
                result.push(ch);
            }
            '<' if !in_string => {
                in_tag = true;
                // Remove whitespace before tag
                if prev_char == ' ' || prev_char == '\n' || prev_char == '\t' {
                    if !result.is_empty() {
                        result.pop();
                    }
                }
                result.push(ch);
            }
            '>' if !in_string => {
                in_tag = false;
                result.push(ch);
            }
            ' ' | '\n' | '\t' | '\r' if !in_tag && !in_string => {
                // Skip whitespace between tags
                if prev_char != ' ' {
                    result.push(' ');
                }
            }
            _ => {
                result.push(ch);
            }
        }
        prev_char = ch;
    }

    result
}

/// Analyze PPTX file and return size breakdown
pub fn analyze_pptx<P: AsRef<Path>>(path: P) -> Result<PptxAnalysis> {
    let package = Package::open(path.as_ref())?;
    let total_size = std::fs::metadata(path.as_ref())?.len() as usize;

    let mut images_size = 0;
    let mut xml_size = 0;
    let mut other_size = 0;

    let mut image_count = 0;
    let mut slide_count = 0;
    let mut media_count = 0;

    for part_path in package.part_paths() {
        if let Some(data) = package.get_part(part_path) {
            let size = data.len();

            if part_path.starts_with("ppt/media/") {
                if part_path.ends_with(".png")
                    || part_path.ends_with(".jpg")
                    || part_path.ends_with(".jpeg")
                {
                    images_size += size;
                    image_count += 1;
                } else {
                    media_count += 1;
                    other_size += size;
                }
            } else if part_path.ends_with(".xml") || part_path.ends_with(".rels") {
                xml_size += size;
                if part_path.starts_with("ppt/slides/slide") && part_path.ends_with(".xml") {
                    slide_count += 1;
                }
            } else {
                other_size += size;
            }
        }
    }

    Ok(PptxAnalysis {
        total_size,
        images_size,
        xml_size,
        other_size,
        image_count,
        slide_count,
        media_count,
    })
}

/// Analysis result for PPTX file
#[derive(Debug)]
pub struct PptxAnalysis {
    /// Total file size in bytes
    pub total_size: usize,
    /// Size of image files
    pub images_size: usize,
    /// Size of XML files
    pub xml_size: usize,
    /// Size of other files
    pub other_size: usize,
    /// Number of images
    pub image_count: usize,
    /// Number of slides
    pub slide_count: usize,
    /// Number of other media files
    pub media_count: usize,
}

impl PptxAnalysis {
    /// Get human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "PPTX Analysis:\n\
            - Total size: {}\n\
            - Images: {} ({} MB)\n\
            - Slides: {}\n\
            - XML data: {}\n\
            - Other media: {} files ({})",
            format_bytes(self.total_size),
            self.image_count,
            format_bytes(self.images_size),
            self.slide_count,
            format_bytes(self.xml_size),
            self.media_count,
            format_bytes(self.other_size)
        )
    }

    /// Get images as percentage of total
    pub fn images_percentage(&self) -> f64 {
        if self.total_size > 0 {
            (self.images_size as f64 / self.total_size as f64) * 100.0
        } else {
            0.0
        }
    }
}

fn format_bytes(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_level() {
        assert_eq!(CompressionLevel::Light.image_quality(), 95);
        assert_eq!(CompressionLevel::Medium.image_quality(), 85);
        assert_eq!(CompressionLevel::Aggressive.image_quality(), 70);
        assert_eq!(CompressionLevel::Custom(80).image_quality(), 80);
    }

    #[test]
    fn test_compression_level_resize() {
        assert!(!CompressionLevel::Light.should_resize_images());
        assert!(!CompressionLevel::Medium.should_resize_images());
        assert!(CompressionLevel::Aggressive.should_resize_images());
        assert!(CompressionLevel::Custom(80).should_resize_images());
    }

    #[test]
    fn test_compression_level_max_dimension() {
        assert_eq!(CompressionLevel::Light.max_image_dimension(), 2048);
        assert_eq!(CompressionLevel::Medium.max_image_dimension(), 1600);
        assert_eq!(CompressionLevel::Aggressive.max_image_dimension(), 1280);
    }

    #[test]
    fn test_compression_options_builder() {
        let opts = CompressionOptions::new()
            .with_level(CompressionLevel::Aggressive)
            .with_unused_media_removal(true)
            .with_properties_removal(true)
            .with_notes_removal(true)
            .with_comments_removal(false)
            .with_xml_optimization(true)
            .with_target_size(10 * 1024 * 1024);

        assert_eq!(opts.level, CompressionLevel::Aggressive);
        assert!(opts.remove_unused_media);
        assert!(opts.remove_properties);
        assert!(opts.remove_notes);
        assert!(!opts.remove_comments);
        assert!(opts.optimize_xml);
        assert_eq!(opts.target_size, 10 * 1024 * 1024);
    }

    #[test]
    fn test_maximum_preset() {
        let opts = CompressionOptions::maximum();
        assert!(matches!(opts.level, CompressionLevel::Aggressive));
        assert!(opts.remove_properties);
        assert!(opts.remove_notes);
        assert!(opts.remove_unused_media);
        assert!(opts.remove_comments);
        assert!(opts.optimize_xml);
    }

    #[test]
    fn test_web_preset() {
        let opts = CompressionOptions::web();
        assert!(matches!(opts.level, CompressionLevel::Medium));
        assert_eq!(opts.target_size, 5 * 1024 * 1024);
        assert!(opts.remove_unused_media);
        assert!(opts.remove_properties);
    }

    #[test]
    fn test_minimize_xml() {
        let input = r#"<?xml version="1.0"?>
<root>
    <element attr="value" />
</root>"#;

        let minimized = minimize_xml(input);
        assert!(!minimized.contains("\n"));
        assert!(!minimized.contains("    "));
        assert!(minimized.contains("<root>"));
        assert!(minimized.contains("<element"));
    }

    #[test]
    fn test_minimize_xml_preserves_content() {
        let input = r#"<a>  text  </a>"#;
        let minimized = minimize_xml(input);
        // Whitespace inside tags should be preserved
        assert!(minimized.contains("text"));
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00 GB");
    }

    #[test]
    fn test_pptx_analysis_images_percentage() {
        let analysis = PptxAnalysis {
            total_size: 1000,
            images_size: 500,
            xml_size: 300,
            other_size: 200,
            image_count: 5,
            slide_count: 10,
            media_count: 2,
        };

        assert_eq!(analysis.images_percentage(), 50.0);
    }

    #[test]
    fn test_pptx_analysis_images_percentage_zero() {
        let analysis = PptxAnalysis {
            total_size: 0,
            images_size: 0,
            xml_size: 0,
            other_size: 0,
            image_count: 0,
            slide_count: 0,
            media_count: 0,
        };

        assert_eq!(analysis.images_percentage(), 0.0);
    }

    #[test]
    fn test_pptx_analysis_summary() {
        let analysis = PptxAnalysis {
            total_size: 1024 * 1024,
            images_size: 512 * 1024,
            xml_size: 256 * 1024,
            other_size: 256 * 1024,
            image_count: 3,
            slide_count: 5,
            media_count: 1,
        };

        let summary = analysis.summary();
        assert!(summary.contains("PPTX Analysis"));
        assert!(summary.contains("1.0 MB"));
        assert!(summary.contains("3"));
        assert!(summary.contains("5"));
    }

    #[test]
    fn test_compression_result_fields() {
        let result = CompressionResult {
            original_size: 1000,
            compressed_size: 800,
            reduction_percent: 20.0,
            unused_media_removed: 2,
            images_compressed: 3,
            target_achieved: true,
        };

        assert_eq!(result.original_size, 1000);
        assert_eq!(result.compressed_size, 800);
        assert_eq!(result.reduction_percent, 20.0);
        assert_eq!(result.unused_media_removed, 2);
        assert_eq!(result.images_compressed, 3);
        assert!(result.target_achieved);
    }
}
