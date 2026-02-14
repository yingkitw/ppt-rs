//! Tests for table XML generation
//!
//! Tests the table XML generation module

use ppt_rs::generator::{Table, TableRow, TableCell, TableBuilder};
use ppt_rs::generator::table::generate_table_xml;

// ============================================================================
// TABLE XML GENERATION TESTS
// ============================================================================

#[test]
fn test_generate_simple_table_xml() {
    let table = Table::from_data(
        vec![vec!["A", "B"], vec!["1", "2"]],
        vec![1000000, 1000000],
        0,
        0,
    );

    let xml = generate_table_xml(&table, 1);
    
    assert!(xml.contains("a:tbl"));
    assert!(xml.contains("a:tr"));
    assert!(xml.contains("a:tc"));
    assert!(xml.contains("a:t>A</a:t>"));
    assert!(xml.contains("a:t>B</a:t>"));
}

#[test]
fn test_generate_table_with_position() {
    let table = Table::from_data(
        vec![vec!["A", "B"]],
        vec![1000000, 1000000],
        500000,
        1000000,
    );

    let xml = generate_table_xml(&table, 1);
    
    assert!(xml.contains("x=\"500000\""));
    assert!(xml.contains("y=\"1000000\""));
}

#[test]
fn test_generate_table_with_dimensions() {
    let table = Table::from_data(
        vec![vec!["A", "B", "C"]],
        vec![1000000, 1000000, 1000000],
        0,
        0,
    );

    let xml = generate_table_xml(&table, 1);
    
    // Width should be sum of column widths: 3000000
    assert!(xml.contains("cx=\"3000000\""));
}

#[test]
fn test_generate_table_with_bold_cells() {
    let cells = vec![
        TableCell::new("Bold").bold(),
        TableCell::new("Regular"),
    ];
    let row = TableRow::new(cells);
    let table = Table::new(vec![row], vec![1000000, 1000000], 0, 0);

    let xml = generate_table_xml(&table, 1);
    
    // Bold cell should have b="1"
    assert!(xml.contains(r#"b="1""#));
    // Regular cell should not have b attribute (simplified format)
    // Just verify the text is there
    assert!(xml.contains("<a:t>Regular</a:t>"));
}

#[test]
fn test_generate_table_with_colors() {
    let cells = vec![
        TableCell::new("Red").background_color("FF0000"),
        TableCell::new("Blue").background_color("0000FF"),
    ];
    let row = TableRow::new(cells);
    let table = Table::new(vec![row], vec![1000000, 1000000], 0, 0);

    let xml = generate_table_xml(&table, 1);
    
    assert!(xml.contains("FF0000"));
    assert!(xml.contains("0000FF"));
}

#[test]
fn test_generate_table_with_special_characters() {
    let cells = vec![
        TableCell::new("Test & <Data>"),
        TableCell::new("More \"Quotes\""),
    ];
    let row = TableRow::new(cells);
    let table = Table::new(vec![row], vec![1000000, 1000000], 0, 0);

    let xml = generate_table_xml(&table, 1);
    
    assert!(xml.contains("&amp;"));
    assert!(xml.contains("&lt;"));
    assert!(xml.contains("&gt;"));
    assert!(xml.contains("&quot;"));
}

#[test]
fn test_generate_3x3_table() {
    let table = Table::from_data(
        vec![
            vec!["A", "B", "C"],
            vec!["1", "2", "3"],
            vec!["X", "Y", "Z"],
        ],
        vec![1000000, 1000000, 1000000],
        0,
        0,
    );

    let xml = generate_table_xml(&table, 1);
    
    // Should have 3 rows
    let row_count = xml.matches("<a:tr").count();
    assert_eq!(row_count, 3);
    
    // Should have 9 cells (3x3)
    let cell_count = xml.matches("<a:tc>").count();
    assert_eq!(cell_count, 9);
}

#[test]
fn test_generate_table_with_builder() {
    let table = TableBuilder::new(vec![1000000, 1000000])
        .position(500000, 1000000)
        .add_simple_row(vec!["Header 1", "Header 2"])
        .add_simple_row(vec!["Data 1", "Data 2"])
        .build();

    let xml = generate_table_xml(&table, 1);
    
    assert!(xml.contains("Header 1"));
    assert!(xml.contains("Header 2"));
    assert!(xml.contains("Data 1"));
    assert!(xml.contains("Data 2"));
}

#[test]
fn test_table_width_calculation() {
    let table = Table::from_data(
        vec![vec!["A", "B", "C"]],
        vec![1000000, 2000000, 3000000],
        0,
        0,
    );

    assert_eq!(table.width(), 6000000);
}

#[test]
fn test_table_height_calculation() {
    let cells1 = vec![TableCell::new("A")];
    let cells2 = vec![TableCell::new("B")];
    let cells3 = vec![TableCell::new("C")];
    
    let row1 = TableRow::new(cells1).with_height(500000);
    let row2 = TableRow::new(cells2).with_height(600000);
    let row3 = TableRow::new(cells3); // Default height
    
    let table = Table::new(vec![row1, row2, row3], vec![1000000], 0, 0);
    
    // 500000 + 600000 + 400000 (default) = 1500000
    assert_eq!(table.height(), 1500000);
}

#[test]
fn test_generate_wide_table() {
    let table = Table::from_data(
        vec![vec!["A", "B", "C", "D", "E"]],
        vec![1000000; 5],
        0,
        0,
    );

    let xml = generate_table_xml(&table, 1);
    
    assert!(xml.contains("a:t>A</a:t>"));
    assert!(xml.contains("a:t>E</a:t>"));
}

#[test]
fn test_generate_tall_table() {
    let mut rows = Vec::new();
    for i in 0..10 {
        let cells = vec![TableCell::new(&format!("Row {}", i))];
        rows.push(TableRow::new(cells));
    }
    
    let table = Table::new(rows, vec![1000000], 0, 0);
    let xml = generate_table_xml(&table, 1);
    
    // Should have 10 rows
    let row_count = xml.matches("<a:tr").count();
    assert_eq!(row_count, 10);
}

#[test]
fn test_generate_table_with_mixed_formatting() {
    let cells = vec![
        TableCell::new("Bold Red").bold().background_color("FF0000"),
        TableCell::new("Regular Blue").background_color("0000FF"),
        TableCell::new("Bold Green").bold().background_color("00FF00"),
    ];
    let row = TableRow::new(cells);
    let table = Table::new(vec![row], vec![1000000, 1000000, 1000000], 0, 0);

    let xml = generate_table_xml(&table, 1);
    
    assert!(xml.contains("FF0000"));
    assert!(xml.contains("0000FF"));
    assert!(xml.contains("00FF00"));
    assert!(xml.matches(r#"b="1""#).count() >= 2);
}

#[test]
fn test_generate_table_shape_id() {
    let table = Table::from_data(
        vec![vec!["A"]],
        vec![1000000],
        0,
        0,
    );

    let xml1 = generate_table_xml(&table, 1);
    let xml2 = generate_table_xml(&table, 2);
    
    assert!(xml1.contains("id=\"1\""));
    assert!(xml2.contains("id=\"2\""));
}

#[test]
fn test_generate_table_xml_structure() {
    let table = Table::from_data(
        vec![vec!["A", "B"]],
        vec![1000000, 1000000],
        0,
        0,
    );

    let xml = generate_table_xml(&table, 1);
    
    // Check main structure
    assert!(xml.contains("p:graphicFrame"));
    assert!(xml.contains("p:nvGraphicFramePr"));
    assert!(xml.contains("p:xfrm"));
    assert!(xml.contains("a:graphic"));
    assert!(xml.contains("a:graphicData"));
    assert!(xml.contains("a:tbl"));
    assert!(xml.contains("a:tblPr"));
    assert!(xml.contains("a:tblGrid"));
    assert!(xml.contains("a:gridCol"));
    assert!(xml.contains("a:tr"));
    assert!(xml.contains("a:tc"));
    assert!(xml.contains("a:txBody"));
}
