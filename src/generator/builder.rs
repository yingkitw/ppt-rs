//! PPTX builder - orchestrates ZIP creation and file writing

use std::io::{Write, Cursor};
use zip::ZipWriter;
use zip::write::FileOptions;
use super::xml::*;
use super::theme_xml::*;
use super::notes_xml::*;
use super::package_xml::{create_content_types_xml_with_notes, create_content_types_xml_with_charts, create_presentation_rels_xml_with_notes, create_slide_rels_xml_with_notes, create_slide_rels_xml_with_charts, create_slide_rels_xml_with_notes_and_charts};

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
    // Check if any slides have notes or charts
    let has_notes = custom_slides
        .map(|slides| slides.iter().any(|s| s.notes.is_some()))
        .unwrap_or(false);
    let has_charts = custom_slides
        .map(|slides| slides.iter().any(|s| !s.charts.is_empty()))
        .unwrap_or(false);
    
    println!("DEBUG: has_notes={}, has_charts={}", has_notes, has_charts);
    if let Some(slides) = custom_slides {
        for (i, slide) in slides.iter().enumerate() {
            println!("DEBUG: Slide {} has {} charts", i+1, slide.charts.len());
        }
    }

    // 1. Content types (with notes or charts if present)
    let content_types = if has_notes {
        create_content_types_xml_with_notes(slide_count, custom_slides)
    } else if has_charts {
        create_content_types_xml_with_charts(slide_count, custom_slides)
    } else {
        create_content_types_xml(slide_count)
    };
    println!("DEBUG: About to write content types XML length: {}", content_types.len());
    println!("DEBUG: Content types XML contains 'xlsx': {}", content_types.contains("xlsx"));
    println!("DEBUG: Content types XML contains 'chartcolorstyle': {}", content_types.contains("chartcolorstyle"));
    println!("DEBUG: First 500 chars of content types: {}", &content_types[..content_types.len().min(500)]);
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
    write_slide_relationships_with_notes(zip, options, custom_slides)?;

    // 6.5. Charts (if charts present)
    write_charts(zip, options, custom_slides)?;

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

    // 8. Slide layouts - generate all 11 layout types to match WPS
    let layout_types = vec![
        "title",           // 1. Title slide
        "obj",             // 2. Title and Content
        "secHead",         // 3. Section Header
        "twoObj",          // 4. Two Content
        "twoTxTwoObj",     // 5. Comparison
        "titleOnly",       // 6. Title Only
        "blank",           // 7. Blank
        "objTx",           // 8. Content with Caption
        "picTx",           // 9. Picture with Caption
        "vertTx",          // 10. Title and Vertical Text
        "vertTitleAndTx",  // 11. Vertical Title and Text
    ];
    
    for (i, layout_type) in layout_types.iter().enumerate() {
        let layout_num = i + 1;
        let slide_layout = create_slide_layout_xml_by_type(layout_type, layout_num);
        zip.start_file(format!("ppt/slideLayouts/slideLayout{}.xml", layout_num), *options)?;
        zip.write_all(slide_layout.as_bytes())?;
        
        // Create relationship file for each layout
        let layout_rels = create_layout_rels_xml_for_layout(layout_num);
        zip.start_file(format!("ppt/slideLayouts/_rels/slideLayout{}.xml.rels", layout_num), *options)?;
        zip.write_all(layout_rels.as_bytes())?;
    }

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

/// Write slide relationship files
#[allow(dead_code)]
fn write_slide_relationships(
    zip: &mut ZipWriter<Cursor<Vec<u8>>>,
    options: &FileOptions,
    slide_count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    for i in 1..=slide_count {
        let slide_rels = create_slide_rels_xml();
        zip.start_file(format!("ppt/slides/_rels/slide{i}.xml.rels"), *options)?;
        zip.write_all(slide_rels.as_bytes())?;
    }
    Ok(())
}

/// Write slide relationship files with notes references
fn write_slide_relationships_with_notes(
    zip: &mut ZipWriter<Cursor<Vec<u8>>>,
    options: &FileOptions,
    custom_slides: Option<&Vec<super::xml::SlideContent>>,
) -> Result<(), Box<dyn std::error::Error>> {
    match custom_slides {
        Some(slides) => {
            let mut global_chart_counter = 0; // Initialize global chart counter
            for (i, slide) in slides.iter().enumerate() {
                let slide_num = i + 1;
                let chart_count = slide.charts.len();
                
                let slide_rels = if slide.notes.is_some() && chart_count > 0 {
                    create_slide_rels_xml_with_notes_and_charts(slide_num, chart_count, global_chart_counter)
                } else if slide.notes.is_some() {
                    create_slide_rels_xml_with_notes(slide_num)
                } else if chart_count > 0 {
                    create_slide_rels_xml_with_charts(slide_num, chart_count, global_chart_counter)
                } else {
                    create_slide_rels_xml()
                };
                
                // Increment global chart counter after processing this slide
                global_chart_counter += chart_count;
                
                zip.start_file(format!("ppt/slides/_rels/slide{slide_num}.xml.rels"), *options)?;
                zip.write_all(slide_rels.as_bytes())?;
            }
        }
        None => {
            // No custom slides, use default relationships
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

/// Write chart XML files and Excel data files
fn write_charts(
    zip: &mut ZipWriter<Cursor<Vec<u8>>>,
    options: &FileOptions,
    custom_slides: Option<&Vec<super::xml::SlideContent>>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(slides) = custom_slides {
        let mut chart_counter = 0; // Initialize chart counter for global chart numbering
        for (i, slide) in slides.iter().enumerate() {
            let slide_num = i + 1;
            for (chart_index, chart) in slide.charts.iter().enumerate() {
                // Calculate global chart number for standard naming (chart1.xml, chart2.xml, etc.)
                let global_chart_num = chart_counter + chart_index + 1;
                
                // Write chart XML with standard naming
                let chart_xml = crate::generator::charts::xml::generate_chart_data_xml(chart);
                zip.start_file(format!("ppt/charts/chart{global_chart_num}.xml"), *options)?;
                zip.write_all(chart_xml.as_bytes())?;
                
                // Write chart style XML with unique naming
                let style_xml = crate::generator::charts::style::generate_chart_style_xml(chart);
                zip.start_file(format!("ppt/charts/style{global_chart_num}.xml"), *options)?;
                zip.write_all(style_xml.as_bytes())?;
                
                // Write chart colors XML with unique naming
                let colors_xml = crate::generator::charts::style::generate_chart_colors_xml(chart);
                zip.start_file(format!("ppt/charts/colors{global_chart_num}.xml"), *options)?;
                zip.write_all(colors_xml.as_bytes())?;
                
                // Write chart relationship file with standard naming
                let chart_relationship_xml = crate::generator::charts::create_chart_relationship_xml_with_styles(
                    global_chart_num,
                    &format!("chart{global_chart_num}_data.xlsx")
                );
                zip.start_file(format!("ppt/charts/_rels/chart{global_chart_num}.xml.rels"), *options)?;
                zip.write_all(chart_relationship_xml.as_bytes())?;
                
                // Write Excel data file (move to embeddings directory)
                let excel_bytes = crate::generator::charts::excel::generate_excel_bytes(chart);
                zip.start_file(format!("ppt/embeddings/chart{global_chart_num}_data.xlsx"), *options)?;
                zip.write_all(&excel_bytes)?;
            }
            // Increment chart counter after processing all charts in this slide
            chart_counter += slide.charts.len();
        }
    }
    Ok(())
}
