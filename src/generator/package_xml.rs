//! Package-level XML generation (content types, relationships, presentation)

use crate::core::append_usize;

const CONTENT_TYPES_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>"#;

const CONTENT_TYPES_EXTENDED_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Default Extension="png" ContentType="image/png"/>
<Default Extension="jpeg" ContentType="image/jpeg"/>
<Default Extension="jpg" ContentType="image/jpeg"/>
<Default Extension="gif" ContentType="image/gif"/>
<Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>"#;

const CONTENT_TYPES_FOOTER: &str = r#"
<Override PartName="/ppt/slideLayouts/slideLayout1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
<Override PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>
<Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
<Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
<Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
</Types>"#;

const SLIDE_OVERRIDE_PREFIX: &str =
    "\n<Override PartName=\"/ppt/slides/slide";
const SLIDE_OVERRIDE_SUFFIX: &str =
    ".xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.slide+xml\"/>";

fn append_slide_overrides(xml: &mut String, slides: usize) {
    for i in 1..=slides {
        xml.push_str(SLIDE_OVERRIDE_PREFIX);
        append_usize(xml, i);
        xml.push_str(SLIDE_OVERRIDE_SUFFIX);
    }
}

/// Create `[Content_Types].xml`
pub fn create_content_types_xml(slides: usize) -> String {
    let mut xml = String::with_capacity(1024 + slides * 140);
    xml.push_str(CONTENT_TYPES_HEADER);
    append_slide_overrides(&mut xml, slides);
    xml.push_str(CONTENT_TYPES_FOOTER);
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
    let mut xml = String::with_capacity(512 + slides * 120);
    xml.push_str(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>
    <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>"#,
    );

    for i in 1..=slides {
        let rid = i + 2;
        xml.push_str("\n    <Relationship Id=\"rId");
        append_usize(&mut xml, rid);
        xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide\" Target=\"slides/slide");
        append_usize(&mut xml, i);
        xml.push_str(".xml\"/>");
    }

    xml.push_str("\n</Relationships>");
    xml
}

/// Create ppt/presentation.xml
pub fn create_presentation_xml(_title: &str, slides: usize) -> String {
    let mut xml = String::with_capacity(768 + slides * 48);
    xml.push_str(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" saveSubsetFonts="1">
<p:sldMasterIdLst>
<p:sldMasterId id="2147483648" r:id="rId1"/>
</p:sldMasterIdLst>
<p:sldIdLst>"#,
    );

    for i in 1..=slides {
        let id = 256 + i;
        let rid = i + 2;
        xml.push_str("\n<p:sldId id=\"");
        append_usize(&mut xml, id);
        xml.push_str("\" r:id=\"rId");
        append_usize(&mut xml, rid);
        xml.push_str("\"/>");
    }

    xml.push_str(
        r#"
</p:sldIdLst>
<p:sldSz cx="9144000" cy="6858000" type="screen4x3"/>
<p:notesSz cx="6858000" cy="9144000"/>
</p:presentation>"#,
    );
    xml
}

/// Create `[Content_Types].xml` with notes and charts support
pub fn create_content_types_xml_with_notes_and_charts(
    slides: usize,
    custom_slides: Option<&[super::slide_content::SlideContent]>,
    chart_count: usize,
) -> String {
    let notes_count = custom_slides
        .map(|slides_slice| slides_slice.iter().filter(|slide| slide.notes.is_some()).count())
        .unwrap_or(0);
    let mut xml = String::with_capacity(2048 + slides * 140 + notes_count * 140 + chart_count * 120);
    xml.push_str(CONTENT_TYPES_EXTENDED_HEADER);
    append_slide_overrides(&mut xml, slides);

    if let Some(slides_slice) = custom_slides {
        for (i, slide) in slides_slice.iter().enumerate() {
            if slide.notes.is_some() {
                let slide_num = i + 1;
                xml.push_str("\n<Override PartName=\"/ppt/notesSlides/notesSlide");
                append_usize(&mut xml, slide_num);
                xml.push_str(".xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml\"/>");
            }
        }
        if notes_count > 0 {
            xml.push_str("\n<Override PartName=\"/ppt/notesMasters/notesMaster1.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml\"/>");
        }
    }

    for i in 1..=chart_count {
        xml.push_str("\n<Override PartName=\"/ppt/charts/chart");
        append_usize(&mut xml, i);
        xml.push_str(".xml\" ContentType=\"application/vnd.openxmlformats-officedocument.drawingml.chart+xml\"/>");
    }

    xml.push_str(CONTENT_TYPES_FOOTER);
    xml
}

/// Create ppt/_rels/presentation.xml.rels with notes master
pub fn create_presentation_rels_xml_with_notes(slides: usize) -> String {
    let mut xml = String::with_capacity(640 + slides * 120);
    xml.push_str(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>
    <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>"#,
    );

    for i in 1..=slides {
        let rid = i + 2;
        xml.push_str("\n    <Relationship Id=\"rId");
        append_usize(&mut xml, rid);
        xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide\" Target=\"slides/slide");
        append_usize(&mut xml, i);
        xml.push_str(".xml\"/>");
    }

    let notes_master_rid = slides + 3;
    xml.push_str("\n    <Relationship Id=\"rId");
    append_usize(&mut xml, notes_master_rid);
    xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster\" Target=\"notesMasters/notesMaster1.xml\"/>");
    xml.push_str("\n</Relationships>");
    xml
}

/// Create slide relationship XML with notes and charts
pub fn create_slide_rels_xml_extended(slide_num: usize, has_notes: bool, chart_rels: &[(String, String)]) -> String {
    let mut xml = String::with_capacity(512 + chart_rels.len() * 96);
    xml.push_str(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>"#,
    );

    if has_notes {
        xml.push_str("\n<Relationship Id=\"rId2\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide\" Target=\"../notesSlides/notesSlide");
        append_usize(&mut xml, slide_num);
        xml.push_str(".xml\"/>");
    }

    for (rid, target) in chart_rels {
        xml.push_str("\n<Relationship Id=\"");
        xml.push_str(rid);
        xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart\" Target=\"");
        xml.push_str(target);
        xml.push_str("\"/>");
    }

    xml.push_str("\n</Relationships>");
    xml
}

/// Create slide relationship XML with notes, charts, and images
pub fn create_slide_rels_xml_with_images(
    slide_num: usize,
    has_notes: bool,
    chart_rels: &[(String, String)],
    image_count: usize,
    image_start_num: usize,
    image_extensions: &[String],
) -> String {
    let mut xml = String::with_capacity(512 + image_count * 96 + chart_rels.len() * 96);
    xml.push_str(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>"#,
    );

    for i in 0..image_count {
        let rel_id = i + 2;
        let img_num = image_start_num + i;
        let ext = image_extensions.get(i).map(|s| s.as_str()).unwrap_or("png");
        xml.push_str("\n<Relationship Id=\"rId");
        append_usize(&mut xml, rel_id);
        xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/image\" Target=\"../media/image");
        append_usize(&mut xml, img_num);
        xml.push('.');
        xml.push_str(ext);
        xml.push_str("\"/>");
    }

    if has_notes {
        let notes_rid = 2 + image_count;
        xml.push_str("\n<Relationship Id=\"rId");
        append_usize(&mut xml, notes_rid);
        xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide\" Target=\"../notesSlides/notesSlide");
        append_usize(&mut xml, slide_num);
        xml.push_str(".xml\"/>");
    }

    for (rid, target) in chart_rels {
        xml.push_str("\n<Relationship Id=\"");
        xml.push_str(rid);
        xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart\" Target=\"");
        xml.push_str(target);
        xml.push_str("\"/>");
    }

    xml.push_str("\n</Relationships>");
    xml
}

pub use crate::core::escape_xml;