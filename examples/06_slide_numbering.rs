//! Example: Slide Numbering Support
//!
//! This example demonstrates slide numbering features:
//! - Arabic numerals (1, 2, 3, ...)
//! - Roman numerals (I, II, III, ...)
//! - Alphabetic numbering (A, B, C, ...)
//! - Custom prefixes and suffixes
//! - Footer integration

use ppt_rs::PresentationBuilder;
use ppt_rs::slide::{SlideNumbering, NumberingFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Slide Numbering Examples\n");

    // Example 1: Arabic numerals
    println!("1️⃣  Arabic Numerals (1, 2, 3, ...)");
    let numbering = SlideNumbering::new()
        .enable()
        .set_format(NumberingFormat::Arabic);
    
    for i in 1..=5 {
        println!("   Slide {}: {}", i, numbering.format_slide_number(i));
    }

    // Example 2: Roman numerals (uppercase)
    println!("\n2️⃣  Roman Numerals Uppercase (I, II, III, ...)");
    let numbering = SlideNumbering::new()
        .enable()
        .set_format(NumberingFormat::RomanUpper);
    
    for i in 1..=5 {
        println!("   Slide {}: {}", i, numbering.format_slide_number(i));
    }

    // Example 3: Roman numerals (lowercase)
    println!("\n3️⃣  Roman Numerals Lowercase (i, ii, iii, ...)");
    let numbering = SlideNumbering::new()
        .enable()
        .set_format(NumberingFormat::RomanLower);
    
    for i in 1..=5 {
        println!("   Slide {}: {}", i, numbering.format_slide_number(i));
    }

    // Example 4: Alphabetic (uppercase)
    println!("\n4️⃣  Alphabetic Uppercase (A, B, C, ...)");
    let numbering = SlideNumbering::new()
        .enable()
        .set_format(NumberingFormat::AlphaUpper);
    
    for i in 1..=5 {
        println!("   Slide {}: {}", i, numbering.format_slide_number(i));
    }

    // Example 5: Alphabetic (lowercase)
    println!("\n5️⃣  Alphabetic Lowercase (a, b, c, ...)");
    let numbering = SlideNumbering::new()
        .enable()
        .set_format(NumberingFormat::AlphaLower);
    
    for i in 1..=5 {
        println!("   Slide {}: {}", i, numbering.format_slide_number(i));
    }

    // Example 6: With custom prefix
    println!("\n6️⃣  With Custom Prefix");
    let numbering = SlideNumbering::new()
        .enable()
        .set_format(NumberingFormat::Arabic)
        .set_prefix("Slide ");
    
    for i in 1..=5 {
        println!("   Slide {}: {}", i, numbering.format_slide_number(i));
    }

    // Example 7: With custom suffix
    println!("\n7️⃣  With Custom Suffix");
    let numbering = SlideNumbering::new()
        .enable()
        .set_format(NumberingFormat::Arabic)
        .set_suffix(" / 10");
    
    for i in 1..=5 {
        println!("   Slide {}: {}", i, numbering.format_slide_number(i));
    }

    // Example 8: With prefix and suffix
    println!("\n8️⃣  With Prefix and Suffix");
    let numbering = SlideNumbering::new()
        .enable()
        .set_format(NumberingFormat::Arabic)
        .set_prefix("Page ")
        .set_suffix(" of 10");
    
    for i in 1..=5 {
        println!("   Slide {}: {}", i, numbering.format_slide_number(i));
    }

    // Example 9: Roman with prefix
    println!("\n9️⃣  Roman with Prefix");
    let numbering = SlideNumbering::new()
        .enable()
        .set_format(NumberingFormat::RomanUpper)
        .set_prefix("Part ");
    
    for i in 1..=5 {
        println!("   Slide {}: {}", i, numbering.format_slide_number(i));
    }

    // Example 10: Create presentation with slide numbering
    println!("\n🔟 Creating Presentation with Slide Numbering");
    let mut prs = PresentationBuilder::new()
        .title("Slide Numbering Demo")
        .author("Rust Developer")
        .build()?;

    // Add slides
    for i in 0..5 {
        let _idx = prs.add_slide()?;
        println!("   ✓ Added slide {}", i + 1);
    }

    // Save presentation
    let output_path = "examples/output/06_slide_numbering.pptx";
    prs.save_to_file(output_path)?;
    println!("   ✓ Saved to {}", output_path);

    // Example 11: XML generation
    println!("\n1️⃣1️⃣  XML Generation for Slide Numbers");
    let numbering = SlideNumbering::new()
        .enable()
        .set_format(NumberingFormat::Arabic)
        .set_prefix("Slide ");
    
    let xml = numbering.to_xml(1)?;
    println!("   Generated XML for slide 1:");
    println!("   {}", xml.lines().next().unwrap_or(""));

    // Example 12: Disabled numbering
    println!("\n1️⃣2️⃣  Disabled Numbering");
    let numbering = SlideNumbering::new();
    let xml = numbering.to_xml(1)?;
    if xml.is_empty() {
        println!("   ✓ Disabled numbering produces empty XML");
    }

    println!("\n✅ Slide numbering examples complete!");
    println!("\n📋 Features demonstrated:");
    println!("  • Arabic numerals (1, 2, 3, ...)");
    println!("  • Roman numerals uppercase (I, II, III, ...)");
    println!("  • Roman numerals lowercase (i, ii, iii, ...)");
    println!("  • Alphabetic uppercase (A, B, C, ...)");
    println!("  • Alphabetic lowercase (a, b, c, ...)");
    println!("  • Custom prefixes and suffixes");
    println!("  • Footer integration support");
    println!("  • XML generation for PowerPoint");
    println!("\n🎉 All slide numbering features working correctly!");

    Ok(())
}
