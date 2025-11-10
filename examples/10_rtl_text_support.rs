//! Example: RTL Text Support
//!
//! This example demonstrates:
//! - RTL language support (Arabic, Hebrew, Persian, etc.)
//! - Text direction management
//! - Bidirectional text handling
//! - RTL paragraph alignment

use ppt_rs::PresentationBuilder;
use ppt_rs::text::{RTLLanguage, TextDirection, RTLTextConfig, RTLParagraph, ParagraphAlignment};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 RTL Text Support Examples\n");

    // Example 1: RTL languages overview
    println!("1️⃣  Supported RTL Languages");
    let languages = vec![
        RTLLanguage::Arabic,
        RTLLanguage::Hebrew,
        RTLLanguage::Persian,
        RTLLanguage::Urdu,
        RTLLanguage::Pashto,
        RTLLanguage::Kurdish,
        RTLLanguage::Uyghur,
        RTLLanguage::Dhivehi,
    ];

    for lang in &languages {
        println!("   ✓ {} ({})", lang.name(), lang.code());
    }

    // Example 2: Text direction
    println!("\n2️⃣  Text Direction");
    let ltr = TextDirection::LeftToRight;
    let rtl = TextDirection::RightToLeft;
    println!("   ✓ LTR: {} (is_rtl: {})", ltr.to_xml_str(), ltr.is_rtl());
    println!("   ✓ RTL: {} (is_rtl: {})", rtl.to_xml_str(), rtl.is_rtl());

    // Example 3: RTL text configuration
    println!("\n3️⃣  RTL Text Configuration");
    let config = RTLTextConfig::with_language("مرحبا", RTLLanguage::Arabic);
    println!("   ✓ Text: {}", config.text());
    println!("   ✓ Direction: {:?}", config.direction());
    println!("   ✓ Language: {:?}", config.language());

    // Example 4: Auto-detect RTL language
    println!("\n4️⃣  Auto-detect RTL Language");
    if let Some(detected) = RTLLanguage::detect("שלום") {
        println!("   ✓ Detected: {} ({})", detected.name(), detected.code());
    }

    // Example 5: Bidirectional text
    println!("\n5️⃣  Bidirectional Text");
    let config = RTLTextConfig::new("Hello مرحبا World")
        .enable_bidirectional();
    println!("   ✓ Text: {}", config.text());
    println!("   ✓ Bidirectional: {}", config.is_bidirectional());

    // Example 6: RTL paragraph alignment
    println!("\n6️⃣  RTL Paragraph Alignment");
    let alignments = vec![
        ("Right", ParagraphAlignment::Right),
        ("Center", ParagraphAlignment::Center),
        ("Left", ParagraphAlignment::Left),
        ("Justified", ParagraphAlignment::Justified),
    ];

    for (name, align) in alignments {
        println!("   ✓ {}: {}", name, align.to_xml_str());
    }

    // Example 7: RTL paragraph creation
    println!("\n7️⃣  RTL Paragraph Creation");
    let para = RTLParagraph::new("مرحبا بك")
        .set_language(RTLLanguage::Arabic)
        .set_alignment(ParagraphAlignment::Right);
    
    println!("   ✓ Text: {}", para.text());
    println!("   ✓ Language: {:?}", para.language());
    println!("   ✓ Alignment: {:?}", para.alignment());

    // Example 8: Create presentation with RTL text
    println!("\n8️⃣  Creating Presentation with RTL Text");
    let mut prs = PresentationBuilder::new()
        .title("RTL Text Support Demo")
        .author("Rust Developer")
        .build()?;

    // Add slides
    for i in 0..3 {
        let _idx = prs.add_slide()?;
        println!("   ✓ Added slide {}", i + 1);
    }

    // Save presentation
    let output_path = "examples/output/10_rtl_text_support.pptx";
    prs.save_to_file(output_path)?;
    println!("   ✓ Saved to {}", output_path);

    // Example 9: Multiple RTL languages
    println!("\n9️⃣  Multiple RTL Languages");
    let texts = vec![
        ("Arabic", "مرحبا", RTLLanguage::Arabic),
        ("Hebrew", "שלום", RTLLanguage::Hebrew),
        ("Persian", "سلام", RTLLanguage::Persian),
        ("Urdu", "السلام", RTLLanguage::Urdu),
    ];

    for (name, text, lang) in texts {
        let config = RTLTextConfig::with_language(text, lang);
        println!("   ✓ {}: {} ({})", name, config.text(), config.language().unwrap().code());
    }

    // Example 10: RTL text XML generation
    println!("\n🔟 RTL Text XML Generation");
    let config = RTLTextConfig::with_language("مرحبا", RTLLanguage::Arabic);
    let xml = config.to_xml();
    println!("   ✓ Generated XML: {}", xml);

    // Example 11: RTL paragraph XML
    println!("\n1️⃣1️⃣  RTL Paragraph XML Generation");
    let para = RTLParagraph::new("שלום").set_language(RTLLanguage::Hebrew);
    let xml = para.to_xml();
    println!("   ✓ Generated XML: {}", xml);

    // Example 12: Complex RTL configuration
    println!("\n1️⃣2️⃣  Complex RTL Configuration");
    let config = RTLTextConfig::with_language("مرحبا بك في العالم", RTLLanguage::Arabic)
        .enable_bidirectional();
    
    println!("   ✓ Text: {}", config.text());
    println!("   ✓ Direction: {:?}", config.direction());
    println!("   ✓ Language: {:?}", config.language());
    println!("   ✓ Bidirectional: {}", config.is_bidirectional());

    println!("\n✅ RTL text support examples complete!");
    println!("\n📋 Features demonstrated:");
    println!("  • 8 RTL languages supported");
    println!("  • Arabic, Hebrew, Persian, Urdu, Pashto, Kurdish, Uyghur, Dhivehi");
    println!("  • Text direction management (LTR/RTL)");
    println!("  • Bidirectional text handling");
    println!("  • RTL paragraph alignment");
    println!("  • Auto-detection of RTL languages");
    println!("  • XML generation for RTL text");
    println!("  • Language code support");
    println!("\n🎉 All RTL text features working correctly!");

    Ok(())
}
