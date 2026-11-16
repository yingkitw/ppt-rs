//! Slide XML generation for different layouts
//!
//! This module handles generating XML for various slide layouts:
//! - Blank slides
//! - Title only slides
//! - Centered title slides
//! - Title and content slides
//! - Two column slides
//! - Title and big content slides

mod common;
mod content;
mod layouts;

use super::slide_content::{SlideContent, SlideLayout};

pub use common::create_slide_rels_xml;

/// Create simple slide XML
pub fn create_slide_xml(slide_num: usize, title: &str) -> String {
    let slide_title = if slide_num == 1 {
        title.to_string()
    } else {
        format!("Slide {slide_num}")
    };

    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
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
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title 1"/>
<p:cNvSpPr>
<a:spLocks noGrp="1"/>
</p:cNvSpPr>
<p:nvPr>
<p:ph type="ctrTitle"/>
</p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US"/>
<a:t>{slide_title}</a:t>
</a:r>
<a:endParaRPr lang="en-US"/>
</a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#
    )
}

/// Create slide XML with content based on layout
pub fn create_slide_xml_with_content(
    _slide_num: usize,
    content: &SlideContent,
    chart_rids: &[String],
    ink_rel_id: Option<&str>,
    media_rids: &[crate::generator::media::MediaRelIds],
) -> String {
    let mut xml = match content.layout {
        SlideLayout::Blank => {
            layouts::create_blank_slide(content, chart_rids, ink_rel_id, media_rids)
        }
        SlideLayout::TitleOnly => {
            layouts::create_title_only_slide(content, chart_rids, ink_rel_id, media_rids)
        }
        SlideLayout::CenteredTitle => {
            layouts::create_centered_title_slide(content, chart_rids, ink_rel_id, media_rids)
        }
        SlideLayout::TitleAndBigContent => {
            layouts::create_title_and_big_content_slide(content, chart_rids, ink_rel_id, media_rids)
        }
        SlideLayout::TwoColumn => {
            layouts::create_two_column_slide(content, chart_rids, ink_rel_id, media_rids)
        }
        SlideLayout::SectionHeader => {
            layouts::create_section_header_slide(content, chart_rids, ink_rel_id, media_rids)
        }
        SlideLayout::TitleAndContent => {
            layouts::create_title_and_content_slide(content, chart_rids, ink_rel_id, media_rids)
        }
    };

    // Inject transition if present
    let transition_xml = content.transition.to_xml();
    if !transition_xml.is_empty() {
        if let Some(pos) = xml.rfind("</p:sld>") {
            xml.insert_str(pos, &transition_xml);
        }
    }

    xml
}

#[cfg(test)]
mod tests {
    use crate::generator::slide::formatting::parse_inline_formatting;

    #[test]
    fn test_parse_inline_formatting_plain() {
        let segments = parse_inline_formatting("Hello world");
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text, "Hello world");
        assert!(!segments[0].bold);
        assert!(!segments[0].italic);
    }

    #[test]
    fn test_parse_inline_formatting_bold() {
        let segments = parse_inline_formatting("This is **bold** text");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].text, "This is ");
        assert!(!segments[0].bold);
        assert_eq!(segments[1].text, "bold");
        assert!(segments[1].bold);
        assert_eq!(segments[2].text, " text");
        assert!(!segments[2].bold);
    }

    #[test]
    fn test_parse_inline_formatting_italic() {
        let segments = parse_inline_formatting("This is *italic* text");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[1].text, "italic");
        assert!(segments[1].italic);
    }

    #[test]
    fn test_parse_inline_formatting_code() {
        let segments = parse_inline_formatting("Use `code` here");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[1].text, "code");
        assert!(segments[1].code);
    }

    #[test]
    fn test_parse_inline_formatting_mixed() {
        let segments = parse_inline_formatting("**bold** and *italic*");
        assert!(segments.iter().any(|s| s.bold && s.text == "bold"));
        assert!(segments.iter().any(|s| s.italic && s.text == "italic"));
    }

    // ---- Placeholder XML emission (placeholder API, v0.2.36 follow-up) ----
    //
    // Generated slides must emit `<p:ph type="…"/>` so PowerPoint recognizes
    // the shape as filling the layout's intended slot (python-pptx parity:
    // `slide.shapes.title` returns the shape with `<p:ph type="title"/>`).
    // Before this work, the active code path emitted bare `<p:nvPr/>` and the
    // layout's title placeholder stayed empty in PowerPoint.

    use super::create_slide_xml_with_content;
    use crate::generator::SlideContent;

    #[test]
    fn title_and_content_emits_title_and_body_placeholders() {
        let slide = SlideContent::new("Hello")
            .add_bullet("One")
            .add_bullet("Two");
        let xml = create_slide_xml_with_content(1, &slide, &[], None, &[]);
        assert!(
            xml.contains(r#"<p:ph type="title"/>"#),
            "title placeholder missing in:\n{xml}"
        );
        assert!(
            xml.contains(r#"<p:ph type="body" idx="1"/>"#),
            "body placeholder missing in:\n{xml}"
        );
    }

    #[test]
    fn centered_title_uses_ctr_title_placeholder() {
        let slide = SlideContent::new("Big Title")
            .layout(crate::generator::SlideLayout::CenteredTitle);
        let xml = create_slide_xml_with_content(1, &slide, &[], None, &[]);
        assert!(
            xml.contains(r#"<p:ph type="ctrTitle"/>"#),
            "ctrTitle placeholder missing in:\n{xml}"
        );
    }

    #[test]
    fn two_column_emits_idx1_and_idx2_body_placeholders() {
        let slide = SlideContent::new("Compare")
            .add_bullet("Left A")
            .add_bullet("Left B")
            .add_bullet("Right A")
            .add_bullet("Right B")
            .layout(crate::generator::SlideLayout::TwoColumn);
        let xml = create_slide_xml_with_content(1, &slide, &[], None, &[]);
        assert!(xml.contains(r#"<p:ph type="body" idx="1"/>"#));
        assert!(xml.contains(r#"<p:ph type="body" idx="2"/>"#));
    }

    #[test]
    fn title_only_layout_emits_only_title_placeholder() {
        let slide = SlideContent::new("Just a Title")
            .layout(crate::generator::SlideLayout::TitleOnly);
        let xml = create_slide_xml_with_content(1, &slide, &[], None, &[]);
        assert!(xml.contains(r#"<p:ph type="title"/>"#));
        // No body placeholder for TitleOnly layout.
        assert!(!xml.contains(r#"<p:ph type="body""#));
    }
}
