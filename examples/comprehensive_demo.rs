//! Comprehensive PPTX Element Showcase
//!
//! A focused demo showing ALL pptx-rs capabilities:
//! 1. All 6 slide layouts
//! 2. Text formatting (bold, italic, underline, colors, sizes)
//! 3. Tables with cell styling
//! 4. Charts (bar, line, pie)
//! 5. Shapes with fills
//! 6. Images (placeholder)
//! 7. Package reading/writing
//! 8. Parts API (SlideLayout, SlideMaster, Theme, Notes, Media, etc.)
//! 9. Elements API (Color, Position, Size, Transform)
//! 10. NEW v0.2.1: BulletStyle (Number, Letter, Roman, Custom)
//! 11. NEW v0.2.1: Text enhancements (Strikethrough, Highlight, Sub/Superscript)
//! 12. NEW v0.2.1: Font size presets
//! 13. NEW v0.2.1: Image from base64/bytes
//! 14. NEW v0.2.1: Theme colors (Corporate, Modern, Vibrant, Dark, Nature, Tech, Carbon)
//! 15. NEW v0.2.1: Material & Carbon Design colors
//! 16. NEW: Digital signatures, Ink annotations, Slide show settings
//! 17. NEW: Print settings, Advanced table merging, Embedded fonts
//! 18. NEW: Flexible Dimension API (EMU/inches/cm/pt/ratio/percent)

use ppt_rs::generator::{
    create_pptx_with_settings, SlideContent, SlideLayout,
    TableRow, TableCell, TableBuilder,
    ChartType, ChartSeries, ChartBuilder,
    Shape, ShapeType, ShapeFill, ShapeLine,
    Image, ImageBuilder,
    Connector, ConnectorLine, ArrowType, ArrowSize, LineDash,
    BulletStyle, BulletPoint,
    // Advanced features actually embedded in PPTX output
    SlideShowSettings, ShowType, PenColor, SlideRange,
    PrintSettings, HandoutLayout as GenHandoutLayout, PrintColorMode, PrintWhat,
    TableMergeMap,
    PresentationSettings,
};
use ppt_rs::generator::shapes::{GradientFill, GradientDirection};
use ppt_rs::prelude::{colors, themes, font_sizes, Dimension};
use ppt_rs::opc::Package;
use ppt_rs::parts::{
    SlideLayoutPart, LayoutType,
    SlideMasterPart,
    ThemePart,
    NotesSlidePart,
    AppPropertiesPart,
    MediaPart, MediaFormat,
    TablePart, TableRowPart, TableCellPart,
    HorizontalAlign, VerticalAlign,
    ContentTypesPart,
    Part,
    // Advanced features
    Animation, AnimationEffect, AnimationTrigger, AnimationDirection,
    SlideTransition, TransitionEffect, SlideAnimations,
    HandoutMasterPart, HandoutLayout,
    CustomXmlPart,
    VbaProjectPart, VbaModule,
    EmbeddedFontCollection, FontEmbedType,
    SmartArtPart, SmartArtLayout,
    Model3DPart, Model3DFormat, CameraPreset,
};
use ppt_rs::elements::{
    Color, RgbColor, SchemeColor,
    Position, Size, Transform,
    EMU_PER_INCH,
};
use ppt_rs::ToXml;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║       PPTX-RS Element Showcase - Complete Coverage           ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let mut slides = Vec::new();

    // =========================================================================
    // SLIDE 1: CenteredTitle Layout + Title Formatting
    // =========================================================================
    println!("📐 Slide 1: CenteredTitle Layout + Title Formatting");
    slides.push(
        SlideContent::new("PPTX-RS Element Showcase")
            .layout(SlideLayout::CenteredTitle)
            .title_size(54)
            .title_bold(true)
            .title_color("1F497D")
    );

    // =========================================================================
    // SLIDE 2: TitleOnly Layout
    // =========================================================================
    println!("📐 Slide 2: TitleOnly Layout");
    slides.push(
        SlideContent::new("Section: Slide Layouts")
            .layout(SlideLayout::TitleOnly)
            .title_size(48)
            .title_bold(true)
            .title_color("C0504D")
    );

    // =========================================================================
    // SLIDE 3: TitleAndContent Layout + All Text Formatting
    // =========================================================================
    println!("📝 Slide 3: TitleAndContent + Text Formatting");
    slides.push(
        SlideContent::new("Text Formatting Options")
            .layout(SlideLayout::TitleAndContent)
            .title_color("1F497D")
            .title_bold(true)
            .title_italic(true)
            .title_underline(true)
            .title_size(44)
            .add_bullet("Normal text (default)")
            .add_bullet("Bold content text")
            .add_bullet("Italic content text")
            .add_bullet("Underlined content")
            .add_bullet("Custom font size (28pt)")
            .add_bullet("Custom color (#4F81BD)")
            .content_bold(true)
            .content_italic(true)
            .content_underline(true)
            .content_size(28)
            .content_color("4F81BD")
    );

    // =========================================================================
    // SLIDE 4: TitleAndBigContent Layout
    // =========================================================================
    println!("📐 Slide 4: TitleAndBigContent Layout");
    slides.push(
        SlideContent::new("Key Highlights")
            .layout(SlideLayout::TitleAndBigContent)
            .title_color("1F497D")
            .add_bullet("Large content area for emphasis")
            .add_bullet("Perfect for key messages")
            .add_bullet("Smaller title, bigger content")
            .content_bold(true)
            .content_size(32)
    );

    // =========================================================================
    // SLIDE 5: TwoColumn Layout
    // =========================================================================
    println!("📐 Slide 5: TwoColumn Layout");
    slides.push(
        SlideContent::new("Two Column Comparison")
            .layout(SlideLayout::TwoColumn)
            .title_color("1F497D")
            .add_bullet("Left Column Item 1")
            .add_bullet("Left Column Item 2")
            .add_bullet("Left Column Item 3")
            .add_bullet("Right Column Item 1")
            .add_bullet("Right Column Item 2")
            .add_bullet("Right Column Item 3")
            .content_size(24)
    );

    // =========================================================================
    // SLIDE 6: Blank Layout
    // =========================================================================
    println!("📐 Slide 6: Blank Layout");
    slides.push(
        SlideContent::new("")
            .layout(SlideLayout::Blank)
    );

    // =========================================================================
    // SLIDE 7: Table with All Cell Styling Options
    // =========================================================================
    println!("📊 Slide 7: Table with Cell Styling");
    let styled_table = TableBuilder::new(vec![1500000, 1500000, 1500000])
        .add_row(TableRow::new(vec![
            TableCell::new("Header 1").bold().background_color("1F497D"),
            TableCell::new("Header 2").bold().background_color("4F81BD"),
            TableCell::new("Header 3").bold().background_color("8064A2"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Bold Cell").bold(),
            TableCell::new("Normal Cell"),
            TableCell::new("Colored").background_color("9BBB59"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Red BG").background_color("C0504D"),
            TableCell::new("Green BG").background_color("9BBB59"),
            TableCell::new("Blue BG").background_color("4F81BD"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Row 3 Col 1"),
            TableCell::new("Row 3 Col 2"),
            TableCell::new("Row 3 Col 3").bold().background_color("F79646"),
        ]))
        .position(500000, 1800000)
        .build();
    
    slides.push(
        SlideContent::new("Table with Cell Styling")
            .table(styled_table)
            .title_color("1F497D")
    );

    // =========================================================================
    // SLIDE 8: Charts (Bar, Line, Pie)
    // =========================================================================
    println!("📈 Slide 8: Chart Types");
    
    // Create chart data structures (for demonstration)
    let _bar_chart = ChartBuilder::new("Sales by Region", ChartType::Bar)
        .categories(vec!["North", "South", "East", "West"])
        .add_series(ChartSeries::new("2023", vec![100.0, 80.0, 120.0, 90.0]))
        .add_series(ChartSeries::new("2024", vec![120.0, 95.0, 140.0, 110.0]))
        .build();
    
    let _line_chart = ChartBuilder::new("Monthly Trend", ChartType::Line)
        .categories(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
        .add_series(ChartSeries::new("Revenue", vec![10.0, 12.0, 15.0, 14.0, 18.0, 22.0]))
        .build();
    
    let _pie_chart = ChartBuilder::new("Market Share", ChartType::Pie)
        .categories(vec!["Product A", "Product B", "Product C", "Others"])
        .add_series(ChartSeries::new("Share", vec![40.0, 30.0, 20.0, 10.0]))
        .build();
    
    slides.push(
        SlideContent::new("Chart Types: Bar, Line, Pie")
            .with_chart()
            .title_color("1F497D")
            .add_bullet("Bar Chart: Compare categories")
            .add_bullet("Line Chart: Show trends over time")
            .add_bullet("Pie Chart: Show proportions")
            .content_size(24)
    );

    // =========================================================================
    // SLIDE 9: Shapes with Different Fills
    // =========================================================================
    println!("🔷 Slide 9: Shapes with Fills");
    
    let rect = Shape::new(ShapeType::Rectangle, 500000, 1600000, 2000000, 1000000)
        .with_fill(ShapeFill::new("4F81BD"))
        .with_text("Rectangle");
    
    let ellipse = Shape::new(ShapeType::Ellipse, 3000000, 1600000, 2000000, 1000000)
        .with_fill(ShapeFill::new("9BBB59"))
        .with_text("Ellipse");
    
    let rounded = Shape::new(ShapeType::RoundedRectangle, 5500000, 1600000, 2000000, 1000000)
        .with_fill(ShapeFill::new("C0504D"))
        .with_text("Rounded");
    
    let triangle = Shape::new(ShapeType::Triangle, 1500000, 3000000, 1500000, 1200000)
        .with_fill(ShapeFill::new("8064A2"))
        .with_text("Triangle");
    
    let diamond = Shape::new(ShapeType::Diamond, 4000000, 3000000, 1500000, 1200000)
        .with_fill(ShapeFill::new("F79646"))
        .with_text("Diamond");
    
    slides.push(
        SlideContent::new("Shape Types with Color Fills")
            .add_shape(rect)
            .add_shape(ellipse)
            .add_shape(rounded)
            .add_shape(triangle)
            .add_shape(diamond)
            .title_color("1F497D")
    );

    // =========================================================================
    // SLIDE 10: Gradient Fills (NEW)
    // =========================================================================
    println!("🌈 Slide 10: Gradient Fills");
    
    // Horizontal gradient
    let gradient_h = Shape::new(ShapeType::Rectangle, 500000, 1600000, 2500000, 1200000)
        .with_gradient(GradientFill::linear("1565C0", "42A5F5", GradientDirection::Horizontal))
        .with_text("Horizontal");
    
    // Vertical gradient
    let gradient_v = Shape::new(ShapeType::Rectangle, 3200000, 1600000, 2500000, 1200000)
        .with_gradient(GradientFill::linear("2E7D32", "81C784", GradientDirection::Vertical))
        .with_text("Vertical");
    
    // Diagonal gradient
    let gradient_d = Shape::new(ShapeType::RoundedRectangle, 5900000, 1600000, 2500000, 1200000)
        .with_gradient(GradientFill::linear("C62828", "EF9A9A", GradientDirection::DiagonalDown))
        .with_text("Diagonal");
    
    // Three-color gradient
    let gradient_3 = Shape::new(ShapeType::Ellipse, 1800000, 3200000, 2500000, 1200000)
        .with_gradient(GradientFill::three_color("FF6F00", "FFC107", "FFEB3B", GradientDirection::Horizontal))
        .with_text("3-Color");
    
    // Custom angle gradient
    let gradient_angle = Shape::new(ShapeType::RoundedRectangle, 4800000, 3200000, 2500000, 1200000)
        .with_gradient(GradientFill::linear("7B1FA2", "E1BEE7", GradientDirection::Angle(135)))
        .with_text("135° Angle");
    
    slides.push(
        SlideContent::new("Gradient Fills - Multiple Directions")
            .add_shape(gradient_h)
            .add_shape(gradient_v)
            .add_shape(gradient_d)
            .add_shape(gradient_3)
            .add_shape(gradient_angle)
            .title_color("1F497D")
    );

    // =========================================================================
    // SLIDE 11: Transparency (NEW)
    // =========================================================================
    println!("👻 Slide 11: Transparency Effects");
    
    // Base shape (fully opaque)
    let base = Shape::new(ShapeType::Rectangle, 1000000, 1800000, 3000000, 2000000)
        .with_fill(ShapeFill::new("1565C0"))
        .with_text("Base (100%)");
    
    // 25% transparent overlay
    let trans_25 = Shape::new(ShapeType::Rectangle, 2000000, 2200000, 2500000, 1500000)
        .with_fill(ShapeFill::new("F44336").with_transparency(25))
        .with_line(ShapeLine::new("B71C1C", 25400))
        .with_text("25% Transparent");
    
    // 50% transparent overlay
    let trans_50 = Shape::new(ShapeType::Ellipse, 4500000, 1800000, 2500000, 2000000)
        .with_fill(ShapeFill::new("4CAF50").with_transparency(50))
        .with_line(ShapeLine::new("1B5E20", 25400))
        .with_text("50% Transparent");
    
    // 75% transparent overlay
    let trans_75 = Shape::new(ShapeType::RoundedRectangle, 5500000, 2500000, 2500000, 1500000)
        .with_fill(ShapeFill::new("FF9800").with_transparency(75))
        .with_line(ShapeLine::new("E65100", 25400))
        .with_text("75% Transparent");
    
    slides.push(
        SlideContent::new("Transparency Effects - Overlapping Shapes")
            .add_shape(base)
            .add_shape(trans_25)
            .add_shape(trans_50)
            .add_shape(trans_75)
            .title_color("1F497D")
    );

    // =========================================================================
    // SLIDE 12: Styled Connectors (NEW)
    // =========================================================================
    println!("🔗 Slide 12: Styled Connectors");
    
    // Create shapes to connect
    let box1 = Shape::new(ShapeType::RoundedRectangle, 500000, 1800000, 1800000, 800000)
        .with_id(100)
        .with_fill(ShapeFill::new("1565C0"))
        .with_text("Start");
    
    let box2 = Shape::new(ShapeType::RoundedRectangle, 3500000, 1800000, 1800000, 800000)
        .with_id(101)
        .with_fill(ShapeFill::new("2E7D32"))
        .with_text("Process");
    
    let box3 = Shape::new(ShapeType::RoundedRectangle, 6500000, 1800000, 1800000, 800000)
        .with_id(102)
        .with_fill(ShapeFill::new("C62828"))
        .with_text("End");
    
    // Straight connector with arrow
    let conn1 = Connector::straight(2300000, 2200000, 3500000, 2200000)
        .with_line(ConnectorLine::new("1565C0", 25400))
        .with_end_arrow(ArrowType::Triangle)
        .with_arrow_size(ArrowSize::Large);
    
    // Elbow connector with stealth arrow
    let conn2 = Connector::elbow(5300000, 2200000, 6500000, 2200000)
        .with_line(ConnectorLine::new("2E7D32", 38100).with_dash(LineDash::Dash))
        .with_end_arrow(ArrowType::Stealth)
        .with_arrow_size(ArrowSize::Medium);
    
    // Curved connector examples
    let box4 = Shape::new(ShapeType::Ellipse, 1000000, 3200000, 1500000, 800000)
        .with_id(103)
        .with_fill(ShapeFill::new("7B1FA2"))
        .with_text("A");
    
    let box5 = Shape::new(ShapeType::Ellipse, 4000000, 3200000, 1500000, 800000)
        .with_id(104)
        .with_fill(ShapeFill::new("00838F"))
        .with_text("B");
    
    let box6 = Shape::new(ShapeType::Ellipse, 7000000, 3200000, 1500000, 800000)
        .with_id(105)
        .with_fill(ShapeFill::new("EF6C00"))
        .with_text("C");
    
    // Curved connector with diamond arrow
    let conn3 = Connector::curved(2500000, 3600000, 4000000, 3600000)
        .with_line(ConnectorLine::new("7B1FA2", 19050).with_dash(LineDash::DashDot))
        .with_arrows(ArrowType::Oval, ArrowType::Diamond);
    
    // Dotted connector
    let conn4 = Connector::straight(5500000, 3600000, 7000000, 3600000)
        .with_line(ConnectorLine::new("00838F", 12700).with_dash(LineDash::Dot))
        .with_end_arrow(ArrowType::Open);
    
    slides.push(
        SlideContent::new("Styled Connectors - Types, Arrows, Dashes")
            .add_shape(box1)
            .add_shape(box2)
            .add_shape(box3)
            .add_shape(box4)
            .add_shape(box5)
            .add_shape(box6)
            .add_connector(conn1)
            .add_connector(conn2)
            .add_connector(conn3)
            .add_connector(conn4)
            .title_color("1F497D")
    );

    // =========================================================================
    // SLIDE 13: Images
    // =========================================================================
    println!("🖼️  Slide 13: Image Placeholders");
    
    let img1 = Image::new("logo.png", 2500000, 1800000, "png")
        .position(500000, 1600000);
    let img2 = Image::new("photo.jpg", 2500000, 1800000, "jpg")
        .position(3500000, 1600000);
    let img3 = Image::new("diagram.png", 2000000, 1800000, "png")
        .position(6500000, 1600000);
    
    slides.push(
        SlideContent::new("Image Placeholders")
            .add_image(img1)
            .add_image(img2)
            .add_image(img3)
            .title_color("1F497D")
    );

    // =========================================================================
    // SLIDE 11: Advanced Table with Borders & Alignment (NEW)
    // =========================================================================
    println!("📊 Slide 11: Advanced Table (borders, alignment, merged cells)");
    
    // Build advanced table using generator's TableBuilder with alignment
    let advanced_table = TableBuilder::new(vec![2000000, 2000000, 2000000, 2000000])
        .add_row(TableRow::new(vec![
            TableCell::new("Q1 2024 Financial Report").bold().background_color("1F4E79").text_color("FFFFFF").align_center().font_size(14),
            TableCell::new("").background_color("1F4E79"),
            TableCell::new("").background_color("1F4E79"),
            TableCell::new("").background_color("1F4E79"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Category").bold().background_color("2E75B6").text_color("FFFFFF").align_center(),
            TableCell::new("Revenue").bold().background_color("2E75B6").text_color("FFFFFF").align_center(),
            TableCell::new("Expenses").bold().background_color("2E75B6").text_color("FFFFFF").align_center(),
            TableCell::new("Profit").bold().background_color("2E75B6").text_color("FFFFFF").align_center(),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Product Sales").text_color("000000").align_left(),
            TableCell::new("$1,250,000").text_color("2E7D32").align_right(),
            TableCell::new("$450,000").text_color("C62828").align_right(),
            TableCell::new("$800,000").bold().text_color("2E7D32").align_right(),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Services").text_color("000000").align_left(),
            TableCell::new("$890,000").text_color("2E7D32").align_right(),
            TableCell::new("$320,000").text_color("C62828").align_right(),
            TableCell::new("$570,000").bold().text_color("2E7D32").align_right(),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Total").bold().background_color("E7E6E6").text_color("000000").align_left(),
            TableCell::new("$2,140,000").bold().background_color("E7E6E6").text_color("000000").align_right(),
            TableCell::new("$770,000").bold().background_color("E7E6E6").text_color("000000").align_right(),
            TableCell::new("$1,370,000").bold().background_color("C6EFCE").text_color("006100").align_right(),
        ]))
        .position(300000, 1600000)
        .build();
    
    slides.push(
        SlideContent::new("Financial Report - Advanced Table")
            .table(advanced_table)
            .title_color("1F4E79")
            .title_bold(true)
    );

    // =========================================================================
    // SLIDE 12: Comparison Matrix Table (NEW)
    // =========================================================================
    println!("📊 Slide 12: Comparison Matrix Table");
    
    let comparison_table = TableBuilder::new(vec![2000000, 1500000, 1500000, 1500000])
        .add_row(TableRow::new(vec![
            TableCell::new("Feature").bold().background_color("4472C4").text_color("FFFFFF"),
            TableCell::new("Basic").bold().background_color("4472C4").text_color("FFFFFF"),
            TableCell::new("Pro").bold().background_color("4472C4").text_color("FFFFFF"),
            TableCell::new("Enterprise").bold().background_color("4472C4").text_color("FFFFFF"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Storage").text_color("000000"),
            TableCell::new("5 GB").text_color("000000"),
            TableCell::new("50 GB").text_color("000000"),
            TableCell::new("Unlimited").bold().text_color("2E7D32"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Users").text_color("000000"),
            TableCell::new("1").text_color("000000"),
            TableCell::new("10").text_color("000000"),
            TableCell::new("Unlimited").bold().text_color("2E7D32"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Support").text_color("000000"),
            TableCell::new("Email").text_color("000000"),
            TableCell::new("24/7 Chat").text_color("000000"),
            TableCell::new("Dedicated").bold().text_color("2E7D32"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("API Access").text_color("000000"),
            TableCell::new("No").text_color("C62828"),
            TableCell::new("Yes").text_color("2E7D32"),
            TableCell::new("Yes + Priority").bold().text_color("2E7D32"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Price/month").bold().background_color("F2F2F2").text_color("000000"),
            TableCell::new("$9").bold().background_color("F2F2F2").text_color("000000"),
            TableCell::new("$29").bold().background_color("F2F2F2").text_color("000000"),
            TableCell::new("$99").bold().background_color("F2F2F2").text_color("000000"),
        ]))
        .position(500000, 1600000)
        .build();
    
    slides.push(
        SlideContent::new("Pricing Comparison Matrix")
            .table(comparison_table)
            .title_color("4472C4")
            .title_bold(true)
    );

    // =========================================================================
    // SLIDE 13: Process Flow with Shapes (NEW - SmartArt-like)
    // =========================================================================
    println!("🔷 Slide 13: Process Flow (SmartArt-style)");
    
    // Create process flow using shapes
    let step1 = Shape::new(ShapeType::RoundedRectangle, 300000, 2000000, 1400000, 800000)
        .with_fill(ShapeFill::new("4472C4"))
        .with_text("1. Research");
    let arrow1 = Shape::new(ShapeType::RightArrow, 1800000, 2200000, 400000, 400000)
        .with_fill(ShapeFill::new("A5A5A5"));
    let step2 = Shape::new(ShapeType::RoundedRectangle, 2300000, 2000000, 1400000, 800000)
        .with_fill(ShapeFill::new("ED7D31"))
        .with_text("2. Design");
    let arrow2 = Shape::new(ShapeType::RightArrow, 3800000, 2200000, 400000, 400000)
        .with_fill(ShapeFill::new("A5A5A5"));
    let step3 = Shape::new(ShapeType::RoundedRectangle, 4300000, 2000000, 1400000, 800000)
        .with_fill(ShapeFill::new("70AD47"))
        .with_text("3. Develop");
    let arrow3 = Shape::new(ShapeType::RightArrow, 5800000, 2200000, 400000, 400000)
        .with_fill(ShapeFill::new("A5A5A5"));
    let step4 = Shape::new(ShapeType::RoundedRectangle, 6300000, 2000000, 1400000, 800000)
        .with_fill(ShapeFill::new("5B9BD5"))
        .with_text("4. Deploy");
    
    slides.push(
        SlideContent::new("Development Process Flow")
            .add_shape(step1)
            .add_shape(arrow1)
            .add_shape(step2)
            .add_shape(arrow2)
            .add_shape(step3)
            .add_shape(arrow3)
            .add_shape(step4)
            .title_color("1F497D")
            .title_bold(true)
    );

    // =========================================================================
    // SLIDE 14: Organization Chart with Shapes (NEW)
    // =========================================================================
    println!("🔷 Slide 14: Organization Chart");
    
    // CEO at top
    let ceo = Shape::new(ShapeType::RoundedRectangle, 3500000, 1400000, 2000000, 600000)
        .with_fill(ShapeFill::new("1F4E79"))
        .with_text("CEO");
    
    // Vertical line from CEO
    let line1 = Shape::new(ShapeType::Rectangle, 4450000, 2000000, 100000, 400000)
        .with_fill(ShapeFill::new("A5A5A5"));
    
    // Horizontal connector
    let hline = Shape::new(ShapeType::Rectangle, 1950000, 2400000, 5100000, 50000)
        .with_fill(ShapeFill::new("A5A5A5"));
    
    // CTO, CFO, COO
    let cto = Shape::new(ShapeType::RoundedRectangle, 1000000, 2600000, 1800000, 500000)
        .with_fill(ShapeFill::new("2E75B6"))
        .with_text("CTO");
    let cfo = Shape::new(ShapeType::RoundedRectangle, 3600000, 2600000, 1800000, 500000)
        .with_fill(ShapeFill::new("2E75B6"))
        .with_text("CFO");
    let coo = Shape::new(ShapeType::RoundedRectangle, 6200000, 2600000, 1800000, 500000)
        .with_fill(ShapeFill::new("2E75B6"))
        .with_text("COO");
    
    // Vertical lines to departments
    let vline1 = Shape::new(ShapeType::Rectangle, 1850000, 2450000, 50000, 150000)
        .with_fill(ShapeFill::new("A5A5A5"));
    let vline2 = Shape::new(ShapeType::Rectangle, 4450000, 2450000, 50000, 150000)
        .with_fill(ShapeFill::new("A5A5A5"));
    let vline3 = Shape::new(ShapeType::Rectangle, 7050000, 2450000, 50000, 150000)
        .with_fill(ShapeFill::new("A5A5A5"));
    
    // Teams under CTO
    let eng = Shape::new(ShapeType::Rectangle, 500000, 3300000, 1200000, 400000)
        .with_fill(ShapeFill::new("BDD7EE"))
        .with_text("Engineering");
    let product = Shape::new(ShapeType::Rectangle, 1800000, 3300000, 1200000, 400000)
        .with_fill(ShapeFill::new("BDD7EE"))
        .with_text("Product");
    
    slides.push(
        SlideContent::new("Organization Structure")
            .add_shape(ceo)
            .add_shape(line1)
            .add_shape(hline)
            .add_shape(cto)
            .add_shape(cfo)
            .add_shape(coo)
            .add_shape(vline1)
            .add_shape(vline2)
            .add_shape(vline3)
            .add_shape(eng)
            .add_shape(product)
            .title_color("1F4E79")
            .title_bold(true)
    );

    // =========================================================================
    // SLIDE 15: PDCA Cycle Diagram (NEW)
    // =========================================================================
    println!("🔷 Slide 15: PDCA Cycle Diagram");
    
    // Four quadrants for PDCA
    let plan = Shape::new(ShapeType::RoundedRectangle, 1500000, 1600000, 2500000, 1500000)
        .with_fill(ShapeFill::new("4472C4"))
        .with_text("PLAN\n\nDefine goals\nand strategy");
    let do_box = Shape::new(ShapeType::RoundedRectangle, 4500000, 1600000, 2500000, 1500000)
        .with_fill(ShapeFill::new("ED7D31"))
        .with_text("DO\n\nImplement\nthe plan");
    let check = Shape::new(ShapeType::RoundedRectangle, 4500000, 3300000, 2500000, 1500000)
        .with_fill(ShapeFill::new("70AD47"))
        .with_text("CHECK\n\nMeasure\nresults");
    let act = Shape::new(ShapeType::RoundedRectangle, 1500000, 3300000, 2500000, 1500000)
        .with_fill(ShapeFill::new("FFC000"))
        .with_text("ACT\n\nAdjust and\nimprove");
    
    // Arrows between quadrants
    let arr1 = Shape::new(ShapeType::RightArrow, 4100000, 2100000, 300000, 300000)
        .with_fill(ShapeFill::new("A5A5A5"));
    let arr2 = Shape::new(ShapeType::DownArrow, 5600000, 3200000, 300000, 200000)
        .with_fill(ShapeFill::new("A5A5A5"));
    let arr3 = Shape::new(ShapeType::LeftArrow, 4100000, 3800000, 300000, 300000)
        .with_fill(ShapeFill::new("A5A5A5"));
    let arr4 = Shape::new(ShapeType::UpArrow, 2600000, 3200000, 300000, 200000)
        .with_fill(ShapeFill::new("A5A5A5"));
    
    slides.push(
        SlideContent::new("PDCA Continuous Improvement Cycle")
            .add_shape(plan)
            .add_shape(do_box)
            .add_shape(check)
            .add_shape(act)
            .add_shape(arr1)
            .add_shape(arr2)
            .add_shape(arr3)
            .add_shape(arr4)
            .title_color("1F497D")
            .title_bold(true)
    );

    // =========================================================================
    // SLIDE 16: Pyramid Diagram (Maslow's Hierarchy) (NEW)
    // =========================================================================
    println!("🔷 Slide 16: Pyramid Diagram");
    
    // Build pyramid from bottom to top
    let level5 = Shape::new(ShapeType::Trapezoid, 500000, 4000000, 8000000, 600000)
        .with_fill(ShapeFill::new("C00000"))
        .with_text("Physiological Needs - Food, Water, Shelter");
    let level4 = Shape::new(ShapeType::Trapezoid, 1000000, 3400000, 7000000, 600000)
        .with_fill(ShapeFill::new("ED7D31"))
        .with_text("Safety Needs - Security, Stability");
    let level3 = Shape::new(ShapeType::Trapezoid, 1500000, 2800000, 6000000, 600000)
        .with_fill(ShapeFill::new("FFC000"))
        .with_text("Love & Belonging - Relationships");
    let level2 = Shape::new(ShapeType::Trapezoid, 2000000, 2200000, 5000000, 600000)
        .with_fill(ShapeFill::new("70AD47"))
        .with_text("Esteem - Achievement, Respect");
    let level1 = Shape::new(ShapeType::Triangle, 2500000, 1500000, 4000000, 700000)
        .with_fill(ShapeFill::new("4472C4"))
        .with_text("Self-Actualization");
    
    slides.push(
        SlideContent::new("Maslow's Hierarchy of Needs")
            .add_shape(level5)
            .add_shape(level4)
            .add_shape(level3)
            .add_shape(level2)
            .add_shape(level1)
            .title_color("1F497D")
            .title_bold(true)
    );

    // =========================================================================
    // SLIDE 17: Venn Diagram (NEW)
    // =========================================================================
    println!("🔷 Slide 17: Venn Diagram");
    
    // Three overlapping circles
    let circle1 = Shape::new(ShapeType::Ellipse, 1500000, 1800000, 3000000, 3000000)
        .with_fill(ShapeFill::new("4472C4"))
        .with_text("Skills");
    let circle2 = Shape::new(ShapeType::Ellipse, 3500000, 1800000, 3000000, 3000000)
        .with_fill(ShapeFill::new("ED7D31"))
        .with_text("Passion");
    let circle3 = Shape::new(ShapeType::Ellipse, 2500000, 3200000, 3000000, 3000000)
        .with_fill(ShapeFill::new("70AD47"))
        .with_text("Market Need");
    
    // Center label
    let center = Shape::new(ShapeType::Ellipse, 3200000, 2800000, 1600000, 800000)
        .with_fill(ShapeFill::new("FFFFFF"))
        .with_text("IKIGAI");
    
    slides.push(
        SlideContent::new("Finding Your Ikigai - Venn Diagram")
            .add_shape(circle1)
            .add_shape(circle2)
            .add_shape(circle3)
            .add_shape(center)
            .title_color("1F497D")
            .title_bold(true)
    );

    // =========================================================================
    // SLIDE 18: Timeline/Roadmap (NEW)
    // =========================================================================
    println!("📊 Slide 18: Project Timeline");
    
    let timeline_table = TableBuilder::new(vec![1500000, 1500000, 1500000, 1500000, 1500000])
        .add_row(TableRow::new(vec![
            TableCell::new("Q1 2024").bold().background_color("4472C4").text_color("FFFFFF"),
            TableCell::new("Q2 2024").bold().background_color("4472C4").text_color("FFFFFF"),
            TableCell::new("Q3 2024").bold().background_color("4472C4").text_color("FFFFFF"),
            TableCell::new("Q4 2024").bold().background_color("4472C4").text_color("FFFFFF"),
            TableCell::new("Q1 2025").bold().background_color("4472C4").text_color("FFFFFF"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Research\n& Planning").background_color("BDD7EE").text_color("1F497D"),
            TableCell::new("Design\nPhase").background_color("BDD7EE").text_color("1F497D"),
            TableCell::new("Development\nSprint 1-3").background_color("C6EFCE").text_color("006100"),
            TableCell::new("Testing\n& QA").background_color("FCE4D6").text_color("C65911"),
            TableCell::new("Launch\n& Support").background_color("E2EFDA").text_color("375623"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("✓ Complete").bold().text_color("2E7D32"),
            TableCell::new("✓ Complete").bold().text_color("2E7D32"),
            TableCell::new("In Progress").text_color("ED7D31"),
            TableCell::new("Planned").text_color("7F7F7F"),
            TableCell::new("Planned").text_color("7F7F7F"),
        ]))
        .position(300000, 2000000)
        .build();
    
    slides.push(
        SlideContent::new("Project Roadmap 2024-2025")
            .table(timeline_table)
            .title_color("1F497D")
            .title_bold(true)
    );

    // =========================================================================
    // SLIDE 19: Dashboard Summary (NEW - using Dimension API)
    // =========================================================================
    println!("🔷 Slide 19: Dashboard with KPIs (Dimension API)");
    
    // KPI boxes using ratio-based positioning — automatically adapts to any slide size
    let kpi1 = Shape::from_dimensions(ShapeType::RoundedRectangle,
        Dimension::percent(3.0), Dimension::percent(23.0),
        Dimension::percent(22.0), Dimension::percent(18.0),
    ).with_fill(ShapeFill::new("4472C4")).with_text("Revenue\n\n$2.14M\n+15% YoY");

    let kpi2 = Shape::from_dimensions(ShapeType::RoundedRectangle,
        Dimension::percent(27.0), Dimension::percent(23.0),
        Dimension::percent(22.0), Dimension::percent(18.0),
    ).with_fill(ShapeFill::new("70AD47")).with_text("Customers\n\n12,450\n+22% YoY");

    let kpi3 = Shape::from_dimensions(ShapeType::RoundedRectangle,
        Dimension::percent(51.0), Dimension::percent(23.0),
        Dimension::percent(22.0), Dimension::percent(18.0),
    ).with_fill(ShapeFill::new("ED7D31")).with_text("NPS Score\n\n72\n+8 pts");

    let kpi4 = Shape::from_dimensions(ShapeType::RoundedRectangle,
        Dimension::percent(75.0), Dimension::percent(23.0),
        Dimension::percent(22.0), Dimension::percent(18.0),
    ).with_fill(ShapeFill::new("5B9BD5")).with_text("Retention\n\n94%\n+3% YoY");
    
    // Status indicators using mixed units: percent for X, inches for size
    let status1 = Shape::new(ShapeType::Ellipse, 0, 0, 0, 0)
        .at(Dimension::percent(14.0), Dimension::percent(42.0))
        .with_dimensions(Dimension::Inches(0.3), Dimension::Inches(0.3))
        .with_fill(ShapeFill::new("70AD47"));
    let status2 = Shape::new(ShapeType::Ellipse, 0, 0, 0, 0)
        .at(Dimension::percent(38.0), Dimension::percent(42.0))
        .with_dimensions(Dimension::Inches(0.3), Dimension::Inches(0.3))
        .with_fill(ShapeFill::new("70AD47"));
    let status3 = Shape::new(ShapeType::Ellipse, 0, 0, 0, 0)
        .at(Dimension::percent(62.0), Dimension::percent(42.0))
        .with_dimensions(Dimension::Inches(0.3), Dimension::Inches(0.3))
        .with_fill(ShapeFill::new("FFC000"));
    let status4 = Shape::new(ShapeType::Ellipse, 0, 0, 0, 0)
        .at(Dimension::percent(86.0), Dimension::percent(42.0))
        .with_dimensions(Dimension::Inches(0.3), Dimension::Inches(0.3))
        .with_fill(ShapeFill::new("70AD47"));
    
    slides.push(
        SlideContent::new("Executive Dashboard - Q1 2024")
            .add_shape(kpi1)
            .add_shape(kpi2)
            .add_shape(kpi3)
            .add_shape(kpi4)
            .add_shape(status1)
            .add_shape(status2)
            .add_shape(status3)
            .add_shape(status4)
            .title_color("1F497D")
            .title_bold(true)
    );

    // =========================================================================
    // SLIDE 20: Summary Slide (NEW)
    // =========================================================================
    println!("📝 Slide 20: Summary with Speaker Notes");
    
    slides.push(
        SlideContent::new("Summary & Next Steps")
            .layout(SlideLayout::TitleAndContent)
            .title_color("1F497D")
            .title_bold(true)
            .add_bullet("Completed: Research, Design, Initial Development")
            .add_bullet("In Progress: Sprint 3 Development")
            .add_bullet("Next: QA Testing Phase (Q4 2024)")
            .add_bullet("Launch Target: Q1 2025")
            .add_bullet("Key Risks: Resource constraints, Timeline pressure")
            .content_size(24)
            .notes("Speaker Notes:\n\n1. Emphasize the progress made\n2. Highlight key achievements\n3. Address any concerns about timeline\n4. Open for Q&A")
    );

    // =========================================================================
    // SLIDE 21: Bullet Styles (NEW v0.2.1)
    // =========================================================================
    println!("🔢 Slide 21: Bullet Styles (NEW)");
    
    // Numbered list
    slides.push(
        SlideContent::new("Bullet Styles - Numbered List")
            .layout(SlideLayout::TitleAndContent)
            .title_color("1F497D")
            .title_bold(true)
            .with_bullet_style(BulletStyle::Number)
            .add_bullet("First numbered item")
            .add_bullet("Second numbered item")
            .add_bullet("Third numbered item")
            .add_bullet("Fourth numbered item")
            .content_size(28)
    );

    // =========================================================================
    // SLIDE 22: Lettered Lists (NEW v0.2.1)
    // =========================================================================
    println!("🔤 Slide 22: Lettered Lists (NEW)");
    
    slides.push(
        SlideContent::new("Bullet Styles - Lettered Lists")
            .layout(SlideLayout::TitleAndContent)
            .title_color("1F497D")
            .title_bold(true)
            .add_lettered("Option A - First choice")
            .add_lettered("Option B - Second choice")
            .add_lettered("Option C - Third choice")
            .add_lettered("Option D - Fourth choice")
            .content_size(28)
    );

    // =========================================================================
    // SLIDE 23: Roman Numerals (NEW v0.2.1)
    // =========================================================================
    println!("🏛️ Slide 23: Roman Numerals (NEW)");
    
    slides.push(
        SlideContent::new("Bullet Styles - Roman Numerals")
            .layout(SlideLayout::TitleAndContent)
            .title_color("1F497D")
            .title_bold(true)
            .with_bullet_style(BulletStyle::RomanUpper)
            .add_bullet("Chapter I - Introduction")
            .add_bullet("Chapter II - Background")
            .add_bullet("Chapter III - Methodology")
            .add_bullet("Chapter IV - Results")
            .add_bullet("Chapter V - Conclusion")
            .content_size(28)
    );

    // =========================================================================
    // SLIDE 24: Custom Bullets (NEW v0.2.1)
    // =========================================================================
    println!("⭐ Slide 24: Custom Bullets (NEW)");
    
    slides.push(
        SlideContent::new("Bullet Styles - Custom Characters")
            .layout(SlideLayout::TitleAndContent)
            .title_color("1F497D")
            .title_bold(true)
            .add_styled_bullet("Star bullet point", BulletStyle::Custom('★'))
            .add_styled_bullet("Arrow bullet point", BulletStyle::Custom('→'))
            .add_styled_bullet("Check bullet point", BulletStyle::Custom('✓'))
            .add_styled_bullet("Diamond bullet point", BulletStyle::Custom('◆'))
            .add_styled_bullet("Heart bullet point", BulletStyle::Custom('♥'))
            .content_size(28)
    );

    // =========================================================================
    // SLIDE 25: Sub-bullets / Hierarchy (NEW v0.2.1)
    // =========================================================================
    println!("📊 Slide 25: Sub-bullets Hierarchy (NEW)");
    
    slides.push(
        SlideContent::new("Bullet Styles - Hierarchical Lists")
            .layout(SlideLayout::TitleAndContent)
            .title_color("1F497D")
            .title_bold(true)
            .add_bullet("Main Topic 1")
            .add_sub_bullet("Supporting detail A")
            .add_sub_bullet("Supporting detail B")
            .add_bullet("Main Topic 2")
            .add_sub_bullet("Supporting detail C")
            .add_sub_bullet("Supporting detail D")
            .add_bullet("Main Topic 3")
            .content_size(24)
    );

    // =========================================================================
    // SLIDE 26: Text Enhancements (NEW v0.2.1)
    // =========================================================================
    println!("✏️ Slide 26: Text Enhancements (NEW)");
    
    // Use BulletPoint with formatting
    let strikethrough_bullet = BulletPoint::new("Strikethrough: This text is crossed out").strikethrough();
    let highlight_bullet = BulletPoint::new("Highlight: Yellow background for emphasis").highlight("FFFF00");
    let subscript_bullet = BulletPoint::new("Subscript: H₂O - for chemical formulas").subscript();
    let superscript_bullet = BulletPoint::new("Superscript: x² - for math expressions").superscript();
    let bold_colored = BulletPoint::new("Combined: Bold + Red color").bold().color("FF0000");
    
    let mut text_enhancements_slide = SlideContent::new("Text Enhancements - New Formatting")
        .layout(SlideLayout::TitleAndContent)
        .title_color("1F497D")
        .title_bold(true)
        .content_size(24);
    text_enhancements_slide.bullets.push(strikethrough_bullet);
    text_enhancements_slide.bullets.push(highlight_bullet);
    text_enhancements_slide.bullets.push(subscript_bullet);
    text_enhancements_slide.bullets.push(superscript_bullet);
    text_enhancements_slide.bullets.push(bold_colored);
    
    slides.push(text_enhancements_slide);

    // =========================================================================
    // SLIDE 27: Font Size Presets (NEW v0.2.1)
    // =========================================================================
    println!("🔤 Slide 27: Font Size Presets (NEW)");
    
    // Demonstrate different font sizes per bullet
    let large_bullet = BulletPoint::new(&format!("LARGE: {}pt - Extra large text", font_sizes::LARGE)).font_size(font_sizes::LARGE);
    let heading_bullet = BulletPoint::new(&format!("HEADING: {}pt - Section headers", font_sizes::HEADING)).font_size(font_sizes::HEADING);
    let body_bullet = BulletPoint::new(&format!("BODY: {}pt - Regular content", font_sizes::BODY)).font_size(font_sizes::BODY);
    let small_bullet = BulletPoint::new(&format!("SMALL: {}pt - Smaller text", font_sizes::SMALL)).font_size(font_sizes::SMALL);
    let caption_bullet = BulletPoint::new(&format!("CAPTION: {}pt - Captions and notes", font_sizes::CAPTION)).font_size(font_sizes::CAPTION);
    
    let mut font_size_slide = SlideContent::new("Font Size Presets - Each line different size")
        .layout(SlideLayout::TitleAndContent)
        .title_color("1F497D")
        .title_bold(true)
        .title_size(font_sizes::TITLE);
    font_size_slide.bullets.push(large_bullet);
    font_size_slide.bullets.push(heading_bullet);
    font_size_slide.bullets.push(body_bullet);
    font_size_slide.bullets.push(small_bullet);
    font_size_slide.bullets.push(caption_bullet);
    
    slides.push(font_size_slide);

    // =========================================================================
    // SLIDE 28: Theme Colors (NEW v0.2.1)
    // =========================================================================
    println!("🎨 Slide 28: Theme Colors (NEW)");
    
    // Create shapes with theme colors
    let corporate_shape = Shape::new(ShapeType::Rectangle, 500000, 1600000, 1800000, 800000)
        .with_fill(ShapeFill::new(themes::CORPORATE.primary))
        .with_text("Corporate");
    
    let modern_shape = Shape::new(ShapeType::Rectangle, 2500000, 1600000, 1800000, 800000)
        .with_fill(ShapeFill::new(themes::MODERN.primary))
        .with_text("Modern");
    
    let vibrant_shape = Shape::new(ShapeType::Rectangle, 4500000, 1600000, 1800000, 800000)
        .with_fill(ShapeFill::new(themes::VIBRANT.primary))
        .with_text("Vibrant");
    
    let dark_shape = Shape::new(ShapeType::Rectangle, 6500000, 1600000, 1800000, 800000)
        .with_fill(ShapeFill::new(themes::DARK.primary))
        .with_text("Dark");
    
    let nature_shape = Shape::new(ShapeType::Rectangle, 500000, 2700000, 1800000, 800000)
        .with_fill(ShapeFill::new(themes::NATURE.primary))
        .with_text("Nature");
    
    let tech_shape = Shape::new(ShapeType::Rectangle, 2500000, 2700000, 1800000, 800000)
        .with_fill(ShapeFill::new(themes::TECH.primary))
        .with_text("Tech");
    
    let carbon_shape = Shape::new(ShapeType::Rectangle, 4500000, 2700000, 1800000, 800000)
        .with_fill(ShapeFill::new(themes::CARBON.primary))
        .with_text("Carbon");
    
    slides.push(
        SlideContent::new("Theme Color Palettes")
            .layout(SlideLayout::TitleAndContent)
            .title_color("1F497D")
            .title_bold(true)
            .add_shape(corporate_shape)
            .add_shape(modern_shape)
            .add_shape(vibrant_shape)
            .add_shape(dark_shape)
            .add_shape(nature_shape)
            .add_shape(tech_shape)
            .add_shape(carbon_shape)
    );

    // =========================================================================
    // SLIDE 29: Material & Carbon Design Colors (NEW v0.2.1)
    // =========================================================================
    println!("🌈 Slide 29: Material & Carbon Colors (NEW)");
    
    // Material Design colors
    let material_red = Shape::new(ShapeType::Rectangle, 500000, 1600000, 1200000, 600000)
        .with_fill(ShapeFill::new(colors::MATERIAL_RED))
        .with_text("M-Red");
    
    let material_blue = Shape::new(ShapeType::Rectangle, 1900000, 1600000, 1200000, 600000)
        .with_fill(ShapeFill::new(colors::MATERIAL_BLUE))
        .with_text("M-Blue");
    
    let material_green = Shape::new(ShapeType::Rectangle, 3300000, 1600000, 1200000, 600000)
        .with_fill(ShapeFill::new(colors::MATERIAL_GREEN))
        .with_text("M-Green");
    
    let material_orange = Shape::new(ShapeType::Rectangle, 4700000, 1600000, 1200000, 600000)
        .with_fill(ShapeFill::new(colors::MATERIAL_ORANGE))
        .with_text("M-Orange");
    
    let material_purple = Shape::new(ShapeType::Rectangle, 6100000, 1600000, 1200000, 600000)
        .with_fill(ShapeFill::new(colors::MATERIAL_PURPLE))
        .with_text("M-Purple");
    
    // Carbon Design colors
    let carbon_blue = Shape::new(ShapeType::Rectangle, 500000, 2500000, 1200000, 600000)
        .with_fill(ShapeFill::new(colors::CARBON_BLUE_60))
        .with_text("C-Blue");
    
    let carbon_green = Shape::new(ShapeType::Rectangle, 1900000, 2500000, 1200000, 600000)
        .with_fill(ShapeFill::new(colors::CARBON_GREEN_50))
        .with_text("C-Green");
    
    let carbon_red = Shape::new(ShapeType::Rectangle, 3300000, 2500000, 1200000, 600000)
        .with_fill(ShapeFill::new(colors::CARBON_RED_60))
        .with_text("C-Red");
    
    let carbon_purple = Shape::new(ShapeType::Rectangle, 4700000, 2500000, 1200000, 600000)
        .with_fill(ShapeFill::new(colors::CARBON_PURPLE_60))
        .with_text("C-Purple");
    
    let carbon_gray = Shape::new(ShapeType::Rectangle, 6100000, 2500000, 1200000, 600000)
        .with_fill(ShapeFill::new(colors::CARBON_GRAY_100))
        .with_text("C-Gray");
    
    slides.push(
        SlideContent::new("Material & Carbon Design Colors")
            .layout(SlideLayout::TitleAndContent)
            .title_color("1F497D")
            .title_bold(true)
            .add_shape(material_red)
            .add_shape(material_blue)
            .add_shape(material_green)
            .add_shape(material_orange)
            .add_shape(material_purple)
            .add_shape(carbon_blue)
            .add_shape(carbon_green)
            .add_shape(carbon_red)
            .add_shape(carbon_purple)
            .add_shape(carbon_gray)
    );

    // =========================================================================
    // SLIDE 30: Image from Base64 (NEW v0.2.1)
    // =========================================================================
    println!("🖼️ Slide 30: Image from Base64 (NEW)");
    
    // 1x1 red PNG pixel in base64
    let _red_pixel_base64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==";
    
    // Create image from base64 (demonstrating the API)
    let _base64_image = ImageBuilder::from_base64(_red_pixel_base64, 914400, 914400, "PNG")
        .position(4000000, 2500000)
        .build();
    
    slides.push(
        SlideContent::new("Image Loading - New Methods")
            .layout(SlideLayout::TitleAndContent)
            .title_color("1F497D")
            .title_bold(true)
            .add_bullet("Image::new(path) - Load from file path")
            .add_bullet("Image::from_base64(data) - Load from base64 string")
            .add_bullet("Image::from_bytes(data) - Load from raw bytes")
            .add_bullet("ImageBuilder for fluent API configuration")
            .add_bullet("Built-in base64 decoder (no external deps)")
            .content_size(24)
    );

    // =========================================================================
    // SLIDE 31: Feature Summary (NEW v0.2.1)
    // =========================================================================
    println!("📋 Slide 31: v0.2.1 Feature Summary (NEW)");
    
    slides.push(
        SlideContent::new("New Features in v0.2.1")
            .layout(SlideLayout::TitleAndContent)
            .title_color("1F497D")
            .title_bold(true)
            .add_numbered("BulletStyle: Number, Letter, Roman, Custom")
            .add_numbered("TextFormat: Strikethrough, Highlight")
            .add_numbered("TextFormat: Subscript, Superscript")
            .add_numbered("Font size presets in prelude")
            .add_numbered("Image::from_base64 and from_bytes")
            .add_numbered("Theme color palettes (7 themes)")
            .add_numbered("Material & Carbon Design colors")
            .content_size(24)
    );

    // =========================================================================
    // SLIDE 32: Slide Show Settings - Comparison Table (REAL: embedded in presProps.xml)
    // =========================================================================
    println!("🎬 Slide 34: Slide Show Settings (Visual)");

    let show_speaker = SlideShowSettings::new().pen_color(PenColor::red());
    let show_kiosk = SlideShowSettings::kiosk();
    let _show_range = SlideShowSettings::new()
        .show_type(ShowType::Browsed)
        .slide_range(SlideRange::Range { start: 1, end: 10 })
        .without_animation(true);
    let _speaker_xml = show_speaker.to_xml();
    let _kiosk_xml = show_kiosk.to_xml();

    let show_table = TableBuilder::new(vec![2000000, 2000000, 2000000, 2000000])
        .add_row(TableRow::new(vec![
            TableCell::new("Setting").bold().background_color("1F4E79").text_color("FFFFFF"),
            TableCell::new("Speaker").bold().background_color("4472C4").text_color("FFFFFF"),
            TableCell::new("Kiosk").bold().background_color("ED7D31").text_color("FFFFFF"),
            TableCell::new("Browsed").bold().background_color("70AD47").text_color("FFFFFF"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Loop").bold().background_color("D6E4F0"),
            TableCell::new("No"), TableCell::new("Yes").bold().text_color("2E7D32"),
            TableCell::new("No"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Narration").bold().background_color("D6E4F0"),
            TableCell::new("Yes"), TableCell::new("No").text_color("C62828"),
            TableCell::new("Yes"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Animation").bold().background_color("D6E4F0"),
            TableCell::new("Yes"), TableCell::new("Yes"),
            TableCell::new("No").text_color("C62828"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Timings").bold().background_color("D6E4F0"),
            TableCell::new("Yes"), TableCell::new("Auto"),
            TableCell::new("Yes"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Slide Range").bold().background_color("D6E4F0"),
            TableCell::new("All"), TableCell::new("All"),
            TableCell::new("1-10"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Pen Color").bold().background_color("D6E4F0"),
            TableCell::new("Red").text_color("FF0000"),
            TableCell::new("Red").text_color("FF0000"),
            TableCell::new("Red").text_color("FF0000"),
        ]))
        .position(300000, 1600000)
        .build();

    // Mode icons
    let icon_speaker = Shape::new(ShapeType::RoundedRectangle, 300000, 4200000, 2500000, 600000)
        .with_fill(ShapeFill::new("4472C4"))
        .with_text("Speaker: Full control");
    let icon_kiosk = Shape::new(ShapeType::RoundedRectangle, 3100000, 4200000, 2500000, 600000)
        .with_fill(ShapeFill::new("ED7D31"))
        .with_text("Kiosk: Auto-loop");
    let icon_browsed = Shape::new(ShapeType::RoundedRectangle, 5900000, 4200000, 2500000, 600000)
        .with_fill(ShapeFill::new("70AD47"))
        .with_text("Browsed: Scrollbar");

    slides.push(
        SlideContent::new("Slide Show Settings - Mode Comparison")
            .table(show_table)
            .add_shape(icon_speaker)
            .add_shape(icon_kiosk)
            .add_shape(icon_browsed)
            .title_color("1F497D")
            .title_bold(true)
    );

    // =========================================================================
    // SLIDE 35: Print Settings - Visual Handout Grid
    // =========================================================================
    println!("🖨️  Slide 35: Print Settings & Handouts (Visual)");

    let print = PrintSettings::new()
        .print_what(PrintWhat::Handouts)
        .color_mode(PrintColorMode::Grayscale)
        .handout_layout(GenHandoutLayout::SlidesPerPage6)
        .frame_slides(true)
        .header("Q1 2025 Strategy Review")
        .footer("Confidential - Internal Use Only")
        .print_date(true)
        .print_page_numbers(true);
    let _prnpr_xml = print.to_prnpr_xml();
    let _handout_xml = print.to_handout_master_xml();

    // Visual: 6-slide handout grid (2 cols x 3 rows)
    let hdr = Shape::new(ShapeType::Rectangle, 300000, 1500000, 4200000, 300000)
        .with_fill(ShapeFill::new("E7E6E6"))
        .with_text("Q1 2025 Strategy Review");
    // Row 1
    let s1 = Shape::new(ShapeType::Rectangle, 400000, 2000000, 1800000, 1100000)
        .with_line(ShapeLine::new("999999", 12700))
        .with_text("Slide 1");
    let s2 = Shape::new(ShapeType::Rectangle, 2500000, 2000000, 1800000, 1100000)
        .with_line(ShapeLine::new("999999", 12700))
        .with_text("Slide 2");
    // Row 2
    let s3 = Shape::new(ShapeType::Rectangle, 400000, 3200000, 1800000, 1100000)
        .with_line(ShapeLine::new("999999", 12700))
        .with_text("Slide 3");
    let s4 = Shape::new(ShapeType::Rectangle, 2500000, 3200000, 1800000, 1100000)
        .with_line(ShapeLine::new("999999", 12700))
        .with_text("Slide 4");
    // Row 3
    let s5 = Shape::new(ShapeType::Rectangle, 400000, 4400000, 1800000, 1100000)
        .with_line(ShapeLine::new("999999", 12700))
        .with_text("Slide 5");
    let s6 = Shape::new(ShapeType::Rectangle, 2500000, 4400000, 1800000, 1100000)
        .with_line(ShapeLine::new("999999", 12700))
        .with_text("Slide 6");
    let ftr = Shape::new(ShapeType::Rectangle, 300000, 5600000, 4200000, 300000)
        .with_fill(ShapeFill::new("E7E6E6"))
        .with_text("Confidential - Internal Use Only");

    // Settings summary table on the right
    let print_table = TableBuilder::new(vec![1800000, 2000000])
        .add_row(TableRow::new(vec![
            TableCell::new("Print Settings").bold().background_color("1F4E79").text_color("FFFFFF"),
            TableCell::new("").background_color("1F4E79"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Print What").bold().background_color("D6E4F0"),
            TableCell::new("Handouts"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Color Mode").bold().background_color("D6E4F0"),
            TableCell::new("Grayscale"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Layout").bold().background_color("D6E4F0"),
            TableCell::new("6 slides/page"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Frame Slides").bold().background_color("D6E4F0"),
            TableCell::new("Yes"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Date").bold().background_color("D6E4F0"),
            TableCell::new("Yes"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Page Numbers").bold().background_color("D6E4F0"),
            TableCell::new("Yes"),
        ]))
        .position(5000000, 1800000)
        .build();

    slides.push(
        SlideContent::new("Print Handout - 6 Slides Per Page")
            .table(print_table)
            .add_shape(hdr)
            .add_shape(s1).add_shape(s2)
            .add_shape(s3).add_shape(s4)
            .add_shape(s5).add_shape(s6)
            .add_shape(ftr)
            .title_color("1F497D")
            .title_bold(true)
    );

    // =========================================================================
    // SLIDE 36: Advanced Table Merging - Actual Merged Table
    // =========================================================================
    println!("📊 Slide 36: Advanced Table Merging (Visual)");

    // Use TableMergeMap to compute states, then build a visual table
    let mut merge_map = TableMergeMap::new(5, 4);
    merge_map.merge_cells(0, 0, 1, 4).unwrap(); // Title row spans all 4 cols
    merge_map.merge_cells(1, 0, 2, 1).unwrap(); // "Products" spans 2 rows
    merge_map.merge_cells(3, 0, 2, 1).unwrap(); // "Services" spans 2 rows

    // Show merge state labels
    let state_00 = merge_map.cell_state(0, 0); // Anchor gridSpan=4
    let state_01 = merge_map.cell_state(0, 1); // HMerge
    let state_10 = merge_map.cell_state(1, 0); // Anchor rowSpan=2
    let state_20 = merge_map.cell_state(2, 0); // VMerge
    println!("   ├── (0,0): {}", state_00.to_xml_attrs().trim());
    println!("   ├── (0,1): {}", state_01.to_xml_attrs().trim());
    println!("   ├── (1,0): {}", state_10.to_xml_attrs().trim());
    println!("   └── (2,0): {}", state_20.to_xml_attrs().trim());

    // Build the table with REAL merge attributes (gridSpan, rowSpan, hMerge, vMerge)
    let merge_table = TableBuilder::new(vec![1500000, 2000000, 2000000, 2000000])
        .add_row(TableRow::new(vec![
            TableCell::new("Q1 2025 Revenue Report").bold().background_color("1F4E79").text_color("FFFFFF").grid_span(4),
            TableCell::new("").background_color("1F4E79").h_merge(),
            TableCell::new("").background_color("1F4E79").h_merge(),
            TableCell::new("").background_color("1F4E79").h_merge(),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Products").bold().background_color("BDD7EE").text_color("1F497D").row_span(2),
            TableCell::new("Hardware").background_color("E2EFDA"),
            TableCell::new("$450,000").text_color("2E7D32").align_right(),
            TableCell::new("+12%").bold().text_color("2E7D32").align_right(),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("").background_color("BDD7EE").v_merge(),
            TableCell::new("Software").background_color("E2EFDA"),
            TableCell::new("$680,000").text_color("2E7D32").align_right(),
            TableCell::new("+25%").bold().text_color("2E7D32").align_right(),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Services").bold().background_color("FCE4D6").text_color("C65911").row_span(2),
            TableCell::new("Consulting").background_color("FFF2CC"),
            TableCell::new("$320,000").text_color("2E7D32").align_right(),
            TableCell::new("+8%").bold().text_color("2E7D32").align_right(),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("").background_color("FCE4D6").v_merge(),
            TableCell::new("Support").background_color("FFF2CC"),
            TableCell::new("$190,000").text_color("2E7D32").align_right(),
            TableCell::new("+5%").bold().text_color("2E7D32").align_right(),
        ]))
        .position(300000, 1600000)
        .build();

    // Legend shapes
    let legend_anchor = Shape::new(ShapeType::RoundedRectangle, 300000, 4400000, 2000000, 400000)
        .with_fill(ShapeFill::new("4472C4"))
        .with_text("Anchor (gridSpan/rowSpan)");
    let legend_hmerge = Shape::new(ShapeType::RoundedRectangle, 2500000, 4400000, 2000000, 400000)
        .with_fill(ShapeFill::new("ED7D31"))
        .with_text("hMerge (col covered)");
    let legend_vmerge = Shape::new(ShapeType::RoundedRectangle, 4700000, 4400000, 2000000, 400000)
        .with_fill(ShapeFill::new("70AD47"))
        .with_text("vMerge (row covered)");
    let legend_normal = Shape::new(ShapeType::RoundedRectangle, 6900000, 4400000, 2000000, 400000)
        .with_fill(ShapeFill::new("A5A5A5"))
        .with_text("Normal (no merge)");

    slides.push(
        SlideContent::new("Advanced Table Merging - Merged Cells")
            .table(merge_table)
            .add_shape(legend_anchor)
            .add_shape(legend_hmerge)
            .add_shape(legend_vmerge)
            .add_shape(legend_normal)
            .title_color("1F497D")
            .title_bold(true)
    );

    // =========================================================================
    // Build PresentationSettings with all advanced features
    // =========================================================================
    println!("\n⚙️  Building Presentation Settings...");

    // Slide show settings (embedded in presProps.xml as <p:showPr>)
    let show_settings = SlideShowSettings::new()
        .show_type(ShowType::Speaker)
        .pen_color(PenColor::red())
        .use_timings(true);
    println!("   ├── Slide Show: Speaker mode, red pen, timings enabled");
    println!("   │   └── XML: {} bytes", show_settings.to_xml().len());

    // Print settings (embedded in presProps.xml as <p:prnPr>)
    let print_settings = PrintSettings::new()
        .print_what(PrintWhat::Handouts)
        .color_mode(PrintColorMode::Grayscale)
        .handout_layout(GenHandoutLayout::SlidesPerPage6)
        .frame_slides(true)
        .header("Q1 2025 Strategy Review")
        .footer("Confidential - Internal Use Only")
        .print_date(true)
        .print_page_numbers(true);
    println!("   └── Print: Handouts, 6/page, grayscale, framed");
    println!("       └── XML: {} bytes", print_settings.to_prnpr_xml().len());

    let pres_settings = PresentationSettings::new()
        .slide_show(show_settings)
        .print(print_settings);
    println!("   All settings configured → presProps.xml");

    // =========================================================================
    // Generate PPTX with real integrated features
    // =========================================================================
    println!("\n📦 Generating PPTX with integrated features...");
    let pptx_data = create_pptx_with_settings(
        "PPTX-RS Element Showcase",
        slides.clone(),
        Some(pres_settings),
    )?;
    fs::write("comprehensive_demo.pptx", &pptx_data)?;
    println!("   ✓ Created comprehensive_demo.pptx ({} slides, {} bytes)", 
             slides.len(), pptx_data.len());

    // =========================================================================
    // Package Analysis - Demonstrate Reading
    // =========================================================================
    println!("\n📖 Package Analysis (Read Capability):");
    
    let package = Package::open("comprehensive_demo.pptx")?;
    let paths = package.part_paths();
    
    let slide_count = paths.iter()
        .filter(|p| p.starts_with("ppt/slides/slide") && p.ends_with(".xml"))
        .count();
    
    println!("   ├── Total parts: {}", package.part_count());
    println!("   ├── Slides: {}", slide_count);
    println!("   └── Package opened and analyzed successfully");

    // =========================================================================
    // NEW: Parts API Demonstration
    // =========================================================================
    println!("\n🧩 Parts API Demonstration:");
    
    // SlideLayoutPart - 11 layout types
    println!("   ┌── SlideLayoutPart (11 layout types):");
    let layouts = [
        LayoutType::Title,
        LayoutType::TitleAndContent,
        LayoutType::SectionHeader,
        LayoutType::TwoContent,
        LayoutType::Comparison,
        LayoutType::TitleOnly,
        LayoutType::Blank,
        LayoutType::ContentWithCaption,
        LayoutType::PictureWithCaption,
        LayoutType::TitleAndVerticalText,
        LayoutType::VerticalTitleAndText,
    ];
    for (i, layout_type) in layouts.iter().enumerate() {
        let layout = SlideLayoutPart::new(i + 1, *layout_type);
        if i < 3 {
            println!("   │   ├── {}: {} ({})", i + 1, layout_type.name(), layout.path());
        }
    }
    println!("   │   └── ... and {} more layout types", layouts.len() - 3);
    
    // SlideMasterPart
    println!("   ├── SlideMasterPart:");
    let mut master = SlideMasterPart::new(1);
    master.set_name("Custom Master");
    master.add_layout_rel_id("rId2");
    master.add_layout_rel_id("rId3");
    println!("   │   ├── Name: {}", master.name());
    println!("   │   ├── Path: {}", master.path());
    println!("   │   └── Layouts: {} linked", master.layout_rel_ids().len());
    
    // ThemePart - Colors and Fonts
    println!("   ├── ThemePart (colors & fonts):");
    let mut theme = ThemePart::new(1);
    theme.set_name("Corporate Theme");
    theme.set_major_font("Arial");
    theme.set_minor_font("Calibri");
    theme.set_color("accent1", "FF5733");
    theme.set_color("accent2", "33FF57");
    let theme_xml = theme.to_xml()?;
    println!("   │   ├── Name: {}", theme.name());
    println!("   │   ├── Major Font: Arial");
    println!("   │   ├── Minor Font: Calibri");
    println!("   │   └── XML size: {} bytes", theme_xml.len());
    
    // NotesSlidePart - Speaker notes
    println!("   ├── NotesSlidePart (speaker notes):");
    let notes = NotesSlidePart::with_text(1, "Remember to:\n- Introduce yourself\n- Explain the agenda\n- Ask for questions");
    let notes_xml = notes.to_xml()?;
    println!("   │   ├── Path: {}", notes.path());
    println!("   │   ├── Text: \"{}...\"", &notes.notes_text()[..20.min(notes.notes_text().len())]);
    println!("   │   └── XML size: {} bytes", notes_xml.len());
    
    // AppPropertiesPart - Application metadata
    println!("   ├── AppPropertiesPart (metadata):");
    let mut app_props = AppPropertiesPart::new();
    app_props.set_company("Acme Corporation");
    app_props.set_slides(slides.len() as u32);
    let app_xml = app_props.to_xml()?;
    println!("   │   ├── Company: Acme Corporation");
    println!("   │   ├── Slides: {}", slides.len());
    println!("   │   └── XML size: {} bytes", app_xml.len());
    
    // MediaPart - Video/Audio formats
    println!("   ├── MediaPart (10 media formats):");
    println!("   │   ├── Video: mp4, webm, avi, wmv, mov");
    println!("   │   ├── Audio: mp3, wav, wma, m4a, ogg");
    let sample_media = MediaPart::new(1, MediaFormat::Mp4, vec![0; 100]);
    println!("   │   └── Sample: {} ({})", sample_media.path(), sample_media.format().mime_type());
    
    // TablePart - Table with formatting
    println!("   ├── TablePart (cell formatting):");
    let table_part = TablePart::new()
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("Header 1").bold().background("4472C4"),
            TableCellPart::new("Header 2").bold().background("4472C4"),
        ]))
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("Data 1").color("333333"),
            TableCellPart::new("Data 2").italic(),
        ]))
        .position(EMU_PER_INCH, EMU_PER_INCH * 2)
        .size(EMU_PER_INCH * 6, EMU_PER_INCH * 2);
    let table_xml = table_part.to_slide_xml(10);
    println!("   │   ├── Rows: {}", table_part.rows.len());
    println!("   │   ├── Features: bold, italic, colors, backgrounds");
    println!("   │   └── XML size: {} bytes", table_xml.len());
    
    // ContentTypesPart
    println!("   └── ContentTypesPart:");
    let mut content_types = ContentTypesPart::new();
    content_types.add_presentation();
    content_types.add_slide(1);
    content_types.add_slide_layout(1);
    content_types.add_slide_master(1);
    content_types.add_theme(1);
    content_types.add_core_properties();
    content_types.add_app_properties();
    let ct_xml = content_types.to_xml()?;
    println!("       ├── Path: {}", content_types.path());
    println!("       └── XML size: {} bytes", ct_xml.len());

    // =========================================================================
    // NEW: Elements API Demonstration
    // =========================================================================
    println!("\n🎨 Elements API Demonstration:");
    
    // Color types
    println!("   ┌── Color Types:");
    let rgb = RgbColor::new(255, 87, 51);
    let rgb_hex = RgbColor::from_hex("#4472C4").unwrap();
    let scheme = SchemeColor::Accent1;
    let color = Color::rgb(100, 149, 237);
    println!("   │   ├── RgbColor::new(255, 87, 51) → {}", rgb.to_hex());
    println!("   │   ├── RgbColor::from_hex(\"#4472C4\") → {}", rgb_hex.to_hex());
    println!("   │   ├── SchemeColor::Accent1 → {}", scheme.as_str());
    println!("   │   └── Color::rgb(100, 149, 237) → XML: {}", color.to_xml().chars().take(30).collect::<String>());
    
    // Position and Size
    println!("   ├── Position & Size (EMU units):");
    let pos = Position::from_inches(1.0, 2.0);
    let size = Size::from_inches(4.0, 3.0);
    println!("   │   ├── Position::from_inches(1.0, 2.0) → x={}, y={}", pos.x, pos.y);
    println!("   │   ├── Size::from_inches(4.0, 3.0) → w={}, h={}", size.width, size.height);
    println!("   │   └── EMU_PER_INCH = {}", EMU_PER_INCH);
    
    // Transform
    println!("   └── Transform (position + size + rotation):");
    let transform = Transform::from_inches(1.0, 1.5, 3.0, 2.0).with_rotation(45.0);
    let transform_xml = transform.to_xml();
    println!("       ├── Transform::from_inches(1.0, 1.5, 3.0, 2.0)");
    println!("       ├── .with_rotation(45.0)");
    println!("       └── XML: {}...", &transform_xml[..50.min(transform_xml.len())]);

    // =========================================================================
    // NEW: Advanced Features Demonstration
    // =========================================================================
    println!("\n🚀 Advanced Features Demonstration:");

    // -------------------------------------------------------------------------
    // Complex Table Examples
    // -------------------------------------------------------------------------
    println!("   ┌── Complex Table Examples:");
    
    // Example 1: Financial Report Table
    println!("   │   ┌── Financial Report Table (5x4 with formatting):");
    let financial_table = TablePart::new()
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("Q1 2024 Financial Summary")
                .col_span(4)
                .bold()
                .center()
                .background("1F4E79")
                .color("FFFFFF")
                .font_size(14)
                .font("Arial Black"),
        ]))
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("Category").bold().center().background("2E75B6").color("FFFFFF"),
            TableCellPart::new("Revenue").bold().center().background("2E75B6").color("FFFFFF"),
            TableCellPart::new("Expenses").bold().center().background("2E75B6").color("FFFFFF"),
            TableCellPart::new("Profit").bold().center().background("2E75B6").color("FFFFFF"),
        ]))
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("Product Sales").align(HorizontalAlign::Left),
            TableCellPart::new("$1,250,000").align(HorizontalAlign::Right).color("2E7D32"),
            TableCellPart::new("$450,000").align(HorizontalAlign::Right).color("C62828"),
            TableCellPart::new("$800,000").align(HorizontalAlign::Right).bold().color("2E7D32"),
        ]))
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("Services").align(HorizontalAlign::Left),
            TableCellPart::new("$890,000").align(HorizontalAlign::Right).color("2E7D32"),
            TableCellPart::new("$320,000").align(HorizontalAlign::Right).color("C62828"),
            TableCellPart::new("$570,000").align(HorizontalAlign::Right).bold().color("2E7D32"),
        ]))
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("Total").bold().background("E7E6E6"),
            TableCellPart::new("$2,140,000").bold().align(HorizontalAlign::Right).background("E7E6E6"),
            TableCellPart::new("$770,000").bold().align(HorizontalAlign::Right).background("E7E6E6"),
            TableCellPart::new("$1,370,000").bold().align(HorizontalAlign::Right).background("C6EFCE").color("006100"),
        ]))
        .position(EMU_PER_INCH / 2, EMU_PER_INCH * 2)
        .size(EMU_PER_INCH * 8, EMU_PER_INCH * 3);
    let fin_xml = financial_table.to_slide_xml(100);
    println!("   │   │   ├── Merged header spanning 4 columns");
    println!("   │   │   ├── Color-coded values (green=positive, red=negative)");
    println!("   │   │   ├── Custom fonts and sizes");
    println!("   │   │   └── XML: {} bytes", fin_xml.len());

    // Example 2: Comparison Matrix
    println!("   │   ├── Comparison Matrix (features vs products):");
    let matrix_table = TablePart::new()
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("Feature").bold().center().background("4472C4").color("FFFFFF"),
            TableCellPart::new("Basic").bold().center().background("4472C4").color("FFFFFF"),
            TableCellPart::new("Pro").bold().center().background("4472C4").color("FFFFFF"),
            TableCellPart::new("Enterprise").bold().center().background("4472C4").color("FFFFFF"),
        ]))
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("Storage").align(HorizontalAlign::Left),
            TableCellPart::new("5 GB").center(),
            TableCellPart::new("50 GB").center(),
            TableCellPart::new("Unlimited").center().bold().color("2E7D32"),
        ]))
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("Users").align(HorizontalAlign::Left),
            TableCellPart::new("1").center(),
            TableCellPart::new("10").center(),
            TableCellPart::new("Unlimited").center().bold().color("2E7D32"),
        ]))
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("Support").align(HorizontalAlign::Left),
            TableCellPart::new("Email").center(),
            TableCellPart::new("24/7 Chat").center(),
            TableCellPart::new("Dedicated").center().bold().color("2E7D32"),
        ]))
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("Price/mo").bold().background("F2F2F2"),
            TableCellPart::new("$9").center().bold().background("F2F2F2"),
            TableCellPart::new("$29").center().bold().background("F2F2F2"),
            TableCellPart::new("$99").center().bold().background("F2F2F2"),
        ]));
    println!("   │   │   └── 5x4 matrix with alternating styles");

    // Example 3: Schedule/Timeline Table
    println!("   │   └── Schedule Table (with row spans):");
    let schedule_table = TablePart::new()
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("Time").bold().center().background("70AD47").color("FFFFFF"),
            TableCellPart::new("Monday").bold().center().background("70AD47").color("FFFFFF"),
            TableCellPart::new("Tuesday").bold().center().background("70AD47").color("FFFFFF"),
        ]))
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("9:00 AM").center().background("E2EFDA"),
            TableCellPart::new("Team Standup").center().row_span(2).valign(VerticalAlign::Middle).background("BDD7EE"),
            TableCellPart::new("Code Review").center(),
        ]))
        .add_row(TableRowPart::new(vec![
            TableCellPart::new("10:00 AM").center().background("E2EFDA"),
            TableCellPart::merged(),
            TableCellPart::new("Sprint Planning").center().background("FCE4D6"),
        ]));
    println!("   │       └── Row spans for multi-hour events");

    // -------------------------------------------------------------------------
    // Complex Animation Sequences
    // -------------------------------------------------------------------------
    println!("   ├── Complex Animation Sequences:");
    
    // Sequence 1: Title entrance with staggered content
    println!("   │   ┌── Staggered Entrance Sequence:");
    let title_anim = Animation::new(2, AnimationEffect::Fade)
        .trigger(AnimationTrigger::OnClick)
        .duration(500);
    let content1 = Animation::new(3, AnimationEffect::FlyIn)
        .trigger(AnimationTrigger::AfterPrevious)
        .direction(AnimationDirection::Left)
        .duration(400)
        .delay(200);
    let content2 = Animation::new(4, AnimationEffect::FlyIn)
        .trigger(AnimationTrigger::AfterPrevious)
        .direction(AnimationDirection::Left)
        .duration(400)
        .delay(100);
    let content3 = Animation::new(5, AnimationEffect::FlyIn)
        .trigger(AnimationTrigger::AfterPrevious)
        .direction(AnimationDirection::Left)
        .duration(400)
        .delay(100);
    let staggered = SlideAnimations::new()
        .add(title_anim)
        .add(content1)
        .add(content2)
        .add(content3)
        .transition(SlideTransition::new(TransitionEffect::Push).direction(AnimationDirection::Left).duration(750));
    let staggered_xml = staggered.to_timing_xml()?;
    println!("   │   │   ├── Title: Fade on click");
    println!("   │   │   ├── Content 1-3: FlyIn with 100ms stagger");
    println!("   │   │   ├── Transition: Push from left (750ms)");
    println!("   │   │   └── XML: {} bytes", staggered_xml.len());

    // Sequence 2: Emphasis and exit
    println!("   │   ├── Emphasis + Exit Sequence:");
    let emphasis = Animation::new(6, AnimationEffect::Pulse)
        .trigger(AnimationTrigger::OnClick)
        .duration(1000)
        .repeat(3);
    let exit = Animation::new(6, AnimationEffect::FadeOut)
        .trigger(AnimationTrigger::AfterPrevious)
        .duration(500);
    let emphasis_seq = SlideAnimations::new()
        .add(emphasis)
        .add(exit);
    println!("   │   │   ├── Pulse 3x on click, then fade out");
    println!("   │   │   └── Same shape, sequential effects");

    // Sequence 3: Motion path
    println!("   │   └── Motion Path Animation:");
    let motion = Animation::new(7, AnimationEffect::Lines)
        .trigger(AnimationTrigger::OnClick)
        .duration(2000);
    println!("   │       └── Custom path: Lines, Arcs, Turns, Loops");

    // -------------------------------------------------------------------------
    // SmartArt Combinations
    // -------------------------------------------------------------------------
    println!("   ├── SmartArt Layout Examples:");
    
    // Process flow
    println!("   │   ┌── Process Flow (5 steps):");
    let process = SmartArtPart::new(1, SmartArtLayout::BasicProcess)
        .add_items(vec!["Research", "Design", "Develop", "Test", "Deploy"])
        .position(EMU_PER_INCH / 2, EMU_PER_INCH * 2)
        .size(EMU_PER_INCH * 8, EMU_PER_INCH * 2);
    println!("   │   │   └── {} nodes in horizontal flow", process.nodes().len());

    // Organization chart
    println!("   │   ├── Organization Chart:");
    let org = SmartArtPart::new(2, SmartArtLayout::OrgChart)
        .add_items(vec!["CEO", "CTO", "CFO", "VP Engineering", "VP Sales"]);
    println!("   │   │   └── Hierarchical structure with {} positions", org.nodes().len());

    // Cycle diagram
    println!("   │   ├── Cycle Diagram:");
    let cycle = SmartArtPart::new(3, SmartArtLayout::BasicCycle)
        .add_items(vec!["Plan", "Do", "Check", "Act"]);
    println!("   │   │   └── PDCA cycle with {} phases", cycle.nodes().len());

    // Venn diagram
    println!("   │   ├── Venn Diagram:");
    let venn = SmartArtPart::new(4, SmartArtLayout::BasicVenn)
        .add_items(vec!["Skills", "Passion", "Market Need"]);
    println!("   │   │   └── 3-circle Venn for Ikigai concept");

    // Pyramid
    println!("   │   └── Pyramid:");
    let pyramid = SmartArtPart::new(5, SmartArtLayout::BasicPyramid)
        .add_items(vec!["Self-Actualization", "Esteem", "Love/Belonging", "Safety", "Physiological"]);
    println!("   │       └── Maslow's hierarchy with {} levels", pyramid.nodes().len());

    // -------------------------------------------------------------------------
    // 3D Model Configurations
    // -------------------------------------------------------------------------
    println!("   ├── 3D Model Configurations:");
    
    // Product showcase
    println!("   │   ┌── Product Showcase:");
    let product_3d = Model3DPart::new(1, Model3DFormat::Glb, vec![0; 100])
        .camera(CameraPreset::IsometricTopUp)
        .rotation(0.0, 45.0, 0.0)
        .zoom(1.2)
        .position(EMU_PER_INCH * 2, EMU_PER_INCH * 2)
        .size(EMU_PER_INCH * 4, EMU_PER_INCH * 4);
    println!("   │   │   ├── Camera: Isometric top-up view");
    println!("   │   │   ├── Rotation: 45° Y-axis for best angle");
    println!("   │   │   └── Zoom: 1.2x for detail");

    // Architectural model
    println!("   │   ├── Architectural Model:");
    let arch_3d = Model3DPart::new(2, Model3DFormat::Gltf, vec![0; 100])
        .camera(CameraPreset::Front)
        .rotation(15.0, -30.0, 0.0)
        .ambient_light("FFFFCC");
    println!("   │   │   ├── Camera: Front view with tilt");
    println!("   │   │   └── Ambient: Warm lighting (FFFFCC)");

    // Technical diagram
    println!("   │   └── Technical Diagram:");
    let tech_3d = Model3DPart::new(3, Model3DFormat::Obj, vec![0; 100])
        .camera(CameraPreset::IsometricOffAxis1Top)
        .rotation(0.0, 0.0, 0.0);
    println!("   │       └── Camera: Off-axis isometric for exploded view");

    // -------------------------------------------------------------------------
    // Theme + Master + Layout Combination
    // -------------------------------------------------------------------------
    println!("   ├── Theme + Master + Layout Integration:");
    
    // Corporate theme
    let mut corp_theme = ThemePart::new(1);
    corp_theme.set_name("Corporate Blue");
    corp_theme.set_major_font("Segoe UI");
    corp_theme.set_minor_font("Segoe UI Light");
    corp_theme.set_color("dk1", "000000");
    corp_theme.set_color("lt1", "FFFFFF");
    corp_theme.set_color("dk2", "1F497D");
    corp_theme.set_color("lt2", "EEECE1");
    corp_theme.set_color("accent1", "4472C4");
    corp_theme.set_color("accent2", "ED7D31");
    corp_theme.set_color("accent3", "A5A5A5");
    corp_theme.set_color("accent4", "FFC000");
    corp_theme.set_color("accent5", "5B9BD5");
    corp_theme.set_color("accent6", "70AD47");
    let theme_xml = corp_theme.to_xml()?;
    println!("   │   ├── Theme: Corporate Blue");
    println!("   │   │   ├── Fonts: Segoe UI / Segoe UI Light");
    println!("   │   │   ├── 12 color slots defined");
    println!("   │   │   └── XML: {} bytes", theme_xml.len());

    // Master with multiple layouts
    let mut corp_master = SlideMasterPart::new(1);
    corp_master.set_name("Corporate Master");
    corp_master.add_layout_rel_id("rId2"); // Title
    corp_master.add_layout_rel_id("rId3"); // Title + Content
    corp_master.add_layout_rel_id("rId4"); // Section Header
    corp_master.add_layout_rel_id("rId5"); // Two Content
    corp_master.add_layout_rel_id("rId6"); // Comparison
    corp_master.add_layout_rel_id("rId7"); // Title Only
    corp_master.add_layout_rel_id("rId8"); // Blank
    println!("   │   └── Master: {} with {} layouts linked", corp_master.name(), corp_master.layout_rel_ids().len());

    // -------------------------------------------------------------------------
    // VBA + Custom XML Integration
    // -------------------------------------------------------------------------
    println!("   ├── VBA + Custom XML Integration:");
    
    // VBA with multiple modules
    let vba_project = VbaProjectPart::new()
        .add_module(VbaModule::new("AutoRun", r#"
Sub Auto_Open()
    MsgBox "Welcome to the presentation!"
End Sub

Sub NavigateToSlide(slideNum As Integer)
    SlideShowWindows(1).View.GotoSlide slideNum
End Sub
"#))
        .add_module(VbaModule::new("DataHelpers", r#"
Function GetCustomData(key As String) As String
    ' Read from Custom XML part
    GetCustomData = ActivePresentation.CustomXMLParts(1).SelectSingleNode("//" & key).Text
End Function
"#))
        .add_module(VbaModule::class("SlideController", r#"
Private currentSlide As Integer

Public Sub NextSlide()
    currentSlide = currentSlide + 1
    NavigateToSlide currentSlide
End Sub
"#));
    println!("   │   ├── VBA Project:");
    println!("   │   │   ├── AutoRun: Auto_Open, NavigateToSlide");
    println!("   │   │   ├── DataHelpers: GetCustomData (reads Custom XML)");
    println!("   │   │   └── SlideController: Class for navigation");

    // Custom XML with structured data
    let app_config = CustomXmlPart::new(1, "presentationConfig")
        .namespace("http://company.com/pptx/config")
        .property("version", "2.1.0")
        .property("author", "Demo User")
        .property("department", "Engineering")
        .property("confidentiality", "Internal")
        .property("lastModified", "2024-01-15T10:30:00Z");
    let config_xml = app_config.to_xml()?;
    println!("   │   └── Custom XML:");
    println!("   │       ├── Namespace: http://company.com/pptx/config");
    println!("   │       ├── Properties: version, author, department, etc.");
    println!("   │       └── XML: {} bytes", config_xml.len());

    // -------------------------------------------------------------------------
    // Embedded Fonts with Variants
    // -------------------------------------------------------------------------
    println!("   ├── Embedded Font Collection:");
    let mut font_collection = EmbeddedFontCollection::new();
    font_collection.add("Corporate Sans", vec![0; 1000]);
    font_collection.add_with_type("Corporate Sans", vec![0; 1000], FontEmbedType::Bold);
    font_collection.add_with_type("Corporate Sans", vec![0; 1000], FontEmbedType::Italic);
    font_collection.add_with_type("Corporate Sans", vec![0; 1000], FontEmbedType::BoldItalic);
    font_collection.add("Code Mono", vec![0; 800]);
    let fonts_xml = font_collection.to_xml();
    println!("   │   ├── Corporate Sans: Regular, Bold, Italic, BoldItalic");
    println!("   │   ├── Code Mono: Regular");
    println!("   │   ├── Total: {} font files", font_collection.len());
    println!("   │   └── XML: {} bytes", fonts_xml.len());

    // -------------------------------------------------------------------------
    // Handout with Full Configuration
    // -------------------------------------------------------------------------
    println!("   └── Handout Master Configuration:");
    let handout = HandoutMasterPart::new()
        .layout(HandoutLayout::SlidesPerPage6)
        .header("Q1 2024 Strategy Review")
        .footer("Confidential - Internal Use Only");
    let handout_xml = handout.to_xml()?;
    println!("       ├── Layout: 6 slides per page");
    println!("       ├── Header: Q1 2024 Strategy Review");
    println!("       ├── Footer: Confidential - Internal Use Only");
    println!("       └── XML: {} bytes", handout_xml.len());

    // =========================================================================
    // Summary
    // =========================================================================
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    Element Coverage Summary                   ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  LAYOUTS (6 types):                                          ║");
    println!("║    ✓ CenteredTitle    ✓ TitleOnly      ✓ TitleAndContent     ║");
    println!("║    ✓ TitleAndBigContent  ✓ TwoColumn   ✓ Blank               ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  TEXT FORMATTING:                                            ║");
    println!("║    ✓ Bold            ✓ Italic         ✓ Underline            ║");
    println!("║    ✓ Font Size       ✓ Font Color     ✓ Title/Content styles ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  TABLES:                                                     ║");
    println!("║    ✓ Multiple rows/columns  ✓ Bold cells  ✓ Background colors║");
    println!("║    ✓ Header styling         ✓ Position control               ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  CHARTS:                                                     ║");
    println!("║    ✓ Bar Chart       ✓ Line Chart     ✓ Pie Chart            ║");
    println!("║    ✓ Multiple series ✓ Categories                            ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  SHAPES:                                                     ║");
    println!("║    ✓ Rectangle       ✓ Ellipse        ✓ RoundedRectangle     ║");
    println!("║    ✓ Triangle        ✓ Diamond        ✓ Color fills          ║");
    println!("║    ✓ Gradient fills  ✓ Transparency   ✓ Text in shapes       ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  CONNECTORS (NEW):                                           ║");
    println!("║    ✓ Straight        ✓ Elbow          ✓ Curved               ║");
    println!("║    ✓ Arrow types     ✓ Dash styles    ✓ Line colors/widths   ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  IMAGES:                                                     ║");
    println!("║    ✓ Image placeholders  ✓ Position   ✓ Dimensions           ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  PACKAGE:                                                    ║");
    println!("║    ✓ Create PPTX     ✓ Read PPTX      ✓ Analyze contents     ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  PARTS API (NEW):                                            ║");
    println!("║    ✓ SlideLayoutPart (11 types)  ✓ SlideMasterPart           ║");
    println!("║    ✓ ThemePart (colors/fonts)    ✓ NotesSlidePart            ║");
    println!("║    ✓ AppPropertiesPart           ✓ MediaPart (10 formats)    ║");
    println!("║    ✓ TablePart (cell formatting) ✓ ContentTypesPart          ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  ELEMENTS API:                                               ║");
    println!("║    ✓ RgbColor        ✓ SchemeColor    ✓ Color enum           ║");
    println!("║    ✓ Position        ✓ Size           ✓ Transform            ║");
    println!("║    ✓ EMU conversions (inches, cm, mm, pt)                    ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  ADVANCED FEATURES (NEW):                                    ║");
    println!("║    ✓ Animation (50+ effects)  ✓ Transitions (27 types)       ║");
    println!("║    ✓ SmartArt (25 layouts)    ✓ 3D Models (GLB/GLTF/OBJ)     ║");
    println!("║    ✓ VBA Macros (.pptm)       ✓ Embedded Fonts               ║");
    println!("║    ✓ Custom XML               ✓ Handout Master               ║");
    println!("║    ✓ Table borders/alignment  ✓ Merged cells                 ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  DIMENSION API (NEW):                                        ║");
    println!("║    ✓ EMU / Inches / Cm / Pt / Ratio / Percent units          ║");
    println!("║    ✓ from_dimensions() constructor                           ║");
    println!("║    ✓ Fluent .at() and .with_dimensions() chaining            ║");
    println!("║    ✓ Mixed-unit positioning (e.g. inches + percent)          ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Output: comprehensive_demo.pptx ({} slides, {} KB)         ║", 
             slides.len(), pptx_data.len() / 1024);
    println!("╚══════════════════════════════════════════════════════════════╝");

    Ok(())
}
