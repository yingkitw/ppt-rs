//! PPTX builder - orchestrates ZIP creation and file writing

use std::io::{Write, Cursor};
use zip::ZipWriter;
use zip::write::FileOptions;
use super::xml::*;

/// Create a minimal but valid PPTX file
pub fn create_pptx(title: &str, slides: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let buffer = Vec::new();
    let cursor = Cursor::new(buffer);
    let mut zip = ZipWriter::new(cursor);
    let options = FileOptions::default();

    write_package_files(&mut zip, &options, title, slides, None)?;

    let cursor = zip.finish()?;
    Ok(cursor.into_inner())
}

/// Create a PPTX file with custom slide content
pub fn create_pptx_with_content(
    title: &str,
    slides: Vec<super::xml::SlideContent>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let buffer = Vec::new();
    let cursor = Cursor::new(buffer);
    let mut zip = ZipWriter::new(cursor);
    let options = FileOptions::default();

    write_package_files(&mut zip, &options, title, slides.len(), Some(&slides))?;

    let cursor = zip.finish()?;
    Ok(cursor.into_inner())
}

/// Write all package files to the ZIP archive
fn write_package_files(
    zip: &mut ZipWriter<Cursor<Vec<u8>>>,
    options: &FileOptions,
    title: &str,
    slide_count: usize,
    custom_slides: Option<&Vec<super::xml::SlideContent>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Content types
    let content_types = create_content_types_xml(slide_count);
    zip.start_file("[Content_Types].xml", *options)?;
    zip.write_all(content_types.as_bytes())?;

    // 2. Package relationships
    let rels = create_rels_xml();
    zip.start_file("_rels/.rels", *options)?;
    zip.write_all(rels.as_bytes())?;

    // 3. Presentation relationships
    let pres_rels = create_presentation_rels_xml(slide_count);
    zip.start_file("ppt/_rels/presentation.xml.rels", *options)?;
    zip.write_all(pres_rels.as_bytes())?;

    // 4. Presentation document
    let presentation = create_presentation_xml(title, slide_count);
    zip.start_file("ppt/presentation.xml", *options)?;
    zip.write_all(presentation.as_bytes())?;

    // 5. Slides
    write_slides(zip, options, slide_count, custom_slides)?;

    // 6. Slide relationships
    write_slide_relationships(zip, options, slide_count)?;

    // 7. Slide layouts
    let slide_layout = create_slide_layout_xml();
    zip.start_file("ppt/slideLayouts/slideLayout1.xml", *options)?;
    zip.write_all(slide_layout.as_bytes())?;

    // 8. Layout relationships
    let layout_rels = create_layout_rels_xml();
    zip.start_file("ppt/slideLayouts/_rels/slideLayout1.xml.rels", *options)?;
    zip.write_all(layout_rels.as_bytes())?;

    // 9. Slide master
    let slide_master = create_slide_master_xml();
    zip.start_file("ppt/slideMasters/slideMaster1.xml", *options)?;
    zip.write_all(slide_master.as_bytes())?;

    // 10. Master relationships
    let master_rels = create_master_rels_xml();
    zip.start_file("ppt/slideMasters/_rels/slideMaster1.xml.rels", *options)?;
    zip.write_all(master_rels.as_bytes())?;

    // 11. Theme
    let theme = create_theme_xml();
    zip.start_file("ppt/theme/theme1.xml", *options)?;
    zip.write_all(theme.as_bytes())?;

    // 12. Core properties
    let core_props = create_core_props_xml(title);
    zip.start_file("docProps/core.xml", *options)?;
    zip.write_all(core_props.as_bytes())?;

    // 13. App properties
    let app_props = create_app_props_xml(slide_count);
    zip.start_file("docProps/app.xml", *options)?;
    zip.write_all(app_props.as_bytes())?;

    Ok(())
}

/// Write slide XML files
fn write_slides(
    zip: &mut ZipWriter<Cursor<Vec<u8>>>,
    options: &FileOptions,
    slide_count: usize,
    custom_slides: Option<&Vec<super::xml::SlideContent>>,
) -> Result<(), Box<dyn std::error::Error>> {
    match custom_slides {
        Some(slides) => {
            for (i, slide) in slides.iter().enumerate() {
                let slide_num = i + 1;
                let slide_xml = create_slide_xml_with_content(slide_num, slide);
                zip.start_file(&format!("ppt/slides/slide{}.xml", slide_num), *options)?;
                zip.write_all(slide_xml.as_bytes())?;
            }
        }
        None => {
            for i in 1..=slide_count {
                let slide_xml = create_slide_xml(i, "Presentation");
                zip.start_file(&format!("ppt/slides/slide{}.xml", i), *options)?;
                zip.write_all(slide_xml.as_bytes())?;
            }
        }
    }
    Ok(())
}

/// Write slide relationship files
fn write_slide_relationships(
    zip: &mut ZipWriter<Cursor<Vec<u8>>>,
    options: &FileOptions,
    slide_count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    for i in 1..=slide_count {
        let slide_rels = create_slide_rels_xml();
        zip.start_file(&format!("ppt/slides/_rels/slide{}.xml.rels", i), *options)?;
        zip.write_all(slide_rels.as_bytes())?;
    }
    Ok(())
}
