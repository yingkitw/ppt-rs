//! Comprehensive tests for different PowerPoint elements
//!
//! Tests various PPTX components: slides, text, formatting, layouts, etc.
//! Also generates example files demonstrating each element type

use ppt_rs::generator::{
    SlideContent, create_pptx_with_content,
};
use std::fs;
use std::path::Path;

// ============================================================================
// SLIDE CONTENT TESTS
// ============================================================================

#[test]
fn test_slide_with_title_only() {
    let slides = [SlideContent::new("Title Only Slide")];
    
    assert_eq!(slides[0].title, "Title Only Slide");
    assert_eq!(slides[0].content.len(), 0);
    assert_eq!(slides[0].title_size, Some(44));
}

#[test]
fn test_slide_with_single_bullet() {
    let slide = SlideContent::new("Single Bullet")
        .add_bullet("One point");
    
    assert_eq!(slide.content.len(), 1);
    assert_eq!(slide.content[0], "One point");
}

#[test]
fn test_slide_with_multiple_bullets() {
    let slide = SlideContent::new("Multiple Bullets")
        .add_bullet("First point")
        .add_bullet("Second point")
        .add_bullet("Third point")
        .add_bullet("Fourth point")
        .add_bullet("Fifth point");
    
    assert_eq!(slide.content.len(), 5);
    assert_eq!(slide.content[0], "First point");
    assert_eq!(slide.content[4], "Fifth point");
}

#[test]
fn test_slide_with_long_text() {
    let long_text = "This is a very long bullet point that contains multiple words and should be properly handled by the PPTX generator without any issues or truncation";
    let slide = SlideContent::new("Long Text Slide")
        .add_bullet(long_text);
    
    assert_eq!(slide.content[0], long_text);
    assert!(slide.content[0].len() > 100);
}

#[test]
fn test_slide_with_special_characters() {
    let slide = SlideContent::new("Special Characters")
        .add_bullet("Symbols: & < > \" '")
        .add_bullet("Numbers: 123 456 789")
        .add_bullet("Mixed: Test@123 & More!");
    
    assert_eq!(slide.content.len(), 3);
    assert!(slide.content[0].contains("&"));
    assert!(slide.content[0].contains("<"));
}

#[test]
fn test_slide_with_empty_bullets() {
    let slide = SlideContent::new("Title")
        .add_bullet("")
        .add_bullet("Valid bullet")
        .add_bullet("");
    
    assert_eq!(slide.content.len(), 3);
}

#[test]
fn test_slide_with_unicode_text() {
    let slide = SlideContent::new("Unicode Test")
        .add_bullet("English text")
        .add_bullet("日本語テキスト")
        .add_bullet("中文文本")
        .add_bullet("Русский текст");
    
    assert_eq!(slide.content.len(), 4);
}

// ============================================================================
// FONT SIZE TESTS
// ============================================================================

#[test]
fn test_slide_with_small_fonts() {
    let slide = SlideContent::new("Small Fonts")
        .title_size(18)
        .content_size(12)
        .add_bullet("Small content");
    
    assert_eq!(slide.title_size, Some(18));
    assert_eq!(slide.content_size, Some(12));
}

#[test]
fn test_slide_with_large_fonts() {
    let slide = SlideContent::new("Large Fonts")
        .title_size(72)
        .content_size(48)
        .add_bullet("Large content");
    
    assert_eq!(slide.title_size, Some(72));
    assert_eq!(slide.content_size, Some(48));
}

#[test]
fn test_slide_with_extreme_font_sizes() {
    let slide = SlideContent::new("Extreme")
        .title_size(8)
        .content_size(96)
        .add_bullet("Huge");
    
    assert_eq!(slide.title_size, Some(8));
    assert_eq!(slide.content_size, Some(96));
}

#[test]
fn test_slide_with_matching_font_sizes() {
    let slide = SlideContent::new("Same Size")
        .title_size(28)
        .content_size(28)
        .add_bullet("Same");
    
    assert_eq!(slide.title_size, slide.content_size);
}

// ============================================================================
// BOLD FORMATTING TESTS
// ============================================================================

#[test]
fn test_slide_with_bold_title() {
    let slide = SlideContent::new("Bold Title")
        .title_bold(true)
        .content_bold(false)
        .add_bullet("Regular content");
    
    assert!(slide.title_bold);
    assert!(!slide.content_bold);
}

#[test]
fn test_slide_with_bold_content() {
    let slide = SlideContent::new("Regular Title")
        .title_bold(false)
        .content_bold(true)
        .add_bullet("Bold content");
    
    assert!(!slide.title_bold);
    assert!(slide.content_bold);
}

#[test]
fn test_slide_with_all_bold() {
    let slide = SlideContent::new("All Bold")
        .title_bold(true)
        .content_bold(true)
        .add_bullet("Bold bullet");
    
    assert!(slide.title_bold);
    assert!(slide.content_bold);
}

#[test]
fn test_slide_with_no_bold() {
    let slide = SlideContent::new("No Bold")
        .title_bold(false)
        .content_bold(false)
        .add_bullet("Regular bullet");
    
    assert!(!slide.title_bold);
    assert!(!slide.content_bold);
}

// ============================================================================
// COMBINED FORMATTING TESTS
// ============================================================================

#[test]
fn test_slide_with_all_formatting() {
    let slide = SlideContent::new("Full Formatting")
        .title_size(56)
        .title_bold(true)
        .content_size(32)
        .content_bold(true)
        .add_bullet("Fully formatted");
    
    assert_eq!(slide.title_size, Some(56));
    assert!(slide.title_bold);
    assert_eq!(slide.content_size, Some(32));
    assert!(slide.content_bold);
}

#[test]
fn test_slide_with_mixed_formatting() {
    let slide = SlideContent::new("Mixed")
        .title_size(48)
        .title_bold(false)
        .content_size(20)
        .content_bold(true)
        .add_bullet("Mixed");
    
    assert_eq!(slide.title_size, Some(48));
    assert!(!slide.title_bold);
    assert_eq!(slide.content_size, Some(20));
    assert!(slide.content_bold);
}

// ============================================================================
// PRESENTATION STRUCTURE TESTS
// ============================================================================

#[test]
fn test_single_slide_presentation() {
    let slides = [SlideContent::new("Only Slide")
            .add_bullet("Content")];
    
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_two_slide_presentation() {
    let slides = [SlideContent::new("Slide 1")
            .add_bullet("First"),
        SlideContent::new("Slide 2")
            .add_bullet("Second")];
    
    assert_eq!(slides.len(), 2);
    assert_eq!(slides[0].title, "Slide 1");
    assert_eq!(slides[1].title, "Slide 2");
}

#[test]
fn test_ten_slide_presentation() {
    let mut slides = Vec::new();
    for i in 1..=10 {
        slides.push(
            SlideContent::new(&format!("Slide {}", i))
                .add_bullet(&format!("Content {}", i))
        );
    }
    
    assert_eq!(slides.len(), 10);
    assert_eq!(slides[0].title, "Slide 1");
    assert_eq!(slides[9].title, "Slide 10");
}

#[test]
fn test_presentation_with_varied_content() {
    let slides = [SlideContent::new("Title Slide")
            .title_size(60)
            .add_bullet("Subtitle"),
        SlideContent::new("Content Slide")
            .add_bullet("Point 1")
            .add_bullet("Point 2")
            .add_bullet("Point 3"),
        SlideContent::new("Conclusion")
            .title_bold(true)
            .content_bold(true)
            .add_bullet("Summary")];
    
    assert_eq!(slides.len(), 3);
    assert_eq!(slides[0].title_size, Some(60));
    assert_eq!(slides[1].content.len(), 3);
    assert!(slides[2].title_bold);
}

// ============================================================================
// PPTX GENERATION TESTS - Different Element Types
// ============================================================================

#[test]
fn test_generate_title_only_slides() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Title Only 1"),
        SlideContent::new("Title Only 2"),
        SlideContent::new("Title Only 3"),
    ];

    let result = create_pptx_with_content("Title Only Slides", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_title_only_slides.pptx";
    let write_result = fs::write(output_path, pptx_data);
    assert!(write_result.is_ok());
    assert!(Path::new(output_path).exists());
}

#[test]
fn test_generate_single_bullet_slides() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Single Bullet 1")
            .add_bullet("One point"),
        SlideContent::new("Single Bullet 2")
            .add_bullet("Another point"),
    ];

    let result = create_pptx_with_content("Single Bullet Slides", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_single_bullet.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_many_bullets_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let mut slide = SlideContent::new("Many Bullets");
    for i in 1..=10 {
        slide = slide.add_bullet(&format!("Bullet point {}", i));
    }

    let slides = vec![slide];

    let result = create_pptx_with_content("Many Bullets", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_many_bullets.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_long_text_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let long_text = "This is a very long bullet point that contains multiple words and demonstrates how the PPTX generator handles extended text content without truncation or formatting issues";
    
    let slides = vec![
        SlideContent::new("Long Text Demonstration")
            .add_bullet(long_text)
            .add_bullet("Another long point with more content that spans multiple words"),
    ];

    let result = create_pptx_with_content("Long Text", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_long_text.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_special_characters_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Special Characters & Symbols")
            .add_bullet("Ampersand: &")
            .add_bullet("Less than: <")
            .add_bullet("Greater than: >")
            .add_bullet("Quotes: \" and '")
            .add_bullet("Mixed: Test & <More> \"Content\""),
    ];

    let result = create_pptx_with_content("Special Characters", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_special_characters.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_unicode_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Unicode Text Support")
            .add_bullet("English: Hello World")
            .add_bullet("日本語: こんにちは")
            .add_bullet("中文: 你好")
            .add_bullet("한국어: 안녕하세요")
            .add_bullet("Русский: Привет"),
    ];

    let result = create_pptx_with_content("Unicode Support", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_unicode.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_font_size_variations() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Tiny Fonts")
            .title_size(12)
            .content_size(8)
            .add_bullet("Very small text"),
        SlideContent::new("Small Fonts")
            .title_size(24)
            .content_size(16)
            .add_bullet("Small text"),
        SlideContent::new("Medium Fonts")
            .title_size(44)
            .content_size(28)
            .add_bullet("Medium text"),
        SlideContent::new("Large Fonts")
            .title_size(64)
            .content_size(40)
            .add_bullet("Large text"),
        SlideContent::new("Huge Fonts")
            .title_size(80)
            .content_size(56)
            .add_bullet("Very large text"),
    ];

    let result = create_pptx_with_content("Font Size Variations", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_font_variations.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_bold_variations() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Regular Title")
            .title_bold(false)
            .content_bold(false)
            .add_bullet("Regular content"),
        SlideContent::new("Bold Title")
            .title_bold(true)
            .content_bold(false)
            .add_bullet("Regular content"),
        SlideContent::new("Regular Title")
            .title_bold(false)
            .content_bold(true)
            .add_bullet("Bold content"),
        SlideContent::new("Bold Title")
            .title_bold(true)
            .content_bold(true)
            .add_bullet("Bold content"),
    ];

    let result = create_pptx_with_content("Bold Variations", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_bold_variations.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_comprehensive_presentation() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Title Slide")
            .title_size(60)
            .title_bold(true)
            .content_size(32)
            .add_bullet("Subtitle with larger font"),
        SlideContent::new("Introduction")
            .add_bullet("Welcome to the presentation")
            .add_bullet("Overview of topics")
            .add_bullet("Agenda for today"),
        SlideContent::new("Key Points")
            .title_bold(true)
            .content_bold(true)
            .add_bullet("Important point 1")
            .add_bullet("Important point 2")
            .add_bullet("Important point 3"),
        SlideContent::new("Details & Examples")
            .title_size(40)
            .content_size(24)
            .add_bullet("Detailed explanation")
            .add_bullet("Real-world examples")
            .add_bullet("Case studies"),
        SlideContent::new("Conclusion")
            .title_bold(true)
            .add_bullet("Summary of key takeaways")
            .add_bullet("Next steps")
            .add_bullet("Thank you"),
    ];

    let result = create_pptx_with_content("Comprehensive Presentation", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_comprehensive.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_minimal_presentation() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Minimal"),
    ];

    let result = create_pptx_with_content("Minimal", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_minimal.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}
