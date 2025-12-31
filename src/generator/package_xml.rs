//! Package-level XML generation (content types, relationships, presentation)

/// Escape special XML characters
pub fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Create [Content_Types].xml
pub fn create_content_types_xml(slides: usize) -> String {
    let mut xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Default Extension="xlsx" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"/>

<Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>"#.to_string();

    for i in 1..=slides {
        xml.push_str(&format!(
            "\n<Override PartName=\"/ppt/slides/slide{i}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.slide+xml\"/>"
        ));
    }

    xml.push_str(r#"
<Override PartName="/ppt/slideLayouts/slideLayout1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
<Override PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>
<Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
<Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
<Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
</Types>"#);
    println!("DEBUG: Final XML content types: {}", xml);
    xml
}

/// Create [Content_Types].xml with chart support
pub fn create_content_types_xml_with_charts(slides: usize, custom_slides: Option<&Vec<super::slide_content::SlideContent>>) -> String {
    println!("DEBUG: create_content_types_xml_with_charts called with {} slides", slides);
    let mut xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="xlsx" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"/>
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>"#.to_string();
    
    println!("DEBUG: Initial XML after header: {}", xml);

    for i in 1..=slides {
        xml.push_str(&format!(
            "\n<Override PartName=\"/ppt/slides/slide{i}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.slide+xml\"/>"
        ));
    }

    // Add chart content types using global chart numbering
    if let Some(slides_vec) = custom_slides {
        println!("DEBUG: Processing {} custom slides", slides_vec.len());
        let mut global_chart_counter = 0; // Initialize chart counter for global chart numbering
        for (i, slide) in slides_vec.iter().enumerate() {
            println!("DEBUG: Slide {} has {} charts", i+1, slide.charts.len());
            if !slide.charts.is_empty() {
                let slide_num = i + 1;
                for chart_index in 0..slide.charts.len() {
                    let global_chart_num = global_chart_counter + chart_index + 1; // Global chart number (1-based)
                    println!("DEBUG: Adding chart content types for global chart {}", global_chart_num);
                    xml.push_str(&format!(
                        "\n<Override PartName=\"/ppt/charts/chart{global_chart_num}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.drawingml.chart+xml\"/>"
                    ));
                    xml.push_str(&format!(
                        "\n<Override PartName=\"/ppt/embeddings/chart{global_chart_num}_data.xlsx\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet\"/>"
                    ));
                    // Add chart style and color content types (using chart 1 for all charts as per WPS format)
                    xml.push_str(&format!(
                        "\n<Override PartName=\"/ppt/charts/colors{global_chart_num}.xml\" ContentType=\"application/vnd.ms-office.chartcolorstyle+xml\"/>"
                    ));
                    xml.push_str(&format!(
                        "\n<Override PartName=\"/ppt/charts/style{global_chart_num}.xml\" ContentType=\"application/vnd.ms-office.chartstyle+xml\"/>"
                    ));
                }
                // Increment chart counter after processing all charts in this slide
                global_chart_counter += slide.charts.len();
            }
        }
    } else {
        println!("DEBUG: No custom slides provided");
    }

    xml.push_str(r#"
<Override PartName="/ppt/slideLayouts/slideLayout1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
<Override PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>
<Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
<Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
<Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
</Types>"#);
    xml
}

/// Create _rels/.rels
pub fn create_rels_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
<Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>"#.to_string()
}

/// Create ppt/_rels/presentation.xml.rels
pub fn create_presentation_rels_xml(slides: usize) -> String {
    let mut xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>
    <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>"#.to_string();

    for i in 1..=slides {
        let rid = i + 2;
        xml.push_str(&format!(
            "\n    <Relationship Id=\"rId{rid}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide\" Target=\"slides/slide{i}.xml\"/>"
        ));
    }

    xml.push_str("\n</Relationships>");
    xml
}

/// Create ppt/presentation.xml
pub fn create_presentation_xml(_title: &str, slides: usize) -> String {
    let mut xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" saveSubsetFonts="1">
<p:sldMasterIdLst>
<p:sldMasterId id="2147483648" r:id="rId1"/>
</p:sldMasterIdLst>
<p:sldIdLst>"#.to_string();

    for i in 1..=slides {
        let id = 256 + i;
        let rid = i + 2;
        xml.push_str(&format!("\n<p:sldId id=\"{id}\" r:id=\"rId{rid}\"/>"));
    }

    xml.push_str(r#"
</p:sldIdLst>
<p:sldSz cx="9144000" cy="6858000" type="screen4x3"/>
<p:notesSz cx="6858000" cy="9144000"/>
</p:presentation>"#);
    xml
}

/// Create [Content_Types].xml with notes support
pub fn create_content_types_xml_with_notes(slides: usize, custom_slides: Option<&Vec<super::slide_content::SlideContent>>) -> String {
    let mut xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Default Extension="xlsx" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"/>

<Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>"#.to_string();

    for i in 1..=slides {
        xml.push_str(&format!(
            "\n<Override PartName=\"/ppt/slides/slide{i}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.slide+xml\"/>"
        ));
    }

    // Add notes content types
    if let Some(slides_vec) = custom_slides {
        for (i, slide) in slides_vec.iter().enumerate() {
            if slide.notes.is_some() {
                let slide_num = i + 1;
                xml.push_str(&format!(
                    "\n<Override PartName=\"/ppt/notesSlides/notesSlide{slide_num}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml\"/>"
                ));
            }
        }
        // Add notes master if any slide has notes
        if slides_vec.iter().any(|s| s.notes.is_some()) {
            xml.push_str("\n<Override PartName=\"/ppt/notesMasters/notesMaster1.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml\"/>");
        }
    }

    xml.push_str(r#"
<Override PartName="/ppt/slideLayouts/slideLayout1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
<Override PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>
<Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
<Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
<Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
</Types>"#);
    xml
}

/// Create ppt/_rels/presentation.xml.rels with notes master
pub fn create_presentation_rels_xml_with_notes(slides: usize) -> String {
    let mut xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>
    <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>"#.to_string();

    for i in 1..=slides {
        let rid = i + 2;
        xml.push_str(&format!(
            "\n    <Relationship Id=\"rId{rid}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide\" Target=\"slides/slide{i}.xml\"/>"
        ));
    }

    // Add notes master relationship
    let notes_master_rid = slides + 3;
    xml.push_str(&format!(
        "\n    <Relationship Id=\"rId{notes_master_rid}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster\" Target=\"notesMasters/notesMaster1.xml\"/>"
    ));

    xml.push_str("\n</Relationships>");
    xml
}

/// Create slide relationship XML with notes reference
pub fn create_slide_rels_xml_with_notes(slide_num: usize) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>
<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide" Target="../notesSlides/notesSlide{slide_num}.xml"/>
</Relationships>"#)
}

/// Create slide relationship XML with chart references
pub fn create_slide_rels_xml_with_charts(_slide_num: usize, chart_count: usize, global_chart_counter: usize) -> String {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>"#);
    
    // Add chart relationships (chart XML files) using global chart numbering
    for i in 0..chart_count {
        let chart_id = i + 2; // Start from rId2 since rId1 is for layout
        let global_chart_num = global_chart_counter + i + 1; // Global chart number (1-based)
        xml.push_str(&format!(
            r#"
<Relationship Id="rId{chart_id}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart" Target="../charts/chart{global_chart_num}.xml"/>"#
        ));
    }
    
    xml.push_str("\n</Relationships>");
    xml
}

/// Create slide relationship XML with both notes and charts
pub fn create_slide_rels_xml_with_notes_and_charts(slide_num: usize, chart_count: usize, global_chart_counter: usize) -> String {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>"#);
    
    // Add chart relationships (chart XML files) using global chart numbering
    for i in 0..chart_count {
        let chart_id = i + 2; // Start from rId2 since rId1 is for layout
        let global_chart_num = global_chart_counter + i + 1; // Global chart number (1-based)
        xml.push_str(&format!(
            r#"
<Relationship Id="rId{chart_id}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart" Target="../charts/chart{global_chart_num}.xml"/>"#
        ));
    }
    
    // Add notes slide relationship
    let notes_id = chart_count + 2; // Continue from where chart IDs left off
    xml.push_str(&format!(
        r#"
<Relationship Id="rId{notes_id}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide" Target="../notesSlides/notesSlide{slide_num}.xml"/>"#
    ));
    
    xml.push_str("\n</Relationships>");
    xml
}
