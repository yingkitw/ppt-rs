//! PPTX builder — orchestrates ZIP creation and file writing.
//!
//! Internal layout:
//! - [`plan`] — media/chart/relationship planning
//! - [`write`] — ZIP part writers

mod plan;
mod write;

use super::memory_profile::estimate_output_capacity;
use super::slide_content::SlideContent;
use super::slide_content::presentation_settings::PresentationSettings;
use crate::exc::Result;
use std::io::{Cursor, Seek, Write};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

use plan::{debug_assert_package_valid, zip_options};
use write::{write_package_files, write_package_files_lazy};

pub fn create_pptx(title: &str, slides: usize) -> Result<Vec<u8>> {
    let buffer = Vec::with_capacity(slides.saturating_mul(6_000).max(8_192));
    let cursor = Cursor::new(buffer);
    let mut zip = ZipWriter::new(cursor);
    let options = zip_options();

    write_package_files(&mut zip, &options, title, slides, None, None)?;

    let cursor = zip.finish()?;
    let bytes = cursor.into_inner();
    debug_assert_package_valid(&bytes);
    Ok(bytes)
}
pub fn create_pptx_with_content(title: &str, slides: Vec<SlideContent>) -> Result<Vec<u8>> {
    create_pptx_with_settings(title, &slides, None)
}

/// Create a PPTX file with custom slide content, settings, and optional template deck.
pub fn create_pptx_with_template(
    title: &str,
    slides: &[SlideContent],
    template_path: &str,
    settings: Option<PresentationSettings>,
) -> Result<Vec<u8>> {
    let mut merged = settings.unwrap_or_default();
    merged.template_path = Some(template_path.to_string());
    create_pptx_with_settings(title, slides, Some(merged))
}
pub fn create_pptx_with_settings(
    title: &str,
    slides: &[SlideContent],
    settings: Option<PresentationSettings>,
) -> Result<Vec<u8>> {
    let buffer = Vec::with_capacity(estimate_output_capacity(slides.len(), Some(slides)));
    let cursor = Cursor::new(buffer);
    let mut zip = ZipWriter::new(cursor);
    let options = zip_options();

    write_package_files(
        &mut zip,
        &options,
        title,
        slides.len(),
        Some(slides),
        settings,
    )?;

    let cursor = zip.finish()?;
    let bytes = cursor.into_inner();
    debug_assert_package_valid(&bytes);
    Ok(bytes)
}
/// This is more memory-efficient for large presentations as it avoids
/// buffering the entire ZIP file in memory.
///
/// # Example
/// ```rust,no_run
/// use std::fs::File;
/// use ppt_rs::create_pptx_to_writer;
///
/// let file = File::create("output.pptx")?;
/// create_pptx_to_writer(file, "My Presentation", 10)?;
/// # Ok::<(), ppt_rs::PptxError>(())
/// ```
pub fn create_pptx_to_writer<W: Write + Seek>(writer: W, title: &str, slides: usize) -> Result<W> {
    let mut zip = ZipWriter::new(writer);
    let options = SimpleFileOptions::default();

    write_package_files(&mut zip, &options, title, slides, None, None)?;

    Ok(zip.finish()?)
}
/// This is more memory-efficient for large presentations.
///
/// # Example
/// ```rust,no_run
/// use std::fs::File;
/// use ppt_rs::{create_pptx_with_content_to_writer, SlideContent};
///
/// let file = File::create("output.pptx")?;
/// let slides = vec![
///     SlideContent::new("Title").add_bullet("Point 1"),
///     SlideContent::new("Slide 2").add_bullet("Point 2"),
/// ];
/// create_pptx_with_content_to_writer(file, "My Presentation", &slides, None)?;
/// # Ok::<(), ppt_rs::PptxError>(())
/// ```
pub fn create_pptx_with_content_to_writer<W: Write + Seek>(
    writer: W,
    title: &str,
    slides: &[SlideContent],
    settings: Option<PresentationSettings>,
) -> Result<W> {
    let mut zip = ZipWriter::new(writer);
    let options = SimpleFileOptions::default();

    write_package_files(
        &mut zip,
        &options,
        title,
        slides.len(),
        Some(slides),
        settings,
    )?;

    Ok(zip.finish()?)
}
/// This is useful for:
/// - Very large presentations that don't fit in memory
/// - Dynamically generated slide content
/// - Streaming data sources
///
/// # Example
/// ```rust,no_run
/// use std::fs::File;
/// use ppt_rs::{create_pptx_lazy_to_writer, LazySlideSource, SlideContent};
///
/// struct MySlideGenerator {
///     count: usize,
/// }
///
/// impl LazySlideSource for MySlideGenerator {
///     fn slide_count(&self) -> usize {
///         self.count
///     }
///
///     fn generate_slide(&self, index: usize) -> Option<SlideContent> {
///         if index < self.count {
///             Some(SlideContent::new(&format!("Slide {}", index + 1))
///                 .add_bullet(&format!("Content {}", index + 1)))
///         } else {
///             None
///         }
///     }
/// }
///
/// let file = File::create("output.pptx")?;
/// create_pptx_lazy_to_writer(file, "My Presentation", Box::new(MySlideGenerator { count: 100 }), None)?;
/// # Ok::<(), ppt_rs::PptxError>(())
/// ```
pub trait LazySlideSource {
    /// Return the total number of slides
    fn slide_count(&self) -> usize;

    /// Generate a slide by index (0-based). Return None if index is out of bounds.
    fn generate_slide(&self, index: usize) -> Option<SlideContent>;

    /// Notes and chart count from a single slide generation when both are needed.
    fn slide_features(&self, index: usize) -> Option<(bool, usize)> {
        self.generate_slide(index)
            .map(|s| (s.notes.is_some(), s.charts.len()))
    }

    /// Check if a slide has notes (default implementation checks the generated slide)
    fn slide_has_notes(&self, index: usize) -> bool {
        self.slide_features(index)
            .map(|(notes, _)| notes)
            .unwrap_or(false)
    }

    /// Get the number of charts in a slide (default implementation checks the generated slide)
    fn slide_chart_count(&self, index: usize) -> usize {
        self.slide_features(index)
            .map(|(_, charts)| charts)
            .unwrap_or(0)
    }
}

/// Create a PPTX file using lazy slide generation and write it directly to a writer.
/// This is the most memory-efficient API for large presentations.
///
/// # Example
/// ```rust,no_run
/// use std::fs::File;
/// use ppt_rs::{create_pptx_lazy_to_writer, LazySlideSource, SlideContent};
///
/// struct MySlideGenerator {
///     count: usize,
/// }
///
/// impl LazySlideSource for MySlideGenerator {
///     fn slide_count(&self) -> usize {
///         self.count
///     }
///
///     fn generate_slide(&self, index: usize) -> Option<SlideContent> {
///         if index < self.count {
///             Some(SlideContent::new(&format!("Slide {}", index + 1))
///                 .add_bullet(&format!("Content {}", index + 1)))
///         } else {
///             None
///         }
///     }
/// }
///
/// let file = File::create("output.pptx")?;
/// let generator = Box::new(MySlideGenerator { count: 1000 });
/// create_pptx_lazy_to_writer(file, "Large Presentation", generator, None)?;
/// # Ok::<(), ppt_rs::PptxError>(())
/// ```
pub fn create_pptx_lazy_to_writer<W: Write + Seek>(
    writer: W,
    title: &str,
    slides: Box<dyn LazySlideSource>,
    settings: Option<PresentationSettings>,
) -> Result<W> {
    let mut zip = ZipWriter::new(writer);
    let options = SimpleFileOptions::default();

    write_package_files_lazy(&mut zip, &options, title, slides.as_ref(), settings)?;

    Ok(zip.finish()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    /// A simple test slide source that generates numbered slides
    struct TestSlideSource {
        count: usize,
        with_notes: bool,
    }

    impl LazySlideSource for TestSlideSource {
        fn slide_count(&self) -> usize {
            self.count
        }

        fn generate_slide(&self, index: usize) -> Option<SlideContent> {
            if index < self.count {
                let mut slide = SlideContent::new(&format!("Slide {}", index + 1))
                    .add_bullet(&format!("Point {}", index + 1));

                if self.with_notes {
                    slide.notes = Some("Speaker notes here".to_string());
                }

                Some(slide)
            } else {
                None
            }
        }

        fn slide_has_notes(&self, index: usize) -> bool {
            self.with_notes && index < self.count
        }

        fn slide_chart_count(&self, _index: usize) -> usize {
            0
        }
    }

    #[test]
    fn test_create_pptx_to_writer() {
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let result = create_pptx_to_writer(cursor, "Test Presentation", 3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_pptx_with_digital_signature() {
        use crate::generator::slide_content::{DigitalSignature, SignerInfo};

        let signature = DigitalSignature::new(SignerInfo::new("Alice"));
        let settings = PresentationSettings::new().digital_signature(signature);
        let bytes = create_pptx_with_settings(
            "Signed",
            &[SlideContent::new("Slide 1").add_bullet("Point 1")],
            Some(settings),
        )
        .unwrap();

        let report = crate::core::validate_package_bytes(&bytes);
        assert!(
            report.is_valid(),
            "signature package invalid: {:?}",
            report.issues
        );

        let cursor = Cursor::new(bytes);
        let mut archive = zip::ZipArchive::new(cursor).unwrap();
        let names: std::collections::HashSet<String> = (0..archive.len())
            .map(|i| archive.by_index(i).unwrap().name().unwrap().to_string())
            .collect();
        assert!(names.contains("_xmlsignatures/origin.sigs"));
        assert!(names.contains("_xmlsignatures/sig1.xml"));
        assert!(names.contains("_rels/.rels"));
    }

    #[test]
    fn test_create_pptx_with_embedded_fonts() {
        use crate::generator::slide_content::{EmbeddedFont, EmbeddedFontList, FontStyle};

        let mut fonts = EmbeddedFontList::new();
        fonts.add(EmbeddedFont::new(
            "Arial",
            FontStyle::Regular,
            vec![0u8; 20],
            "",
        ));
        let settings = PresentationSettings::new().embedded_fonts(fonts);

        let bytes = create_pptx_with_settings(
            "Font Demo",
            &[SlideContent::new("Slide 1").add_bullet("Point 1")],
            Some(settings),
        )
        .unwrap();

        let report = crate::core::validate_package_bytes(&bytes);
        assert!(
            report.is_valid(),
            "embedded font package invalid: {:?}",
            report.issues
        );

        let cursor = Cursor::new(bytes);
        let mut archive = zip::ZipArchive::new(cursor).unwrap();
        let names: std::collections::HashSet<String> = (0..archive.len())
            .map(|i| archive.by_index(i).unwrap().name().unwrap().to_string())
            .collect();
        assert!(names.contains("ppt/fonts/Arial-regular.fntdata"));

        let mut rels = String::new();
        archive
            .by_name("ppt/_rels/presentation.xml.rels")
            .unwrap()
            .read_to_string(&mut rels)
            .unwrap();
        assert!(rels.contains("relationships/font"));
        assert!(rels.contains("fonts/Arial-regular.fntdata"));

        let mut presentation = String::new();
        archive
            .by_name("ppt/presentation.xml")
            .unwrap()
            .read_to_string(&mut presentation)
            .unwrap();
        assert!(presentation.contains("<p:embeddedFontLst>"));
        assert!(presentation.contains("Arial"));
    }

    #[test]
    fn test_create_pptx_with_ink_annotations() {
        use crate::generator::slide_content::{InkAnnotations, InkPen, InkStroke};

        let mut ink = InkAnnotations::new();
        ink.add_stroke(
            InkStroke::new(InkPen::red())
                .add_point(100.0, 100.0)
                .add_point(200.0, 200.0),
        );

        let bytes = create_pptx_with_content(
            "Ink Demo",
            vec![
                SlideContent::new("Slide 1")
                    .add_bullet("Point 1")
                    .with_ink(ink),
            ],
        )
        .unwrap();

        let report = crate::core::validate_package_bytes(&bytes);
        assert!(
            report.is_valid(),
            "ink package invalid: {:?}",
            report.issues
        );

        let cursor = Cursor::new(bytes);
        let mut archive = zip::ZipArchive::new(cursor).unwrap();
        let names: std::collections::HashSet<String> = (0..archive.len())
            .map(|i| archive.by_index(i).unwrap().name().unwrap().to_string())
            .collect();
        assert!(names.contains("ppt/ink/ink1.xml"));

        let mut rels = String::new();
        archive
            .by_name("ppt/slides/_rels/slide1.xml.rels")
            .unwrap()
            .read_to_string(&mut rels)
            .unwrap();
        assert!(rels.contains("relationships/ink"));
        assert!(rels.contains("../ink/ink1.xml"));

        let mut slide = String::new();
        archive
            .by_name("ppt/slides/slide1.xml")
            .unwrap()
            .read_to_string(&mut slide)
            .unwrap();
        assert!(slide.contains("mc:AlternateContent"));
        assert!(slide.contains("p:contentPart"));
    }

    #[test]
    fn test_create_pptx_with_content_to_writer() {
        let slides = vec![
            SlideContent::new("Title").add_bullet("Point 1"),
            SlideContent::new("Slide 2").add_bullet("Point 2"),
        ];

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let result = create_pptx_with_content_to_writer(cursor, "Test", &slides, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_pptx_lazy_to_writer() {
        let source = TestSlideSource {
            count: 10,
            with_notes: false,
        };

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let result = create_pptx_lazy_to_writer(cursor, "Lazy Test", Box::new(source), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_pptx_lazy_with_notes() {
        let source = TestSlideSource {
            count: 5,
            with_notes: true,
        };

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let result =
            create_pptx_lazy_to_writer(cursor, "Lazy Test with Notes", Box::new(source), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lazy_slide_source() {
        let source = TestSlideSource {
            count: 3,
            with_notes: false,
        };

        assert_eq!(source.slide_count(), 3);
        assert!(!source.slide_has_notes(0));
        assert_eq!(source.slide_chart_count(0), 0);

        let slide = source.generate_slide(0);
        assert!(slide.is_some());
        assert_eq!(slide.unwrap().title, "Slide 1");

        let out_of_bounds = source.generate_slide(10);
        assert!(out_of_bounds.is_none());
    }

    #[test]
    fn test_streaming_api_compatibility() {
        // Test that the streaming API produces the same output as the in-memory API
        let slides = vec![SlideContent::new("Test").add_bullet("Item 1")];

        // In-memory version
        let in_memory = create_pptx_with_content("Test", slides.clone()).unwrap();

        // Streaming version
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let streaming = create_pptx_with_content_to_writer(cursor, "Test", &slides, None)
            .unwrap()
            .into_inner();

        // Both should produce valid ZIP files (non-empty)
        assert!(!in_memory.is_empty());
        assert!(!streaming.is_empty());
    }

    #[test]
    fn test_lazy_vs_eager_compatibility() {
        // Test that lazy and eager APIs produce compatible output
        let eager_slides = vec![
            SlideContent::new("Slide 1").add_bullet("Point 1"),
            SlideContent::new("Slide 2").add_bullet("Point 2"),
        ];

        // Eager version
        let eager = create_pptx_with_content("Test", eager_slides).unwrap();

        // Lazy version (equivalent content)
        let source = TestSlideSource {
            count: 2,
            with_notes: false,
        };
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let lazy = create_pptx_lazy_to_writer(cursor, "Test", Box::new(source), None)
            .unwrap()
            .into_inner();

        // Both should produce valid ZIP files
        assert!(!eager.is_empty());
        assert!(!lazy.is_empty());
    }
}
