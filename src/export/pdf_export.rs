//! Pure-Rust PDF export module
//!
//! Exports presentations to PDF using the `pdfrs` crate (no LibreOffice /
//! Poppler / LaTeX required). The presentation is first rendered to a
//! CommonMark-style Markdown document via [`crate::export::md`], then the
//! Markdown is parsed into structured elements and rendered to PDF bytes
//! by `pdfrs`.
//!
//! Enable with the `pdf-native` Cargo feature:
//!
//! ```toml
//! [dependencies]
//! ppt-rs = { version = "0.2", features = ["pdf-native"] }
//! ```
//!
//! # Quick start
//!
//! ```rust,no_run
//! # #[cfg(feature = "pdf-native")] {
//! use ppt_rs::api::Presentation;
//! use ppt_rs::generator::SlideContent;
//! use ppt_rs::export::pdf_export::{export_to_pdf, PdfExportOptions};
//!
//! let pres = Presentation::with_title("Demo")
//!     .add_slide(SlideContent::new("Slide 1").add_bullet("Hello"))
//!     .add_slide(SlideContent::new("Slide 2").add_bullet("World"));
//!
//! export_to_pdf(&pres, "out.pdf", &PdfExportOptions::default()).unwrap();
//! # }
//! ```

use crate::api::Presentation;
use crate::exc::{PptxError, Result};
use std::path::Path;

/// Page orientation for PDF output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PdfOrientation {
    #[default]
    Portrait,
    Landscape,
}

/// Options for pure-Rust PDF export.
#[derive(Debug, Clone)]
pub struct PdfExportOptions {
    /// Page orientation (default portrait).
    pub orientation: PdfOrientation,
    /// Base font name (default `"Helvetica"`). Pass a `pdfrs` Base-14 name or
    /// the path to a TTF file.
    pub font: String,
    /// Base font size in points (default `12.0`).
    pub font_size: f32,
    /// Whether to include the YAML frontmatter in the rendered Markdown
    /// (default `true`).
    pub include_frontmatter: bool,
    /// Whether to include speaker notes (default `true`).
    pub include_notes: bool,
    /// Whether to include image references (default `true`).
    pub include_images: bool,
}

impl Default for PdfExportOptions {
    fn default() -> Self {
        Self {
            orientation: PdfOrientation::Portrait,
            font: "Helvetica".to_string(),
            font_size: 12.0,
            include_frontmatter: true,
            include_notes: true,
            include_images: true,
        }
    }
}

impl PdfExportOptions {
    /// Create new options with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set page orientation.
    pub fn with_orientation(mut self, orientation: PdfOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set base font name (Base-14 or TTF path).
    pub fn with_font(mut self, font: impl Into<String>) -> Self {
        self.font = font.into();
        self
    }

    /// Set base font size in points.
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size.max(1.0);
        self
    }

    /// Toggle YAML frontmatter in the rendered Markdown.
    pub fn with_frontmatter(mut self, include: bool) -> Self {
        self.include_frontmatter = include;
        self
    }

    /// Toggle speaker notes in the rendered Markdown.
    pub fn with_notes(mut self, include: bool) -> Self {
        self.include_notes = include;
        self
    }

    /// Toggle image references in the rendered Markdown.
    pub fn with_images(mut self, include: bool) -> Self {
        self.include_images = include;
        self
    }

    /// Landscape preset.
    pub fn landscape() -> Self {
        Self::default().with_orientation(PdfOrientation::Landscape)
    }
}

/// Render a presentation to PDF bytes using the pure-Rust `pdfrs` engine.
///
/// No external binaries required. Returns the raw PDF byte buffer.
pub fn export_to_pdf_bytes(
    presentation: &Presentation,
    options: &PdfExportOptions,
) -> Result<Vec<u8>> {
    let md = crate::export::md::export_to_markdown_with_options(
        presentation,
        &crate::export::md::MarkdownOptions {
            include_frontmatter: options.include_frontmatter,
            slide_separator: "---".to_string(),
            include_notes: options.include_notes,
            use_gfm_tables: true,
            include_images: options.include_images,
            include_slide_numbers: true,
        },
    )?;

    let elements = pdfrs::elements::parse_markdown(&md);
    let layout = match options.orientation {
        PdfOrientation::Portrait => pdfrs::pdf_generator::PageLayout::portrait(),
        PdfOrientation::Landscape => pdfrs::pdf_generator::PageLayout::landscape(),
    };

    pdfrs::pdf_generator::generate_pdf_bytes(&elements, &options.font, options.font_size, layout)
        .map_err(|e| PptxError::Generic(format!("pdfrs generation failed: {e}")))
}

/// Render a presentation to a PDF file using the pure-Rust `pdfrs` engine.
pub fn export_to_pdf<P: AsRef<Path>>(
    presentation: &Presentation,
    output_path: P,
    options: &PdfExportOptions,
) -> Result<Vec<u8>> {
    let bytes = export_to_pdf_bytes(presentation, options)?;
    std::fs::write(output_path.as_ref(), &bytes)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::SlideContent;

    #[test]
    fn test_options_default() {
        let opts = PdfExportOptions::default();
        assert_eq!(opts.orientation, PdfOrientation::Portrait);
        assert_eq!(opts.font, "Helvetica");
        assert!((opts.font_size - 12.0).abs() < f32::EPSILON);
        assert!(opts.include_frontmatter);
        assert!(opts.include_notes);
        assert!(opts.include_images);
    }

    #[test]
    fn test_options_builder() {
        let opts = PdfExportOptions::new()
            .with_orientation(PdfOrientation::Landscape)
            .with_font("Times-Roman")
            .with_font_size(10.0)
            .with_frontmatter(false)
            .with_notes(false)
            .with_images(false);

        assert_eq!(opts.orientation, PdfOrientation::Landscape);
        assert_eq!(opts.font, "Times-Roman");
        assert!((opts.font_size - 10.0).abs() < f32::EPSILON);
        assert!(!opts.include_frontmatter);
        assert!(!opts.include_notes);
        assert!(!opts.include_images);
    }

    #[test]
    fn test_options_landscape_preset() {
        let opts = PdfExportOptions::landscape();
        assert_eq!(opts.orientation, PdfOrientation::Landscape);
    }

    #[test]
    fn test_options_font_size_floors_at_one() {
        let opts = PdfExportOptions::new().with_font_size(0.0);
        assert!(opts.font_size >= 1.0);
    }

    #[test]
    fn test_export_to_pdf_bytes_simple() {
        let pres = Presentation::with_title("Native PDF")
            .add_slide(SlideContent::new("Slide 1").add_bullet("Hello"))
            .add_slide(SlideContent::new("Slide 2").add_bullet("World"));

        let bytes = export_to_pdf_bytes(&pres, &PdfExportOptions::default()).unwrap();
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[..5], b"%PDF-");
    }

    #[test]
    fn test_export_to_pdf_bytes_landscape() {
        let pres = Presentation::with_title("Landscape")
            .add_slide(SlideContent::new("Only"));

        let opts = PdfExportOptions::landscape();
        let bytes = export_to_pdf_bytes(&pres, &opts).unwrap();
        assert_eq!(&bytes[..5], b"%PDF-");
    }

    #[test]
    fn test_export_to_pdf_file() {
        let pres = Presentation::with_title("File")
            .add_slide(SlideContent::new("Hi").add_bullet("Bullet"));

        let path = std::env::temp_dir().join(format!(
            "ppt_rs_pdf_export_{}.pdf",
            uuid::Uuid::new_v4()
        ));
        let opts = PdfExportOptions::new().with_frontmatter(false).with_notes(false);

        export_to_pdf(&pres, &path, &opts).unwrap();
        let read_back = std::fs::read(&path).unwrap();
        assert_eq!(&read_back[..5], b"%PDF-");
        assert!(pdfrs::pdf::validate_pdf_bytes(&read_back).valid);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_export_to_pdf_bytes_without_frontmatter() {
        let pres = Presentation::with_title("NoFrontmatter")
            .add_slide(SlideContent::new("S1").add_bullet("A"));

        let opts = PdfExportOptions::new().with_frontmatter(false);
        let bytes = export_to_pdf_bytes(&pres, &opts).unwrap();
        assert_eq!(&bytes[..5], b"%PDF-");
    }

    #[test]
    fn test_export_empty_presentation_still_produces_pdf() {
        let pres = Presentation::with_title("Empty");
        let bytes = export_to_pdf_bytes(&pres, &PdfExportOptions::default()).unwrap();
        assert_eq!(&bytes[..5], b"%PDF-");
    }

    #[test]
    fn test_orientation_default_is_portrait() {
        assert_eq!(PdfOrientation::default(), PdfOrientation::Portrait);
    }
}