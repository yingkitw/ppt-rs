//! Image export module
//!
//! Provides functionality to export presentations and individual slides
//! to image formats (PNG, JPEG).
//!
//! Uses LibreOffice for rendering (same approach as PDF export).

use crate::api::Presentation;
use crate::exc::{PptxError, Result};
use std::path::Path;
use std::process::Command;

/// Image format for export
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageFormat {
    /// PNG format (lossless, good for graphics)
    Png,
    /// JPEG format (lossy, good for photos)
    Jpeg,
}

impl ImageFormat {
    /// Get file extension
    pub fn extension(&self) -> &'static str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpg",
        }
    }

    /// Get MIME type
    pub fn mime_type(&self) -> &'static str {
        match self {
            ImageFormat::Png => "image/png",
            ImageFormat::Jpeg => "image/jpeg",
        }
    }
}

impl Default for ImageFormat {
    fn default() -> Self {
        ImageFormat::Png
    }
}

/// Options for image export
#[derive(Debug, Clone)]
pub struct ImageExportOptions {
    /// Image format (PNG or JPEG)
    pub format: ImageFormat,
    /// DPI/resolution (default 150)
    pub dpi: u32,
    /// JPEG quality (0-100, default 90)
    pub jpeg_quality: u8,
    /// Output width in pixels (0 = auto based on DPI)
    pub width: u32,
    /// Output height in pixels (0 = auto based on DPI)
    pub height: u32,
    /// Export all slides or specific slide (0 = all, 1+ = specific slide)
    pub slide_number: usize,
}

impl Default for ImageExportOptions {
    fn default() -> Self {
        Self {
            format: ImageFormat::Png,
            dpi: 150,
            jpeg_quality: 90,
            width: 0,
            height: 0,
            slide_number: 0,
        }
    }
}

impl ImageExportOptions {
    /// Create new options with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set image format
    pub fn with_format(mut self, format: ImageFormat) -> Self {
        self.format = format;
        self
    }

    /// Set DPI
    pub fn with_dpi(mut self, dpi: u32) -> Self {
        self.dpi = dpi;
        self
    }

    /// Set JPEG quality
    pub fn with_jpeg_quality(mut self, quality: u8) -> Self {
        self.jpeg_quality = quality.min(100);
        self
    }

    /// Set output dimensions
    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set specific slide to export (1-based, 0 = all)
    pub fn with_slide(mut self, slide: usize) -> Self {
        self.slide_number = slide;
        self
    }

    /// High quality preset (300 DPI, PNG)
    pub fn high_quality() -> Self {
        Self {
            format: ImageFormat::Png,
            dpi: 300,
            jpeg_quality: 95,
            width: 0,
            height: 0,
            slide_number: 0,
        }
    }

    /// Web optimized preset (96 DPI, JPEG)
    pub fn web_optimized() -> Self {
        Self {
            format: ImageFormat::Jpeg,
            dpi: 96,
            jpeg_quality: 85,
            width: 0,
            height: 0,
            slide_number: 0,
        }
    }
}

/// Export presentation to images
///
/// Uses LibreOffice for rendering. Requires LibreOffice to be installed.
///
/// # Arguments
/// * `presentation` - The presentation to export
/// * `output_dir` - Directory to save images
/// * `options` - Export options
///
/// # Returns
/// Vector of paths to generated image files
pub fn export_to_images<P: AsRef<Path>>(
    presentation: &Presentation,
    output_dir: P,
    options: &ImageExportOptions,
) -> Result<Vec<std::path::PathBuf>> {
    // First save presentation to temporary PPTX file
    let temp_dir = std::env::temp_dir();
    let temp_pptx = temp_dir.join("temp_export.pptx");
    presentation.save(&temp_pptx)?;

    let output_dir = output_dir.as_ref();
    std::fs::create_dir_all(output_dir)?;

    // Use LibreOffice to convert to images
    let result = export_pptx_to_images(&temp_pptx, output_dir, options);

    // Cleanup temp file
    let _ = std::fs::remove_file(&temp_pptx);

    result
}

/// Export a specific slide to an image
pub fn export_slide_to_image<P: AsRef<Path>>(
    presentation: &Presentation,
    slide_number: usize,
    output_path: P,
    options: &ImageExportOptions,
) -> Result<std::path::PathBuf> {
    if slide_number == 0 || slide_number > presentation.slide_count() {
        return Err(PptxError::InvalidOperation(format!(
            "Invalid slide number: {} (presentation has {} slides)",
            slide_number,
            presentation.slide_count()
        )));
    }

    let mut slide_options = options.clone();
    slide_options.slide_number = slide_number;

    let output_dir = output_path
        .as_ref()
        .parent()
        .unwrap_or(std::path::Path::new("."));
    let file_stem = output_path
        .as_ref()
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("slide");

    let paths = export_to_images(presentation, output_dir, &slide_options)?;

    // Find the specific slide file
    let expected_name = format!("Slide{}.{}", slide_number, slide_options.format.extension());
    for path in paths {
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name == expected_name || name.contains(&format!("Slide{}", slide_number)) {
                // Rename to requested output path if different
                if path != output_path.as_ref() {
                    std::fs::rename(&path, &output_path)?;
                    return Ok(output_path.as_ref().to_path_buf());
                }
                return Ok(path);
            }
        }
    }

    Err(PptxError::Generic(String::from("Export failed")))
}

/// Internal function to export PPTX file to images using LibreOffice
fn export_pptx_to_images<P: AsRef<Path>, Q: AsRef<Path>>(
    pptx_path: P,
    output_dir: Q,
    options: &ImageExportOptions,
) -> Result<Vec<std::path::PathBuf>> {
    let pptx_path = pptx_path.as_ref();
    let output_dir = output_dir.as_ref();

    // Check if LibreOffice is available
    if !is_libreoffice_available() {
        return Err(PptxError::Generic(String::from(
            "LibreOffice not found"
        )));
    }

    // Build LibreOffice command
    let ext = options.format.extension();
    let convert_opt = format!("{}:ExportNotesPages=false", ext);
    
    let mut cmd = Command::new("soffice");
    cmd.arg("--headless")
        .arg("--convert-to")
        .arg(&convert_opt)
        .arg("--outdir")
        .arg(output_dir)
        .arg(pptx_path);

    // Execute conversion
    let output = cmd.output().map_err(|e| {
        PptxError::Generic(format!("Failed to execute LibreOffice: {}", e))
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(PptxError::Generic(format!(
            "LibreOffice conversion failed: {}",
            stderr
        )));
    }

    // Collect output files
    let mut image_files = Vec::new();
    let file_stem = pptx_path.file_stem().and_then(|s| s.to_str()).unwrap_or("slide");

    for entry in std::fs::read_dir(output_dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(file_ext) = path.extension().and_then(|e| e.to_str()) {
            if file_ext.eq_ignore_ascii_case(ext) {
                // Check if it is from our conversion
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    if name.starts_with(file_stem) || name.starts_with("Slide") {
                        image_files.push(path);
                    }
                }
            }
        }
    }

    // Sort by slide number
    image_files.sort_by(|a, b| {
        let a_num = extract_slide_number(a);
        let b_num = extract_slide_number(b);
        a_num.cmp(&b_num)
    });

    Ok(image_files)
}

/// Check if LibreOffice is available
fn is_libreoffice_available() -> bool {
    Command::new("soffice").arg("--version").output().is_ok()
}

/// Extract slide number from filename
fn extract_slide_number(path: &std::path::Path) -> usize {
    path.file_stem()
        .and_then(|s| s.to_str())
        .and_then(|name| {
            // Extract number from "Slide1" or "slide1" or "temp_export1"
            let digits: String = name.chars().filter(|c| c.is_ascii_digit()).collect();
            digits.parse().ok()
        })
        .unwrap_or(0)
}

/// Render presentation thumbnail (first slide only)
pub fn render_thumbnail<P: AsRef<Path>>(
    presentation: &Presentation,
    output_path: P,
    width: u32,
) -> Result<std::path::PathBuf> {
    let options = ImageExportOptions::new()
        .with_format(ImageFormat::Png)
        .with_slide(1);

    // Calculate DPI based on desired width
    let dpi = (width as f32 / 10.0) as u32;

    let options = ImageExportOptions {
        dpi,
        ..options
    };

    export_slide_to_image(presentation, 1, output_path, &options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::SlideContent;

    #[test]
    fn test_image_format_extension() {
        assert_eq!(ImageFormat::Png.extension(), "png");
        assert_eq!(ImageFormat::Jpeg.extension(), "jpg");
    }

    #[test]
    fn test_image_export_options() {
        let opts = ImageExportOptions::new()
            .with_format(ImageFormat::Jpeg)
            .with_dpi(200)
            .with_jpeg_quality(85);

        assert_eq!(opts.format, ImageFormat::Jpeg);
        assert_eq!(opts.dpi, 200);
        assert_eq!(opts.jpeg_quality, 85);
    }

    #[test]
    fn test_high_quality_preset() {
        let opts = ImageExportOptions::high_quality();
        assert_eq!(opts.dpi, 300);
        assert_eq!(opts.format, ImageFormat::Png);
    }

    #[test]
    fn test_web_optimized_preset() {
        let opts = ImageExportOptions::web_optimized();
        assert_eq!(opts.dpi, 96);
        assert_eq!(opts.format, ImageFormat::Jpeg);
        assert_eq!(opts.jpeg_quality, 85);
    }

    #[test]
    fn test_extract_slide_number() {
        let path = std::path::Path::new("Slide1.png");
        assert_eq!(extract_slide_number(path), 1);

        let path = std::path::Path::new("slide12.jpg");
        assert_eq!(extract_slide_number(path), 12);

        let path = std::path::Path::new("temp_export5.png");
        assert_eq!(extract_slide_number(path), 5);
    }
}
