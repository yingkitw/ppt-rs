//! Text formatting utilities for slide XML generation
//!
//! Handles inline markdown formatting (bold, italic, code) and
//! generates corresponding PPTX XML text runs.

use crate::core::escape_xml;

/// A text segment with formatting
#[derive(Debug, Clone)]
pub struct TextSegment {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
}

/// Parse markdown-style inline formatting into segments
pub fn parse_inline_formatting(text: &str) -> Vec<TextSegment> {
    let mut segments = Vec::new();
    let mut current_text = String::new();
    let mut chars = text.chars().peekable();
    let mut bold = false;
    let mut italic = false;
    let mut code = false;
    
    while let Some(c) = chars.next() {
        match c {
            '`' if !code => {
                if !current_text.is_empty() {
                    segments.push(TextSegment {
                        text: current_text.clone(),
                        bold,
                        italic,
                        code: false,
                    });
                    current_text.clear();
                }
                code = true;
            }
            '`' if code => {
                segments.push(TextSegment {
                    text: current_text.clone(),
                    bold: false,
                    italic: false,
                    code: true,
                });
                current_text.clear();
                code = false;
            }
            '*' | '_' if !code => {
                if chars.peek() == Some(&c) {
                    chars.next();
                    if !current_text.is_empty() {
                        segments.push(TextSegment {
                            text: current_text.clone(),
                            bold,
                            italic,
                            code: false,
                        });
                        current_text.clear();
                    }
                    bold = !bold;
                } else {
                    if !current_text.is_empty() {
                        segments.push(TextSegment {
                            text: current_text.clone(),
                            bold,
                            italic,
                            code: false,
                        });
                        current_text.clear();
                    }
                    italic = !italic;
                }
            }
            _ => {
                current_text.push(c);
            }
        }
    }
    
    if !current_text.is_empty() {
        segments.push(TextSegment {
            text: current_text,
            bold,
            italic,
            code,
        });
    }
    
    if segments.is_empty() {
        segments.push(TextSegment {
            text: text.to_string(),
            bold: false,
            italic: false,
            code: false,
        });
    }
    
    segments
}

/// Generate XML runs for rich text with inline formatting
pub fn generate_rich_text_runs(
    text: &str,
    base_size: u32,
    base_bold: bool,
    base_italic: bool,
    base_color: Option<&str>,
) -> String {
    let segments = parse_inline_formatting(text);
    let mut xml = String::new();
    
    for segment in segments {
        let size = base_size;
        let bold = base_bold || segment.bold;
        let italic = base_italic || segment.italic;
        let escaped_text = escape_xml(&segment.text);
        
        if segment.code {
            xml.push_str(&format!(
                r#"<a:r><a:rPr lang="en-US" sz="{}" dirty="0"><a:latin typeface="Consolas"/><a:solidFill><a:srgbClr val="C7254E"/></a:solidFill></a:rPr><a:t>{}</a:t></a:r>"#,
                size, escaped_text
            ));
        } else {
            let mut props = format!(
                r#"<a:rPr lang="en-US" sz="{}" b="{}" i="{}" dirty="0""#,
                size,
                if bold { "1" } else { "0" },
                if italic { "1" } else { "0" }
            );
            
            if let Some(color) = base_color {
                props.push('>');
                let clean_color = color.trim_start_matches('#').to_uppercase();
                props.push_str(&format!(r#"<a:solidFill><a:srgbClr val="{}"/></a:solidFill>"#, clean_color));
                props.push_str("</a:rPr>");
            } else {
                props.push_str("/>");
            }
            
            xml.push_str(&format!(r#"<a:r>{}<a:t>{}</a:t></a:r>"#, props, escaped_text));
        }
    }
    
    xml
}

/// Generate text properties XML with formatting
pub fn generate_text_props(
    size: u32,
    bold: bool,
    italic: bool,
    underline: bool,
    color: Option<&str>,
) -> String {
    let mut props = format!(
        r#"<a:rPr lang="en-US" sz="{}" b="{}" i="{}" dirty="0""#,
        size,
        if bold { "1" } else { "0" },
        if italic { "1" } else { "0" }
    );

    if underline {
        props.push_str(r#" u="sng""#);
    }

    props.push('>');

    if let Some(hex_color) = color {
        let clean_color = hex_color.trim_start_matches('#').to_uppercase();
        props.push_str(&format!(
            r#"<a:solidFill><a:srgbClr val="{clean_color}"/></a:solidFill>"#
        ));
    }

    props.push_str("</a:rPr>");
    props
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_plain_text() {
        let segments = parse_inline_formatting("Hello world");
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text, "Hello world");
        assert!(!segments[0].bold);
        assert!(!segments[0].italic);
    }

    #[test]
    fn test_parse_bold() {
        let segments = parse_inline_formatting("Hello **bold** world");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[1].text, "bold");
        assert!(segments[1].bold);
    }

    #[test]
    fn test_parse_italic() {
        let segments = parse_inline_formatting("Hello *italic* world");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[1].text, "italic");
        assert!(segments[1].italic);
    }

    #[test]
    fn test_parse_code() {
        let segments = parse_inline_formatting("Hello `code` world");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[1].text, "code");
        assert!(segments[1].code);
    }

    #[test]
    fn test_generate_rich_text() {
        let xml = generate_rich_text_runs("Hello **bold**", 1400, false, false, None);
        assert!(xml.contains("b=\"1\""));
        assert!(xml.contains("Hello"));
        assert!(xml.contains("bold"));
    }

    #[test]
    fn test_generate_text_props() {
        let props = generate_text_props(1400, true, false, false, Some("FF0000"));
        assert!(props.contains("b=\"1\""));
        assert!(props.contains("FF0000"));
    }
}
