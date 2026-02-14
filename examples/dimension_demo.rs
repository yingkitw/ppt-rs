//! Dimension API Demo — Flexible Positioning & Sizing
//!
//! Showcases all Dimension unit types and fluent APIs:
//! - EMU, Inches, Cm, Pt, Ratio, Percent
//! - Shape::from_dimensions(), .at(), .with_dimensions()
//! - Image::at(), .with_dimensions()
//! - Prelude helpers: shapes::dim(), shapes::rect_ratio(), shapes::text_box_ratio()
//! - FlexPosition / FlexSize structs
//! - Mixed-unit positioning
//!
//! Run with: cargo run --example dimension_demo

use ppt_rs::generator::{
    create_pptx_with_content, SlideContent, SlideLayout,
    Shape, ShapeType, ShapeFill, ShapeLine,
};
use ppt_rs::core::{Dimension, FlexPosition, FlexSize, SLIDE_WIDTH_EMU, SLIDE_HEIGHT_EMU};
use ppt_rs::prelude::shapes;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║       Dimension API Demo — Flexible Positioning & Sizing     ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let mut slides = Vec::new();

    // =========================================================================
    // SLIDE 1: Title
    // =========================================================================
    slides.push(
        SlideContent::new("Dimension API — Flexible Positioning & Sizing")
            .layout(SlideLayout::CenteredTitle)
            .title_size(44)
            .title_bold(true)
            .title_color("1F497D")
    );

    // =========================================================================
    // SLIDE 2: All Unit Types Side-by-Side
    // =========================================================================
    println!("📏 Slide 2: All Unit Types");

    // Each shape is 1 inch wide, positioned using a different unit type
    let emu_shape = Shape::from_dimensions(ShapeType::Rectangle,
        Dimension::Emu(457200), Dimension::Inches(1.5),
        Dimension::Emu(1371600), Dimension::Inches(0.8),
    ).with_fill(ShapeFill::new("1565C0")).with_text("EMU");

    let inch_shape = Shape::from_dimensions(ShapeType::Rectangle,
        Dimension::Inches(2.0), Dimension::Inches(1.5),
        Dimension::Inches(1.5), Dimension::Inches(0.8),
    ).with_fill(ShapeFill::new("2E7D32")).with_text("Inches");

    let cm_shape = Shape::from_dimensions(ShapeType::Rectangle,
        Dimension::Cm(9.0), Dimension::Inches(1.5),
        Dimension::Cm(3.81), Dimension::Inches(0.8),
    ).with_fill(ShapeFill::new("C62828")).with_text("Cm");

    let pt_shape = Shape::from_dimensions(ShapeType::Rectangle,
        Dimension::Pt(324.0), Dimension::Inches(1.5),
        Dimension::Pt(108.0), Dimension::Inches(0.8),
    ).with_fill(ShapeFill::new("7B1FA2")).with_text("Pt");

    let ratio_shape = Shape::from_dimensions(ShapeType::Rectangle,
        Dimension::Ratio(0.52), Dimension::Inches(1.5),
        Dimension::Ratio(0.15), Dimension::Inches(0.8),
    ).with_fill(ShapeFill::new("EF6C00")).with_text("Ratio");

    let pct_shape = Shape::from_dimensions(ShapeType::Rectangle,
        Dimension::percent(69.0), Dimension::Inches(1.5),
        Dimension::percent(15.0), Dimension::Inches(0.8),
    ).with_fill(ShapeFill::new("00838F")).with_text("Percent");

    // Labels row
    let label = Shape::from_dimensions(ShapeType::Rectangle,
        Dimension::Inches(0.5), Dimension::Inches(0.8),
        Dimension::Inches(9.0), Dimension::Inches(0.5),
    ).with_text("Each shape below uses a different unit type for X position:");

    slides.push(
        SlideContent::new("All Dimension Unit Types")
            .layout(SlideLayout::TitleOnly)
            .title_color("1F497D").title_bold(true)
            .add_shape(label)
            .add_shape(emu_shape)
            .add_shape(inch_shape)
            .add_shape(cm_shape)
            .add_shape(pt_shape)
            .add_shape(ratio_shape)
            .add_shape(pct_shape)
    );

    // =========================================================================
    // SLIDE 3: Ratio-Based Grid Layout
    // =========================================================================
    println!("📐 Slide 3: Ratio-Based Grid (auto-adapts to slide size)");

    let margin = 0.03;  // 3% margin
    let gap = 0.02;     // 2% gap
    let cell_w = (1.0 - 2.0 * margin - 2.0 * gap) / 3.0;
    let cell_h = (0.7 - 2.0 * gap) / 3.0;  // 70% of slide height for grid
    let y_start = 0.22; // below title

    let colors = [
        "1565C0", "2E7D32", "C62828",
        "7B1FA2", "EF6C00", "00838F",
        "AD1457", "4E342E", "37474F",
    ];
    let labels = [
        "Top-Left", "Top-Center", "Top-Right",
        "Mid-Left", "Mid-Center", "Mid-Right",
        "Bot-Left", "Bot-Center", "Bot-Right",
    ];

    let mut grid_slide = SlideContent::new("Ratio-Based 3x3 Grid Layout")
        .layout(SlideLayout::TitleOnly)
        .title_color("1F497D").title_bold(true);

    for row in 0..3 {
        for col in 0..3 {
            let idx = row * 3 + col;
            let x = margin + col as f64 * (cell_w + gap);
            let y = y_start + row as f64 * (cell_h + gap);
            let shape = Shape::from_dimensions(ShapeType::RoundedRectangle,
                Dimension::Ratio(x), Dimension::Ratio(y),
                Dimension::Ratio(cell_w), Dimension::Ratio(cell_h),
            ).with_fill(ShapeFill::new(colors[idx])).with_text(labels[idx]);
            grid_slide = grid_slide.add_shape(shape);
        }
    }

    slides.push(grid_slide);

    // =========================================================================
    // SLIDE 4: Mixed-Unit Positioning
    // =========================================================================
    println!("🔀 Slide 4: Mixed-Unit Positioning");

    // Title area: inches for position, ratio for width
    let title_box = Shape::from_dimensions(ShapeType::RoundedRectangle,
        Dimension::Inches(0.5), Dimension::Inches(1.5),
        Dimension::Ratio(0.9), Dimension::Cm(2.0),
    ).with_fill(ShapeFill::new("1F497D")).with_text("Inches X + Ratio Width + Cm Height");

    // Content area: cm for position, pt for size
    let content_box = Shape::from_dimensions(ShapeType::Rectangle,
        Dimension::Cm(2.0), Dimension::Cm(6.0),
        Dimension::Pt(432.0), Dimension::Pt(108.0),  // 6in x 1.5in
    ).with_fill(ShapeFill::new("2E7D32")).with_text("Cm position + Pt size");

    // Footer area: percent for everything
    let footer_box = Shape::from_dimensions(ShapeType::Rectangle,
        Dimension::percent(5.0), Dimension::percent(75.0),
        Dimension::percent(90.0), Dimension::percent(10.0),
    ).with_fill(ShapeFill::new("C62828")).with_text("100% percent-based");

    // Sidebar: EMU for position, inches for size
    let sidebar = Shape::from_dimensions(ShapeType::Rectangle,
        Dimension::Emu(8000000), Dimension::Inches(1.5),
        Dimension::Inches(1.0), Dimension::Ratio(0.6),
    ).with_fill(ShapeFill::new("7B1FA2")).with_text("EMU + Inches + Ratio");

    slides.push(
        SlideContent::new("Mixed-Unit Positioning")
            .layout(SlideLayout::TitleOnly)
            .title_color("1F497D").title_bold(true)
            .add_shape(title_box)
            .add_shape(content_box)
            .add_shape(footer_box)
            .add_shape(sidebar)
    );

    // =========================================================================
    // SLIDE 5: Fluent .at() and .with_dimensions() Chaining
    // =========================================================================
    println!("🔗 Slide 5: Fluent Chaining API");

    // Build shapes step by step with chaining
    let shape1 = Shape::new(ShapeType::Ellipse, 0, 0, 0, 0)
        .at(Dimension::percent(10.0), Dimension::percent(25.0))
        .with_dimensions(Dimension::Inches(2.5), Dimension::Inches(2.5))
        .with_fill(ShapeFill::new("1565C0"))
        .with_text(".at() + .with_dimensions()");

    let shape2 = Shape::new(ShapeType::RoundedRectangle, 0, 0, 0, 0)
        .at(Dimension::Inches(4.0), Dimension::Cm(5.0))
        .with_dimensions(Dimension::Ratio(0.3), Dimension::Inches(2.0))
        .with_fill(ShapeFill::new("2E7D32"))
        .with_line(ShapeLine::new("1B5E20", 25400))
        .with_text("Chained with fill + line");

    let shape3 = Shape::new(ShapeType::Star5, 0, 0, 0, 0)
        .at(Dimension::percent(70.0), Dimension::percent(55.0))
        .with_dimensions(Dimension::Inches(2.0), Dimension::Inches(2.0))
        .with_fill(ShapeFill::new("FFC107"))
        .with_rotation(15)
        .with_text("+ rotation");

    slides.push(
        SlideContent::new("Fluent .at() and .with_dimensions() Chaining")
            .layout(SlideLayout::TitleOnly)
            .title_color("1F497D").title_bold(true)
            .add_shape(shape1)
            .add_shape(shape2)
            .add_shape(shape3)
    );

    // =========================================================================
    // SLIDE 6: Prelude Shape Builders
    // =========================================================================
    println!("🧰 Slide 6: Prelude Shape Builders");

    // shapes::dim() — generic Dimension-based builder
    let dim_shape = shapes::dim(ShapeType::Diamond,
        Dimension::percent(5.0), Dimension::percent(25.0),
        Dimension::percent(25.0), Dimension::percent(35.0),
    ).with_fill(ShapeFill::new("7B1FA2")).with_text("shapes::dim()");

    // shapes::rect_ratio() — ratio-based rectangle
    let ratio_rect = shapes::rect_ratio(0.35, 0.25, 0.28, 0.35)
        .with_fill(ShapeFill::new("EF6C00")).with_text("shapes::rect_ratio()");

    // shapes::text_box_ratio() — ratio-based text box
    let ratio_text = shapes::text_box_ratio(0.68, 0.25, 0.28, 0.35, "shapes::text_box_ratio()")
        .with_fill(ShapeFill::new("00838F"));

    // Traditional shapes::rect() still works (inches)
    let inch_rect = shapes::rect(1.0, 5.0, 3.0, 1.0)
        .with_fill(ShapeFill::new("A5A5A5")).with_text("shapes::rect() (inches)");

    slides.push(
        SlideContent::new("Prelude Shape Builders")
            .layout(SlideLayout::TitleOnly)
            .title_color("1F497D").title_bold(true)
            .add_shape(dim_shape)
            .add_shape(ratio_rect)
            .add_shape(ratio_text)
            .add_shape(inch_rect)
    );

    // =========================================================================
    // SLIDE 7: FlexPosition & FlexSize Structs
    // =========================================================================
    println!("📦 Slide 7: FlexPosition & FlexSize");

    // Demonstrate FlexPosition and FlexSize for reusable layout definitions
    let header_pos = FlexPosition::new(Dimension::percent(5.0), Dimension::percent(20.0));
    let header_size = FlexSize::new(Dimension::percent(90.0), Dimension::percent(12.0));
    let (hx, hy) = header_pos.to_emu();
    let (hw, hh) = header_size.to_emu();
    let header = Shape::new(ShapeType::RoundedRectangle, hx, hy, hw, hh)
        .with_fill(ShapeFill::new("1F497D"))
        .with_text("FlexPosition + FlexSize → header");

    let body_pos = FlexPosition::new(Dimension::percent(5.0), Dimension::percent(35.0));
    let body_size = FlexSize::new(Dimension::percent(60.0), Dimension::percent(50.0));
    let (bx, by) = body_pos.to_emu();
    let (bw, bh) = body_size.to_emu();
    let body = Shape::new(ShapeType::Rectangle, bx, by, bw, bh)
        .with_fill(ShapeFill::new("E8EAF6"))
        .with_line(ShapeLine::new("3F51B5", 12700))
        .with_text("Body area (60% x 50%)");

    let sidebar_pos = FlexPosition::new(Dimension::percent(68.0), Dimension::percent(35.0));
    let sidebar_size = FlexSize::new(Dimension::percent(27.0), Dimension::percent(50.0));
    let (sx, sy) = sidebar_pos.to_emu();
    let (sw, sh) = sidebar_size.to_emu();
    let sidebar_shape = Shape::new(ShapeType::Rectangle, sx, sy, sw, sh)
        .with_fill(ShapeFill::new("FFF3E0"))
        .with_line(ShapeLine::new("EF6C00", 12700))
        .with_text("Sidebar (27% x 50%)");

    slides.push(
        SlideContent::new("FlexPosition & FlexSize — Reusable Layouts")
            .layout(SlideLayout::TitleOnly)
            .title_color("1F497D").title_bold(true)
            .add_shape(header)
            .add_shape(body)
            .add_shape(sidebar_shape)
    );

    // =========================================================================
    // SLIDE 8: Real-World Dashboard with Dimension API
    // =========================================================================
    println!("📊 Slide 8: Real-World Dashboard");

    // 4 evenly-spaced KPI cards using percent
    let kpi_colors = ["1565C0", "2E7D32", "EF6C00", "7B1FA2"];
    let kpi_labels = [
        "Revenue\n$2.14M\n+15%",
        "Users\n12,450\n+22%",
        "NPS\n72\n+8 pts",
        "Uptime\n99.9%\n+0.1%",
    ];

    let mut dashboard = SlideContent::new("KPI Dashboard — Dimension API")
        .layout(SlideLayout::TitleOnly)
        .title_color("1F497D").title_bold(true);

    for i in 0..4 {
        let x_pct = 3.0 + i as f64 * 24.5;
        let card = Shape::from_dimensions(ShapeType::RoundedRectangle,
            Dimension::percent(x_pct), Dimension::percent(22.0),
            Dimension::percent(22.0), Dimension::percent(30.0),
        ).with_fill(ShapeFill::new(kpi_colors[i])).with_text(kpi_labels[i]);
        dashboard = dashboard.add_shape(card);
    }

    // Bottom chart placeholder
    let chart_area = Shape::from_dimensions(ShapeType::Rectangle,
        Dimension::percent(3.0), Dimension::percent(58.0),
        Dimension::percent(94.0), Dimension::percent(35.0),
    ).with_fill(ShapeFill::new("ECEFF1"))
     .with_line(ShapeLine::new("B0BEC5", 12700))
     .with_text("Chart Area (94% x 35%)");
    dashboard = dashboard.add_shape(chart_area);

    slides.push(dashboard);

    // =========================================================================
    // SLIDE 9: Unit Equivalence Reference
    // =========================================================================
    println!("📖 Slide 9: Unit Equivalence Reference");

    slides.push(
        SlideContent::new("Dimension Unit Reference")
            .layout(SlideLayout::TitleAndContent)
            .title_color("1F497D").title_bold(true)
            .add_bullet(&format!("1 inch = {} EMU = Dimension::Inches(1.0)", 914400))
            .add_bullet(&format!("1 cm   = {} EMU = Dimension::Cm(1.0)", 360000))
            .add_bullet(&format!("1 pt   = {} EMU = Dimension::Pt(1.0)", 12700))
            .add_bullet(&format!("Slide width  = {} EMU = 10 inches", SLIDE_WIDTH_EMU))
            .add_bullet(&format!("Slide height = {} EMU = 7.5 inches", SLIDE_HEIGHT_EMU))
            .add_bullet("Ratio(0.1) on X = 10% of slide width = 1 inch")
            .add_bullet("Ratio(0.5) on Y = 50% of slide height = 3.75 inches")
            .add_bullet("percent(50.0) = Ratio(0.5)")
            .content_size(22)
    );

    // =========================================================================
    // Generate PPTX
    // =========================================================================
    fs::create_dir_all("examples/output")?;
    let num_slides = slides.len();
    let pptx_data = create_pptx_with_content("Dimension API Demo", slides)?;
    fs::write("examples/output/dimension_demo.pptx", &pptx_data)?;

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                 Dimension API Demo Complete                   ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Output: examples/output/dimension_demo.pptx                 ║");
    println!("║  Slides: {}                                                   ║", num_slides);
    println!("║  Size:   {} KB                                               ║", pptx_data.len() / 1024);
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Showcased:                                                  ║");
    println!("║    ✓ All 6 unit types: EMU, Inches, Cm, Pt, Ratio, Percent   ║");
    println!("║    ✓ Shape::from_dimensions() constructor                    ║");
    println!("║    ✓ Fluent .at() and .with_dimensions() chaining            ║");
    println!("║    ✓ Mixed-unit positioning                                  ║");
    println!("║    ✓ Prelude helpers: dim(), rect_ratio(), text_box_ratio()  ║");
    println!("║    ✓ FlexPosition & FlexSize structs                         ║");
    println!("║    ✓ Ratio-based grid layout (auto-adapts)                   ║");
    println!("║    ✓ Real-world KPI dashboard                                ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    Ok(())
}
