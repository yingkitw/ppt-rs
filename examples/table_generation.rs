//! Example: Table generation and integration
//!
//! Demonstrates the simplified table helpers (`QuickTable`, `table_from_data`)
//! and classic `TableCell` styling for advanced cases.
//! Run with: cargo run --example table_generation

use ppt_rs::generator::{SlideContent, create_pptx_with_content};
use ppt_rs::prelude::{QuickTable, table_from_data, header_cell, cell, highlight_cell};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║        Generating Table Examples                          ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    fs::create_dir_all("examples/output")?;

    println!("1. Creating simple table...");
    create_simple_table_example()?;
    println!("   ✓ Created: examples/output/simple_table.pptx");
    println!();

    println!("2. Creating styled table...");
    create_styled_table_example()?;
    println!("   ✓ Created: examples/output/styled_table.pptx");
    println!();

    println!("3. Creating data table...");
    create_data_table_example()?;
    println!("   ✓ Created: examples/output/data_table.pptx");
    println!();

    println!("4. Creating multiple tables...");
    create_multiple_tables_example()?;
    println!("   ✓ Created: examples/output/multiple_tables.pptx");
    println!();

    println!("✅ All table examples generated successfully!");
    println!();
    println!("Generated files:");
    println!("  - examples/output/simple_table.pptx");
    println!("  - examples/output/styled_table.pptx");
    println!("  - examples/output/data_table.pptx");
    println!("  - examples/output/multiple_tables.pptx");

    Ok(())
}

fn create_simple_table_example() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![
        vec!["Name", "Age"],
        vec!["Alice", "30"],
        vec!["Bob", "25"],
    ];
    let table = table_from_data(&data, Some(vec![2.0, 2.0]))
        .position(500000, 1500000)
        .build();

    let slides = vec![
        SlideContent::new("Simple 2x2 Table")
            .add_bullet("Built with table_from_data()")
            .add_bullet("Headers and data rows"),
        SlideContent::new("Table Data").table(table),
    ];

    let pptx_data = create_pptx_with_content("Simple Table", slides)?;
    fs::write("examples/output/simple_table.pptx", pptx_data)?;
    Ok(())
}

fn create_styled_table_example() -> Result<(), Box<dyn std::error::Error>> {
    let table = QuickTable::new(3)
        .header(&["Name", "Age", "City"])
        .row(&["Alice", "30", "NYC"])
        .row(&["Bob", "28", "LA"])
        .row(&["Carol", "35", "Chicago"])
        .at(0.5, 1.5)
        .build();

    let slides = vec![
        SlideContent::new("Styled Table")
            .title_bold(true)
            .title_color("003366")
            .add_bullet("Built with QuickTable::header() + row()"),
        SlideContent::new("People Data").table(table),
    ];

    let pptx_data = create_pptx_with_content("Styled Table", slides)?;
    fs::write("examples/output/styled_table.pptx", pptx_data)?;
    Ok(())
}

fn create_data_table_example() -> Result<(), Box<dyn std::error::Error>> {
    let table = QuickTable::with_widths(&[2.5, 2.0, 1.5])
        .styled_row(vec![
            header_cell("Product"),
            header_cell("Revenue"),
            header_cell("Growth"),
        ])
        .styled_row(vec![
            cell("Product A"),
            cell("$100K"),
            highlight_cell("+15%", "C6EFCE"),
        ])
        .styled_row(vec![
            cell("Product B"),
            cell("$150K"),
            highlight_cell("+22%", "C6EFCE"),
        ])
        .styled_row(vec![
            cell("Product C"),
            cell("$200K"),
            highlight_cell("+18%", "C6EFCE"),
        ])
        .at(0.5, 1.5)
        .build();

    let slides = vec![
        SlideContent::new("Product Revenue")
            .title_bold(true)
            .title_color("1F497D")
            .add_bullet("QuickTable with header_cell / highlight_cell"),
        SlideContent::new("Revenue Data").table(table),
    ];

    let pptx_data = create_pptx_with_content("Data Table", slides)?;
    fs::write("examples/output/data_table.pptx", pptx_data)?;
    Ok(())
}

fn create_multiple_tables_example() -> Result<(), Box<dyn std::error::Error>> {
    let employees = QuickTable::new(3)
        .header(&["ID", "Name", "Department"])
        .row(&["001", "Alice", "Engineering"])
        .row(&["002", "Bob", "Sales"])
        .row(&["003", "Carol", "Marketing"])
        .at(0.5, 1.5)
        .build();

    let projects = QuickTable::new(3)
        .header(&["Project", "Status", "Owner"])
        .row(&["Project A", "In Progress", "Alice"])
        .row(&["Project B", "Completed", "Bob"])
        .row(&["Project C", "Planning", "Carol"])
        .at(0.5, 1.5)
        .build();

    let slides = vec![
        SlideContent::new("Employee Directory").table(employees),
        SlideContent::new("Project Status").table(projects),
        SlideContent::new("Summary")
            .add_bullet("Total Employees: 3")
            .add_bullet("Active Projects: 3")
            .add_bullet("Completion Rate: 33%"),
    ];

    let pptx_data = create_pptx_with_content("Multiple Tables", slides)?;
    fs::write("examples/output/multiple_tables.pptx", pptx_data)?;
    Ok(())
}
