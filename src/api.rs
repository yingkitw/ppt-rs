//! Public API module
//!
//! High-level API for working with PowerPoint presentations.

use crate::exc::{Result, PptxError};
use crate::opc::Package;
use crate::generator::{SlideContent, create_pptx_with_content, Image};
use crate::import::import_pptx;
use crate::export::html::export_to_html;
use std::io::{Read, Seek};
use std::path::Path;
use std::process::Command;

/// Represents a PowerPoint presentation
#[derive(Debug, Clone, Default)]
pub struct Presentation {
    title: String,
    slides: Vec<SlideContent>,
}

impl Presentation {
    /// Create a new empty presentation
    pub fn new() -> Self {
        Presentation {
            title: String::new(),
            slides: Vec::new(),
        }
    }

    /// Create a presentation with a title
    pub fn with_title(title: &str) -> Self {
        Presentation {
            title: title.to_string(),
            slides: Vec::new(),
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

    /// Build the presentation as PPTX bytes
    pub fn build(&self) -> Result<Vec<u8>> {
        if self.slides.is_empty() {
            return Err(PptxError::InvalidState("Presentation has no slides".into()));
        }
        create_pptx_with_content(&self.title, self.slides.clone())
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

    /// Export the presentation to PDF using LibreOffice
    /// 
    /// Requires LibreOffice to be installed and available via `soffice` command.
    /// On macOS, it also checks `/Applications/LibreOffice.app/Contents/MacOS/soffice`.
    pub fn save_as_pdf<P: AsRef<Path>>(&self, output_path: P) -> Result<()> {
        // Create a temp file
        let temp_dir = std::env::temp_dir();
        let temp_filename = format!("ppt_rs_{}.pptx", uuid::Uuid::new_v4());
        let temp_path = temp_dir.join(&temp_filename);
        
        // Save current presentation to temp file
        self.save(&temp_path)?;
        
        // Try to find soffice
        let soffice_cmd = if cfg!(target_os = "macos") {
            if Path::new("/Applications/LibreOffice.app/Contents/MacOS/soffice").exists() {
                "/Applications/LibreOffice.app/Contents/MacOS/soffice"
            } else {
                "soffice"
            }
        } else {
            "soffice"
        };

        // Get output directory
        let output_parent = output_path.as_ref().parent().unwrap_or(Path::new("."));
        
        // Run conversion
        // soffice --headless --convert-to pdf <temp_path> --outdir <output_dir>
        let result = Command::new(soffice_cmd)
            .arg("--headless")
            .arg("--convert-to")
            .arg("pdf")
            .arg(&temp_path)
            .arg("--outdir")
            .arg(output_parent)
            .output();

        // Clean up temp file (ignore error)
        let _ = std::fs::remove_file(&temp_path);

        match result {
            Ok(output) => {
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(PptxError::Generic(format!("LibreOffice conversion failed: {}", stderr)));
                }
            },
            Err(e) => {
                return Err(PptxError::Generic(format!("Failed to execute libreoffice: {}", e)));
            }
        }
        
        // LibreOffice creates file with same basename but .pdf extension in outdir
        // The generated file will be temp_filename.pdf (since input was temp_filename.pptx)
        let generated_pdf_name = temp_filename.replace(".pptx", ".pdf");
        let generated_pdf_path = output_parent.join(&generated_pdf_name);
        
        if generated_pdf_path.exists() {
             std::fs::rename(&generated_pdf_path, output_path.as_ref())?;
             Ok(())
        } else {
             Err(PptxError::Generic("PDF output file not found".to_string()))
        }
    }

    /// Export slides to PNG images
    /// 
    /// Requires LibreOffice (for PDF conversion) and `pdftoppm` (from poppler).
    /// Images will be named `slide-1.png`, `slide-2.png`, etc. in the output directory.
    pub fn save_as_png<P: AsRef<Path>>(&self, output_dir: P) -> Result<()> {
        let output_dir = output_dir.as_ref();
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)?;
        }

        // Create temp PDF
        let temp_dir = std::env::temp_dir();
        let temp_pdf_name = format!("ppt_rs_temp_{}.pdf", uuid::Uuid::new_v4());
        let temp_pdf_path = temp_dir.join(&temp_pdf_name);
        
        // Convert to PDF first
        self.save_as_pdf(&temp_pdf_path)?;
        
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
            return Err(PptxError::NotFound(format!("PDF file not found: {}", path.display())));
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
            if let Some(start) = name.rfind('-') {
                if let Some(end) = name.rfind('.') {
                    if start < end {
                        if let Ok(num) = name[start+1..end].parse::<u32>() {
                            return num;
                        }
                    }
                }
            }
            0 // Fallback
        });
        
        for entry in entries {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "png") {
                // Create slide with full screen image
                let image = Image::from_path(&path)
                    .map_err(|e| PptxError::Generic(e))?;
                
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
}

/// Open a presentation from a file path
pub fn open<P: AsRef<Path>>(path: P) -> Result<Package> {
    Package::open(path)
}

/// Open a presentation from a reader
pub fn open_reader<R: Read + Seek>(reader: R) -> Result<Package> {
    Package::open_reader(reader)
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
        let pres = Presentation::with_title("Test")
            .add_slide(SlideContent::new("Slide 1"));
        
        let result = pres.build();
        assert!(result.is_ok());
    }
}
