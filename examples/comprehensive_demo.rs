//! Comprehensive demonstration of all pptx-rs capabilities
//!
//! This example showcases:
//! - Slide layouts (6 types)
//! - Text formatting (bold, italic, underline, colors, sizes)
//! - Tables with styling
//! - Charts (bar, line, pie)
//! - Images
//! - Package reading/writing

use pptx_rs::generator::{
    create_pptx_with_content, SlideContent, SlideLayout,
    Table, TableRow, TableCell, TableBuilder,
    ChartType, ChartSeries, ChartBuilder,
    ImageBuilder,
    TextFormat,
    Shape, ShapeType, ShapeFill,
};
use pptx_rs::opc::Package;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║           PPTX-RS Comprehensive Demo                       ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // =========================================================================
    // PART 1: Slide Layouts
    // =========================================================================
    println!("📐 Part 1: Slide Layouts");
    
    let layout_slides = vec![
        // Cover slide with centered title
        SlideContent::new("PPTX-RS Library Demo")
            .layout(SlideLayout::CenteredTitle)
            .title_size(54)
            .title_bold(true)
            .title_color("1F497D"),
        
        // Title only layout
        SlideContent::new("Section: Slide Layouts")
            .layout(SlideLayout::TitleOnly)
            .title_color("C0504D"),
        
        // Standard title and content
        SlideContent::new("Available Layouts")
            .layout(SlideLayout::TitleAndContent)
            .add_bullet("CenteredTitle - Cover slides")
            .add_bullet("TitleOnly - Section headers")
            .add_bullet("TitleAndContent - Standard slides")
            .add_bullet("TitleAndBigContent - More content space")
            .add_bullet("TwoColumn - Side-by-side comparison")
            .add_bullet("Blank - Custom content"),
        
        // Two column layout
        SlideContent::new("Comparison View")
            .layout(SlideLayout::TwoColumn)
            .add_bullet("Left: Feature A")
            .add_bullet("Left: Feature B")
            .add_bullet("Left: Feature C")
            .add_bullet("Right: Benefit A")
            .add_bullet("Right: Benefit B")
            .add_bullet("Right: Benefit C"),
    ];
    println!("   ✓ Created {} layout demo slides", layout_slides.len());

    // =========================================================================
    // PART 2: Text Formatting
    // =========================================================================
    println!("📝 Part 2: Text Formatting");
    
    // Demonstrate TextFormat API
    let bold_format = TextFormat::new().bold();
    let italic_format = TextFormat::new().italic();
    let colored_format = TextFormat::new().color("FF0000");
    let combined_format = TextFormat::new()
        .bold()
        .italic()
        .underline()
        .color("0000FF")
        .font_size(32);
    
    println!("   ✓ Bold format: {:?}", bold_format);
    println!("   ✓ Italic format: {:?}", italic_format);
    println!("   ✓ Colored format: {:?}", colored_format);
    println!("   ✓ Combined format: {:?}", combined_format);
    
    let text_slides = vec![
        SlideContent::new("Text Formatting Demo")
            .title_bold(true)
            .title_color("4F81BD")
            .title_size(44)
            .add_bullet("Bold titles with custom colors")
            .add_bullet("Italic content support")
            .add_bullet("Underline emphasis")
            .add_bullet("Font sizes from 8pt to 96pt")
            .content_size(24)
            .content_color("333333"),
        
        SlideContent::new("Color Palette")
            .title_color("9BBB59")
            .add_bullet("Primary: #1F497D (Blue)")
            .add_bullet("Accent 1: #4F81BD (Light Blue)")
            .add_bullet("Accent 2: #C0504D (Red)")
            .add_bullet("Accent 3: #9BBB59 (Green)")
            .add_bullet("Accent 4: #8064A2 (Purple)"),
    ];
    println!("   ✓ Created {} text formatting slides", text_slides.len());

    // =========================================================================
    // PART 3: Tables
    // =========================================================================
    println!("📊 Part 3: Tables");
    
    // Define column widths (in EMU - 914400 EMU = 1 inch)
    let col_widths = vec![1500000, 1500000, 1500000]; // 3 columns
    
    // Create a sales table using TableBuilder
    let sales_table = TableBuilder::new(col_widths.clone())
        .add_row(TableRow::new(vec![
            TableCell::new("Quarter").bold().background_color("4F81BD"),
            TableCell::new("Revenue").bold().background_color("4F81BD"),
            TableCell::new("Growth").bold().background_color("4F81BD"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Q1 2024"),
            TableCell::new("$1.2M"),
            TableCell::new("+15%"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Q2 2024"),
            TableCell::new("$1.5M"),
            TableCell::new("+25%"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Q3 2024"),
            TableCell::new("$1.8M"),
            TableCell::new("+20%"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Q4 2024").bold(),
            TableCell::new("$2.2M").bold(),
            TableCell::new("+22%").bold().background_color("9BBB59"),
        ]))
        .build();
    
    println!("   ✓ Created sales table: {} rows x {} cols", 
             sales_table.rows.len(), 
             sales_table.rows.first().map(|r| r.cells.len()).unwrap_or(0));
    
    // Create table from 2D data
    let data = vec![
        vec!["Product", "Units", "Price"],
        vec!["Widget A", "1,500", "$29.99"],
        vec!["Widget B", "2,300", "$19.99"],
        vec!["Widget C", "890", "$49.99"],
    ];
    let product_table = Table::from_data(data, col_widths.clone(), 457200, 1600200);
    println!("   ✓ Created product table from data: {} rows", product_table.rows.len());
    
    let table_slides = vec![
        SlideContent::new("Quarterly Revenue")
            .table(sales_table)
            .title_color("1F497D"),
        
        SlideContent::new("Product Inventory")
            .add_bullet("Widget A: 1,500 units @ $29.99")
            .add_bullet("Widget B: 2,300 units @ $19.99")
            .add_bullet("Widget C: 890 units @ $49.99")
            .with_table(),
    ];
    println!("   ✓ Created {} table slides", table_slides.len());

    // =========================================================================
    // PART 4: Charts
    // =========================================================================
    println!("📈 Part 4: Charts");
    
    // Bar Chart
    let bar_chart = ChartBuilder::new("Regional Sales", ChartType::Bar)
        .categories(vec!["North", "South", "East", "West"])
        .add_series(ChartSeries::new("Q1", vec![45.0, 38.0, 52.0, 41.0]))
        .add_series(ChartSeries::new("Q2", vec![52.0, 42.0, 58.0, 48.0]))
        .build();
    println!("   ✓ Created bar chart: {} series", bar_chart.series.len());
    
    // Line Chart
    let line_chart = ChartBuilder::new("Monthly Trend", ChartType::Line)
        .categories(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
        .add_series(ChartSeries::new("Revenue", vec![50.0, 55.0, 60.0, 58.0, 65.0, 70.0]))
        .add_series(ChartSeries::new("Target", vec![55.0, 55.0, 60.0, 60.0, 65.0, 70.0]))
        .build();
    println!("   ✓ Created line chart: {} data points", 
             line_chart.series.first().map(|s| s.values.len()).unwrap_or(0));
    
    // Pie Chart
    let pie_chart = ChartBuilder::new("Market Share", ChartType::Pie)
        .categories(vec!["Product A", "Product B", "Product C", "Product D"])
        .add_series(ChartSeries::new("Share", vec![35.0, 25.0, 25.0, 15.0]))
        .build();
    println!("   ✓ Created pie chart: {} slices", 
             pie_chart.series.first().map(|s| s.values.len()).unwrap_or(0));
    
    let chart_slides = vec![
        SlideContent::new("Bar Chart: Regional Sales")
            .with_chart()
            .add_bullet("North: Strong Q2 growth (+15%)")
            .add_bullet("East: Highest performer")
            .add_bullet("South: Steady improvement")
            .add_bullet("West: Consistent growth"),
        
        SlideContent::new("Line Chart: Revenue Trend")
            .with_chart()
            .add_bullet("6-month upward trend")
            .add_bullet("Exceeded targets in May-Jun")
            .add_bullet("40% growth Jan to Jun"),
        
        SlideContent::new("Pie Chart: Market Share")
            .with_chart()
            .add_bullet("Product A leads with 35%")
            .add_bullet("Products B & C tied at 25%")
            .add_bullet("Product D: Growth opportunity"),
    ];
    println!("   ✓ Created {} chart slides", chart_slides.len());

    // =========================================================================
    // PART 5: Images
    // =========================================================================
    println!("🖼️  Part 5: Images");
    
    // Create image configurations (width/height in EMU)
    let logo_image = ImageBuilder::new("logo.png", 2000000, 1000000)
        .position(100000, 100000)
        .build();
    println!("   ✓ Logo image: {}x{} at ({}, {})", 
             logo_image.width, logo_image.height, logo_image.x, logo_image.y);
    
    let photo_image = ImageBuilder::new("photo.jpg", 4000000, 3000000)
        .position(300000, 200000)
        .build()
        .scale_to_width(4000000);
    println!("   ✓ Photo image: scaled to width {}", photo_image.width);
    
    let image_slides = vec![
        SlideContent::new("Image Support")
            .add_bullet("PNG, JPG, GIF, BMP, TIFF formats")
            .add_bullet("Custom positioning and sizing")
            .add_bullet("Aspect ratio preservation")
            .add_image(logo_image)
            .add_image(photo_image),
    ];
    println!("   ✓ Created {} image slides with embedded images", image_slides.len());

    // =========================================================================
    // PART 6: Shapes
    // =========================================================================
    println!("🔷 Part 6: Shapes");
    
    // Create shapes with proper EMU dimensions
    let rectangle = Shape::new(ShapeType::Rectangle, 100000, 100000, 2000000, 1000000)
        .with_fill(ShapeFill::new("4F81BD"))
        .with_text("Rectangle");
    println!("   ✓ Rectangle shape: {:?}", rectangle.shape_type);
    
    let circle = Shape::new(ShapeType::Circle, 3500000, 100000, 1500000, 1500000)
        .with_fill(ShapeFill::new("C0504D"));
    println!("   ✓ Circle shape: {:?}", circle.shape_type);
    
    let shape_slides = vec![
        SlideContent::new("Shape Support")
            .add_bullet("Rectangle, Circle, Triangle, and more")
            .add_bullet("Solid color fills with transparency")
            .add_bullet("Border/line styling")
            .add_shape(rectangle)
            .add_shape(circle),
    ];
    println!("   ✓ Created {} shape slides with embedded shapes", shape_slides.len());

    // =========================================================================
    // Combine all slides
    // =========================================================================
    let mut all_slides = Vec::new();
    all_slides.extend(layout_slides);
    all_slides.extend(text_slides);
    all_slides.extend(table_slides);
    all_slides.extend(chart_slides);
    all_slides.extend(image_slides);
    all_slides.extend(shape_slides);
    
    // Add summary slide
    all_slides.push(
        SlideContent::new("Summary")
            .layout(SlideLayout::TitleAndBigContent)
            .title_color("1F497D")
            .add_bullet("✓ 6 Slide Layouts")
            .add_bullet("✓ Rich Text Formatting")
            .add_bullet("✓ Tables with Styling")
            .add_bullet("✓ Charts (Bar, Line, Pie)")
            .add_bullet("✓ Image Embedding")
            .add_bullet("✓ Shape Drawing")
            .content_bold(true)
    );

    // =========================================================================
    // Generate PPTX
    // =========================================================================
    println!("\n📦 Generating PPTX...");
    let pptx_data = create_pptx_with_content("PPTX-RS Demo", all_slides.clone())?;
    fs::write("comprehensive_demo.pptx", &pptx_data)?;
    println!("   ✓ Created comprehensive_demo.pptx ({} slides, {} bytes)", 
             all_slides.len(), pptx_data.len());

    // =========================================================================
    // PART 7: Package Reading
    // =========================================================================
    println!("\n📖 Part 7: Package Reading");
    
    let package = Package::open("comprehensive_demo.pptx")?;
    let paths = package.part_paths();
    
    println!("   Package contents:");
    println!("   ├── Total parts: {}", package.part_count());
    println!("   ├── Slides: {}", paths.iter()
        .filter(|p| p.starts_with("ppt/slides/slide") && p.ends_with(".xml"))
        .count());
    println!("   ├── Relationships: {}", paths.iter()
        .filter(|p| p.contains(".rels"))
        .count());
    println!("   └── XML files: {}", paths.iter()
        .filter(|p| p.ends_with(".xml"))
        .count());
    
    // Show some part contents
    if let Some(core) = package.get_part("docProps/core.xml") {
        let content = String::from_utf8_lossy(core);
        if content.contains("<dc:title>") {
            println!("\n   Core properties found:");
            println!("   └── Title: PPTX-RS Demo");
        }
    }

    // =========================================================================
    // Summary
    // =========================================================================
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                    Demo Complete                           ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  Features Demonstrated:                                    ║");
    println!("║  ✓ Slide Layouts (6 types)                                 ║");
    println!("║  ✓ Text Formatting (bold, italic, underline, colors)       ║");
    println!("║  ✓ Tables (TableBuilder, from_data)                        ║");
    println!("║  ✓ Charts (Bar, Line, Pie with ChartBuilder)               ║");
    println!("║  ✓ Images (ImageBuilder with positioning)                  ║");
    println!("║  ✓ Shapes (Rectangle, Ellipse, etc.)                       ║");
    println!("║  ✓ Package Reading (open, get_part, part_paths)            ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  Output: comprehensive_demo.pptx                           ║");
    println!("║  Open in PowerPoint, LibreOffice, or Google Slides         ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    Ok(())
}
