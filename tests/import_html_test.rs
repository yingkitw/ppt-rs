//! Comprehensive integration tests for HTML to PPTX conversion
//!
//! Tests all aspects of HTML parsing and PPTX generation:
//! - Element parsing (headings, paragraphs, lists, tables, code, images, blockquotes)
//! - Attribute handling (img src/alt, etc.)
//! - Entity decoding
//! - Script/style skipping
//! - Edge cases (empty input, nested elements, special characters)
//! - PPTX structure validation

use ppt_rs::generator::create_pptx_with_content;
use ppt_rs::import::{parse_html, parse_html_with_options, Html2Ppt, HtmlParseOptions};
use zip::ZipArchive;

/// Helper: validate the structure of a generated PPTX
fn validate_pptx_structure(pptx_data: &[u8]) {
    let cursor = std::io::Cursor::new(pptx_data);
    let mut archive = ZipArchive::new(cursor).expect("Should be a valid ZIP archive");

    let mut found = std::collections::HashSet::new();
    for i in 0..archive.len() {
        let file = archive.by_index(i).expect("Should read entry");
        found.insert(file.name().to_string());
    }

    assert!(
        found.contains("[Content_Types].xml"),
        "Missing [Content_Types].xml"
    );
    assert!(found.contains("_rels/.rels"), "Missing _rels/.rels");
    assert!(
        found.contains("ppt/presentation.xml"),
        "Missing ppt/presentation.xml"
    );
}

fn parse_and_validate(html: &str) -> Vec<u8> {
    let slides = parse_html(html).expect("Should parse HTML");
    assert!(!slides.is_empty(), "Should produce at least one slide");
    let pptx = create_pptx_with_content("Test", slides).expect("Should generate PPTX");
    assert!(!pptx.is_empty(), "PPTX data should not be empty");
    validate_pptx_structure(&pptx);
    pptx
}

// ============================================================================
// Element Parsing Tests
// ============================================================================

#[test]
fn test_multiple_h1_slides() {
    let html = "<h1>Slide A</h1><h1>Slide B</h1><h1>Slide C</h1>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides.len(), 3);
    assert_eq!(slides[0].title, "Slide A");
    assert_eq!(slides[1].title, "Slide B");
    assert_eq!(slides[2].title, "Slide C");
}

#[test]
fn test_h2_as_bullet() {
    let html = "<h1>Main</h1><h2>Sub Heading</h2><p>Content</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides.len(), 1);
    assert!(slides[0].content.iter().any(|c| c.contains("Sub Heading")));
}

#[test]
fn test_paragraphs_become_bullets() {
    let html = "<h1>Title</h1><p>First para</p><p>Second para</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content.len(), 2);
    assert_eq!(slides[0].content[0], "First para");
    assert_eq!(slides[0].content[1], "Second para");
}

#[test]
fn test_unordered_list() {
    let html = "<h1>Items</h1><ul><li>A</li><li>B</li><li>C</li></ul>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content.len(), 3);
    assert_eq!(slides[0].content[0], "A");
    assert_eq!(slides[0].content[1], "B");
    assert_eq!(slides[0].content[2], "C");
}

#[test]
fn test_ordered_list() {
    let html = "<h1>Steps</h1><ol><li>First</li><li>Second</li></ol>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content.len(), 2);
    assert_eq!(slides[0].content[0], "First");
}

#[test]
fn test_table_parsing() {
    let html = r#"
        <h1>Data</h1>
        <table>
            <tr><th>Name</th><th>Age</th></tr>
            <tr><td>Alice</td><td>30</td></tr>
            <tr><td>Bob</td><td>25</td></tr>
        </table>
    "#;
    let slides = parse_html(html).unwrap();
    assert!(slides[0].table.is_some(), "Should have a table");
    let table = slides[0].table.as_ref().unwrap();
    assert_eq!(table.rows.len(), 3, "Should have 3 rows (1 header + 2 data)");
    assert_eq!(table.rows[0].cells.len(), 2, "Should have 2 columns");
}

#[test]
fn test_code_block_parsing() {
    let html = "<h1>Code</h1><pre><code>let x = 42;</code></pre>";
    let slides = parse_html(html).unwrap();
    assert!(!slides[0].code_blocks.is_empty());
    assert!(slides[0].code_blocks[0].code.contains("let x = 42;"));
}

#[test]
fn test_blockquote_notes() {
    let html = "<h1>Slide</h1><p>Content</p><blockquote>This is a note.</blockquote>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].notes, Some("This is a note.".to_string()));
}

#[test]
fn test_image_placeholder() {
    let html = r#"<h1>Gallery</h1><img src="photo.png" alt="Sunset">"#;
    let slides = parse_html(html).unwrap();
    assert!(slides[0].content.iter().any(|c| c.contains("Sunset")));
}

#[test]
fn test_image_without_alt() {
    let html = r#"<h1>Gallery</h1><img src="photo.png">"#;
    let slides = parse_html(html).unwrap();
    assert!(slides[0].content.iter().any(|c| c.contains("[Image: image]")));
}

// ============================================================================
// Formatting Tests
// ============================================================================

#[test]
fn test_bold_formatting() {
    let html = "<h1>Test</h1><p><strong>Bold text</strong></p>";
    let slides = parse_html(html).unwrap();
    assert!(slides[0].content[0].contains("**Bold text**"));
}

#[test]
fn test_b_tag_for_bold() {
    let html = "<h1>Test</h1><p><b>Also bold</b></p>";
    let slides = parse_html(html).unwrap();
    assert!(slides[0].content[0].contains("**Also bold**"));
}

#[test]
fn test_italic_formatting() {
    let html = "<h1>Test</h1><p><em>Italic text</em></p>";
    let slides = parse_html(html).unwrap();
    eprintln!("content[0] = {:?}", slides[0].content[0]);
    assert!(slides[0].content[0].contains("*Italic text*"));
}

#[test]
fn test_line_break() {
    let html = "<h1>Test</h1><p>Line1<br>Line2</p>";
    let slides = parse_html(html).unwrap();
    // <br> should create a newline in the text
    assert!(!slides[0].content.is_empty());
}

#[test]
fn test_horizontal_rule_break() {
    let html = "<h1>Slide 1</h1><hr><h1>Slide 2</h1>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides.len(), 2);
}

// ============================================================================
// Entity Decoding Tests
// ============================================================================

#[test]
fn test_ampersand_entity() {
    let html = "<h1>Title</h1><p>AT&amp;T</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content[0], "AT&T");
}

#[test]
fn test_lt_gt_entities() {
    let html = "<h1>Title</h1><p>&lt;tag&gt;</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content[0], "<tag>");
}

#[test]
fn test_quote_entity() {
    let html = "<h1>Title</h1><p>&quot;quoted&quot;</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content[0], "\"quoted\"");
}

#[test]
fn test_numeric_entity() {
    let html = "<h1>Title</h1><p>&#x26;</p>"; // hex &#x26; = &
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content[0], "&");
}

#[test]
fn test_nbsp_entity() {
    let html = "<h1>Title</h1><p>Hello&nbsp;World</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content[0], "Hello\u{00a0}World");
}

// ============================================================================
// Attribute Handling Tests
// ============================================================================

#[test]
fn test_img_with_single_quotes() {
    let html = "<h1>Test</h1><img src='pic.jpg' alt='My Photo'>";
    let slides = parse_html(html).unwrap();
    assert!(slides[0].content.iter().any(|c| c.contains("My Photo")));
}

#[test]
fn test_div_with_multiple_classes() {
    let html = r#"<div class="container main"><h1>Title</h1><p>Text</p></div>"#;
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].title, "Title");
    assert_eq!(slides[0].content[0], "Text");
}

// ============================================================================
// Skip Tags Tests
// ============================================================================

#[test]
fn test_script_tag_skipped() {
    let html = "<h1>Real</h1><script>var x = 1;</script><p>Content</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content.len(), 1);
    assert_eq!(slides[0].content[0], "Content");
}

#[test]
fn test_style_tag_skipped() {
    let html = "<h1>Real</h1><style>.cls{color:red;}</style><p>Visible</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content.len(), 1);
    assert_eq!(slides[0].content[0], "Visible");
}

#[test]
fn test_noscript_tag_skipped() {
    let html = "<h1>Real</h1><noscript>JS disabled</noscript><p>Content</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content.len(), 1);
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_empty_html_returns_error() {
    let result = parse_html("<html></html>");
    assert!(result.is_err());
}

#[test]
fn test_only_whitespace_returns_error() {
    let result = parse_html("   \n   ");
    assert!(result.is_err());
}

#[test]
fn test_self_closing_tags() {
    let html = "<h1>Test</h1><p>Text1</p><br><p>Text2</p>";
    let slides = parse_html(html).unwrap();
    assert!(!slides[0].content.is_empty());
}

#[test]
fn test_no_h1_with_content() {
    let html = "<p>Just a paragraph without any heading</p><p>Another</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_deeply_nested_divs() {
    let html = "<div><div><div><div><h1>Deep</h1><p>Nested content</p></div></div></div></div>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].title, "Deep");
    assert_eq!(slides[0].content[0], "Nested content");
}

#[test]
fn test_special_characters_in_text() {
    let html = "<h1>Title</h1><p>Special: ©®™ ± § ¶</p>";
    let slides = parse_html(html).unwrap();
    assert!(slides[0].content[0].contains('©'));
}

#[test]
fn test_unicode_text() {
    let html = "<h1>标题</h1><p>中文内容 日本語 한국어</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].title, "标题");
    assert!(slides[0].content[0].contains("中文内容"));
}

#[test]
fn test_multiple_blockquotes() {
    let html = "<h1>S</h1><blockquote>Note 1</blockquote><blockquote>Note 2</blockquote>";
    let slides = parse_html(html).unwrap();
    // Only the last blockquote survives (they overwrite)
    assert_eq!(slides[0].notes, Some("Note 2".to_string()));
}

#[test]
fn test_html_with_doctype_and_comments() {
    let html = "<!-- comment --><!DOCTYPE html><h1>Title</h1><p>Content</p><!-- another -->";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].title, "Title");
    assert_eq!(slides[0].content[0], "Content");
}

// ============================================================================
// PPTX Generation & Validation Tests
// ============================================================================

#[test]
fn test_generate_valid_pptx_from_html() {
    let html = r#"
        <h1>Test Presentation</h1>
        <p>Validating PPTX structure.</p>
        <h1>Slide Two</h1>
        <ul><li>Item 1</li><li>Item 2</li></ul>
    "#;
    parse_and_validate(html);
}

#[test]
fn test_generate_pptx_with_table() {
    let html = r#"
        <h1>Table Slide</h1>
        <table>
            <tr><th>A</th><th>B</th></tr>
            <tr><td>1</td><td>2</td></tr>
        </table>
    "#;
    parse_and_validate(html);
}

#[test]
fn test_generate_pptx_with_code() {
    let html = "<h1>Code</h1><pre><code>fn main() {}</code></pre>";
    let pptx = parse_and_validate(html);
    assert!(!pptx.is_empty());
}

#[test]
fn test_large_html_preserves_content() {
    let mut html = String::from("<h1>Lots of Content</h1>");
    for i in 0..20 {
        html.push_str(&format!("<p>Bullet point number {i}</p>"));
    }
    let slides = parse_html(&html).unwrap();
    assert_eq!(slides[0].content.len(), 10); // limited by default max_bullets
}

// ============================================================================
// HtmlParseOptions Tests
// ============================================================================

#[test]
fn test_options_limit_slides() {
    let html = (0..10).map(|i| format!("<h1>Slide {i}</h1>")).collect::<String>();
    let opts = HtmlParseOptions::new().max_slides(3);
    let slides = parse_html_with_options(&html, opts).unwrap();
    assert_eq!(slides.len(), 3);
}

#[test]
fn test_options_limit_bullets() {
    let html = "<h1>Title</h1>".to_string()
        + &(0..20).map(|i| format!("<p>Bullet {i}</p>")).collect::<String>();
    let opts = HtmlParseOptions::new().max_bullets(3);
    let slides = parse_html_with_options(&html, opts).unwrap();
    assert_eq!(slides[0].content.len(), 3);
}

#[test]
fn test_options_disable_images() {
    let html = r#"<h1>Title</h1><img src="x.jpg" alt="Photo"><p>Text</p>"#;
    let opts = HtmlParseOptions::new().include_images(false);
    let slides = parse_html_with_options(html, opts).unwrap();
    // Should only have the <p> content, no image placeholder
    assert_eq!(slides[0].content.len(), 1);
}

#[test]
fn test_options_disable_code() {
    let html = "<h1>Title</h1><pre><code>let x = 1;</code></pre>";
    let opts = HtmlParseOptions::new().include_code(false);
    let slides = parse_html_with_options(html, opts).unwrap();
    assert!(slides[0].code_blocks.is_empty());
}

#[test]
fn test_options_disable_tables() {
    let html = "<h1>Title</h1><table><tr><td>Data</td></tr></table>";
    let opts = HtmlParseOptions::new().include_tables(false);
    let slides = parse_html_with_options(html, opts).unwrap();
    assert!(slides[0].table.is_none());
}

#[test]
fn test_options_builder_pattern() {
    let opts = HtmlParseOptions::new()
        .max_slides(10)
        .max_bullets(5)
        .include_code(false)
        .include_images(false)
        .include_tables(false);

    assert_eq!(opts.max_slides, 10);
    assert_eq!(opts.max_bullets, 5);
    assert!(!opts.include_code);
    assert!(!opts.include_images);
    assert!(!opts.include_tables);
}

// ============================================================================
// Html2Ppt Struct Tests
// ============================================================================

#[test]
fn test_html2ppt_struct_parse() {
    let converter = Html2Ppt::new();
    let slides = converter.parse("<h1>Hello</h1><p>World</p>").unwrap();
    assert_eq!(slides.len(), 1);
    assert_eq!(slides[0].title, "Hello");
}

#[test]
fn test_html2ppt_struct_with_options() {
    let opts = HtmlParseOptions::new().max_slides(1);
    let converter = Html2Ppt::with_options(opts);
    let slides = converter
        .parse("<h1>A</h1><h1>B</h1><h1>C</h1>")
        .unwrap();
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_html2ppt_default() {
    let converter = Html2Ppt::default();
    let slides = converter.parse("<h1>Test</h1>").unwrap();
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_html2ppt_parse_file_not_found() {
    let converter = Html2Ppt::new();
    let result = converter.parse_file("/nonexistent/path.html");
    assert!(result.is_err());
}

// ============================================================================
// HTML Structure Tests
// ============================================================================

#[test]
fn test_div_as_content_container() {
    let html = "<div><h1>In Div</h1><p>Content in div</p></div>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].title, "In Div");
    assert!(slides[0].content.iter().any(|c| c.contains("Content in div")));
}

#[test]
fn test_article_tag() {
    let html = "<article><h1>Article</h1><p>Article content</p></article>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].title, "Article");
}

#[test]
fn test_section_tag() {
    let html = "<section><h1>Section</h1><p>Section content</p></section>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].title, "Section");
}

#[test]
fn test_mixed_content_order() {
    let html = "<h1>Title</h1><p>First</p><h2>Sub</h2><p>Second</p><p>Third</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content.len(), 4);
}

// ============================================================================
// Performance / Stress Tests
// ============================================================================

#[test]
fn test_large_html_no_crash() {
    let mut html = String::from("<h1>Large</h1>");
    for i in 0..100 {
        html.push_str(&format!("<p>Paragraph {i} with some text content to make it realistic.</p>"));
    }
    let result = parse_html(&html);
    assert!(result.is_ok());
    let slides = result.unwrap();
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_deep_nesting_no_stack_overflow() {
    let mut html = String::from("<div>");
    for _ in 0..500 {
        html.push_str("<div>");
    }
    html.push_str("<h1>Deep</h1><p>Bottom</p>");
    for _ in 0..500 {
        html.push_str("</div>");
    }
    let result = parse_html(&html);
    assert!(result.is_ok());
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_empty_string_error() {
    let result = parse_html("");
    assert!(result.is_err());
}

#[test]
fn test_only_tags_no_text_error() {
    let result = parse_html("<html><head></head><body><div></div></body></html>");
    assert!(result.is_err());
}

#[test]
fn test_html2ppt_error_message() {
    let result = parse_html("<html></html>");
    match result {
        Err(msg) => assert!(!msg.is_empty(), "Error message should not be empty"),
        Ok(_) => panic!("Expected error"),
    }
}

// ============================================================================
// Comprehensive Content Tests
// ============================================================================

#[test]
fn test_ordered_list_preserves_content() {
    let html = "<h1>Steps</h1><ol><li>Step A</li><li>Step B</li><li>Step C</li></ol>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content.len(), 3);
    assert_eq!(slides[0].content[0], "Step A");
    assert_eq!(slides[0].content[1], "Step B");
    assert_eq!(slides[0].content[2], "Step C");
}

#[test]
fn test_br_creates_newline_in_paragraph() {
    let html = "<h1>Title</h1><p>Line A<br>Line B<br>Line C</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content.len(), 1);
    assert!(slides[0].content[0].contains("Line A\nLine B\nLine C"));
}

#[test]
fn test_link_text_preserved() {
    let html = r#"<h1>Links</h1><p>Click <a href="https://example.com">here</a> now</p>"#;
    let slides = parse_html(html).unwrap();
    assert!(slides[0].content[0].contains("Click here now"));
}

#[test]
fn test_h2_as_bold_subheading() {
    let html = "<h1>Main</h1><h2>Sub One</h2><p>Text A</p><h2>Sub Two</h2><p>Text B</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content.len(), 4);
    assert!(slides[0].content[0].contains("**Sub One**"));
    assert!(slides[0].content[2].contains("**Sub Two**"));
}

#[test]
fn test_h3_h4_h5_h6_as_bold() {
    let html = "<h1>Title</h1><h3>Level 3</h3><p>Content</p><h4>Level 4</h4>";
    let slides = parse_html(html).unwrap();
    assert!(slides[0].content.iter().any(|c| c.contains("**Level 3**")));
    assert!(slides[0].content.iter().any(|c| c.contains("**Level 4**")));
}

#[test]
fn test_content_before_first_h1_creates_default_title() {
    let html = "<p>Intro paragraph</p><p>More intro</p><h1>Real Title</h1><p>After title</p>";
    let slides = parse_html(html).unwrap();
    // Content before h1 creates auto-slide "Overview", then h1 creates "Real Title"
    assert_eq!(slides.len(), 2);
    // First slide: auto-titled from presentation_title fallback
    assert!(slides[0].content.iter().any(|c| c.contains("Intro paragraph")));
    assert!(slides[0].content.iter().any(|c| c.contains("More intro")));
    // Second slide: explicitly titled from h1
    assert_eq!(slides[1].title, "Real Title");
    assert!(slides[1].content.iter().any(|c| c.contains("After title")));
}

#[test]
fn test_multiline_code_block_preserved() {
    let html = "<h1>Code</h1><pre><code>fn hello() {\n    println!(\"hi\");\n}</code></pre>";
    let slides = parse_html(html).unwrap();
    assert!(!slides[0].code_blocks.is_empty());
    let code = &slides[0].code_blocks[0].code;
    assert!(code.contains("fn hello()"));
    assert!(code.contains("println!"));
}

#[test]
fn test_empty_table_does_not_crash() {
    let html = "<h1>Title</h1><table></table><p>After empty table</p>";
    let slides = parse_html(html).unwrap();
    assert!(slides[0].table.is_none());
    assert!(slides[0].content.iter().any(|c| c.contains("After empty table")));
}

#[test]
fn test_table_with_irregular_columns() {
    let html = "<h1>Data</h1><table><tr><th>A</th><th>B</th><th>C</th></tr><tr><td>1</td></tr></table>";
    let slides = parse_html(html).unwrap();
    assert!(slides[0].table.is_some());
}

#[test]
fn test_unicode_and_symbols_mixed() {
    let html = "<h1>Mixed</h1><p>Unicode: 你好 日本語 Español</p><p>Symbols: ©®™ €£¥ §¶</p>";
    let slides = parse_html(html).unwrap();
    assert!(slides[0].content[0].contains("你好"));
    assert!(slides[0].content[0].contains("Español"));
    assert!(slides[0].content[1].contains("©"));
}

#[test]
fn test_mixed_case_tags() {
    let html = "<H1>Title</H1><P>Paragraph</P><STRONG>Bold</STRONG>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides.len(), 1);
    assert_eq!(slides[0].title, "Title");
}

#[test]
fn test_multiple_hr_in_sequence() {
    let html = "<h1>Slide 1</h1><p>Content</p><hr><hr><hr><h1>Slide 2</h1>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides.len(), 2);
}

#[test]
fn test_blockquote_after_hr_on_new_slide() {
    let html = "<h1>Slide 1</h1><p>Content</p><hr><p>Slide 2 content</p><blockquote>Note for slide 2</blockquote>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides.len(), 2);
    // The blockquote should apply to slide 2 (the p creates it, blockquote adds notes)
    assert_eq!(slides[1].notes, Some("Note for slide 2".to_string()));
}

#[test]
fn test_nested_bold_italic() {
    let html = "<h1>Format</h1><p><strong>Bold <em>BoldItalic</em></strong> <em>Italic</em></p>";
    let slides = parse_html(html).unwrap();
    let c = &slides[0].content[0];
    assert!(c.contains("**Bold *BoldItalic***"));
    assert!(c.contains("*Italic*"));
}

#[test]
fn test_entity_variations() {
    let html = "<h1>Entities</h1><p>&amp; &lt; &gt; &quot; &apos; &nbsp; &#x26; &#38; &#x20AC;</p>";
    let slides = parse_html(html).unwrap();
    let c = &slides[0].content[0];
    assert!(c.contains('&'));
    assert!(c.contains('<'));
    assert!(c.contains('>'));
    assert!(c.contains('"'));
    assert!(c.contains('\''));
    assert!(c.contains('\u{00a0}'));
    assert!(c.contains('€'));
}

#[test]
fn test_large_text_block_no_crash() {
    let mut html = String::from("<h1>Large Text</h1><p>");
    html.push_str(&"A".repeat(100_000));
    html.push_str("</p>");
    let result = parse_html(&html);
    assert!(result.is_ok());
    let slides = result.unwrap();
    assert!(!slides[0].content.is_empty());
    assert!(slides[0].content[0].len() > 50000);
}

#[test]
fn test_article_section_div_all_produce_bullets() {
    // Only 8 chars in h2 + items to ensure we don't hit max_bullets
    let html = "<h1>Title</h1><div><p>Div content</p></div><article><p>Article content</p></article><section><p>Section content</p></section><main><p>Main content</p></main>";
    let slides = parse_html(html).unwrap();
    assert!(slides[0].content.iter().any(|c| c.contains("Div content")));
    assert!(slides[0].content.iter().any(|c| c.contains("Article content")));
    assert!(slides[0].content.iter().any(|c| c.contains("Section content")));
    assert!(slides[0].content.iter().any(|c| c.contains("Main content")));
}

#[test]
fn test_html_with_title_tag_fallback() {
    let html = "<html><head><title>My Presentation</title></head><body><h1>Slide One</h1></body></html>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].title, "Slide One");
    // The <title> content should be skipped, but the first h1 is the title
    // (title tag content should never appear in slide output)
    assert!(!slides[0].content.iter().any(|c| c.contains("My Presentation")));
}

#[test]
fn test_mixed_content_before_and_after_table() {
    let html = r#"
        <h1>Report</h1>
        <p>Summary of findings.</p>
        <table>
            <tr><th>Item</th><th>Result</th></tr>
            <tr><td>Test A</td><td>Pass</td></tr>
        </table>
        <p>Additional notes after table.</p>
    "#;
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content.len(), 2);
    assert!(slides[0].content[0].contains("Summary"));
    assert!(slides[0].content[1].contains("Additional notes"));
    assert!(slides[0].table.is_some());
}

#[test]
fn test_self_closing_br_hr_img_in_attributes() {
    let html = r#"<h1>Title</h1><p>Text<br><br>More text</p><hr><h1>Slide 2</h1><img src="test.png" alt="Test">"#;
    let slides = parse_html(html).unwrap();
    assert_eq!(slides.len(), 2);
    assert!(slides[0].content[0].contains("Text\n\nMore text"));
    assert!(slides[1].content.iter().any(|c| c.contains("[Image: Test]")));
}

#[test]
fn test_form_nav_svg_canvas_iframe_skipped() {
    let html = "<h1>Real</h1><form>form data</form><nav>nav links</nav><svg>svg content</svg><canvas>canvas</canvas><iframe>iframe</iframe><p>Visible</p>";
    let slides = parse_html(html).unwrap();
    assert_eq!(slides[0].content.len(), 1);
    assert_eq!(slides[0].content[0], "Visible");
}

#[test]
fn test_options_disable_tables_preserves_other_content() {
    let html = "<h1>Title</h1><p>Before</p><table><tr><td>Data</td></tr></table><p>After</p>";
    let opts = HtmlParseOptions::new().include_tables(false);
    let slides = parse_html_with_options(html, opts).unwrap();
    assert!(slides[0].table.is_none());
    assert!(slides[0].content.iter().any(|c| c.contains("Before")));
    assert!(slides[0].content.iter().any(|c| c.contains("After")));
}

#[test]
fn test_section_content_nested_deep_with_skip_tags() {
    let html = "<h1>Title</h1><section><div><article><p>Deep content</p></article></div></section>";
    let slides = parse_html(html).unwrap();
    assert!(slides[0].content.iter().any(|c| c.contains("Deep content")));
}

#[test]
fn test_content_with_code_entities() {
    let html = "<h1>Code</h1><pre><code>if x &lt; 5 &amp;&amp; y &gt; 10 {\n    return true;\n}</code></pre>";
    let slides = parse_html(html).unwrap();
    assert!(!slides[0].code_blocks.is_empty());
    let code = &slides[0].code_blocks[0].code;
    assert!(code.contains("if x < 5 && y > 10"));
}

    #[test]
    fn test_parse_comprehensive_fixture() {
        let html = std::fs::read_to_string("tests/fixtures/html/comprehensive.html")
            .expect("Should read comprehensive fixture");
        let slides = parse_html(&html).expect("Should parse comprehensive HTML");
        assert!(!slides.is_empty(), "Should produce slides");
        assert_eq!(slides.len(), 11, "Should produce exactly 11 slides");

        // -----------------------------------------------------------------------
        // Slide 0: "Welcome to html2ppt" — also tests styled <h1> (title-level CSS)
        // -----------------------------------------------------------------------
        let s0 = &slides[0];
        assert_eq!(s0.title, "Welcome to html2ppt");
        assert_eq!(s0.content.len(), 3);
        assert_eq!(s0.content[0], "**Bold** and *italic* and **also bold** and *also italic* text.");
        assert_eq!(s0.content[1], "Visit Example website for details.");
        assert_eq!(s0.content[2], "Line one\nLine two\nLine three");
        assert_eq!(s0.bullets.len(), 3);
        // No inline CSS on these paragraphs — format should be None
        assert!(s0.bullets[0].format.is_none());
        assert!(s0.bullets[1].format.is_none());
        assert!(s0.bullets[2].format.is_none());
        // Styled <h1> — title-level properties
        assert_eq!(s0.title_color, Some("000080".to_string())); // navy
        assert_eq!(s0.title_size, Some(36)); // 48px → 36pt
        assert!(s0.title_bold);
        assert!(!s0.title_italic);
        assert!(!s0.title_underline);
        assert!(!s0.has_table);
        assert!(!s0.has_image);
        assert!(s0.code_blocks.is_empty());
        assert!(s0.notes.is_none());

        // -----------------------------------------------------------------------
        // Slide 1: "Inline CSS Styles" — per-bullet inline CSS verification
        // -----------------------------------------------------------------------
        let s1 = &slides[1];
        assert_eq!(s1.title, "Inline CSS Styles");
        assert_eq!(s1.content.len(), 7);
        assert_eq!(s1.content[0], "This text is red from inline style.");
        assert_eq!(s1.content[1], "This text is blue and larger.");
        assert_eq!(s1.content[2], "This paragraph has a yellow highlight.");
        assert_eq!(s1.content[3], "Bold, italic, and purple.");
        assert_eq!(s1.content[4], "This text is underlined.");
        assert_eq!(s1.content[5], "This text is struck through.");
        assert_eq!(s1.content[6], "This uses Courier New font.");
        assert_eq!(s1.bullets.len(), 7);
        // b[0]: color:red
        assert!(s1.bullets[0].format.is_some());
        let f0 = s1.bullets[0].format.as_ref().unwrap();
        assert_eq!(f0.color, Some("FF0000".to_string()));
        assert!(!f0.bold);
        assert!(!f0.italic);
        assert!(!f0.underline);
        assert!(!f0.strikethrough);
        assert!(f0.highlight.is_none());
        assert!(f0.font_size.is_none());
        // b[1]: color:blue; font-size:22px
        let f1 = s1.bullets[1].format.as_ref().unwrap();
        assert_eq!(f1.color, Some("0000FF".to_string()));
        assert_eq!(f1.font_size, Some(17)); // 22px → 17pt
        assert!(f1.highlight.is_none());
        // b[2]: background-color:yellow → highlight
        let f2 = s1.bullets[2].format.as_ref().unwrap();
        assert_eq!(f2.highlight, Some("FFFF00".to_string()));
        assert!(f2.color.is_none());
        // b[3]: font-weight:bold; font-style:italic; color:purple
        let f3 = s1.bullets[3].format.as_ref().unwrap();
        assert!(f3.bold);
        assert!(f3.italic);
        assert_eq!(f3.color, Some("800080".to_string()));
        // b[4]: text-decoration:underline
        let f4 = s1.bullets[4].format.as_ref().unwrap();
        assert!(f4.underline);
        assert!(!f4.strikethrough);
        // b[5]: text-decoration:line-through → strikethrough
        let f5 = s1.bullets[5].format.as_ref().unwrap();
        assert!(f5.strikethrough);
        assert!(!f5.underline);
        // b[6]: font-family:Courier New
        let f6 = s1.bullets[6].format.as_ref().unwrap();
        assert_eq!(f6.font_family, Some("Courier New".to_string()));

        // -----------------------------------------------------------------------
        // Slide 2: "Lists & Sections" — ul/ol, h2 sub-headings, containers
        // -----------------------------------------------------------------------
        let s2 = &slides[2];
        assert_eq!(s2.title, "Lists & Sections");
        assert_eq!(s2.content.len(), 10); // max_bullets=10; "Section content." is cut
        assert_eq!(s2.content[0], "**Fruits**");     // h2 → bold
        assert_eq!(s2.content[1], "Apples");
        assert_eq!(s2.content[2], "Bananas");
        assert_eq!(s2.content[3], "Cherries");
        assert_eq!(s2.content[4], "**Steps**");       // h2 → bold
        assert_eq!(s2.content[5], "First step");
        assert_eq!(s2.content[6], "Second step");
        assert_eq!(s2.content[7], "Third step");
        assert_eq!(s2.content[8], "Content inside a div.");
        assert_eq!(s2.content[9], "Article content.");
        // No inline styles in this slide
        for (i, b) in s2.bullets.iter().enumerate() {
            assert!(b.format.is_none(), "Slide 2 bullet {i} should have no format");
            assert_eq!(b.level, 0);
        }

        // -----------------------------------------------------------------------
        // Slide 3: "Heading Levels" — h2–h6 as bold sub-headings
        // -----------------------------------------------------------------------
        let s3 = &slides[3];
        assert_eq!(s3.title, "Heading Levels");
        assert_eq!(s3.content.len(), 10); // max_bullets=10; "Content under level 6." is cut
        assert_eq!(s3.content[0], "Sub-headings render as bold bullet points:");
        assert_eq!(s3.content[1], "**Level 2 Heading**");
        assert_eq!(s3.content[2], "Content under level 2.");
        assert_eq!(s3.content[3], "**Level 3 Heading**");
        assert_eq!(s3.content[4], "Content under level 3.");
        assert_eq!(s3.content[5], "**Level 4 Heading**");
        assert_eq!(s3.content[6], "Content under level 4.");
        assert_eq!(s3.content[7], "**Level 5 Heading**");
        assert_eq!(s3.content[8], "Content under level 5.");
        assert_eq!(s3.content[9], "**Level 6 Heading**");
        for b in &s3.bullets {
            assert!(b.format.is_none());
        }

        // -----------------------------------------------------------------------
        // Slide 4: "Data Tables" — table presence and content
        // -----------------------------------------------------------------------
        let s4 = &slides[4];
        assert_eq!(s4.title, "Data Tables");
        assert_eq!(s4.content.len(), 1);
        assert_eq!(s4.content[0], "Financial results for Q1 through Q3.");
        assert!(s4.has_table);
        assert!(!s4.has_image);
        assert!(s4.table.is_some());
        let table = s4.table.as_ref().unwrap();
        assert_eq!(table.rows.len(), 4);
        assert_eq!(table.rows[0].cells.len(), 4);
        assert_eq!(table.rows[0].cells[0].text, "Quarter");
        assert_eq!(table.rows[0].cells[1].text, "Revenue");
        assert_eq!(table.rows[0].cells[2].text, "Costs");
        assert_eq!(table.rows[0].cells[3].text, "Profit");
        assert_eq!(table.rows[1].cells[0].text, "Q1");
        assert_eq!(table.rows[1].cells[1].text, "$100K");
        assert_eq!(table.rows[1].cells[2].text, "$60K");
        assert_eq!(table.rows[1].cells[3].text, "$40K");
        assert_eq!(table.rows[2].cells[0].text, "Q2");
        assert_eq!(table.rows[2].cells[1].text, "$120K");
        assert_eq!(table.rows[2].cells[2].text, "$70K");
        assert_eq!(table.rows[2].cells[3].text, "$50K");
        assert_eq!(table.rows[3].cells[0].text, "Q3");
        assert_eq!(table.rows[3].cells[1].text, "$150K");
        assert_eq!(table.rows[3].cells[2].text, "$90K");
        assert_eq!(table.rows[3].cells[3].text, "$60K");

        // -----------------------------------------------------------------------
        // Slide 5: "Code & Notes" — code block + speaker notes
        // -----------------------------------------------------------------------
        let s5 = &slides[5];
        assert_eq!(s5.title, "Code & Notes");
        assert_eq!(s5.content.len(), 1);
        assert_eq!(s5.content[0], "Rust factorial example:");
        assert_eq!(s5.code_blocks.len(), 1);
        assert!(s5.code_blocks[0].code.contains("fn factorial(n: u64) -> u64"));
        assert!(s5.code_blocks[0].code.contains("match n"));
        assert!(s5.code_blocks[0].code.contains("n * factorial(n - 1)"));
        assert_eq!(s5.notes, Some("Recursion is elegant but watch the call stack depth!".to_string()));
        assert!(!s5.has_table);
        assert!(!s5.has_image);

        // -----------------------------------------------------------------------
        // Slide 6: "Formatting Combinations" — nested bold/italic, inline code, link
        // -----------------------------------------------------------------------
        let s6 = &slides[6];
        assert_eq!(s6.title, "Formatting Combinations");
        assert_eq!(s6.content.len(), 4);
        assert_eq!(s6.content[0], "Text with **bold**, *italic*, and ***bold italic*** combined.");
        assert_eq!(s6.content[1], "Inline code snippets in paragraphs.");
        assert_eq!(s6.content[2], "A link with bold text in context.");
        assert_eq!(s6.content[3], "Green text with **bigger bold** inside.");
        // b[3] has inline style (color:green)
        assert!(s6.bullets[3].format.is_some());
        assert_eq!(s6.bullets[3].format.as_ref().unwrap().color, Some("008000".to_string()));
        // b[0-2] have no inline styles
        assert!(s6.bullets[0].format.is_none());
        assert!(s6.bullets[1].format.is_none());
        assert!(s6.bullets[2].format.is_none());

        // -----------------------------------------------------------------------
        // Slide 7: "Images & Entities" — img placeholders, entity decoding
        // -----------------------------------------------------------------------
        let s7 = &slides[7];
        assert_eq!(s7.title, "Images & Entities");
        assert_eq!(s7.content.len(), 7);
        assert_eq!(s7.content[0], "[Image: Q4 Projections]");   // img with alt
        assert_eq!(s7.content[1], "Chart:");
        assert_eq!(s7.content[2], "[Image: image]");             // img without alt
        assert_eq!(s7.content[3], "Image without alt:");
        assert_eq!(s7.content[4], "Entities: AT&T <html> \"quote\" 'apos' value\u{00a0}here");
        assert_eq!(s7.content[5], "Numeric: & # $ €");
        assert_eq!(s7.content[6], "Entity demo with inline style.");
        // b[6] has color:darkblue and font-size:20px
        let f7 = s7.bullets[6].format.as_ref().unwrap();
        assert_eq!(f7.color, Some("00008B".to_string())); // darkblue
        assert_eq!(f7.font_size, Some(15)); // 20px → 15pt
        // Verify entity decoding details
        assert!(s7.content[4].contains("AT&T"));
        assert!(s7.content[4].contains("<html>"));
        assert!(s7.content[4].contains("\"quote\""));
        assert!(s7.content[5].contains("€"));

        // -----------------------------------------------------------------------
        // Slide 8: HR break — new slide from <hr> within Images & Entities
        // -----------------------------------------------------------------------
        let s8 = &slides[8];
        assert_eq!(s8.title, "Welcome to html2ppt"); // inherits presentation_title
        assert_eq!(s8.content.len(), 1);
        assert_eq!(s8.content[0], "Content that appears after a horizontal rule break.");
        assert!(s8.table.is_none());
        assert!(s8.code_blocks.is_empty());
        assert!(s8.notes.is_none());

        // -----------------------------------------------------------------------
        // Slide 9: "Unicode & Symbols" — multi-script, symbols
        // -----------------------------------------------------------------------
        let s9 = &slides[9];
        assert_eq!(s9.title, "Unicode & Symbols");
        assert_eq!(s9.content.len(), 2);
        assert_eq!(s9.content[0], "Languages: 你好, 日本語, 한국어, Español, Français, Русский");
        assert_eq!(s9.content[1], "Symbols: © 2026 ® Registered ™ Trademark € £ ¥ ₹");
        assert!(s9.content[0].contains("你好"));
        assert!(s9.content[0].contains("日本語"));
        assert!(s9.content[0].contains("Español"));
        assert!(s9.content[1].contains("©"));
        assert!(s9.content[1].contains("€"));
        assert!(s9.content[1].contains("¥"));
        assert!(s9.content[1].contains("₹"));

        // -----------------------------------------------------------------------
        // Slide 10: "Style Inheritance" — parent → child CSS inheritance + override
        // -----------------------------------------------------------------------
        let s10 = &slides[10];
        assert_eq!(s10.title, "Style Inheritance");
        assert_eq!(s10.content.len(), 3);
        assert_eq!(s10.content[0], "This paragraph inherits teal color from the parent div.");
        assert_eq!(s10.content[1], "This one is bold and still teal from inheritance.");
        assert_eq!(s10.content[2], "This overrides the parent color to dark orange.");
        // b[0]: inherits color:teal from <div>
        assert!(s10.bullets[0].format.is_some());
        let f10_0 = s10.bullets[0].format.as_ref().unwrap();
        assert_eq!(f10_0.color, Some("008080".to_string())); // teal
        assert!(!f10_0.bold);
        assert!(!f10_0.italic);
        // b[1]: inherits teal + adds font-weight:bold
        assert!(s10.bullets[1].format.is_some());
        let f10_1 = s10.bullets[1].format.as_ref().unwrap();
        assert_eq!(f10_1.color, Some("008080".to_string())); // inherited teal
        assert!(f10_1.bold);
        // b[2]: overrides color to darkorange
        assert!(s10.bullets[2].format.is_some());
        let f10_2 = s10.bullets[2].format.as_ref().unwrap();
        assert_eq!(f10_2.color, Some("FF8C00".to_string())); // darkorange
        assert!(!f10_2.bold);
    }

#[test]
fn test_generate_pptx_from_comprehensive_html() {
    let html = std::fs::read_to_string("tests/fixtures/html/comprehensive.html")
        .expect("Should read comprehensive fixture");
    parse_and_validate(&html);
}

#[test]
fn test_presentation_api_with_html_content() {
    use ppt_rs::api::Presentation;
    
    let html = "<h1>API Demo</h1><p>Slide via Presentation API</p><h1>Second Slide</h1><ul><li>Item one</li><li>Item two</li></ul>";
    let slides = parse_html(html).unwrap();
    
    let pres = Presentation::with_title("API Test")
        .add_slide(slides[0].clone())
        .add_slide(slides[1].clone());
    
    assert_eq!(pres.slide_count(), 2);
    assert_eq!(pres.get_title(), "API Test");
    assert_eq!(pres.slides()[0].title, "API Demo");
    
    // Verify it builds to valid PPTX
    let pptx = pres.build().expect("Should build PPTX");
    validate_pptx_structure(&pptx);
}

#[test]
fn test_html2ppt_default_options_equal_new() {
    let default1 = HtmlParseOptions::new();
    let default2 = HtmlParseOptions::default();
    assert_eq!(default1.max_slides, default2.max_slides);
    assert_eq!(default1.max_bullets, default2.max_bullets);
    assert!(default1.include_code);
    assert!(default1.include_tables);
    assert!(default1.include_images);
}
