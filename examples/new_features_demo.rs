//! New Features Demonstration
//!
//! This example demonstrates the newly implemented features:
//! - BulletStyle: numbered, lettered, roman numerals
//! - Text formatting: strikethrough, highlight, subscript/superscript
//! - Image from base64
//! - Font size presets

use ppt_rs::prelude::*;
use ppt_rs::pptx;
use ppt_rs::generator::{BulletStyle, ImageBuilder};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Create output directory
    std::fs::create_dir_all("examples/output")?;

    // 1. Bullet Styles Demo
    println!("Creating bullet styles demo...");
    
    // Numbered list slide
    let numbered_slide = SlideContent::new("Numbered List")
        .with_bullet_style(BulletStyle::Number)
        .add_bullet("First step")
        .add_bullet("Second step")
        .add_bullet("Third step")
        .add_bullet("Fourth step");
    
    // Lettered list slide
    let lettered_slide = SlideContent::new("Lettered List")
        .add_lettered("Option A")
        .add_lettered("Option B")
        .add_lettered("Option C");
    
    // Roman numerals slide
    let roman_slide = SlideContent::new("Roman Numerals")
        .with_bullet_style(BulletStyle::RomanUpper)
        .add_bullet("Chapter I")
        .add_bullet("Chapter II")
        .add_bullet("Chapter III");
    
    // Mixed bullets with sub-bullets
    let mixed_slide = SlideContent::new("Mixed Bullets")
        .add_bullet("Main point")
        .add_sub_bullet("Supporting detail 1")
        .add_sub_bullet("Supporting detail 2")
        .add_bullet("Another main point")
        .add_sub_bullet("More details");
    
    // Custom bullet slide
    let custom_slide = SlideContent::new("Custom Bullets")
        .add_styled_bullet("Star bullet", BulletStyle::Custom('★'))
        .add_styled_bullet("Arrow bullet", BulletStyle::Custom('→'))
        .add_styled_bullet("Check bullet", BulletStyle::Custom('✓'))
        .add_styled_bullet("Diamond bullet", BulletStyle::Custom('◆'));
    
    let bullet_demo = pptx!("Bullet Styles Demo")
        .title_slide("Bullet Styles - Demonstrating various list formats")
        .content_slide(numbered_slide)
        .content_slide(lettered_slide)
        .content_slide(roman_slide)
        .content_slide(mixed_slide)
        .content_slide(custom_slide)
        .build()?;
    
    std::fs::write("examples/output/bullet_styles.pptx", bullet_demo)?;
    println!("  ✓ Created examples/output/bullet_styles.pptx");

    // 2. Text Formatting Demo
    println!("Creating text formatting demo...");
    
    let text_demo = pptx!("Text Formatting Demo")
        .title_slide("Text Formatting - Strikethrough, highlight, subscript, superscript")
        .slide("Text Styles", &[
            "Normal text for comparison",
            "This demonstrates various formatting options",
            "Use TextFormat for rich text styling",
        ])
        .slide("Font Size Presets", &[
            &format!("TITLE: {} (44pt)", font_sizes::TITLE / 100),
            &format!("SUBTITLE: {} (32pt)", font_sizes::SUBTITLE / 100),
            &format!("HEADING: {} (28pt)", font_sizes::HEADING / 100),
            &format!("BODY: {} (18pt)", font_sizes::BODY / 100),
            &format!("SMALL: {} (14pt)", font_sizes::SMALL / 100),
            &format!("CAPTION: {} (12pt)", font_sizes::CAPTION / 100),
        ])
        .slide("Text Effects", &[
            "Strikethrough: For deleted text",
            "Highlight: For emphasized text",
            "Subscript: H₂O style formatting",
            "Superscript: x² style formatting",
        ])
        .build()?;
    
    std::fs::write("examples/output/text_formatting.pptx", text_demo)?;
    println!("  ✓ Created examples/output/text_formatting.pptx");

    // 3. Image from Base64 Demo
    println!("Creating image demo...");
    
    // 1x1 red PNG pixel in base64
    let red_pixel_base64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==";
    
    // Create image from base64
    let img = ImageBuilder::from_base64(red_pixel_base64, inches(2.0), inches(2.0), "PNG")
        .position(inches(4.0), inches(3.0))
        .build();
    
    let image_slide = SlideContent::new("Image from Base64")
        .add_bullet("Images can be loaded from base64 encoded data")
        .add_bullet("Useful for embedding images without file access")
        .add_bullet("Supports PNG, JPEG, GIF formats")
        .add_image(img);
    
    let image_demo = pptx!("Image Features Demo")
        .title_slide("Image Features - Loading images from various sources")
        .content_slide(image_slide)
        .slide("Image Sources", &[
            "Image::new(filename) - From file path",
            "Image::from_base64(data) - From base64 string",
            "Image::from_bytes(data) - From raw bytes",
            "ImageBuilder for fluent API",
        ])
        .build()?;
    
    std::fs::write("examples/output/image_features.pptx", image_demo)?;
    println!("  ✓ Created examples/output/image_features.pptx");

    // 4. Theme Colors Demo
    println!("Creating themes demo...");
    
    let all_themes = themes::all();
    let theme_info: Vec<String> = all_themes.iter().map(|t| {
        format!("{}: Primary={}, Accent={}", t.name, t.primary, t.accent)
    }).collect();
    
    let themes_demo = pptx!("Theme Colors Demo")
        .title_slide("Theme Colors - Predefined color palettes")
        .slide("Available Themes", &theme_info.iter().map(|s| s.as_str()).collect::<Vec<_>>())
        .slide("Color Constants", &[
            &format!("Material Red: {}", colors::MATERIAL_RED),
            &format!("Material Blue: {}", colors::MATERIAL_BLUE),
            &format!("Material Green: {}", colors::MATERIAL_GREEN),
            &format!("Carbon Blue 60: {}", colors::CARBON_BLUE_60),
            &format!("Carbon Gray 100: {}", colors::CARBON_GRAY_100),
        ])
        .build()?;
    
    std::fs::write("examples/output/themes_demo.pptx", themes_demo)?;
    println!("  ✓ Created examples/output/themes_demo.pptx");

    // 5. Complete Feature Showcase
    println!("Creating complete feature showcase...");
    
    let showcase = pptx!("ppt-rs v0.2.1 Features")
        .title_slide("New Features in ppt-rs v0.2.1")
        .slide("Bullet Formatting", &[
            "BulletStyle::Number - 1, 2, 3...",
            "BulletStyle::LetterLower/Upper - a, b, c / A, B, C",
            "BulletStyle::RomanLower/Upper - i, ii, iii / I, II, III",
            "BulletStyle::Custom(char) - Any custom character",
            "Sub-bullets with indentation",
        ])
        .slide("Text Enhancements", &[
            "TextFormat::strikethrough() - Strike through text",
            "TextFormat::highlight(color) - Background highlight",
            "TextFormat::subscript() - H₂O style",
            "TextFormat::superscript() - x² style",
            "font_sizes module with presets",
        ])
        .slide("Image Loading", &[
            "Image::from_base64() - Base64 encoded images",
            "Image::from_bytes() - Raw byte arrays",
            "ImageSource enum for flexible handling",
            "Built-in base64 decoder",
        ])
        .slide("Templates", &[
            "templates::business_proposal()",
            "templates::status_report()",
            "templates::training_material()",
            "templates::technical_doc()",
            "templates::simple()",
        ])
        .slide("Themes & Colors", &[
            "themes::CORPORATE, MODERN, VIBRANT, DARK",
            "themes::NATURE, TECH, CARBON",
            "colors module with Material Design colors",
            "colors module with IBM Carbon colors",
        ])
        .build()?;
    
    std::fs::write("examples/output/feature_showcase.pptx", showcase)?;
    println!("  ✓ Created examples/output/feature_showcase.pptx");

    println!("\n✅ All demos created successfully!");
    println!("   Check examples/output/ for the generated files.");
    
    Ok(())
}

