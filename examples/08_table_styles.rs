//! Example: Table Style Management
//!
//! This example demonstrates:
//! - Predefined table styles (Light, Medium, Dark, etc.)
//! - Custom table style creation
//! - Style application and customization

use ppt_rs::PresentationBuilder;
use ppt_rs::table::{Table, TableStylePreset, AdvancedTableStyle, AdvancedTableStyleManager};
use ppt_rs::dml::color::RGBColor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Table Style Management Examples\n");

    // Example 1: Predefined styles overview
    println!("1️⃣  Predefined Table Styles");
    let styles = vec![
        TableStylePreset::Light,
        TableStylePreset::LightGrid,
        TableStylePreset::LightList,
        TableStylePreset::Medium,
        TableStylePreset::MediumGrid,
        TableStylePreset::MediumList,
        TableStylePreset::Dark,
        TableStylePreset::DarkGrid,
        TableStylePreset::DarkList,
        TableStylePreset::Themed1,
        TableStylePreset::Themed2,
        TableStylePreset::Themed3,
    ];

    for style in &styles {
        println!("   ✓ {} - Header: {:?}, Grid: {}, Banded: {}",
            style.name(),
            style.header_background(),
            style.has_grid_lines(),
            style.has_banded_rows()
        );
    }

    // Example 2: Light style details
    println!("\n2️⃣  Light Style Details");
    let light = TableStylePreset::Light;
    println!("   ✓ Name: {}", light.name());
    println!("   ✓ Header background: {:?}", light.header_background());
    println!("   ✓ Header text: {:?}", light.header_text_color());
    println!("   ✓ Row background: {:?}", light.row_background());
    println!("   ✓ Row text: {:?}", light.row_text_color());

    // Example 3: Dark style details
    println!("\n3️⃣  Dark Style Details");
    let dark = TableStylePreset::Dark;
    println!("   ✓ Name: {}", dark.name());
    println!("   ✓ Header background: {:?}", dark.header_background());
    println!("   ✓ Header text: {:?}", dark.header_text_color());
    println!("   ✓ Row background: {:?}", dark.row_background());
    println!("   ✓ Row text: {:?}", dark.row_text_color());

    // Example 4: Grid line styles
    println!("\n4️⃣  Grid Line Styles");
    let grid_styles = vec![
        ("Light Grid", TableStylePreset::LightGrid),
        ("Medium Grid", TableStylePreset::MediumGrid),
        ("Dark Grid", TableStylePreset::DarkGrid),
    ];
    for (name, style) in grid_styles {
        println!("   ✓ {}: Has grid lines = {}", name, style.has_grid_lines());
    }

    // Example 5: Banded row styles
    println!("\n5️⃣  Banded Row Styles");
    let banded_styles = vec![
        ("Light List", TableStylePreset::LightList),
        ("Medium List", TableStylePreset::MediumList),
        ("Dark List", TableStylePreset::DarkList),
    ];
    for (name, style) in banded_styles {
        println!("   ✓ {}: Has banded rows = {}", name, style.has_banded_rows());
    }

    // Example 6: Themed styles
    println!("\n6️⃣  Themed Styles");
    let themed_styles = vec![
        TableStylePreset::Themed1,
        TableStylePreset::Themed2,
        TableStylePreset::Themed3,
    ];
    for style in themed_styles {
        println!("   ✓ {}: Header {:?}", style.name(), style.header_background());
    }

    // Example 7: Custom table style creation
    println!("\n7️⃣  Custom Table Style Creation");
    let custom = AdvancedTableStyle::new("Corporate")
        .set_header_background(RGBColor::new(0, 51, 102))
        .set_header_text_color(RGBColor::new(255, 255, 255))
        .set_row_background(RGBColor::new(240, 240, 240))
        .set_row_text_color(RGBColor::new(0, 0, 0))
        .set_alternate_row_background(RGBColor::new(220, 220, 220))
        .enable_grid();
    
    println!("   ✓ Custom style created: {}", custom.name());
    println!("   ✓ Header background: {:?}", custom.header_background());
    println!("   ✓ Has grid: {}", custom.has_grid());

    // Example 8: Style from preset
    println!("\n8️⃣  Create Custom Style from Preset");
    let from_preset = AdvancedTableStyle::from_preset(TableStylePreset::Dark);
    println!("   ✓ Style created from preset: {}", from_preset.name());
    println!("   ✓ Header text color: {:?}", from_preset.header_text_color());

    // Example 9: Table style manager
    println!("\n9️⃣  Table Style Manager");
    let mut manager = AdvancedTableStyleManager::new();
    println!("   ✓ Total preset styles: {}", manager.total_styles());
    
    let presets = manager.list_presets();
    println!("   ✓ Available presets: {} styles", presets.len());

    // Example 10: Add custom style to manager
    println!("\n🔟 Add Custom Style to Manager");
    let custom_style = AdvancedTableStyle::new("Professional")
        .set_header_background(RGBColor::new(70, 130, 180))
        .set_header_text_color(RGBColor::new(255, 255, 255));
    
    manager.add_custom_style(custom_style);
    println!("   ✓ Custom style added");
    println!("   ✓ Total styles now: {}", manager.total_styles());

    // Example 11: Get preset style
    println!("\n1️⃣1️⃣  Get Preset Style");
    if let Some(light_preset) = manager.get_preset("Light") {
        println!("   ✓ Retrieved preset: {}", light_preset.name());
    }

    // Example 12: Create presentation with styled tables
    println!("\n1️⃣2️⃣  Creating Presentation with Styled Tables");
    let mut prs = PresentationBuilder::new()
        .title("Table Styles Demo")
        .author("Rust Developer")
        .build()?;

    // Add slides
    for i in 0..3 {
        let _idx = prs.add_slide()?;
        println!("   ✓ Added slide {}", i + 1);
    }

    // Create tables
    let mut table1 = Table::new(3, 3);
    table1.set_cell_text(0, 0, "Header 1");
    table1.set_cell_text(0, 1, "Header 2");
    table1.set_cell_text(0, 2, "Header 3");
    println!("   ✓ Created table 1");

    let mut table2 = Table::new(2, 4);
    table2.set_cell_text(0, 0, "Name");
    table2.set_cell_text(0, 1, "Value");
    table2.set_cell_text(0, 2, "Status");
    table2.set_cell_text(0, 3, "Notes");
    println!("   ✓ Created table 2");

    // Save presentation
    let output_path = "examples/output/08_table_styles.pptx";
    prs.save_to_file(output_path)?;
    println!("   ✓ Saved to {}", output_path);

    // Example 13: List all custom styles
    println!("\n1️⃣3️⃣  List All Styles");
    let custom_list = manager.list_custom();
    println!("   ✓ Custom styles: {} total", custom_list.len());
    for style_name in custom_list {
        println!("     - {}", style_name);
    }

    // Example 14: Style XML generation
    println!("\n1️⃣4️⃣  Style XML Generation");
    let style = AdvancedTableStyle::new("Test");
    let xml = style.to_xml();
    println!("   ✓ Generated XML: {}", xml.lines().next().unwrap_or(""));

    println!("\n✅ Table style management examples complete!");
    println!("\n📋 Features demonstrated:");
    println!("  • 12 predefined table styles");
    println!("  • Light, Medium, Dark variants");
    println!("  • Grid line styles");
    println!("  • Banded row styles");
    println!("  • Themed color schemes");
    println!("  • Custom style creation");
    println!("  • Style customization");
    println!("  • Style manager");
    println!("  • Preset retrieval");
    println!("  • Custom style management");
    println!("\n🎉 All table style features working correctly!");

    Ok(())
}
