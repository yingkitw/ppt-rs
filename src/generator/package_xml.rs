//! Package-level XML generation (content types, relationships, presentation)

use crate::core::{append_usize, escape_xml};
use crate::generator::slide_content::presentation_settings::PresentationSettings;
use crate::generator::slide_content::embedded_fonts::EmbeddedFontList;
use crate::generator::charts::chart_embedding_filename;
use crate::generator::layout_parts::append_layout_content_type_overrides;
use crate::generator::layout_parts::STANDARD_LAYOUT_COUNT;
use crate::generator::theme_xml::layout_rel_target;

/// First slide master id (`0x80000000`). Layout and notes master ids follow in the same id space.
pub const SLIDE_MASTER_ID: u32 = 2_147_483_648;
/// Id for the first (blank) slide layout on slideMaster1.
pub const SLIDE_LAYOUT_ID: u32 = SLIDE_MASTER_ID + 1;
/// Id for notesMaster1 — after all slide layout ids on slideMaster1.
pub const NOTES_MASTER_ID: u32 = SLIDE_LAYOUT_ID + STANDARD_LAYOUT_COUNT as u32;
/// Id for handoutMaster1.
pub const HANDOUT_MASTER_ID: u32 = NOTES_MASTER_ID + 1;

const DEFAULT_TEXT_STYLE: &str = r#"<p:defaultTextStyle><a:defPPr><a:defRPr lang="en-US"/></a:defPPr><a:lvl1pPr marL="0" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1"><a:defRPr sz="1800" kern="1200"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill><a:latin typeface="+mn-lt"/><a:ea typeface="+mn-ea"/><a:cs typeface="+mn-cs"/></a:defRPr></a:lvl1pPr><a:lvl2pPr marL="457200" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1"><a:defRPr sz="1800" kern="1200"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill><a:latin typeface="+mn-lt"/><a:ea typeface="+mn-ea"/><a:cs typeface="+mn-cs"/></a:defRPr></a:lvl2pPr><a:lvl3pPr marL="914400" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1"><a:defRPr sz="1800" kern="1200"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill><a:latin typeface="+mn-lt"/><a:ea typeface="+mn-ea"/><a:cs typeface="+mn-cs"/></a:defRPr></a:lvl3pPr><a:lvl4pPr marL="1371600" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1"><a:defRPr sz="1800" kern="1200"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill><a:latin typeface="+mn-lt"/><a:ea typeface="+mn-ea"/><a:cs typeface="+mn-cs"/></a:defRPr></a:lvl4pPr><a:lvl5pPr marL="1828800" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1"><a:defRPr sz="1800" kern="1200"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill><a:latin typeface="+mn-lt"/><a:ea typeface="+mn-ea"/><a:cs typeface="+mn-cs"/></a:defRPr></a:lvl5pPr><a:lvl6pPr marL="2286000" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1"><a:defRPr sz="1800" kern="1200"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill><a:latin typeface="+mn-lt"/><a:ea typeface="+mn-ea"/><a:cs typeface="+mn-cs"/></a:defRPr></a:lvl6pPr><a:lvl7pPr marL="2743200" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1"><a:defRPr sz="1800" kern="1200"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill><a:latin typeface="+mn-lt"/><a:ea typeface="+mn-ea"/><a:cs typeface="+mn-cs"/></a:defRPr></a:lvl7pPr><a:lvl8pPr marL="3200400" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1"><a:defRPr sz="1800" kern="1200"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill><a:latin typeface="+mn-lt"/><a:ea typeface="+mn-ea"/><a:cs typeface="+mn-cs"/></a:defRPr></a:lvl8pPr><a:lvl9pPr marL="3657600" algn="l" defTabSz="457200" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1"><a:defRPr sz="1800" kern="1200"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill><a:latin typeface="+mn-lt"/><a:ea typeface="+mn-ea"/><a:cs typeface="+mn-cs"/></a:defRPr></a:lvl9pPr></p:defaultTextStyle>"#;

const CONTENT_TYPES_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>"#;

fn append_image_content_type_defaults(xml: &mut String, media_exts: &[String]) {
    let mut has_jpg = false;
    let mut has_png = false;
    let mut has_gif = false;
    for ext in media_exts {
        match ext.as_str() {
            "jpg" | "jpeg" => has_jpg = true,
            "png" => has_png = true,
            "gif" => has_gif = true,
            _ => {}
        }
    }
    if has_jpg {
        xml.push_str(r#"<Default Extension="jpg" ContentType="image/jpeg"/>"#);
    }
    if has_png {
        xml.push_str(r#"<Default Extension="png" ContentType="image/png"/>"#);
    }
    if has_gif {
        xml.push_str(r#"<Default Extension="gif" ContentType="image/gif"/>"#);
    }
}

pub fn content_types_opening(media_exts: &[String], chart_count: usize) -> String {
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">"#,
    );
    append_image_content_type_defaults(&mut xml, media_exts);
    xml.push_str(
        r#"<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>"#,
    );
    if chart_count > 0 {
        xml.push_str(
            r#"<Default Extension="xlsx" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"/>"#,
        );
    }
    xml.push_str(
        r#"<Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>"#,
    );
    xml
}

const CONTENT_TYPES_FOOTER: &str = r#"
<Override PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>
<Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
<Override PartName="/ppt/tableStyles.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"/>
<Override PartName="/ppt/viewProps.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml"/>
<Override PartName="/ppt/presProps.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"/>
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

/// Relationship id for slide master (always rId1).
pub const SLIDE_MASTER_REL_ID: usize = 1;

/// First slide relationship id in `presentation.xml.rels` (immediately after slide master).
pub fn first_slide_rel_id(_has_notes: bool, _has_handout: bool) -> usize {
    2
}

/// Relationship id for slide `slide_num` (1-based).
pub fn slide_rel_id(slide_num: usize, _has_notes: bool, _has_handout: bool) -> usize {
    slide_num + 1
}

/// Notes master relationship id (after all slides).
pub fn notes_master_rel_id(slide_count: usize) -> usize {
    slide_count + 2
}

/// Handout master relationship id (after slides and optional notes master).
pub fn handout_master_rel_id(slide_count: usize, has_notes: bool) -> usize {
    slide_count + 2 + usize::from(has_notes)
}

/// presProps relationship id (after optional masters).
pub fn pres_props_rel_id(slide_count: usize, has_notes: bool, has_handout: bool) -> usize {
    slide_count + 2 + usize::from(has_notes) + usize::from(has_handout)
}

/// viewProps relationship id.
pub fn view_props_rel_id(slide_count: usize, has_notes: bool, has_handout: bool) -> usize {
    pres_props_rel_id(slide_count, has_notes, has_handout) + 1
}

/// theme1 relationship id.
pub fn theme_rel_id(slide_count: usize, has_notes: bool, has_handout: bool) -> usize {
    pres_props_rel_id(slide_count, has_notes, has_handout) + 2
}

/// tableStyles relationship id.
pub fn table_styles_rel_id(slide_count: usize, has_notes: bool, has_handout: bool) -> usize {
    pres_props_rel_id(slide_count, has_notes, has_handout) + 3
}

/// Slide id value for slide `slide_num` (1-based). PowerPoint starts at 256.
pub fn slide_id_value(slide_num: usize) -> usize {
    255 + slide_num
}

/// Create `[Content_Types].xml`
pub fn create_content_types_xml(slides: usize) -> String {
    let mut xml = String::with_capacity(1024 + slides * 140);
    xml.push_str(CONTENT_TYPES_HEADER);
    append_slide_overrides(&mut xml, slides);
    append_layout_content_type_overrides(&mut xml, STANDARD_LAYOUT_COUNT);
    xml.push_str(CONTENT_TYPES_FOOTER);
    xml
}

const PACKAGE_RELS_XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
<Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>"#;

/// Create _rels/.rels
pub fn create_rels_xml() -> &'static str {
    PACKAGE_RELS_XML
}

/// Create _rels/.rels with an optional digital signature origin relationship.
pub fn create_rels_xml_with_signature(has_signature: bool) -> String {
    let mut xml = String::from(PACKAGE_RELS_XML);
    if has_signature {
        // Insert signature origin relationship before </Relationships>.
        // The static XML contains rId1-rId3, so rId4 is the next free id.
        if let Some(pos) = xml.rfind("</Relationships>") {
            xml.insert_str(
                pos,
                r#"<Relationship Id="rId4" Type="http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin" Target="_xmlsignatures/origin.sigs"/>"#,
            );
        }
    }
    xml
}

/// Create ppt/_rels/presentation.xml.rels
pub fn create_presentation_rels_xml(slides: usize) -> String {
    create_presentation_rels_xml_full(slides, false, false)
}

/// Create ppt/_rels/presentation.xml.rels with notes master
pub fn create_presentation_rels_xml_with_notes(slides: usize) -> String {
    create_presentation_rels_xml_full(slides, true, false)
}

/// Create ppt/_rels/presentation.xml.rels with optional notes/handout masters.
pub fn create_presentation_rels_xml_full(
    slides: usize,
    has_notes: bool,
    has_handout: bool,
) -> String {
    let mut xml = String::with_capacity(768 + slides * 120);
    xml.push_str(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>"#,
    );

    for i in 1..=slides {
        let rid = slide_rel_id(i, has_notes, has_handout);
        xml.push_str("\n<Relationship Id=\"rId");
        append_usize(&mut xml, rid);
        xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide\" Target=\"slides/slide");
        append_usize(&mut xml, i);
        xml.push_str(".xml\"/>");
    }

    if has_notes {
        xml.push_str("\n<Relationship Id=\"rId");
        append_usize(&mut xml, notes_master_rel_id(slides));
        xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster\" Target=\"notesMasters/notesMaster1.xml\"/>");
    }

    if has_handout {
        xml.push_str("\n<Relationship Id=\"rId");
        append_usize(&mut xml, handout_master_rel_id(slides, has_notes));
        xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster\" Target=\"handoutMasters/handoutMaster1.xml\"/>");
    }

    xml.push_str("\n<Relationship Id=\"rId");
    append_usize(&mut xml, pres_props_rel_id(slides, has_notes, has_handout));
    xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps\" Target=\"presProps.xml\"/>");

    xml.push_str("\n<Relationship Id=\"rId");
    append_usize(&mut xml, view_props_rel_id(slides, has_notes, has_handout));
    xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps\" Target=\"viewProps.xml\"/>");

    xml.push_str("\n<Relationship Id=\"rId");
    append_usize(&mut xml, theme_rel_id(slides, has_notes, has_handout));
    xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme\" Target=\"theme/theme1.xml\"/>");

    xml.push_str("\n<Relationship Id=\"rId");
    append_usize(&mut xml, table_styles_rel_id(slides, has_notes, has_handout));
    xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles\" Target=\"tableStyles.xml\"/>");

    xml.push_str("\n</Relationships>");
    xml
}

/// Create ppt/_rels/presentation.xml.rels including embedded font relationships.
pub fn create_presentation_rels_xml_full_with_fonts(
    slides: usize,
    has_notes: bool,
    has_handout: bool,
    fonts: &EmbeddedFontList,
) -> String {
    let mut xml = create_presentation_rels_xml_full(slides, has_notes, has_handout);

    for font in fonts.fonts() {
        let target = font.rel_target();
        xml.insert_str(
            xml.rfind("</Relationships>").unwrap_or(xml.len()),
            &format!(
                r#"<Relationship Id="{}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/font" Target="{}"/>"#,
                font.relationship_id,
                escape_xml(&target),
            ),
        );
    }

    xml
}

/// Default table style list required by PowerPoint for presentations with tables.
pub fn create_table_styles_xml() -> &'static str {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:tblStyleLst xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" def="{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}"/>"#
}

/// Create ppt/presProps.xml with optional slide-show and handout print settings.
pub fn create_pres_props_xml(settings: Option<&PresentationSettings>) -> String {
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><p:presentationPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">"#,
    );

    if let Some(s) = settings {
        if let Some(ref show) = s.slide_show {
            xml.push_str(&show.to_xml());
        }
        // Handout print preferences are expressed via the handout master part.
        // PowerPoint strips `<p:prnPr>` from presProps on repair when it is paired
        // with a packaged handout master, so we do not emit it during generation.
    }

    xml.push_str(
        r#"<p:extLst><p:ext uri="{E76CE94A-603C-4142-B9EB-6D1370010A27}"><p14:discardImageEditData xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main" val="0"/></p:ext><p:ext uri="{D31A062A-798A-4329-ABDD-BBA856620510}"><p14:defaultImageDpi xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main" val="0"/></p:ext><p:ext uri="{FD5EFAAD-0ECE-453E-9831-46B23BE46B34}"><p15:chartTrackingRefBased xmlns:p15="http://schemas.microsoft.com/office/powerpoint/2012/main" val="0"/></p:ext></p:extLst></p:presentationPr>"#,
    );
    xml
}

/// Default view properties part required by PowerPoint.
pub fn create_view_props_xml() -> &'static str {
    // All children of CT_ViewProperties are optional, but when present they
    // must carry their own required descendants (e.g. normalViewPr needs
    // restoredLeft + restoredTop; slideViewPr needs cSldViewPr). Emitting
    // empty `<p:normalViewPr/>` / `<p:slideViewPr/>` violates the schema and
    // triggers a PowerPoint repair prompt. `lastView` is omitted because
    // `sldThumbnailView` is a non-standard enum extension with the same effect.
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:viewPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:normalViewPr><p:restoredLeft sz="15620"/><p:restoredTop sz="94660"/></p:normalViewPr>
<p:slideViewPr><p:cSldViewPr><p:cViewPr varScale="1"><p:scale><a:sx n="64" d="100"/><a:sy n="64" d="100"/></p:scale><p:origin x="-1392" y="-96"/></p:cViewPr><p:guideLst/></p:cSldViewPr></p:slideViewPr>
<p:notesTextViewPr><p:cViewPr><p:scale><a:sx n="1" d="1"/><a:sy n="1" d="1"/></p:scale><p:origin x="0" y="0"/></p:cViewPr></p:notesTextViewPr>
<p:gridSpacing cx="72008" cy="72008"/>
</p:viewPr>"#
}

/// Create ppt/presentation.xml
pub fn create_presentation_xml(
    _title: &str,
    slides: usize,
    has_notes: bool,
    has_handout: bool,
) -> String {
    let mut xml = String::with_capacity(896 + slides * 48);
    xml.push_str(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" saveSubsetFonts="1" autoCompressPictures="0">
<p:sldMasterIdLst>
<p:sldMasterId id=""#,
    );
    append_usize(&mut xml, SLIDE_MASTER_ID as usize);
    xml.push_str(r#"" r:id="rId1"/>
</p:sldMasterIdLst>"#);

    if has_notes {
        xml.push_str("\n<p:notesMasterIdLst>\n<p:notesMasterId r:id=\"rId");
        append_usize(&mut xml, notes_master_rel_id(slides));
        xml.push_str("\"/>\n</p:notesMasterIdLst>");
    }

    if has_handout {
        xml.push_str("\n<p:handoutMasterIdLst>\n<p:handoutMasterId r:id=\"rId");
        append_usize(&mut xml, handout_master_rel_id(slides, has_notes));
        xml.push_str("\"/>\n</p:handoutMasterIdLst>");
    }

    xml.push_str("\n<p:sldIdLst>");

    for i in 1..=slides {
        let id = slide_id_value(i);
        let rid = slide_rel_id(i, has_notes, has_handout);
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
"#,
    );
    xml.push_str(DEFAULT_TEXT_STYLE);
    xml.push_str(
        r#"<p:extLst><p:ext uri="{EFAFB233-063F-42B5-8137-9DF3F51BA10A}"><p15:sldGuideLst xmlns:p15="http://schemas.microsoft.com/office/powerpoint/2012/main"/></p:ext></p:extLst></p:presentation>"#,
    );
    xml
}

/// Create ppt/presentation.xml including an embedded font list.
pub fn create_presentation_xml_with_fonts(
    title: &str,
    slides: usize,
    has_notes: bool,
    has_handout: bool,
    fonts: &EmbeddedFontList,
) -> String {
    let mut xml = create_presentation_xml(title, slides, has_notes, has_handout);
    let font_xml = fonts.to_xml();
    if !font_xml.is_empty() {
        // Insert <p:embeddedFontLst> before the closing </p:presentation>.
        if let Some(pos) = xml.rfind("</p:presentation>") {
            xml.insert_str(pos, &font_xml);
        }
    }
    xml
}

/// Create `[Content_Types].xml` with notes, charts, and optional handout master.
pub fn create_content_types_xml_with_notes_and_charts(
    slides: usize,
    custom_slides: Option<&[super::slide_content::SlideContent]>,
    chart_count: usize,
    has_handout: bool,
    media_exts: &[String],
) -> String {
    let notes_count = custom_slides
        .map(|slides_slice| slides_slice.iter().filter(|slide| slide.notes.is_some()).count())
        .unwrap_or(0);
    let mut xml = String::with_capacity(2048 + slides * 140 + notes_count * 140 + chart_count * 120);
    xml.push_str(&content_types_opening(media_exts, chart_count));
    append_slide_overrides(&mut xml, slides);

    if let Some(slides_slice) = custom_slides {
        let mut notes_index = 0usize;
        for slide in slides_slice {
            if slide.notes.is_some() {
                notes_index += 1;
                xml.push_str("\n<Override PartName=\"/ppt/notesSlides/notesSlide");
                append_usize(&mut xml, notes_index);
                xml.push_str(".xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml\"/>");
            }
        }
        if notes_count > 0 {
            xml.push_str("\n<Override PartName=\"/ppt/notesMasters/notesMaster1.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml\"/>");
            xml.push_str("\n<Override PartName=\"/ppt/theme/theme2.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.theme+xml\"/>");
        }
    }

    if has_handout {
        xml.push_str("\n<Override PartName=\"/ppt/handoutMasters/handoutMaster1.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml\"/>");
        xml.push_str("\n<Override PartName=\"/ppt/theme/theme3.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.theme+xml\"/>");
    }

    for i in 1..=chart_count {
        xml.push_str("\n<Override PartName=\"/ppt/charts/chart");
        append_usize(&mut xml, i);
        xml.push_str(".xml\" ContentType=\"application/vnd.openxmlformats-officedocument.drawingml.chart+xml\"/>");
        xml.push_str("\n<Override PartName=\"/ppt/embeddings/");
        xml.push_str(&chart_embedding_filename(i));
        xml.push_str("\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet\"/>");
    }

    append_layout_content_type_overrides(&mut xml, STANDARD_LAYOUT_COUNT);
    xml.push_str(CONTENT_TYPES_FOOTER);
    xml
}

/// Append the digital signature content type entries to an existing
/// `[Content_Types].xml` string (inserted before `</Types>`).
pub fn append_digital_signature_content_type(xml: &mut String) {
    if let Some(pos) = xml.rfind("</Types>") {
        xml.insert_str(
            pos,
            r#"<Default Extension="sigs" ContentType="application/vnd.openxmlformats-package.digital-signature-origin"/><Override PartName="/_xmlsignatures/sig1.xml" ContentType="application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml"/>"#,
        );
    }
}

/// Append the embedded font content type default (`.fntdata`) to an existing
/// `[Content_Types].xml` string (inserted before `</Types>`).
pub fn append_embedded_font_content_type(xml: &mut String) {
    if let Some(pos) = xml.rfind("</Types>") {
        xml.insert_str(
            pos,
            r#"<Default Extension="fntdata" ContentType="application/x-fontdata"/>"#,
        );
    }
}

/// Append ink annotation content type overrides to an existing
/// `[Content_Types].xml` string (inserted before `</Types>`).
pub fn append_ink_content_types(xml: &mut String, ink_count: usize) {
    if ink_count == 0 {
        return;
    }
    if let Some(pos) = xml.rfind("</Types>") {
        let mut overrides = String::with_capacity(ink_count * 140);
        for i in 1..=ink_count {
            overrides.push_str(&format!(
                "\n<Override PartName=\"/ppt/ink/ink{i}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.ink+xml\"/>"
            ));
        }
        xml.insert_str(pos, &overrides);
    }
}

/// Handout master relationship XML (theme link).
pub fn create_handout_master_rels_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="../theme/theme3.xml"/>
</Relationships>"#.to_string()
}

/// Create slide relationship XML with notes and charts
pub fn create_slide_rels_xml_extended(
    layout_number: usize,
    has_notes: bool,
    notes_part_num: usize,
    chart_rels: &[(String, String)],
) -> String {
    let layout_target = layout_rel_target(layout_number);
    let mut xml = String::with_capacity(512 + chart_rels.len() * 96);
    xml.push_str(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target=""#,
    );
    xml.push_str(&layout_target);
    xml.push_str("\"/>");

    if has_notes {
        xml.push_str("\n<Relationship Id=\"rId2\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide\" Target=\"../notesSlides/notesSlide");
        append_usize(&mut xml, notes_part_num);
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

/// Create slide relationship XML with notes, charts, images, and optional ink.
pub fn create_slide_rels_xml_with_images(
    layout_number: usize,
    has_notes: bool,
    notes_part_num: usize,
    chart_rels: &[(String, String)],
    images: &[(usize, String)],
    hyperlink_rels: &[String],
    ink_rel: Option<(usize, usize)>,
) -> String {
    let layout_target = layout_rel_target(layout_number);
    let mut xml = String::with_capacity(
        512 + images.len() * 96 + chart_rels.len() * 96 + hyperlink_rels.len() * 160 + 128,
    );
    xml.push_str(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target=""#,
    );
    xml.push_str(&layout_target);
    xml.push_str("\"/>");

    let mut next_rid = 2usize;
    if has_notes {
        xml.push_str("\n<Relationship Id=\"rId");
        append_usize(&mut xml, next_rid);
        xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide\" Target=\"../notesSlides/notesSlide");
        append_usize(&mut xml, notes_part_num);
        xml.push_str(".xml\"/>");
        next_rid += 1;
    }

    for (i, (image_num, ext)) in images.iter().enumerate() {
        xml.push_str("\n<Relationship Id=\"rId");
        append_usize(&mut xml, next_rid + i);
        xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/image\" Target=\"../media/image");
        append_usize(&mut xml, *image_num);
        xml.push_str(".");
        xml.push_str(ext);
        xml.push_str("\"/>");
    }

    for (rid, target) in chart_rels {
        xml.push_str("\n<Relationship Id=\"");
        xml.push_str(rid);
        xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart\" Target=\"");
        xml.push_str(target);
        xml.push_str("\"/>");
    }

    if let Some((rid, ink_num)) = ink_rel {
        xml.push_str("\n<Relationship Id=\"rId");
        append_usize(&mut xml, rid);
        xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/ink\" Target=\"../ink/ink");
        append_usize(&mut xml, ink_num);
        xml.push_str(".xml\"/>");
    }

    for rel in hyperlink_rels {
        xml.push('\n');
        xml.push_str(rel);
    }

    xml.push_str("\n</Relationships>");
    xml
}
