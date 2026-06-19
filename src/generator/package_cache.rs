//! Cached static package parts reused across every generated deck.

use std::sync::OnceLock;

use super::layout_parts::{self, STANDARD_LAYOUT_COUNT};
use super::slide_content::print_settings::PrintSettings;
use super::theme_xml::create_slide_master_xml;

const LAYOUT_RELS_XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="../slideMasters/slideMaster1.xml"/>
</Relationships>"#;

static MASTER_RELS: OnceLock<String> = OnceLock::new();
static DEFAULT_MASTER: OnceLock<String> = OnceLock::new();
static DEFAULT_LAYOUTS: OnceLock<Vec<String>> = OnceLock::new();

/// Whether print settings change layout/master placeholder XML.
pub fn print_affects_theme_parts(print: Option<&PrintSettings>) -> bool {
    print.is_some_and(|p| {
        p.header.is_some()
            || p.footer.is_some()
            || p.print_date
            || p.print_page_numbers
    })
}

/// Cached `ppt/slideLayouts/_rels/slideLayoutN.xml.rels` (identical for every layout).
pub fn layout_rels_xml() -> &'static str {
    LAYOUT_RELS_XML
}

/// Cached `ppt/slideMasters/_rels/slideMaster1.xml.rels` for the standard layout count.
pub fn master_rels_xml() -> &'static str {
    MASTER_RELS
        .get_or_init(|| layout_parts::create_master_rels_xml(STANDARD_LAYOUT_COUNT))
        .as_str()
}

/// Cached slide master without footer/header placeholders.
pub fn default_slide_master_xml() -> &'static str {
    DEFAULT_MASTER
        .get_or_init(|| create_slide_master_xml(None))
        .as_str()
}

/// Cached `slideLayoutN.xml` without footer/header flags.
pub fn default_layout_xml(layout_number: usize) -> &'static str {
    default_layouts()[layout_number - 1].as_str()
}

fn default_layouts() -> &'static Vec<String> {
    DEFAULT_LAYOUTS.get_or_init(|| {
        (1..=STANDARD_LAYOUT_COUNT)
            .map(|n| layout_parts::create_slide_layout_xml(n, None))
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cached_layouts_are_populated_once() {
        assert_eq!(default_layouts().len(), STANDARD_LAYOUT_COUNT);
        assert!(default_layout_xml(1).contains("Title Slide"));
        assert!(default_slide_master_xml().contains("sldMaster"));
        assert!(master_rels_xml().contains("slideLayout7.xml"));
    }
}
