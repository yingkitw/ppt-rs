//! Color and Table API Demo
//!
//! Demonstrates the new simplified color and table APIs with:
//! - Color aliases (red, blue, green, etc.)
//! - Color adjustments (lighter, darker, opacity)
//! - RGB color creation
//! - Simplified table creation
//! - Quick table builder

use ppt_rs::prelude::*;
use ppt_rs::helpers::{
    material_red, material_pink, material_purple, material_blue, material_green,
    material_orange, material_amber, material_teal, material_brown,
};

fn main() -> Result<()> {
    println!("🎨 Creating presentation with enhanced color and table APIs...");

    let mut slides = Vec::new();

    // =========================================================================
    // SLIDE 1: Color Aliases
    // =========================================================================
    println!("🎨 Slide 1: Color Aliases");
    
    slides.push(
        SlideContent::new("Color Aliases - Popular Colors")
            .add_shape(
                rect(0.5, 1.5, 1.5, 0.8)
                    .fill(red().to_color())
                    .text("Red")
            )
            .add_shape(
                rect(2.5, 1.5, 1.5, 0.8)
                    .fill(green().to_color())
                    .text("Green")
            )
            .add_shape(
                rect(4.5, 1.5, 1.5, 0.8)
                    .fill(blue().to_color())
                    .text("Blue")
            )
            .add_shape(
                rect(6.5, 1.5, 1.5, 0.8)
                    .fill(orange().to_color())
                    .text("Orange")
            )
            .add_shape(
                rect(1.5, 2.8, 1.5, 0.8)
                    .fill(purple().to_color())
                    .text("Purple")
            )
            .add_shape(
                rect(3.5, 2.8, 1.5, 0.8)
                    .fill(pink().to_color())
                    .text("Pink")
            )
            .add_shape(
                rect(5.5, 2.8, 1.5, 0.8)
                    .fill(brown().to_color())
                    .text("Brown")
            )
    );

    // =========================================================================
    // SLIDE 2: Color Adjustments - Lighter/Darker
    // =========================================================================
    println!("🌈 Slide 2: Color Adjustments");
    
    let base_color = material_blue();
    
    slides.push(
        SlideContent::new("Color Adjustments - Lighter & Darker")
            .add_shape(
                rect(1.0, 1.5, 1.5, 0.8)
                    .fill(base_color.darker(0.4).to_color())
                    .text("Darker 40%")
            )
            .add_shape(
                rect(2.8, 1.5, 1.5, 0.8)
                    .fill(base_color.darker(0.2).to_color())
                    .text("Darker 20%")
            )
            .add_shape(
                rect(4.6, 1.5, 1.5, 0.8)
                    .fill(base_color.to_color())
                    .text("Base Color")
            )
            .add_shape(
                rect(2.8, 2.8, 1.5, 0.8)
                    .fill(base_color.lighter(0.2).to_color())
                    .text("Lighter 20%")
            )
            .add_shape(
                rect(4.6, 2.8, 1.5, 0.8)
                    .fill(base_color.lighter(0.4).to_color())
                    .text("Lighter 40%")
            )
    );

    // =========================================================================
    // SLIDE 3: Color Mixing
    // =========================================================================
    println!("🎨 Slide 3: Color Mixing");
    
    let color1 = red();
    let color2 = blue();
    
    slides.push(
        SlideContent::new("Color Mixing - Red + Blue")
            .add_shape(
                rect(1.0, 1.5, 1.5, 1.2)
                    .fill(color1.to_color())
                    .text("Red\n100%")
            )
            .add_shape(
                rect(2.8, 1.5, 1.5, 1.2)
                    .fill(color1.mix(&color2, 0.25).to_color())
                    .text("Mix\n25%")
            )
            .add_shape(
                rect(4.6, 1.5, 1.5, 1.2)
                    .fill(color1.mix(&color2, 0.5).to_color())
                    .text("Mix\n50%")
            )
            .add_shape(
                rect(6.4, 1.5, 1.5, 1.2)
                    .fill(color1.mix(&color2, 0.75).to_color())
                    .text("Mix\n75%")
            )
            .add_shape(
                rect(3.7, 3.2, 1.5, 1.2)
                    .fill(color2.to_color())
                    .text("Blue\n100%")
            )
    );

    // =========================================================================
    // SLIDE 4: Material Design Colors
    // =========================================================================
    println!("🎨 Slide 4: Material Design Colors");
    
    slides.push(
        SlideContent::new("Material Design Color Palette")
            .add_shape(
                rect(0.5, 1.5, 1.3, 0.7)
                    .fill(material_red().to_color())
                    .text("Red")
            )
            .add_shape(
                rect(2.1, 1.5, 1.3, 0.7)
                    .fill(material_pink().to_color())
                    .text("Pink")
            )
            .add_shape(
                rect(3.7, 1.5, 1.3, 0.7)
                    .fill(material_purple().to_color())
                    .text("Purple")
            )
            .add_shape(
                rect(5.3, 1.5, 1.3, 0.7)
                    .fill(material_blue().to_color())
                    .text("Blue")
            )
            .add_shape(
                rect(6.9, 1.5, 1.3, 0.7)
                    .fill(material_green().to_color())
                    .text("Green")
            )
            .add_shape(
                rect(1.3, 2.7, 1.3, 0.7)
                    .fill(material_orange().to_color())
                    .text("Orange")
            )
            .add_shape(
                rect(2.9, 2.7, 1.3, 0.7)
                    .fill(material_amber().to_color())
                    .text("Amber")
            )
            .add_shape(
                rect(4.5, 2.7, 1.3, 0.7)
                    .fill(material_teal().to_color())
                    .text("Teal")
            )
            .add_shape(
                rect(6.1, 2.7, 1.3, 0.7)
                    .fill(material_brown().to_color())
                    .text("Brown")
            )
    );

    // =========================================================================
    // SLIDE 5: Simple Table
    // =========================================================================
    println!("📊 Slide 5: Simple Table");
    
    let simple = simple_table(4, 3)
        .position(inches(1.0), inches(1.5))
        .build();
    
    slides.push(
        SlideContent::new("Simple Table - 4 Rows × 3 Columns")
            .table(simple)
    );

    // =========================================================================
    // SLIDE 6: Table from Data
    // =========================================================================
    println!("📊 Slide 6: Table from Data");
    
    let data = vec![
        vec!["Product", "Q1", "Q2", "Q3", "Q4"],
        vec!["Widget A", "$125K", "$142K", "$158K", "$171K"],
        vec!["Widget B", "$98K", "$105K", "$112K", "$119K"],
        vec!["Widget C", "$67K", "$73K", "$81K", "$89K"],
    ];
    
    let mut data_builder = table_from_data(&data, Some(vec![2.5, 1.5, 1.5, 1.5, 1.5]));
    data_builder = data_builder.position(inches(0.5), inches(1.5));
    let data_table = data_builder.build();
    
    slides.push(
        SlideContent::new("Table from Data - Sales Report")
            .table(data_table)
    );

    // =========================================================================
    // SLIDE 7: Quick Table Builder
    // =========================================================================
    println!("📊 Slide 7: Quick Table Builder");
    
    let quick = QuickTable::new(4)
        .header(&["Name", "Role", "Department", "Status"])
        .row(&["Alice Johnson", "Engineer", "Product", "Active"])
        .row(&["Bob Smith", "Designer", "UX", "Active"])
        .row(&["Carol White", "Manager", "Operations", "On Leave"])
        .at(0.5, 1.5)
        .build();
    
    slides.push(
        SlideContent::new("Quick Table - Employee Directory")
            .table(quick)
    );

    // =========================================================================
    // SLIDE 8: Table with Header
    // =========================================================================
    println!("📊 Slide 8: Table with Header");
    
    let mut header_builder = table_with_header(&["Metric", "Target", "Actual", "Variance"], 3);
    header_builder = header_builder.position(inches(1.0), inches(1.5));
    let header_table = header_builder.build();
    
    slides.push(
        SlideContent::new("Table with Auto-Styled Header")
            .table(header_table)
    );

    // =========================================================================
    // SLIDE 9: Advanced Table with Colors
    // =========================================================================
    println!("📊 Slide 9: Advanced Table with Colors");
    
    let advanced = QuickTable::with_widths(&[2.0, 1.5, 1.5, 2.0])
        .styled_row(vec![
            header_cell("Task"),
            header_cell("Priority"),
            header_cell("Status"),
            header_cell("Owner"),
        ])
        .styled_row(vec![
            cell("API Simplification"),
            highlight_cell("High", &material_red().to_hex()),
            highlight_cell("Done", &material_green().to_hex()),
            cell("Team A"),
        ])
        .styled_row(vec![
            cell("Documentation"),
            highlight_cell("Medium", &material_orange().to_hex()),
            cell("In Progress"),
            cell("Team B"),
        ])
        .styled_row(vec![
            cell("Testing"),
            highlight_cell("Low", &material_blue().lighter(0.3).to_hex()),
            cell("Pending"),
            cell("Team C"),
        ])
        .at(0.8, 1.5)
        .build();
    
    slides.push(
        SlideContent::new("Advanced Table - Task Tracker")
            .table(advanced)
    );

    // =========================================================================
    // SLIDE 10: Color Utilities Summary
    // =========================================================================
    println!("📋 Slide 10: Summary");
    
    slides.push(
        SlideContent::new("API Improvements Summary")
            .add_bullet("Color Aliases: red(), blue(), green(), orange(), etc.")
            .add_bullet("Color Adjustments: .lighter(), .darker(), .opacity()")
            .add_bullet("Color Mixing: .mix(other, ratio)")
            .add_bullet("Color Effects: .grayscale(), .invert()")
            .add_bullet("")
            .add_bullet("Simple Tables: simple_table(rows, cols)")
            .add_bullet("Data Tables: table_from_data(&data, widths)")
            .add_bullet("Quick Builder: QuickTable::new(cols).header().row()")
            .add_bullet("Helper Cells: header_cell(), highlight_cell()")
    );

    // =========================================================================
    // Build Presentation
    // =========================================================================
    
    let pptx_data = create_pptx_with_content("Color & Table API Demo", slides)?;
    std::fs::write("color_and_table_demo.pptx", pptx_data)?;
    
    println!("✅ Created color_and_table_demo.pptx");
    println!("📊 10 slides demonstrating:");
    println!("   - Color aliases and adjustments");
    println!("   - Color mixing and effects");
    println!("   - Material Design colors");
    println!("   - Simple table creation");
    println!("   - Table from data");
    println!("   - Quick table builder");
    println!("   - Advanced styled tables");
    
    Ok(())
}
