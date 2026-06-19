//! Theme, master, and layout XML generation

use crate::core::append_usize;
use crate::generator::presentation_theme::PresentationTheme;
use crate::generator::slide_content::print_settings::PrintSettings;

use super::layout_parts::{self, slide_master_footer_shapes, STANDARD_LAYOUT_COUNT};
use crate::generator::package_xml::SLIDE_LAYOUT_ID;

pub use super::layout_parts::{append_layout_content_type_overrides, layout_rel_target, STANDARD_LAYOUT_COUNT as LAYOUT_COUNT};

/// Create theme XML (Office default when no custom theme is provided)
pub fn create_theme_xml(theme: Option<&PresentationTheme>) -> String {
    match theme {
        Some(t) => t.to_theme_xml(),
        None => crate::generator::presentation_theme::office_theme_xml().to_string(),
    }
}

/// Create layout relationships XML (layout → slide master)
pub fn create_layout_rels_xml() -> &'static str {
    layout_parts::create_layout_rels_xml()
}

/// Create slide master XML with all standard layouts and optional footer placeholders.
pub fn create_slide_master_xml(print: Option<&PrintSettings>) -> String {
    const TX_STYLES: &str = include_str!("slide_master_txstyles.xml");
    let footer_shapes = slide_master_footer_shapes(print);

    let mut layout_ids = String::new();
    for i in 0..STANDARD_LAYOUT_COUNT {
        let id = SLIDE_LAYOUT_ID as usize + i;
        let rid = i + 1;
        layout_ids.push_str("\n<p:sldLayoutId id=\"");
        append_usize(&mut layout_ids, id);
        layout_ids.push_str("\" r:id=\"rId");
        append_usize(&mut layout_ids, rid);
        layout_ids.push_str("\"/>");
    }

    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
<p:bg>
<p:bgRef idx="1001">
<a:schemeClr val="bg1"/>
</p:bgRef>
</p:bg>
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
{footer_shapes}
</p:spTree>
</p:cSld>
<p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>
<p:sldLayoutIdLst>{layout_ids}
</p:sldLayoutIdLst>
{TX_STYLES}
</p:sldMaster>"#
    )
}

/// Create master relationships XML for all standard layouts.
pub fn create_master_rels_xml() -> String {
    layout_parts::create_master_rels_xml(STANDARD_LAYOUT_COUNT)
}
