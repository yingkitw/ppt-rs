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
mod layouts;
mod content;

use super::slide_content::SlideContent;
use crate::generator::layouts::create_slide_xml_for_layout;

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
<a:rPr lang="en-US" smtClean="0"/>
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
pub fn create_slide_xml_with_content(_slide_num: usize, content: &SlideContent) -> String {
    create_slide_xml_for_layout(content)
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
}
