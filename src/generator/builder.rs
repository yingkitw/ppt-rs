//! PPTX builder - orchestrates ZIP creation and file writing

use std::io::{Write, Cursor};
use zip::ZipWriter;
use zip::write::FileOptions;
use super::xml::*;
use super::notes_xml::*;
use super::package_xml::{
    create_content_types_xml_with_notes_and_charts,
    create_presentation_rels_xml_with_notes,
    create_slide_rels_xml_extended
};
use crate::generator::charts::generate_chart_part_xml;

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
    // Check if any slides have notes and calculate chart info
    let has_notes = custom_slides
        .map(|slides| slides.iter().any(|s| s.notes.is_some()))
        .unwrap_or(false);
    
    let mut total_charts = 0;
    let mut slide_chart_start_indices = Vec::new();
    if let Some(slides) = custom_slides {
        for slide in slides {
            slide_chart_start_indices.push(total_charts + 1);
            total_charts += slide.charts.len();
        }
    }

    // 1. Content types (with notes and charts)
    let content_types = create_content_types_xml_with_notes_and_charts(slide_count, custom_slides, total_charts);
    zip.start_file("[Content_Types].xml", *options)?;
    zip.write_all(content_types.as_bytes())?;

    // 2. Package relationships
    let rels = create_rels_xml();
    zip.start_file("_rels/.rels", *options)?;
    zip.write_all(rels.as_bytes())?;

    // 3. Presentation relationships (with notes master if notes present)
    let pres_rels = if has_notes {
        create_presentation_rels_xml_with_notes(slide_count)
    } else {
        create_presentation_rels_xml(slide_count)
    };
    zip.start_file("ppt/_rels/presentation.xml.rels", *options)?;
    zip.write_all(pres_rels.as_bytes())?;

    // 4. Presentation document
    let presentation = create_presentation_xml(title, slide_count);
    zip.start_file("ppt/presentation.xml", *options)?;
    zip.write_all(presentation.as_bytes())?;

    // 5. Slides (and notes if present)
    write_slides(zip, options, slide_count, custom_slides)?;

    // 6. Slide relationships (with notes references if present)
    write_slide_relationships_extended(zip, options, custom_slides, &slide_chart_start_indices, slide_count)?;

    // 7. Notes relationships (if notes present)
    if has_notes {
        write_notes_relationships(zip, options, custom_slides)?;
        
        // Notes master
        let notes_master = create_notes_master_xml();
        zip.start_file("ppt/notesMasters/notesMaster1.xml", *options)?;
        zip.write_all(notes_master.as_bytes())?;
        
        // Notes master relationships
        let notes_master_rels = create_notes_master_rels_xml();
        zip.start_file("ppt/notesMasters/_rels/notesMaster1.xml.rels", *options)?;
        zip.write_all(notes_master_rels.as_bytes())?;
    }

    // 8. Slide layouts
    let slide_layout = create_slide_layout_xml();
    zip.start_file("ppt/slideLayouts/slideLayout1.xml", *options)?;
    zip.write_all(slide_layout.as_bytes())?;

    // 9. Layout relationships
    let layout_rels = create_layout_rels_xml();
    zip.start_file("ppt/slideLayouts/_rels/slideLayout1.xml.rels", *options)?;
    zip.write_all(layout_rels.as_bytes())?;

    // 10. Slide master
    let slide_master = create_slide_master_xml();
    zip.start_file("ppt/slideMasters/slideMaster1.xml", *options)?;
    zip.write_all(slide_master.as_bytes())?;

    // 11. Master relationships
    let master_rels = create_master_rels_xml();
    zip.start_file("ppt/slideMasters/_rels/slideMaster1.xml.rels", *options)?;
    zip.write_all(master_rels.as_bytes())?;

    // 12. Theme
    let theme = create_theme_xml();
    zip.start_file("ppt/theme/theme1.xml", *options)?;
    zip.write_all(theme.as_bytes())?;

    // 13. Core properties
    let core_props = create_core_props_xml(title);
    zip.start_file("docProps/core.xml", *options)?;
    zip.write_all(core_props.as_bytes())?;

    // 14. App properties
    let app_props = create_app_props_xml(slide_count);
    zip.start_file("docProps/app.xml", *options)?;
    zip.write_all(app_props.as_bytes())?;

    // 15. Charts
    if total_charts > 0 {
        write_charts(zip, options, custom_slides, &slide_chart_start_indices)?;
    }

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

/// Write slide relationship files with notes and charts
fn write_slide_relationships_extended(
    zip: &mut ZipWriter<Cursor<Vec<u8>>>,
    options: &FileOptions,
    custom_slides: Option<&Vec<super::xml::SlideContent>>,
    slide_chart_start_indices: &[usize],
    slide_count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    match custom_slides {
        Some(slides) => {
            for (i, slide) in slides.iter().enumerate() {
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

/// Write chart files
fn write_charts(
    zip: &mut ZipWriter<Cursor<Vec<u8>>>,
    options: &FileOptions,
    custom_slides: Option<&Vec<super::xml::SlideContent>>,
    slide_chart_start_indices: &[usize],
) -> Result<(), Box<dyn std::error::Error>> {
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

/// Write notes relationship files
fn write_notes_relationships(
    zip: &mut ZipWriter<Cursor<Vec<u8>>>,
    options: &FileOptions,
    custom_slides: Option<&Vec<super::xml::SlideContent>>,
) -> Result<(), Box<dyn std::error::Error>> {
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
