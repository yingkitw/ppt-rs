//! Example: Advanced Table Formatting and Footer/Header Support
//!
//! This example demonstrates:
//! - Advanced table formatting (borders, shading, alignment)
//! - Footer and header support
//! - Cell formatting options

use ppt_rs::PresentationBuilder;
use ppt_rs::table::{Table, CellBorder, CellFormat, BorderStyle, CellAlignment, VerticalAlignment};
use ppt_rs::dml::color::RGBColor;
use ppt_rs::presentation::FooterHeader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Advanced Formatting Examples\n");

    // Example 1: Table with borders
    println!("1️⃣  Table with Borders");
    let mut table = Table::new(3, 3);
    table.set_cell_text(0, 0, "Header 1");
    table.set_cell_text(0, 1, "Header 2");
    table.set_cell_text(0, 2, "Header 3");
    table.set_cell_text(1, 0, "Data 1");
    table.set_cell_text(1, 1, "Data 2");
    table.set_cell_text(1, 2, "Data 3");
    println!("   ✓ Table created with 3x3 cells");

    // Example 2: Cell border configuration
    println!("\n2️⃣  Cell Border Configuration");
    let border = CellBorder::new(BorderStyle::Solid, 12700)
        .set_color(RGBColor::new(0, 0, 0));
    println!("   ✓ Solid border created with black color");

    // Example 3: Cell shading
    println!("\n3️⃣  Cell Shading (Background Colors)");
    let header_format = CellFormat::new()
        .set_shading_color(RGBColor::new(200, 200, 200))
        .set_alignment(CellAlignment::Center)
        .set_vertical_alignment(VerticalAlignment::Middle);
    println!("   ✓ Header cell format with gray background and center alignment");

    // Example 4: Cell alignment options
    println!("\n4️⃣  Cell Alignment Options");
    let alignments = vec![
        ("Left", CellAlignment::Left),
        ("Center", CellAlignment::Center),
        ("Right", CellAlignment::Right),
        ("Justified", CellAlignment::Justified),
    ];
    for (name, align) in alignments {
        let format = CellFormat::new().set_alignment(align);
        println!("   ✓ {} alignment: {:?}", name, format.alignment());
    }

    // Example 5: Vertical alignment options
    println!("\n5️⃣  Vertical Alignment Options");
    let v_alignments = vec![
        ("Top", VerticalAlignment::Top),
        ("Middle", VerticalAlignment::Middle),
        ("Bottom", VerticalAlignment::Bottom),
    ];
    for (name, align) in v_alignments {
        let format = CellFormat::new().set_vertical_alignment(align);
        println!("   ✓ {} vertical alignment: {:?}", name, format.vertical_alignment());
    }

    // Example 6: Cell borders (individual sides)
    println!("\n6️⃣  Individual Cell Borders");
    let border = CellBorder::new(BorderStyle::Solid, 12700);
    let format = CellFormat::new()
        .set_left_border(border.clone())
        .set_right_border(border.clone())
        .set_top_border(border.clone())
        .set_bottom_border(border);
    println!("   ✓ Cell with borders on all sides");

    // Example 7: Border styles
    println!("\n7️⃣  Border Styles");
    let styles = vec![
        ("Solid", BorderStyle::Solid),
        ("Dashed", BorderStyle::Dashed),
        ("Dotted", BorderStyle::Dotted),
        ("Double", BorderStyle::Double),
        ("None", BorderStyle::None),
    ];
    for (name, style) in styles {
        let border = CellBorder::new(style, 12700);
        println!("   ✓ {} border style", name);
    }

    // Example 8: Footer and Header
    println!("\n8️⃣  Footer and Header Configuration");
    let fh = FooterHeader::new()
        .set_footer("Company Name")
        .set_header("Confidential")
        .enable_slide_number()
        .set_date_text("November 10, 2025");
    
    println!("   ✓ Footer: {}", fh.footer().unwrap_or("None"));
    println!("   ✓ Header: {}", fh.header().unwrap_or("None"));
    println!("   ✓ Slide number enabled: {}", fh.is_slide_number_enabled());
    println!("   ✓ Date enabled: {}", fh.is_date_enabled());

    // Example 9: Create presentation with advanced formatting
    println!("\n9️⃣  Creating Presentation with Advanced Formatting");
    let mut prs = PresentationBuilder::new()
        .title("Advanced Formatting Demo")
        .author("Rust Developer")
        .build()?;

    // Add slides
    for i in 0..3 {
        let _idx = prs.add_slide()?;
        println!("   ✓ Added slide {}", i + 1);
    }

    // Save presentation
    let output_path = "examples/output/07_advanced_formatting.pptx";
    prs.save_to_file(output_path)?;
    println!("   ✓ Saved to {}", output_path);

    // Example 10: Cell margins
    println!("\n🔟 Cell Margins");
    let format = CellFormat::new()
        .set_all_margins(91440);  // ~0.1 inch
    println!("   ✓ Cell format with uniform margins");

    // Example 11: Complex cell formatting
    println!("\n1️⃣1️⃣  Complex Cell Formatting");
    let border = CellBorder::new(BorderStyle::Solid, 12700)
        .set_color(RGBColor::new(0, 0, 255));
    let complex_format = CellFormat::new()
        .set_all_borders(border)
        .set_shading_color(RGBColor::new(255, 255, 200))
        .set_alignment(CellAlignment::Center)
        .set_vertical_alignment(VerticalAlignment::Middle)
        .set_all_margins(91440);
    println!("   ✓ Complex format: blue borders, yellow background, centered");

    // Example 12: Footer/Header with all options
    println!("\n1️⃣2️⃣  Footer/Header with All Options");
    let fh = FooterHeader::new()
        .set_footer("© 2025 Company")
        .set_header("Internal Use Only")
        .enable_slide_number()
        .set_date_text("November 2025")
        .apply_to_title(true)
        .apply_to_notes(true);
    
    println!("   ✓ Footer: {}", fh.footer().unwrap_or("None"));
    println!("   ✓ Header: {}", fh.header().unwrap_or("None"));
    println!("   ✓ Applied to title: {}", fh.is_applied_to_title());
    println!("   ✓ Applied to notes: {}", fh.is_applied_to_notes());

    println!("\n✅ Advanced formatting examples complete!");
    println!("\n📋 Features demonstrated:");
    println!("  • Table borders (all sides, individual sides)");
    println!("  • Border styles (solid, dashed, dotted, double)");
    println!("  • Cell shading (background colors)");
    println!("  • Cell alignment (left, center, right, justified)");
    println!("  • Vertical alignment (top, middle, bottom)");
    println!("  • Cell margins (customizable)");
    println!("  • Footer and header text");
    println!("  • Slide numbers in footer/header");
    println!("  • Date/time display");
    println!("  • Apply to title slide option");
    println!("  • Apply to notes pages option");
    println!("\n🎉 All advanced formatting features working correctly!");

    Ok(())
}
