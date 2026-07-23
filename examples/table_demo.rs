use ppt_rs::generator::{SlideContent, Table, TableRow, TableCell, create_pptx_with_content};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let slides = vec![
        // Slide 1: Title
        SlideContent::new("Table Examples")
            .add_bullet("Demonstrating table rendering in PPTX"),

        // Slide 2: Simple 2x3 table
        SlideContent::new("Employee Data")
            .table(create_employee_table()),

        // Slide 3: Styled table with colors
        SlideContent::new("Sales Summary")
            .table(create_sales_table()),

        // Slide 4: Data table
        SlideContent::new("Quarterly Results")
            .table(create_quarterly_table()),
    ];

    let pptx_data = create_pptx_with_content("Table Demo", slides)?;
    std::fs::write("table_demo.pptx", pptx_data)?;
    println!("✓ Created table_demo.pptx with 4 slides containing tables");

    Ok(())
}

fn create_employee_table() -> Table {
    let header_cells = vec![
        TableCell::new("Name").bold().background_color("4F81BD"),
        TableCell::new("Department").bold().background_color("4F81BD"),
        TableCell::new("Status").bold().background_color("4F81BD"),
    ];
    let header_row = TableRow::new(header_cells);

    let rows = vec![
        TableRow::new(vec![
            TableCell::new("Alice Johnson"),
            TableCell::new("Engineering"),
            TableCell::new("Active"),
        ]),
        TableRow::new(vec![
            TableCell::new("Bob Smith"),
            TableCell::new("Sales"),
            TableCell::new("Active"),
        ]),
        TableRow::new(vec![
            TableCell::new("Carol Davis"),
            TableCell::new("Marketing"),
            TableCell::new("On Leave"),
        ]),
        TableRow::new(vec![
            TableCell::new("David Wilson"),
            TableCell::new("Engineering"),
            TableCell::new("Active"),
        ]),
        TableRow::new(vec![
            TableCell::new("Emma Brown"),
            TableCell::new("HR"),
            TableCell::new("Active"),
        ]),
    ];

    Table::new(
        [vec![header_row], rows].concat(),
        vec![2000000, 2500000, 1500000],
        500000,
        1500000,
    )
}

fn create_sales_table() -> Table {
    let header_cells = vec![
        TableCell::new("Product").bold().background_color("003366"),
        TableCell::new("Revenue").bold().background_color("003366"),
        TableCell::new("Growth").bold().background_color("003366"),
    ];
    let header_row = TableRow::new(header_cells);

    let rows = vec![
        TableRow::new(vec![
            TableCell::new("Cloud Services"),
            TableCell::new("$450K"),
            TableCell::new("+28%"),
        ]),
        TableRow::new(vec![
            TableCell::new("Enterprise Suite"),
            TableCell::new("$320K"),
            TableCell::new("+15%"),
        ]),
        TableRow::new(vec![
            TableCell::new("Developer Tools"),
            TableCell::new("$280K"),
            TableCell::new("+42%"),
        ]),
        TableRow::new(vec![
            TableCell::new("Mobile App"),
            TableCell::new("$195K"),
            TableCell::new("+35%"),
        ]),
        TableRow::new(vec![
            TableCell::new("Support Services"),
            TableCell::new("$155K"),
            TableCell::new("+12%"),
        ]),
    ];

    Table::new(
        [vec![header_row], rows].concat(),
        vec![2200000, 2000000, 1800000],
        500000,
        1500000,
    )
}

fn create_quarterly_table() -> Table {
    let header_cells = vec![
        TableCell::new("Quarter").bold().background_color("1F497D"),
        TableCell::new("Revenue").bold().background_color("1F497D"),
        TableCell::new("Profit").bold().background_color("1F497D"),
    ];
    let header_row = TableRow::new(header_cells);

    let q1_cells = vec![
        TableCell::new("Q1 2025"),
        TableCell::new("$450K"),
        TableCell::new("$90K"),
    ];
    let q1 = TableRow::new(q1_cells);

    let q2_cells = vec![
        TableCell::new("Q2 2025"),
        TableCell::new("$520K"),
        TableCell::new("$110K"),
    ];
    let q2 = TableRow::new(q2_cells);

    let q3_cells = vec![
        TableCell::new("Q3 2025"),
        TableCell::new("$580K"),
        TableCell::new("$130K"),
    ];
    let q3 = TableRow::new(q3_cells);

    Table::new(
        vec![header_row, q1, q2, q3],
        vec![2000000, 2000000, 2000000],
        500000,
        1500000,
    )
}
