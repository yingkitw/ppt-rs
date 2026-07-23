//! Public API module
//!
//! High-level API for working with PowerPoint presentations.

use crate::exc::{messages, PptxError, Result};
use crate::export::html::export_to_html;
use crate::generator::{create_pptx_with_settings, Image, PresentationSettings, PresentationTheme, SlideContent};
use crate::import::import_pptx;
use std::path::Path;
use std::process::Command;

/// Represents a PowerPoint presentation
#[derive(Debug, Clone, Default)]
pub struct Presentation {
    title: String,
    slides: Vec<SlideContent>,
    settings: Option<PresentationSettings>,
}

impl Presentation {
    /// Create a new empty presentation
    pub fn new() -> Self {
        Presentation {
            title: String::new(),
            slides: Vec::new(),
            settings: None,
        }
    }

    /// Create a presentation with a title
    pub fn with_title(title: &str) -> Self {
        Presentation {
            title: title.to_string(),
            slides: Vec::new(),
            settings: None,
        }
    }

    /// Set the presentation title
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Add a slide to the presentation
    pub fn add_slide(mut self, slide: SlideContent) -> Self {
        self.slides.push(slide);
        self
    }

    /// Append slides from another presentation
    pub fn add_presentation(mut self, other: Presentation) -> Self {
        self.slides.extend(other.slides);
        self
    }

    /// Get the number of slides
    pub fn slide_count(&self) -> usize {
        self.slides.len()
    }

    /// Get the slides in the presentation
    pub fn slides(&self) -> &[SlideContent] {
        &self.slides
    }

    /// Get the presentation title
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Apply a custom color/font theme to the generated PPTX
    pub fn with_theme(mut self, theme: PresentationTheme) -> Self {
        let mut settings = self.settings.take().unwrap_or_default();
        settings.theme = Some(theme);
        self.settings = Some(settings);
        self
    }

    /// Set presentation-level settings (theme, slide show, print, etc.)
    pub fn with_settings(mut self, settings: PresentationSettings) -> Self {
        self.settings = Some(settings);
        self
    }

    /// Build the presentation as PPTX bytes
    pub fn build(&self) -> Result<Vec<u8>> {
        if self.slides.is_empty() {
            return Err(PptxError::InvalidState(
                messages::must_not_be_empty("presentation slides"),
            ));
        }
        create_pptx_with_settings(&self.title, &self.slides, self.settings.clone())
            .map_err(|e| PptxError::Generic(e.to_string()))
    }

    /// Consume the presentation and build PPTX bytes without cloning slide data.
    pub fn into_bytes(self) -> Result<Vec<u8>> {
        if self.slides.is_empty() {
            return Err(PptxError::InvalidState(
                messages::must_not_be_empty("presentation slides"),
            ));
        }
        create_pptx_with_settings(&self.title, &self.slides, self.settings)
            .map_err(|e| PptxError::Generic(e.to_string()))
    }

    /// Save the presentation to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let data = self.build()?;
        std::fs::write(path, data)?;
        Ok(())
    }

    /// Create a presentation from a PPTX file
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy();
        import_pptx(&path_str)
    }

    /// Export the presentation to HTML
    pub fn save_as_html<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let html = export_to_html(self)?;
        std::fs::write(path, html)?;
        Ok(())
    }

    /// Export the presentation to PDF using pure-Rust vector rendering.
    ///
    /// Each slide is rendered as a native PDF page with title, bullets,
    /// and content drawn as vector text and graphics. No external binaries
    /// (LibreOffice, Poppler, etc.) are required.
    pub fn save_as_pdf<P: AsRef<Path>>(&self, output_path: P) -> Result<()> {
        crate::export::slide_render::render_to_pdf(self, output_path)?;
        Ok(())
    }

    /// Export the presentation to PDF with custom options using the pure-Rust `pdfrs` engine.
    ///
    /// Like [`Presentation::save_as_pdf`] but allows customizing orientation,
    /// font, font size, and which Markdown sections are included.
    ///
    /// # Arguments
    /// * `output_path` - Path to the PDF file to write.
    /// * `options` - [`crate::export::pdf_export::PdfExportOptions`] controlling
    ///   orientation, font, font size, and which Markdown sections are included.
    #[cfg(feature = "pdf-native")]
    pub fn save_as_pdf_via_pdfrs<P: AsRef<Path>>(
        &self,
        output_path: P,
        options: &crate::export::pdf_export::PdfExportOptions,
    ) -> Result<()> {
        crate::export::pdf_export::export_to_pdf(self, output_path, options)?;
        Ok(())
    }

    /// Render the presentation to PDF bytes using the pure-Rust `pdfrs` engine.
    ///
    /// Returns the raw PDF byte buffer instead of writing to disk. Useful
    /// for embedding in HTTP responses, mailing systems, or piping to other
    /// tools.
    ///
    /// Requires the `pdf-native` Cargo feature.
    #[cfg(feature = "pdf-native")]
    pub fn to_pdf_bytes(
        &self,
        options: &crate::export::pdf_export::PdfExportOptions,
    ) -> Result<Vec<u8>> {
        crate::export::pdf_export::export_to_pdf_bytes(self, options)
    }

    /// Export slides to PNG images
    ///
    /// Requires `pdftoppm` (from poppler) to be installed.
    /// Images will be named `slide-1.png`, `slide-2.png`, etc. in the output directory.
    pub fn save_as_png<P: AsRef<Path>>(&self, output_dir: P) -> Result<()> {
        let output_dir = output_dir.as_ref();
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)?;
        }

        // Create temp PDF using pure-Rust pdfrs engine
        let temp_dir = std::env::temp_dir();
        let temp_pdf_name = format!("ppt_rs_temp_{}.pdf", uuid::Uuid::new_v4());
        let temp_pdf_path = temp_dir.join(&temp_pdf_name);

        // Convert to PDF first (no LibreOffice needed)
        let bytes = crate::export::slide_render::render_to_pdf_bytes(self)?;
        std::fs::write(&temp_pdf_path, &bytes)?;

        // Convert PDF to PNGs using pdftoppm
        // pdftoppm -png <pdf_file> <image_prefix>
        let prefix = output_dir.join("slide");

        let status = Command::new("pdftoppm")
            .arg("-png")
            .arg(&temp_pdf_path)
            .arg(&prefix)
            .status()
            .map_err(|e| PptxError::Generic(format!("Failed to execute pdftoppm: {}", e)))?;

        // Cleanup temp PDF
        let _ = std::fs::remove_file(&temp_pdf_path);

        if !status.success() {
            return Err(PptxError::Generic("pdftoppm conversion failed".to_string()));
        }

        Ok(())
    }

    /// Create a presentation from a PDF file (each page becomes a slide)
    ///
    /// Requires `pdftoppm` (from poppler) to be installed.
    pub fn from_pdf<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(PptxError::NotFound(format!(
                "PDF file not found: {}",
                path.display()
            )));
        }

        // Create temp dir for images
        let temp_dir = std::env::temp_dir().join(format!("ppt_rs_import_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir)?;

        // Convert PDF to PNGs
        let prefix = temp_dir.join("page");

        let status = Command::new("pdftoppm")
            .arg("-png")
            .arg(path)
            .arg(&prefix)
            .status()
            .map_err(|e| PptxError::Generic(format!("Failed to execute pdftoppm: {}", e)))?;

        if !status.success() {
            let _ = std::fs::remove_dir_all(&temp_dir);
            return Err(PptxError::Generic("pdftoppm failed".to_string()));
        }

        // Read images and create slides
        let mut pres = Presentation::new();
        // Set title from filename
        if let Some(stem) = path.file_stem() {
            pres = pres.title(&stem.to_string_lossy());
        }

        // Read dir
        let mut entries: Vec<_> = std::fs::read_dir(&temp_dir)?
            .filter_map(|e| e.ok())
            .collect();

        // Sort by filename to ensure page order
        // pdftoppm names files like page-1.png, page-2.png... page-10.png
        // Default string sort might put page-10 before page-2
        // We need to sort by length then by name, or rely on pdftoppm zero padding (it usually does -01 if needed, but safer to trust number)
        // pdftoppm default is -1, -2... -10.
        // So page-1.png, page-10.png, page-2.png.
        // We need natural sort.
        entries.sort_by_key(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            // Extract number from end
            // "page-1.png" -> 1
            if let Some(start) = name.rfind('-')
                && let Some(end) = name.rfind('.')
                    && start < end
                        && let Ok(num) = name[start + 1..end].parse::<u32>() {
                            return num;
                        }
            0 // Fallback
        });

        for entry in entries {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "png") {
                // Create slide with full screen image
                let image = Image::from_path(&path).map_err(PptxError::Generic)?;

                // Add image to slide
                // Use a default layout?
                // Just create a slide with this image
                // We'll center it.
                // Assuming standard 16:9 slide (10x5.625 inches) -> 9144000 x 5143500 EMU
                // But we don't know image dimensions here easily without reading it.
                // Image builder defaults to auto size?
                // Let's just add it.

                let mut slide = SlideContent::new("");
                slide.images.push(image);
                pres = pres.add_slide(slide);
            }
        }

        let _ = std::fs::remove_dir_all(&temp_dir);
        Ok(pres)
    }

    /// Export the presentation to Markdown format
    ///
    /// # Arguments
    /// * `path` - Output file path
    ///
    /// # Example
    /// ```
    /// # use ppt_rs::api::Presentation;
    /// # use ppt_rs::generator::SlideContent;
    /// # let pres = Presentation::with_title("My Presentation")
    /// #     .add_slide(SlideContent::new("Slide 1").add_bullet("Point 1"));
    /// # // pres.save_as_markdown("output.md").unwrap();
    /// ```
    pub fn save_as_markdown<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        use crate::export::md::export_to_markdown;
        let md = export_to_markdown(self)?;
        std::fs::write(path, md)?;
        Ok(())
    }

    /// Export the presentation to Markdown with custom options
    pub fn save_as_markdown_with_options<P: AsRef<Path>>(
        &self,
        path: P,
        options: &crate::export::md::MarkdownOptions,
    ) -> Result<()> {
        use crate::export::md::export_to_markdown_with_options;
        let md = export_to_markdown_with_options(self, options)?;
        std::fs::write(path, md)?;
        Ok(())
    }

    /// Export slides to image files (PNG/JPEG)
    ///
    /// Uses LibreOffice for raster image rendering. Requires LibreOffice to be installed.
    ///
    /// # Arguments
    /// * `output_dir` - Directory to save images
    /// * `options` - Image export options (format, DPI, quality)
    ///
    /// # Returns
    /// Vector of paths to generated image files
    pub fn save_as_images<P: AsRef<Path>>(
        &self,
        output_dir: P,
        options: &crate::export::image_export::ImageExportOptions,
    ) -> Result<Vec<std::path::PathBuf>> {
        use crate::export::image_export::export_to_images;
        export_to_images(self, output_dir, options)
    }

    /// Export a specific slide to an image file
    ///
    /// # Arguments
    /// * `slide_number` - 1-based slide number
    /// * `output_path` - Output file path
    /// * `options` - Image export options
    pub fn save_slide_as_image<P: AsRef<Path>>(
        &self,
        slide_number: usize,
        output_path: P,
        options: &crate::export::image_export::ImageExportOptions,
    ) -> Result<std::path::PathBuf> {
        use crate::export::image_export::export_slide_to_image;
        export_slide_to_image(self, slide_number, output_path, options)
    }

    /// Render a thumbnail of the first slide
    ///
    /// # Arguments
    /// * `output_path` - Output file path
    /// * `width` - Desired width in pixels
    pub fn save_thumbnail<P: AsRef<Path>>(&self, output_path: P, width: u32) -> Result<std::path::PathBuf> {
        use crate::export::image_export::render_thumbnail;
        render_thumbnail(self, output_path, width)
    }

    /// Compress and optimize the presentation
    ///
    /// Saves a compressed version with reduced file size.
    ///
    /// # Arguments
    /// * `output_path` - Path for compressed PPTX file
    /// * `options` - Compression options (level, features to remove)
    ///
    /// # Returns
    /// Compression result with statistics
    ///
    /// # Example
    /// ```
    /// # use ppt_rs::api::Presentation;
    /// # use ppt_rs::opc::compress::CompressionOptions;
    /// # let pres = Presentation::with_title("Large Presentation");
    /// # let options = CompressionOptions::web();
    /// # // let result = pres.compress("optimized.pptx", &options).unwrap();
    /// # // println!("Reduced by {:.1}%", result.reduction_percent);
    /// ```
    pub fn compress<P: AsRef<Path>>(
        &self,
        output_path: P,
        options: &crate::opc::compress::CompressionOptions,
    ) -> Result<crate::opc::compress::CompressionResult> {
        // First save to temp file
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join(format!("compress_{}.pptx", uuid::Uuid::new_v4()));
        self.save(&temp_path)?;

        // Compress
        let result = crate::opc::compress::compress_pptx(&temp_path, output_path, options);

        // Cleanup
        let _ = std::fs::remove_file(&temp_path);

        result
    }

    /// Get file size analysis
    ///
    /// Returns analysis of what contributes to file size.
    pub fn analyze_size(&self) -> Result<crate::opc::compress::PptxAnalysis> {
        // Save to temp file for analysis
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join(format!("analyze_{}.pptx", uuid::Uuid::new_v4()));
        self.save(&temp_path)?;

        let analysis = crate::opc::compress::analyze_pptx(&temp_path);

        // Cleanup
        let _ = std::fs::remove_file(&temp_path);

        analysis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentation_builder() {
        let pres = Presentation::with_title("Test")
            .add_slide(SlideContent::new("Slide 1").add_bullet("Point 1"));

        assert_eq!(pres.get_title(), "Test");
        assert_eq!(pres.slide_count(), 1);
    }

    #[test]
    fn test_presentation_build() {
        let pres = Presentation::with_title("Test").add_slide(SlideContent::new("Slide 1"));

        let result = pres.build();
        assert!(result.is_ok());
    }
}
