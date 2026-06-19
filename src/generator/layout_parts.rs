//! Slide layout part XML (`ppt/slideLayouts/slideLayoutN.xml`) with placeholders.

use crate::core::append_usize;
use crate::core::escape_xml;
use crate::generator::slide_content::print_settings::PrintSettings;

/// Number of standard layouts emitted on slide master 1.
pub const STANDARD_LAYOUT_COUNT: usize = 7;

fn placeholder(id: u32, name: &str, ph_type: &str, ph_idx: Option<u32>, x: u32, y: u32, cx: u32, cy: u32) -> String {
    let idx_xml = ph_idx
        .map(|i| format!(r#" idx="{i}""#))
        .unwrap_or_default();
    format!(
        r#"<p:sp><p:nvSpPr><p:cNvPr id="{id}" name="{name}"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="{ph_type}"{idx_xml}/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="{x}" y="{y}"/><a:ext cx="{cx}" cy="{cy}"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:noFill/></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:endParaRPr lang="en-US"/></a:p></p:txBody></p:sp>"#
    )
}

fn layout_shell(type_attr: &str, name: &str, shapes: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="{type_attr}" preserve="1">
<p:cSld name="{name}">
<p:spTree>
<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
<p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr>
{shapes}
</p:spTree>
</p:cSld>
<p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr>
<p:hf/>
</p:sldLayout>"#
    )
}

/// Footer/header/date/slide-number placeholders on the slide master.
pub fn slide_master_footer_shapes(print: Option<&PrintSettings>) -> String {
    let Some(p) = print else {
        return String::new();
    };
    let mut xml = String::new();
    let mut id = 2u32;

    if let Some(ref header) = p.header {
        xml.push_str(&placeholder(
            id,
            "Header Placeholder",
            "hdr",
            None,
            312_420,
            213_360,
            8_519_760,
            365_125,
        ));
        id += 1;
        let _ = header; // placeholder inherits text via slide; fixed text optional in master
    }

    if let Some(ref footer) = p.footer {
        xml.push_str(&format!(
            r#"<p:sp><p:nvSpPr><p:cNvPr id="{id}" name="Footer Placeholder"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="ftr" sz="quarter"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="3124200" y="6356350"/><a:ext cx="2895600" cy="365125"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:noFill/></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr lang="en-US" sz="1200"/><a:t>{}</a:t></a:r></a:p></p:txBody></p:sp>"#,
            escape_xml(footer)
        ));
        id += 1;
    }

    if p.print_date {
        xml.push_str(&format!(
            r#"<p:sp><p:nvSpPr><p:cNvPr id="{id}" name="Date Placeholder"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="dt" sz="half"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="457200" y="6356350"/><a:ext cx="2133600" cy="365125"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:noFill/></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:fld id="{{B6F15528-F159-4107-2D14-000000000000}}" type="datetimeFigureOut"><a:rPr lang="en-US" sz="1200"/><a:t>6/1/2026</a:t></a:fld><a:endParaRPr lang="en-US" sz="1200"/></a:p></p:txBody></p:sp>"#
        ));
        id += 1;
    }

    if p.print_page_numbers {
        xml.push_str(&format!(
            r#"<p:sp><p:nvSpPr><p:cNvPr id="{id}" name="Slide Number Placeholder"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="sldNum" sz="quarter"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="8610600" y="6356350"/><a:ext cx="533400" cy="365125"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:noFill/></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:fld id="{{B6F15528-F159-4107-2D14-000000000001}}" type="slidenum"><a:rPr lang="en-US" sz="1200"/><a:t>‹#›</a:t></a:fld><a:endParaRPr lang="en-US" sz="1200"/></a:p></p:txBody></p:sp>"#
        ));
    }

    xml
}

/// Generate layout XML for layout index `n` (1-based).
pub fn create_slide_layout_xml(n: usize, _print: Option<&PrintSettings>) -> String {
    match n {
        1 => layout_shell(
            "title",
            "Title Slide",
            &format!(
                "{}{}",
                placeholder(2, "Title", "ctrTitle", None, 1_524_000, 1_828_800, 6_096_000, 1_828_800),
                placeholder(3, "Subtitle", "subTitle", None, 1_524_000, 3_657_600, 6_096_000, 914_400),
            ),
        ),
        2 => layout_shell(
            "obj",
            "Title and Content",
            &format!(
                "{}{}",
                placeholder(2, "Title", "title", None, 457_200, 274_638, 8_229_600, 1_143_000),
                placeholder(3, "Content", "body", Some(1), 457_200, 1_600_200, 8_229_600, 4_525_963),
            ),
        ),
        3 => layout_shell(
            "twoObj",
            "Two Content",
            &format!(
                "{}{}{}",
                placeholder(2, "Title", "title", None, 457_200, 274_638, 8_229_600, 1_143_000),
                placeholder(3, "Content Left", "body", Some(1), 457_200, 1_600_200, 4_025_400, 4_525_963),
                placeholder(4, "Content Right", "body", Some(2), 4_661_400, 1_600_200, 4_025_400, 4_525_963),
            ),
        ),
        4 => layout_shell(
            "secHead",
            "Section Header",
            &format!(
                "{}{}",
                placeholder(2, "Title", "title", None, 457_200, 1_600_200, 8_229_600, 1_828_800),
                placeholder(3, "Subtitle", "body", Some(1), 457_200, 3_657_600, 8_229_600, 1_371_600),
            ),
        ),
        5 => layout_shell("blank", "Blank", ""),
        6 => layout_shell(
            "titleOnly",
            "Title Only",
            &placeholder(2, "Title", "title", None, 457_200, 274_638, 8_229_600, 1_143_000),
        ),
        7 => layout_shell(
            "obj",
            "Title and Big Content",
            &format!(
                "{}{}",
                placeholder(2, "Title", "title", None, 457_200, 274_638, 8_229_600, 800_000),
                placeholder(3, "Content", "body", Some(1), 457_200, 1_200_000, 8_229_600, 5_000_000),
            ),
        ),
        _ => layout_shell("blank", "Blank", ""),
    }
}

pub fn create_layout_rels_xml() -> &'static str {
    super::package_cache::layout_rels_xml()
}

pub fn create_master_rels_xml(layout_count: usize) -> String {
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">"#,
    );
    for i in 1..=layout_count {
        xml.push_str("\n<Relationship Id=\"rId");
        append_usize(&mut xml, i);
        xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout\" Target=\"../slideLayouts/slideLayout");
        append_usize(&mut xml, i);
        xml.push_str(".xml\"/>");
    }
    xml.push_str("\n<Relationship Id=\"rId");
    append_usize(&mut xml, layout_count + 1);
    xml.push_str("\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme\" Target=\"../theme/theme1.xml\"/>\n</Relationships>");
    xml
}

pub fn append_layout_content_type_overrides(xml: &mut String, layout_count: usize) {
    for i in 1..=layout_count {
        xml.push_str("\n<Override PartName=\"/ppt/slideLayouts/slideLayout");
        append_usize(xml, i);
        xml.push_str(".xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml\"/>");
    }
}

pub fn layout_rel_target(layout_number: usize) -> String {
    let mut target = String::from("../slideLayouts/slideLayout");
    append_usize(&mut target, layout_number);
    target.push_str(".xml");
    target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_standard_layouts_have_placeholders_or_blank() {
        for n in 1..=STANDARD_LAYOUT_COUNT {
            let xml = create_slide_layout_xml(n, None);
            assert!(xml.contains("p:sldLayout"), "layout {n}");
            assert!(xml.contains("type="), "layout {n}");
        }
        assert!(create_slide_layout_xml(2, None).contains(r#"type="body""#));
        assert!(create_slide_layout_xml(4, None).contains("secHead"));
    }

    #[test]
    fn test_master_rels_lists_all_layouts_before_theme() {
        let rels = create_master_rels_xml(7);
        assert!(rels.contains("slideLayout7.xml"));
        assert!(rels.contains(r#"Id="rId8""#));
        assert!(rels.contains("theme/theme1.xml"));
    }
}
