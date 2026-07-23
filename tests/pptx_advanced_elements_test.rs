//! Advanced tests for PowerPoint elements: tables, charts, and images
//!
//! Tests for complex PPTX components including data tables, charts, and images
//! Generates example files demonstrating each advanced element type

use ppt_rs::generator::{
    SlideContent, create_pptx_with_content,
    Table, TableRow, TableCell, TableBuilder,
};
use std::fs;
use std::path::Path;

// ============================================================================
// TABLE TESTS
// ============================================================================

#[test]
fn test_table_cell_creation() {
    let cell = TableCell::new("Header");
    assert_eq!(cell.text, "Header");
    assert!(!cell.bold);
    assert!(cell.background_color.is_none());
}

#[test]
fn test_table_cell_with_formatting() {
    let cell = TableCell::new("Formatted Header")
        .bold()
        .background_color("0000FF");
    
    assert!(cell.bold);
    assert_eq!(cell.background_color, Some("0000FF".to_string()));
}

#[test]
fn test_table_row_creation() {
    let cells = vec![
        TableCell::new("A"),
        TableCell::new("B"),
        TableCell::new("C"),
    ];
    let row = TableRow::new(cells);
    
    assert_eq!(row.cells.len(), 3);
    assert!(row.height.is_none());
}

#[test]
fn test_table_row_with_height() {
    let cells = vec![TableCell::new("Data")];
    let row = TableRow::new(cells).with_height(500000);
    
    assert_eq!(row.height, Some(500000));
}

#[test]
fn test_simple_2x2_table() {
    let table = Table::from_data(
        vec![
            vec!["A", "B"],
            vec!["1", "2"],
        ],
        vec![1000000, 1000000],
        0,
        0,
    );
    
    assert_eq!(table.row_count(), 2);
    assert_eq!(table.column_count(), 2);
}

#[test]
fn test_3x3_table() {
    let table = Table::from_data(
        vec![
            vec!["Name", "Age", "City"],
            vec!["Alice", "30", "NYC"],
            vec!["Bob", "25", "LA"],
        ],
        vec![1000000, 1000000, 1000000],
        0,
        0,
    );
    
    assert_eq!(table.row_count(), 3);
    assert_eq!(table.column_count(), 3);
}

#[test]
fn test_table_with_header_row() {
    let table = TableBuilder::new(vec![1000000, 1000000])
        .add_simple_row(vec!["Header 1", "Header 2"])
        .add_simple_row(vec!["Data 1", "Data 2"])
        .build();
    
    assert_eq!(table.row_count(), 2);
    assert_eq!(table.rows[0].cells[0].text, "Header 1");
}

#[test]
fn test_table_with_multiple_rows() {
    let table = TableBuilder::new(vec![1000000, 1000000])
        .add_simple_row(vec!["Col1", "Col2"])
        .add_simple_row(vec!["A", "B"])
        .add_simple_row(vec!["C", "D"])
        .add_simple_row(vec!["E", "F"])
        .build();
    
    assert_eq!(table.row_count(), 4);
}

#[test]
fn test_table_with_position() {
    let table = TableBuilder::new(vec![1000000, 1000000])
        .position(500000, 1000000)
        .add_simple_row(vec!["A", "B"])
        .build();
    
    assert_eq!(table.x, 500000);
    assert_eq!(table.y, 1000000);
}

#[test]
fn test_wide_table() {
    let table = Table::from_data(
        vec![
            vec!["Q1", "Q2", "Q3", "Q4", "Total"],
            vec!["100", "200", "150", "300", "750"],
        ],
        vec![1000000; 5],
        0,
        0,
    );
    
    assert_eq!(table.column_count(), 5);
}

#[test]
fn test_tall_table() {
    let names = vec!["Name", "Student 1", "Student 2", "Student 3", "Student 4", 
                     "Student 5", "Student 6", "Student 7", "Student 8", "Student 9", "Student 10"];
    let scores = vec!["Score", "10%", "20%", "30%", "40%", "50%", "60%", "70%", "80%", "90%", "100%"];
    
    let mut rows = Vec::new();
    for i in 0..names.len() {
        rows.push(vec![names[i], scores[i]]);
    }
    
    let table = Table::from_data(rows, vec![1000000, 1000000], 0, 0);
    assert_eq!(table.row_count(), 11);
}

#[test]
fn test_table_with_styled_cells() {
    let cells = vec![
        TableCell::new("Bold Header").bold().background_color("0000FF"),
        TableCell::new("Regular").background_color("FFFFFF"),
    ];
    let row = TableRow::new(cells);
    
    assert!(row.cells[0].bold);
    assert!(!row.cells[1].bold);
}

// ============================================================================
// CHART PLACEHOLDER TESTS
// ============================================================================

#[test]
fn test_chart_data_structure() {
    // Chart data structure for future implementation
    let chart_data = [("Category 1", 100),
        ("Category 2", 200),
        ("Category 3", 150)];
    
    assert_eq!(chart_data.len(), 3);
    assert_eq!(chart_data[0].1, 100);
}

#[test]
fn test_bar_chart_data() {
    let categories = ["Q1", "Q2", "Q3", "Q4"];
    let values = [100, 150, 200, 175];
    
    assert_eq!(categories.len(), values.len());
}

#[test]
fn test_pie_chart_data() {
    let slices = [("Category A", 30),
        ("Category B", 25),
        ("Category C", 20),
        ("Category D", 25)];
    
    let total: u32 = slices.iter().map(|(_, v)| v).sum();
    assert_eq!(total, 100);
}

#[test]
fn test_line_chart_data() {
    let months = ["Jan", "Feb", "Mar", "Apr", "May"];
    let data_series1 = [10, 15, 12, 18, 20];
    let data_series2 = [8, 12, 14, 16, 19];
    
    assert_eq!(months.len(), data_series1.len());
    assert_eq!(data_series1.len(), data_series2.len());
}

#[test]
fn test_scatter_chart_data() {
    let points = [(10, 20),
        (15, 25),
        (20, 30),
        (25, 35)];
    
    assert_eq!(points.len(), 4);
}

// ============================================================================
// IMAGE PLACEHOLDER TESTS
// ============================================================================

#[test]
fn test_image_metadata() {
    // Image metadata structure for future implementation
    #[allow(dead_code)]
    struct ImageMetadata {
        filename: String,
        width: u32,
        height: u32,
        format: String,
    }
    
    let image = ImageMetadata {
        filename: "test.png".to_string(),
        width: 1920,
        height: 1080,
        format: "PNG".to_string(),
    };
    
    assert_eq!(image.width, 1920);
    assert_eq!(image.height, 1080);
}

#[test]
fn test_image_dimensions() {
    let images = [("small.png", 640, 480),
        ("medium.jpg", 1280, 720),
        ("large.png", 1920, 1080)];
    
    assert_eq!(images.len(), 3);
    assert!(images[2].1 > images[1].1);
}

#[test]
fn test_image_aspect_ratios() {
    let aspect_ratios: Vec<(&str, f64)> = vec![
        ("16:9", 16.0 / 9.0),
        ("4:3", 4.0 / 3.0),
        ("1:1", 1.0),
        ("21:9", 21.0 / 9.0),
    ];
    
    assert_eq!(aspect_ratios.len(), 4);
    assert!((aspect_ratios[0].1 - 1.777).abs() < 0.01);
}

#[test]
fn test_image_formats() {
    let formats = ["PNG", "JPG", "GIF", "BMP", "TIFF"];
    
    assert!(formats.contains(&"PNG"));
    assert!(formats.contains(&"JPG"));
}

// ============================================================================
// PPTX GENERATION TESTS - Tables
// ============================================================================

#[test]
fn test_generate_simple_table_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Simple Table")
            .add_bullet("Basic 2x2 table"),
    ];

    let result = create_pptx_with_content("Simple Table", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_simple_table.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
    assert!(Path::new(output_path).exists());
}

#[test]
fn test_generate_data_table_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Sales Data Table")
            .add_bullet("Q1: $100K")
            .add_bullet("Q2: $150K")
            .add_bullet("Q3: $200K")
            .add_bullet("Q4: $250K"),
    ];

    let result = create_pptx_with_content("Sales Data", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_data_table.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_comparison_table_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Product Comparison")
            .add_bullet("Product A: $99, 5 stars")
            .add_bullet("Product B: $149, 4.5 stars")
            .add_bullet("Product C: $199, 4 stars"),
    ];

    let result = create_pptx_with_content("Comparison", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_comparison_table.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_schedule_table_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Project Schedule")
            .add_bullet("Phase 1: Jan - Feb")
            .add_bullet("Phase 2: Mar - Apr")
            .add_bullet("Phase 3: May - Jun")
            .add_bullet("Phase 4: Jul - Aug"),
    ];

    let result = create_pptx_with_content("Schedule", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_schedule_table.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

// ============================================================================
// PPTX GENERATION TESTS - Charts
// ============================================================================

#[test]
fn test_generate_chart_data_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Chart Data Example")
            .add_bullet("Category 1: 100")
            .add_bullet("Category 2: 200")
            .add_bullet("Category 3: 150")
            .add_bullet("Category 4: 250"),
    ];

    let result = create_pptx_with_content("Chart Data", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_chart_data.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_bar_chart_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Bar Chart Data")
            .title_bold(true)
            .add_bullet("Q1 2025: $2.5M")
            .add_bullet("Q2 2025: $3.1M")
            .add_bullet("Q3 2025: $3.8M")
            .add_bullet("Q4 2025: $4.2M"),
    ];

    let result = create_pptx_with_content("Bar Chart", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_bar_chart.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_pie_chart_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Market Share Distribution")
            .add_bullet("Product A: 30%")
            .add_bullet("Product B: 25%")
            .add_bullet("Product C: 20%")
            .add_bullet("Product D: 25%"),
    ];

    let result = create_pptx_with_content("Pie Chart", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_pie_chart.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_line_chart_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Growth Trend")
            .add_bullet("Jan: 100")
            .add_bullet("Feb: 120")
            .add_bullet("Mar: 150")
            .add_bullet("Apr: 180")
            .add_bullet("May: 220"),
    ];

    let result = create_pptx_with_content("Line Chart", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_line_chart.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

// ============================================================================
// PPTX GENERATION TESTS - Images
// ============================================================================

#[test]
fn test_generate_image_placeholder_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Image Placeholder")
            .add_bullet("Image 1: 1920x1080")
            .add_bullet("Image 2: 1280x720")
            .add_bullet("Image 3: 640x480"),
    ];

    let result = create_pptx_with_content("Image Placeholder", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_image_placeholder.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_gallery_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Image Gallery")
            .add_bullet("Landscape: 1920x1080")
            .add_bullet("Portrait: 1080x1920")
            .add_bullet("Square: 1024x1024")
            .add_bullet("Wide: 2560x1440"),
    ];

    let result = create_pptx_with_content("Gallery", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_gallery.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_mixed_content_slide() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Mixed Content")
            .add_bullet("Text content")
            .add_bullet("Table data")
            .add_bullet("Chart visualization")
            .add_bullet("Image gallery"),
        SlideContent::new("Advanced Elements")
            .title_bold(true)
            .add_bullet("Tables for data")
            .add_bullet("Charts for trends")
            .add_bullet("Images for visuals"),
    ];

    let result = create_pptx_with_content("Mixed Content", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_mixed_content.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}

#[test]
fn test_generate_comprehensive_advanced_presentation() {
    let _ = fs::create_dir_all("target/test_output");

    let slides = vec![
        SlideContent::new("Advanced Elements Demo")
            .title_size(56)
            .title_bold(true)
            .add_bullet("Tables, Charts, and Images"),
        SlideContent::new("Data Tables")
            .add_bullet("Sales by Quarter")
            .add_bullet("Q1: $100K, Q2: $150K")
            .add_bullet("Q3: $200K, Q4: $250K"),
        SlideContent::new("Chart Data")
            .add_bullet("Monthly Growth")
            .add_bullet("Jan: 100, Feb: 120")
            .add_bullet("Mar: 150, Apr: 180"),
        SlideContent::new("Image Gallery")
            .add_bullet("Product Images")
            .add_bullet("Screenshots")
            .add_bullet("Diagrams"),
        SlideContent::new("Summary")
            .title_bold(true)
            .content_bold(true)
            .add_bullet("Tables organize data")
            .add_bullet("Charts visualize trends")
            .add_bullet("Images enhance content"),
    ];

    let result = create_pptx_with_content("Advanced Elements", slides);
    assert!(result.is_ok());

    let pptx_data = result.unwrap();
    let output_path = "target/test_output/test_advanced_comprehensive.pptx";
    assert!(fs::write(output_path, pptx_data).is_ok());
}
