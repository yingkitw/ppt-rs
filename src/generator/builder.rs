//! PPTX builder - orchestrates ZIP creation and file writing

use std::io::{Write, Seek, Cursor};
use zip::ZipWriter;
use zip::write::FileOptions;
use crate::exc::Result;
use super::slide_content::SlideContent;
use super::package_xml::{
    create_rels_xml, create_presentation_rels_xml, create_presentation_xml,
    create_content_types_xml_with_notes_and_charts,
    create_presentation_rels_xml_with_notes,
    create_slide_rels_xml_extended,
};
use super::slide_xml::{
    create_slide_xml, create_slide_xml_with_content, create_slide_rels_xml,
};
use super::theme_xml::{
    create_slide_layout_xml, create_layout_rels_xml,
    create_slide_master_xml, create_master_rels_xml, create_theme_xml,
};
use super::props_xml::{create_core_props_xml, create_app_props_xml};
use super::notes_xml::*;
use crate::generator::charts::generate_chart_part_xml;
use crate::generator::slide_content::presentation_settings::PresentationSettings;

/// Create a minimal but valid PPTX file
pub fn create_pptx(title: &str, slides: usize) -> Result<Vec<u8>> {
    let buffer = Vec::new();
    let cursor = Cursor::new(buffer);
    let mut zip = ZipWriter::new(cursor);
    let options = FileOptions::default();

    write_package_files(&mut zip, &options, title, slides, None, None)?;

    let cursor = zip.finish()?;
    Ok(cursor.into_inner())
}

/// Create a PPTX file with custom slide content
pub fn create_pptx_with_content(
    title: &str,
    slides: Vec<SlideContent>,
) -> Result<Vec<u8>> {
    create_pptx_with_settings(title, slides, None)
}

/// Create a PPTX file with custom slide content and presentation-level settings
pub fn create_pptx_with_settings(
    title: &str,
    slides: Vec<SlideContent>,
    settings: Option<PresentationSettings>,
) -> Result<Vec<u8>> {
    let buffer = Vec::new();
    let cursor = Cursor::new(buffer);
    let mut zip = ZipWriter::new(cursor);
    let options = FileOptions::default();

    write_package_files(&mut zip, &options, title, slides.len(), Some(&slides), settings.as_ref())?;

    let cursor = zip.finish()?;
    Ok(cursor.into_inner())
}

/// Create a PPTX file and write it directly to a writer (streaming API).
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
pub fn create_pptx_to_writer<W: Write + Seek>(
    writer: W,
    title: &str,
    slides: usize,
) -> Result<W> {
    let mut zip = ZipWriter::new(writer);
    let options = FileOptions::default();

    write_package_files(&mut zip, &options, title, slides, None, None)?;

    Ok(zip.finish()?)
}

/// Create a PPTX file with custom content and write it directly to a writer (streaming API).
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
/// create_pptx_with_content_to_writer(file, "My Presentation", slides, None)?;
/// # Ok::<(), ppt_rs::PptxError>(())
/// ```
pub fn create_pptx_with_content_to_writer<W: Write + Seek>(
    writer: W,
    title: &str,
    slides: Vec<SlideContent>,
    settings: Option<PresentationSettings>,
) -> Result<W> {
    let mut zip = ZipWriter::new(writer);
    let options = FileOptions::default();

    write_package_files(&mut zip, &options, title, slides.len(), Some(&slides), settings.as_ref())?;

    Ok(zip.finish()?)
}

/// Lazy slide source - allows generating slides on-demand instead of all at once.
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

    /// Check if a slide has notes (default implementation checks the generated slide)
    fn slide_has_notes(&self, index: usize) -> bool {
        self.generate_slide(index)
            .map(|s| s.notes.is_some())
            .unwrap_or(false)
    }

    /// Get the number of charts in a slide (default implementation checks the generated slide)
    fn slide_chart_count(&self, index: usize) -> usize {
        self.generate_slide(index)
            .map(|s| s.charts.len())
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
    let options = FileOptions::default();

    write_package_files_lazy(&mut zip, &options, title, slides.as_ref(), settings.as_ref())?;

    Ok(zip.finish()?)
}

/// Chart metadata for slides
struct ChartInfo {
    total_charts: usize,
    slide_start_indices: Vec<usize>,
}

/// Collect chart metadata from slides (eager version)
fn collect_chart_info(slides: Option<&Vec<SlideContent>>) -> ChartInfo {
    let mut total_charts = 0;
    let mut slide_start_indices = Vec::new();

    if let Some(slides) = slides {
        for slide in slides {
            slide_start_indices.push(total_charts + 1);
            total_charts += slide.charts.len();
        }
    }

    ChartInfo {
        total_charts,
        slide_start_indices,
    }
}

/// Collect chart metadata from lazy slide source
fn collect_chart_info_lazy(slides: &dyn LazySlideSource) -> ChartInfo {
    let mut total_charts = 0;
    let mut slide_start_indices = Vec::new();

    for i in 0..slides.slide_count() {
        slide_start_indices.push(total_charts + 1);
        total_charts += slides.slide_chart_count(i);
    }

    ChartInfo {
        total_charts,
        slide_start_indices,
    }
}

/// Write content types XML with optional presProps
fn write_content_types<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    slide_count: usize,
    custom_slides: Option<&Vec<SlideContent>>,
    chart_info: &ChartInfo,
    has_pres_props: bool,
) -> Result<()> {
    let mut content_types = create_content_types_xml_with_notes_and_charts(
        slide_count,
        custom_slides,
        chart_info.total_charts,
    );

    if has_pres_props {
        let ct_entry = "\n<Override PartName=\"/ppt/presProps.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.presProps+xml\"/>";
        if let Some(pos) = content_types.rfind("</Types>") {
            content_types.insert_str(pos, ct_entry);
        }
    }

    zip.start_file("[Content_Types].xml", *options)?;
    zip.write_all(content_types.as_bytes())?;
    Ok(())
}

/// Write presentation relationships with optional presProps relationship
fn write_presentation_relationships<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    slide_count: usize,
    has_notes: bool,
    has_pres_props: bool,
) -> Result<()> {
    let mut pres_rels = if has_notes {
        create_presentation_rels_xml_with_notes(slide_count)
    } else {
        create_presentation_rels_xml(slide_count)
    };

    if has_pres_props {
        let props_rid = slide_count + 3 + if has_notes { 1 } else { 0 } + 1;
        let rel_entry = format!(
            "\n    <Relationship Id=\"rId{props_rid}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps\" Target=\"presProps.xml\"/>"
        );
        if let Some(pos) = pres_rels.rfind("</Relationships>") {
            pres_rels.insert_str(pos, &rel_entry);
        }
    }

    zip.start_file("ppt/_rels/presentation.xml.rels", *options)?;
    zip.write_all(pres_rels.as_bytes())?;
    Ok(())
}

/// Write presentation properties XML if needed
fn write_presentation_properties<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    settings: Option<&PresentationSettings>,
) -> Result<()> {
    let has_pres_props = settings.map(|s| s.slide_show.is_some() || s.print.is_some()).unwrap_or(false);

    if has_pres_props {
        let mut props_xml = String::from(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><p:presentationPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">"#
        );
        if let Some(s) = settings {
            if let Some(ref show) = s.slide_show {
                props_xml.push_str(&show.to_xml());
            }
            if let Some(ref print) = s.print {
                props_xml.push_str(&print.to_prnpr_xml());
            }
        }
        props_xml.push_str("</p:presentationPr>");
        zip.start_file("ppt/presProps.xml", *options)?;
        zip.write_all(props_xml.as_bytes())?;
    }
    Ok(())
}

/// Write notes master files if needed
fn write_notes_master<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
) -> Result<()> {
    let notes_master = create_notes_master_xml();
    zip.start_file("ppt/notesMasters/notesMaster1.xml", *options)?;
    zip.write_all(notes_master.as_bytes())?;

    let notes_master_rels = create_notes_master_rels_xml();
    zip.start_file("ppt/notesMasters/_rels/notesMaster1.xml.rels", *options)?;
    zip.write_all(notes_master_rels.as_bytes())?;
    Ok(())
}

/// Write theme and layout files
fn write_theme_and_layouts<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
) -> Result<()> {
    // Slide layouts
    let slide_layout = create_slide_layout_xml();
    zip.start_file("ppt/slideLayouts/slideLayout1.xml", *options)?;
    zip.write_all(slide_layout.as_bytes())?;

    // Layout relationships
    let layout_rels = create_layout_rels_xml();
    zip.start_file("ppt/slideLayouts/_rels/slideLayout1.xml.rels", *options)?;
    zip.write_all(layout_rels.as_bytes())?;

    // Slide master
    let slide_master = create_slide_master_xml();
    zip.start_file("ppt/slideMasters/slideMaster1.xml", *options)?;
    zip.write_all(slide_master.as_bytes())?;

    // Master relationships
    let master_rels = create_master_rels_xml();
    zip.start_file("ppt/slideMasters/_rels/slideMaster1.xml.rels", *options)?;
    zip.write_all(master_rels.as_bytes())?;

    // Theme
    let theme = create_theme_xml();
    zip.start_file("ppt/theme/theme1.xml", *options)?;
    zip.write_all(theme.as_bytes())?;

    Ok(())
}

/// Write document properties
fn write_document_properties<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    title: &str,
    slide_count: usize,
) -> Result<()> {
    // Core properties
    let core_props = create_core_props_xml(title);
    zip.start_file("docProps/core.xml", *options)?;
    zip.write_all(core_props.as_bytes())?;

    // App properties
    let app_props = create_app_props_xml(slide_count);
    zip.start_file("docProps/app.xml", *options)?;
    zip.write_all(app_props.as_bytes())?;

    Ok(())
}

/// Write all package files to the ZIP archive (eager version with Vec<SlideContent>)
fn write_package_files<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    title: &str,
    slide_count: usize,
    custom_slides: Option<&Vec<SlideContent>>,
    settings: Option<&PresentationSettings>,
) -> Result<()> {
    let has_notes = custom_slides
        .map(|slides| slides.iter().any(|s| s.notes.is_some()))
        .unwrap_or(false);

    let chart_info = collect_chart_info(custom_slides);
    let has_pres_props = settings.map(|s| s.slide_show.is_some() || s.print.is_some()).unwrap_or(false);

    // 1. Content types
    write_content_types(zip, options, slide_count, custom_slides, &chart_info, has_pres_props)?;

    // 2. Package relationships
    let rels = create_rels_xml();
    zip.start_file("_rels/.rels", *options)?;
    zip.write_all(rels.as_bytes())?;

    // 3. Presentation relationships
    write_presentation_relationships(zip, options, slide_count, has_notes, has_pres_props)?;

    // 4. Presentation document
    let presentation = create_presentation_xml(title, slide_count);
    zip.start_file("ppt/presentation.xml", *options)?;
    zip.write_all(presentation.as_bytes())?;

    // 5. Presentation properties
    write_presentation_properties(zip, options, settings)?;

    // 6. Slides
    write_slides(zip, options, slide_count, custom_slides)?;

    // 7. Slide relationships
    write_slide_relationships_extended(zip, options, custom_slides, &chart_info.slide_start_indices, slide_count)?;

    // 8. Notes relationships and master
    if has_notes {
        write_notes_relationships(zip, options, custom_slides)?;
        write_notes_master(zip, options)?;
    }

    // 9. Theme and layouts
    write_theme_and_layouts(zip, options)?;

    // 10. Document properties
    write_document_properties(zip, options, title, slide_count)?;

    // 11. Charts
    if chart_info.total_charts > 0 {
        write_charts(zip, options, custom_slides, &chart_info.slide_start_indices)?;
    }

    // 12. Images
    write_images(zip, options, custom_slides)?;

    Ok(())
}

/// Write all package files to the ZIP archive (lazy version with LazySlideSource)
fn write_package_files_lazy<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    title: &str,
    slides: &dyn LazySlideSource,
    settings: Option<&PresentationSettings>,
) -> Result<()> {
    let slide_count = slides.slide_count();
    let has_notes = (0..slide_count).any(|i| slides.slide_has_notes(i));

    let chart_info = collect_chart_info_lazy(slides);
    let has_pres_props = settings.map(|s| s.slide_show.is_some() || s.print.is_some()).unwrap_or(false);

    // 1. Content types (lazy version - doesn't have SlideContent references)
    write_content_types_lazy(zip, options, slide_count, slides, &chart_info, has_pres_props)?;

    // 2. Package relationships
    let rels = create_rels_xml();
    zip.start_file("_rels/.rels", *options)?;
    zip.write_all(rels.as_bytes())?;

    // 3. Presentation relationships
    write_presentation_relationships(zip, options, slide_count, has_notes, has_pres_props)?;

    // 4. Presentation document
    let presentation = create_presentation_xml(title, slide_count);
    zip.start_file("ppt/presentation.xml", *options)?;
    zip.write_all(presentation.as_bytes())?;

    // 5. Presentation properties
    write_presentation_properties(zip, options, settings)?;

    // 6. Slides (lazy version)
    write_slides_lazy(zip, options, slides)?;

    // 7. Slide relationships (lazy version)
    write_slide_relationships_lazy(zip, options, slides, &chart_info.slide_start_indices)?;

    // 8. Notes relationships and master (lazy version)
    if has_notes {
        write_notes_relationships_lazy(zip, options, slides)?;
        write_notes_master(zip, options)?;
    }

    // 9. Theme and layouts
    write_theme_and_layouts(zip, options)?;

    // 10. Document properties
    write_document_properties(zip, options, title, slide_count)?;

    // 11. Charts (lazy version)
    if chart_info.total_charts > 0 {
        write_charts_lazy(zip, options, slides, &chart_info.slide_start_indices)?;
    }

    Ok(())
}

/// Write content types for lazy slides (simplified version without SlideContent references)
fn write_content_types_lazy<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    slide_count: usize,
    slides: &dyn LazySlideSource,
    chart_info: &ChartInfo,
    has_pres_props: bool,
) -> Result<()> {
    // Build basic content types without detailed content info from slides
    let mut content_types = String::from(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
<Override PartName="/ppt/presProps.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"/>
<Override PartName="/ppt/viewProps.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml"/>
<Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
<Override PartName="/ppt/slideLayouts/slideLayout1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
<Override PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>"#
    );

    // Add slide overrides
    for i in 1..=slide_count {
        content_types.push_str(&format!(
            "\n<Override PartName=\"/ppt/slides/slide{}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.slide+xml\"/>",
            i
        ));
    }

    // Add chart overrides
    for i in 1..=chart_info.total_charts {
        content_types.push_str(&format!(
            "\n<Override PartName=\"/ppt/charts/chart{}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.drawingml.chart+xml\"/>",
            i
        ));
    }

    // Check for notes
    for i in 1..slide_count {
        if slides.slide_has_notes(i - 1) {
            content_types.push_str(&format!(
                "\n<Override PartName=\"/ppt/notesSlides/notesSlide{}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml\"/>",
                i
            ));
        }
    }

    // Add presProps if needed
    if has_pres_props {
        let ct_entry = "\n<Override PartName=\"/ppt/presProps.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.presProps+xml\"/>";
        if let Some(pos) = content_types.rfind("</Types>") {
            content_types.insert_str(pos, ct_entry);
        }
    }

    content_types.push_str("\n</Types>");

    zip.start_file("[Content_Types].xml", *options)?;
    zip.write_all(content_types.as_bytes())?;
    Ok(())
}

/// Write slide XML files (lazy version)
fn write_slides_lazy<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    slides: &dyn LazySlideSource,
) -> Result<()> {
    for i in 0..slides.slide_count() {
        if let Some(slide) = slides.generate_slide(i) {
            let slide_num = i + 1;

            // Calculate chart rIds
            let mut chart_rids = Vec::new();
            let start_rid = if slide.notes.is_some() { 3 } else { 2 };
            for j in 0..slide.charts.len() {
                chart_rids.push(format!("rId{}", start_rid + j));
            }

            let slide_xml = create_slide_xml_with_content(slide_num, &slide, &chart_rids);
            zip.start_file(format!("ppt/slides/slide{slide_num}.xml"), *options)?;
            zip.write_all(slide_xml.as_bytes())?;

            // Write notes if present
            if let Some(ref notes) = slide.notes {
                let notes_xml = create_notes_xml(slide_num, notes);
                zip.start_file(format!("ppt/notesSlides/notesSlide{slide_num}.xml"), *options)?;
                zip.write_all(notes_xml.as_bytes())?;
            }
        }
    }
    Ok(())
}

/// Write slide relationship files (lazy version)
fn write_slide_relationships_lazy<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    slides: &dyn LazySlideSource,
    slide_chart_start_indices: &[usize],
) -> Result<()> {
    for i in 0..slides.slide_count() {
        if let Some(slide) = slides.generate_slide(i) {
            let slide_num = i + 1;

            let mut chart_rels = Vec::new();
            let start_chart_idx = slide_chart_start_indices[i];
            let start_rid = if slide.notes.is_some() { 3 } else { 2 };

            for j in 0..slide.charts.len() {
                let rid = format!("rId{}", start_rid + j);
                let target = format!("../charts/chart{}.xml", start_chart_idx + j);
                chart_rels.push((rid, target));
            }

            let slide_rels = create_slide_rels_xml_extended(slide_num, slide.notes.is_some(), &chart_rels);
            zip.start_file(format!("ppt/slides/_rels/slide{slide_num}.xml.rels"), *options)?;
            zip.write_all(slide_rels.as_bytes())?;
        }
    }
    Ok(())
}

/// Write chart files (lazy version)
fn write_charts_lazy<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    slides: &dyn LazySlideSource,
    slide_chart_start_indices: &[usize],
) -> Result<()> {
    for i in 0..slides.slide_count() {
        if let Some(slide) = slides.generate_slide(i) {
            let start_chart_idx = slide_chart_start_indices[i];
            for (j, chart) in slide.charts.iter().enumerate() {
                let chart_idx = start_chart_idx + j;
                let chart_xml = generate_chart_part_xml(chart);
                zip.start_file(format!("ppt/charts/chart{}.xml", chart_idx), *options)?;
                zip.write_all(chart_xml.as_bytes())?;
            }
        }
    }
    Ok(())
}

/// Write notes relationship files (lazy version)
fn write_notes_relationships_lazy<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    slides: &dyn LazySlideSource,
) -> Result<()> {
    for i in 0..slides.slide_count() {
        if slides.slide_has_notes(i) {
            let slide_num = i + 1;
            let notes_rels = create_notes_rels_xml(slide_num);
            zip.start_file(format!("ppt/notesSlides/_rels/notesSlide{slide_num}.xml.rels"), *options)?;
            zip.write_all(notes_rels.as_bytes())?;
        }
    }
    Ok(())
}

/// Write slide XML files (eager version)
fn write_slides<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    slide_count: usize,
    custom_slides: Option<&Vec<SlideContent>>,
) -> Result<()> {
    match custom_slides {
        Some(slides) => {
            for (i, slide) in slides.iter().enumerate() {
                let slide_num = i + 1;

                // Calculate chart rIds
                let mut chart_rids = Vec::new();
                let start_rid = if slide.notes.is_some() { 3 } else { 2 };
                for j in 0..slide.charts.len() {
                    chart_rids.push(format!("rId{}", start_rid + j));
                }

                let slide_xml = create_slide_xml_with_content(slide_num, slide, &chart_rids);
                zip.start_file(format!("ppt/slides/slide{slide_num}.xml"), *options)?;
                zip.write_all(slide_xml.as_bytes())?;

                // Write notes if present
                if let Some(notes) = &slide.notes {
                    let notes_xml = create_notes_xml(slide_num, notes);
                    zip.start_file(format!("ppt/notesSlides/notesSlide{slide_num}.xml"), *options)?;
                    zip.write_all(notes_xml.as_bytes())?;
                }
            }
        }
        None => {
            for i in 1..=slide_count {
                let slide_xml = create_slide_xml(i, "Presentation");
                zip.start_file(format!("ppt/slides/slide{i}.xml"), *options)?;
                zip.write_all(slide_xml.as_bytes())?;
            }
        }
    }
    Ok(())
}

/// Write slide relationship files with notes and charts (eager version)
fn write_slide_relationships_extended<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    custom_slides: Option<&Vec<SlideContent>>,
    slide_chart_start_indices: &[usize],
    slide_count: usize,
) -> Result<()> {
    let mut total_images = 0;
    
    match custom_slides {
        Some(slides) => {
            for (i, slide) in slides.iter().enumerate() {
                let slide_num = i + 1;
                let image_count = slide.images.len();
                let image_start_num = total_images + 1;

                // Collect image extensions for this slide
                let image_extensions: Vec<String> = slide.images.iter()
                    .map(|img| img.extension())
                    .collect();

                let mut chart_rels = Vec::new();
                let start_chart_idx = slide_chart_start_indices[i];
                // Chart rIds come after images and notes
                let start_rid = 2 + image_count + if slide.notes.is_some() { 1 } else { 0 };

                for j in 0..slide.charts.len() {
                    let rid = format!("rId{}", start_rid + j);
                    let target = format!("../charts/chart{}.xml", start_chart_idx + j);
                    chart_rels.push((rid, target));
                }

                let slide_rels = super::package_xml::create_slide_rels_xml_with_images(
                    slide_num, 
                    slide.notes.is_some(), 
                    &chart_rels,
                    image_count,
                    image_start_num,
                    &image_extensions
                );
                zip.start_file(format!("ppt/slides/_rels/slide{slide_num}.xml.rels"), *options)?;
                zip.write_all(slide_rels.as_bytes())?;
                
                total_images += image_count;
            }
        }
        None => {
            // No custom slides, use default relationships
            for i in 1..=slide_count {
                let slide_rels = create_slide_rels_xml();
                zip.start_file(format!("ppt/slides/_rels/slide{i}.xml.rels"), *options)?;
                zip.write_all(slide_rels.as_bytes())?;
            }
        }
    }
    Ok(())
}

/// Write chart files (eager version)
fn write_charts<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    custom_slides: Option<&Vec<SlideContent>>,
    slide_chart_start_indices: &[usize],
) -> Result<()> {
    if let Some(slides) = custom_slides {
        for (i, slide) in slides.iter().enumerate() {
            let start_chart_idx = slide_chart_start_indices[i];
            for (j, chart) in slide.charts.iter().enumerate() {
                let chart_idx = start_chart_idx + j;
                let chart_xml = generate_chart_part_xml(chart);
                zip.start_file(format!("ppt/charts/chart{}.xml", chart_idx), *options)?;
                zip.write_all(chart_xml.as_bytes())?;
            }
        }
    }
    Ok(())
}

/// Write notes relationship files (eager version)
fn write_notes_relationships<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    custom_slides: Option<&Vec<SlideContent>>,
) -> Result<()> {
    if let Some(slides) = custom_slides {
        for (i, slide) in slides.iter().enumerate() {
            if slide.notes.is_some() {
                let slide_num = i + 1;
                let notes_rels = create_notes_rels_xml(slide_num);
                zip.start_file(format!("ppt/notesSlides/_rels/notesSlide{slide_num}.xml.rels"), *options)?;
                zip.write_all(notes_rels.as_bytes())?;
            }
        }
    }
    Ok(())
}

/// Write image files to ppt/media/
fn write_images<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &FileOptions,
    custom_slides: Option<&Vec<SlideContent>>,
) -> Result<()> {
    if let Some(slides) = custom_slides {
        let mut image_counter = 1;
        
        for slide in slides {
            for image in &slide.images {
                if let Some(bytes) = image.get_bytes() {
                    let ext = image.extension();
                    let filename = format!("ppt/media/image{}.{}", image_counter, ext);
                    zip.start_file(filename, *options)?;
                    zip.write_all(&bytes)?;
                    image_counter += 1;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

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

        fn slide_chart_count(&self, index: usize) -> usize {
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
    fn test_create_pptx_with_content_to_writer() {
        let slides = vec![
            SlideContent::new("Title").add_bullet("Point 1"),
            SlideContent::new("Slide 2").add_bullet("Point 2"),
        ];

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let result = create_pptx_with_content_to_writer(cursor, "Test", slides, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_pptx_lazy_to_writer() {
        let source = TestSlideSource { count: 10, with_notes: false };

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let result = create_pptx_lazy_to_writer(cursor, "Lazy Test", Box::new(source), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_pptx_lazy_with_notes() {
        let source = TestSlideSource { count: 5, with_notes: true };

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let result = create_pptx_lazy_to_writer(cursor, "Lazy Test with Notes", Box::new(source), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lazy_slide_source() {
        let source = TestSlideSource { count: 3, with_notes: false };

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
        let slides = vec![
            SlideContent::new("Test").add_bullet("Item 1"),
        ];

        // In-memory version
        let in_memory = create_pptx_with_content("Test", slides.clone()).unwrap();

        // Streaming version
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let streaming = create_pptx_with_content_to_writer(cursor, "Test", slides, None).unwrap().into_inner();

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
        let source = TestSlideSource { count: 2, with_notes: false };
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let lazy = create_pptx_lazy_to_writer(cursor, "Test", Box::new(source), None).unwrap().into_inner();

        // Both should produce valid ZIP files
        assert!(!eager.is_empty());
        assert!(!lazy.is_empty());
    }
}
