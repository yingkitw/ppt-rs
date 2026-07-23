use ppt_rs::generator::{SlideContent, Table, TableRow, TableCell, create_pptx_with_content};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let slides = vec![
        // Slide 1: Title
        SlideContent::new("Table Text Formatting Examples")
            .add_bullet("Demonstrating rich text content in table cells"),

        // Slide 2: Text formatting examples
        SlideContent::new("Text Formatting in Tables")
            .table(create_text_formatting_table()),

        // Slide 3: Color examples
        SlideContent::new("Text and Background Colors")
            .table(create_color_table()),

        // Slide 4: Font examples
        SlideContent::new("Font Size and Family")
            .table(create_font_table()),

        // Slide 5: Combined formatting
        SlideContent::new("Combined Formatting")
            .table(create_combined_table()),
    ];

    let pptx_data = create_pptx_with_content("Table Text Formatting", slides)?;
    std::fs::write("examples/output/table_text_formatting.pptx", pptx_data)?;
    println!("✓ Created table_text_formatting.pptx with rich text formatting examples");

    Ok(())
}

fn create_text_formatting_table() -> Table {
    let header_cells = vec![
        TableCell::new("Style").bold().background_color("4F81BD"),
        TableCell::new("Example").bold().background_color("4F81BD"),
        TableCell::new("Description").bold().background_color("4F81BD"),
    ];
    let header_row = TableRow::new(header_cells);

    let rows = vec![
        TableRow::new(vec![
            TableCell::new("Bold"),
            TableCell::new("Bold Text").bold(),
            TableCell::new("Text with bold formatting"),
        ]),
        TableRow::new(vec![
            TableCell::new("Italic"),
            TableCell::new("Italic Text").italic(),
            TableCell::new("Text with italic formatting"),
        ]),
        TableRow::new(vec![
            TableCell::new("Underline"),
            TableCell::new("Underlined Text").underline(),
            TableCell::new("Text with underline formatting"),
        ]),
        TableRow::new(vec![
            TableCell::new("Bold + Italic"),
            TableCell::new("Bold Italic Text").bold().italic(),
            TableCell::new("Text with both bold and italic"),
        ]),
        TableRow::new(vec![
            TableCell::new("All Three"),
            TableCell::new("Bold Italic Underlined").bold().italic().underline(),
            TableCell::new("Text with all formatting options"),
        ]),
    ];

    Table::new(
        [vec![header_row], rows].concat(),
        vec![2000000, 3000000, 3000000],
        500000,
        1500000,
    )
}

fn create_color_table() -> Table {
    let header_cells = vec![
        TableCell::new("Text Color").bold().background_color("1F497D"),
        TableCell::new("Background").bold().background_color("1F497D"),
        TableCell::new("Example").bold().background_color("1F497D"),
    ];
    let header_row = TableRow::new(header_cells);

    let rows = vec![
        TableRow::new(vec![
            TableCell::new("Red"),
            TableCell::new("White"),
            TableCell::new("Red Text").text_color("FF0000").background_color("FFFFFF"),
        ]),
        TableRow::new(vec![
            TableCell::new("Blue"),
            TableCell::new("Yellow"),
            TableCell::new("Blue Text").text_color("0000FF").background_color("FFFF00"),
        ]),
        TableRow::new(vec![
            TableCell::new("Green"),
            TableCell::new("Light Gray"),
            TableCell::new("Green Text").text_color("00FF00").background_color("E0E0E0"),
        ]),
        TableRow::new(vec![
            TableCell::new("Purple"),
            TableCell::new("White"),
            TableCell::new("Purple Text").text_color("800080").background_color("FFFFFF"),
        ]),
    ];

    Table::new(
        [vec![header_row], rows].concat(),
        vec![2000000, 2000000, 4000000],
        500000,
        1500000,
    )
}

fn create_font_table() -> Table {
    let header_cells = vec![
        TableCell::new("Font Size").bold().background_color("366092"),
        TableCell::new("Font Family").bold().background_color("366092"),
        TableCell::new("Example").bold().background_color("366092"),
    ];
    let header_row = TableRow::new(header_cells);

    let rows = vec![
        TableRow::new(vec![
            TableCell::new("12pt"),
            TableCell::new("Arial"),
            TableCell::new("Small Arial Text").font_size(12).font_family("Arial"),
        ]),
        TableRow::new(vec![
            TableCell::new("18pt"),
            TableCell::new("Calibri"),
            TableCell::new("Medium Calibri Text").font_size(18).font_family("Calibri"),
        ]),
        TableRow::new(vec![
            TableCell::new("24pt"),
            TableCell::new("Times New Roman"),
            TableCell::new("Large Times Text").font_size(24).font_family("Times New Roman"),
        ]),
        TableRow::new(vec![
            TableCell::new("32pt"),
            TableCell::new("Arial"),
            TableCell::new("Extra Large Arial").font_size(32).font_family("Arial"),
        ]),
    ];

    Table::new(
        [vec![header_row], rows].concat(),
        vec![2000000, 2500000, 3500000],
        500000,
        1500000,
    )
}

fn create_combined_table() -> Table {
    let header_cells = vec![
        TableCell::new("Feature").bold().background_color("C0504D"),
        TableCell::new("Styled Example").bold().background_color("C0504D"),
    ];
    let header_row = TableRow::new(header_cells);

    let rows = vec![
        TableRow::new(vec![
            TableCell::new("Important Header"),
            TableCell::new("Critical Data")
                .bold()
                .text_color("FFFFFF")
                .background_color("C0504D")
                .font_size(20),
        ]),
        TableRow::new(vec![
            TableCell::new("Emphasis"),
            TableCell::new("Highlighted Text")
                .bold()
                .italic()
                .text_color("0000FF")
                .font_size(18)
                .font_family("Calibri"),
        ]),
        TableRow::new(vec![
            TableCell::new("Warning"),
            TableCell::new("Warning Message")
                .bold()
                .underline()
                .text_color("FF6600")
                .background_color("FFF4E6")
                .font_size(16),
        ]),
        TableRow::new(vec![
            TableCell::new("Success"),
            TableCell::new("Success Indicator")
                .bold()
                .text_color("00AA00")
                .background_color("E6F7E6")
                .font_size(18)
                .font_family("Arial"),
        ]),
    ];

    Table::new(
        [vec![header_row], rows].concat(),
        vec![3000000, 5000000],
        500000,
        1500000,
    )
}

