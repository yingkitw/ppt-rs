//! Example demonstrating shape creation in PPTX
//!
//! Shows various shape types, fills, lines, and text in shapes.
//! NEW: Demonstrates the flexible Dimension API for positioning and sizing.

use ppt_rs::generator::{
    Shape, ShapeType,
    generate_shape_xml, generate_shapes_xml, generate_connector_xml,
    inches_to_emu, cm_to_emu,
};
use ppt_rs::core::Dimension;
use ppt_rs::prelude::{hex, ShapeExt};

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║         PPTX Shapes Demo                                   ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // =========================================================================
    // Basic Shapes
    // =========================================================================
    println!("📐 Basic Shapes:");
    
    let basic_shapes = [
        ShapeType::Rectangle,
        ShapeType::RoundedRectangle,
        ShapeType::Ellipse,
        ShapeType::Triangle,
        ShapeType::Diamond,
        ShapeType::Pentagon,
        ShapeType::Hexagon,
        ShapeType::Octagon,
    ];
    
    for shape_type in &basic_shapes {
        println!("   {} → {}", shape_type.display_name(), shape_type.preset_name());
    }

    // =========================================================================
    // Arrow Shapes
    // =========================================================================
    println!("\n➡️  Arrow Shapes:");
    
    let arrow_shapes = [
        ShapeType::RightArrow,
        ShapeType::LeftArrow,
        ShapeType::UpArrow,
        ShapeType::DownArrow,
        ShapeType::LeftRightArrow,
        ShapeType::UpDownArrow,
    ];
    
    for shape_type in &arrow_shapes {
        println!("   {} → {}", shape_type.display_name(), shape_type.preset_name());
    }

    // =========================================================================
    // Star and Banner Shapes
    // =========================================================================
    println!("\n⭐ Stars and Banners:");
    
    let star_shapes = [
        ShapeType::Star4,
        ShapeType::Star5,
        ShapeType::Star6,
        ShapeType::Star8,
        ShapeType::Ribbon,
        ShapeType::Wave,
    ];
    
    for shape_type in &star_shapes {
        println!("   {} → {}", shape_type.display_name(), shape_type.preset_name());
    }

    // =========================================================================
    // Callout Shapes
    // =========================================================================
    println!("\n💬 Callout Shapes:");
    
    let callout_shapes = [
        ShapeType::WedgeRectCallout,
        ShapeType::WedgeEllipseCallout,
        ShapeType::CloudCallout,
    ];
    
    for shape_type in &callout_shapes {
        println!("   {} → {}", shape_type.display_name(), shape_type.preset_name());
    }

    // =========================================================================
    // Flow Chart Shapes
    // =========================================================================
    println!("\n📊 Flow Chart Shapes:");
    
    let flowchart_shapes = [
        ShapeType::FlowChartProcess,
        ShapeType::FlowChartDecision,
        ShapeType::FlowChartTerminator,
        ShapeType::FlowChartDocument,
    ];
    
    for shape_type in &flowchart_shapes {
        println!("   {} → {}", shape_type.display_name(), shape_type.preset_name());
    }

    // =========================================================================
    // Other Shapes
    // =========================================================================
    println!("\n🎨 Other Shapes:");
    
    let other_shapes = [
        ShapeType::Heart,
        ShapeType::Lightning,
        ShapeType::Sun,
        ShapeType::Moon,
        ShapeType::Cloud,
    ];
    
    for shape_type in &other_shapes {
        println!("   {} → {}", shape_type.display_name(), shape_type.preset_name());
    }

    // =========================================================================
    // Shape with Fill
    // =========================================================================
    println!("\n🎨 Shape with Fill:");
    
    let filled_shape = Shape::new(
        ShapeType::Rectangle,
        inches_to_emu(1.0),
        inches_to_emu(1.0),
        inches_to_emu(3.0),
        inches_to_emu(2.0),
    ).fill(hex("4472C4")); // Blue fill
    
    let xml = generate_shape_xml(&filled_shape, 1);
    println!("   Generated XML ({} chars)", xml.len());
    println!("   Contains fill: {}", xml.contains("solidFill"));

    // =========================================================================
    // Shape with Line
    // =========================================================================
    println!("\n📏 Shape with Line:");
    
    let outlined_shape = Shape::new(
        ShapeType::Ellipse,
        inches_to_emu(1.0),
        inches_to_emu(1.0),
        inches_to_emu(2.0),
        inches_to_emu(2.0),
    ).stroke(hex("FF0000"), 2.0); // Red outline, 2pt
    
    let xml = generate_shape_xml(&outlined_shape, 2);
    println!("   Generated XML ({} chars)", xml.len());
    println!("   Contains line: {}", xml.contains("a:ln"));

    // =========================================================================
    // Shape with Text
    // =========================================================================
    println!("\n📝 Shape with Text:");
    
    let text_shape = Shape::new(
        ShapeType::RoundedRectangle,
        cm_to_emu(5.0),
        cm_to_emu(3.0),
        cm_to_emu(8.0),
        cm_to_emu(4.0),
    )
    .fill(hex("70AD47")) // Green fill
    .text("Click Here!");
    
    let xml = generate_shape_xml(&text_shape, 3);
    println!("   Generated XML ({} chars)", xml.len());
    println!("   Contains text: {}", xml.contains("Click Here!"));

    // =========================================================================
    // Multiple Shapes
    // =========================================================================
    println!("\n📦 Multiple Shapes:");
    
    let shapes = vec![
        Shape::new(ShapeType::Rectangle, 0, 0, 1000000, 500000)
            .fill(hex("FF0000")),
        Shape::new(ShapeType::Ellipse, 1200000, 0, 500000, 500000)
            .fill(hex("00FF00")),
        Shape::new(ShapeType::Triangle, 1900000, 0, 500000, 500000)
            .fill(hex("0000FF")),
    ];
    
    let xml = generate_shapes_xml(&shapes, 10);
    println!("   Generated {} shapes", shapes.len());
    println!("   Total XML: {} chars", xml.len());

    // =========================================================================
    // Connector (Arrow Line)
    // =========================================================================
    println!("\n🔗 Connector:");
    
    let connector_xml = generate_connector_xml(
        0, 0,
        inches_to_emu(3.0), inches_to_emu(2.0),
        100,
        "000000",
        12700, // 1pt line
    );
    println!("   Generated connector XML ({} chars)", connector_xml.len());
    println!("   Has arrow head: {}", connector_xml.contains("triangle"));

    // =========================================================================
    // Flexible Dimension API (NEW)
    // =========================================================================
    println!("\n📏 Flexible Dimension API (NEW):");

    // 1. Shape using ratio-based positioning (% of slide)
    let ratio_shape = Shape::from_dimensions(
        ShapeType::Rectangle,
        Dimension::Ratio(0.1), Dimension::Ratio(0.2),   // 10% from left, 20% from top
        Dimension::Ratio(0.8), Dimension::Ratio(0.6),   // 80% wide, 60% tall
    ).fill(hex("4472C4")).text("Ratio-based");

    let xml = generate_shape_xml(&ratio_shape, 20);
    println!("   Ratio-based shape: {}x{} EMU at ({}, {})",
        ratio_shape.width, ratio_shape.height, ratio_shape.x, ratio_shape.y);
    println!("   Generated XML ({} chars)", xml.len());

    // 2. Mixed units: inches for position, ratio for size
    let mixed_shape = Shape::from_dimensions(
        ShapeType::RoundedRectangle,
        Dimension::Inches(1.0), Dimension::Cm(3.0),     // 1 inch from left, 3cm from top
        Dimension::Ratio(0.5), Dimension::Inches(1.5),  // 50% slide width, 1.5 inches tall
    ).fill(hex("70AD47")).text("Mixed units");

    println!("   Mixed-unit shape: {}x{} EMU at ({}, {})",
        mixed_shape.width, mixed_shape.height, mixed_shape.x, mixed_shape.y);

    // 3. Fluent .at() and .with_dimensions() chaining
    let fluent_shape = Shape::new(ShapeType::Ellipse, 0, 0, 0, 0)
        .at(Dimension::percent(50.0), Dimension::percent(50.0))  // center of slide
        .with_dimensions(Dimension::Inches(2.0), Dimension::Inches(2.0))
        .fill(hex("C0504D"))
        .text("Centered");

    println!("   Fluent chained shape: {}x{} EMU at ({}, {})",
        fluent_shape.width, fluent_shape.height, fluent_shape.x, fluent_shape.y);

    // 4. Percent helper (syntactic sugar for Ratio)
    let percent_shape = Shape::from_dimensions(
        ShapeType::Diamond,
        Dimension::percent(40.0), Dimension::percent(30.0),
        Dimension::percent(20.0), Dimension::percent(40.0),
    ).fill(hex("8064A2"));

    println!("   Percent-based shape: {}x{} EMU at ({}, {})",
        percent_shape.width, percent_shape.height, percent_shape.x, percent_shape.y);

    // 5. Points (useful for font-relative sizing)
    let pt_shape = Shape::from_dimensions(
        ShapeType::Rectangle,
        Dimension::Pt(72.0), Dimension::Pt(72.0),   // 1 inch = 72pt
        Dimension::Pt(360.0), Dimension::Pt(144.0), // 5 inches x 2 inches
    ).fill(hex("F79646")).text("Points");

    println!("   Point-based shape: {}x{} EMU at ({}, {})",
        pt_shape.width, pt_shape.height, pt_shape.x, pt_shape.y);

    // 6. All Dimension types side by side (same 1-inch result)
    println!("\n   Unit equivalence (all = 1 inch = 914400 EMU):");
    let units = [
        ("Emu(914400)", Dimension::Emu(914400)),
        ("Inches(1.0)", Dimension::Inches(1.0)),
        ("Cm(2.54)",    Dimension::Cm(2.54)),
        ("Pt(72.0)",    Dimension::Pt(72.0)),
        ("Ratio(0.1)",  Dimension::Ratio(0.1)),  // 10% of slide width (10 inches)
    ];
    for (label, dim) in &units {
        println!("     {:<16} → {} EMU", label, dim.to_emu_x());
    }

    // =========================================================================
    // Summary
    // =========================================================================
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                    Demo Complete                           ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  Shape Types Available: 40+                                ║");
    println!("║  Features:                                                 ║");
    println!("║  ✓ Basic shapes (rect, ellipse, triangle, etc.)            ║");
    println!("║  ✓ Arrow shapes (8 directions)                             ║");
    println!("║  ✓ Stars and banners                                       ║");
    println!("║  ✓ Callouts                                                ║");
    println!("║  ✓ Flow chart shapes                                       ║");
    println!("║  ✓ Fill colors with transparency                           ║");
    println!("║  ✓ Line/border styling                                     ║");
    println!("║  ✓ Text inside shapes                                      ║");
    println!("║  ✓ Connectors with arrow heads                             ║");
    println!("║  ✓ NEW: Flexible Dimension API (EMU/inches/cm/pt/ratio)    ║");
    println!("║  ✓ NEW: Fluent .at() and .with_dimensions() chaining       ║");
    println!("║  ✓ NEW: Percent-based positioning                          ║");
    println!("╚════════════════════════════════════════════════════════════╝");
}
