//! Actual chart demonstration with real charts in slides

use ppt_rs::generator::{
    create_pptx_with_content, SlideContent, SlideLayout,
    ChartType, ChartSeries, ChartBuilder,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Creating Presentation with Real Charts ===\n");

    // Slide 1: Title
    let title_slide = SlideContent::new("Real Charts Demo")
        .add_bullet("This presentation contains actual charts")
        .add_bullet("Not just text descriptions")
        .layout(SlideLayout::TitleAndContent);

    // Slide 2: Bar Chart - Actual data
    let bar_chart = ChartBuilder::new("Quarterly Sales", ChartType::Bar)
        .categories(vec!["Q1", "Q2", "Q3", "Q4"])
        .add_series(ChartSeries::new("2023", vec![100.0, 120.0, 140.0, 160.0]))
        .add_series(ChartSeries::new("2024", vec![110.0, 135.0, 155.0, 180.0]))
        .position(2675890, 1725930)  // Position on slide (WPS reference)
        .size(6839585, 3959860)     // Size (WPS reference)
        .build();

    let bar_slide = SlideContent::new("Bar Chart: Sales Comparison")
        .add_chart(bar_chart)
        .add_bullet("2024 shows consistent growth over 2023")
        .add_bullet("Q4 is the strongest quarter");

    // Slide 3: Line Chart - Trend analysis
    let line_chart = ChartBuilder::new("Monthly Revenue Trend", ChartType::Line)
        .categories(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
        .add_series(ChartSeries::new("Revenue (K$)", vec![50.0, 55.0, 62.0, 58.0, 68.0, 75.0]))
        .position(2675890, 1725930)
        .size(6839585, 3959860)
        .build();

    let line_slide = SlideContent::new("Line Chart: Revenue Trend")
        .add_chart(line_chart)
        .add_bullet("Steady upward trend from January to June")
        .add_bullet("Peak in June: $75K");

    // Slide 4: Pie Chart - Market share
    let pie_chart = ChartBuilder::new("Market Share Distribution", ChartType::Pie)
        .categories(vec!["Product A", "Product B", "Product C", "Others"])
        .add_series(ChartSeries::new("Share %", vec![35.0, 28.0, 22.0, 15.0]))
        .position(2675890, 1725930)
        .size(6839585, 3959860)  // Use standard WPS reference coordinates
        .build();

    let pie_slide = SlideContent::new("Pie Chart: Market Share")
        .add_chart(pie_chart)
        .add_bullet("Product A leads with 35% market share")
        .add_bullet("Top 3 products account for 85% of market");

    // Slide 5: Scatter Chart - Correlation analysis
    let scatter_chart = ChartBuilder::new("Price vs Sales Correlation", ChartType::Scatter)
        .categories(vec!["10", "20", "30", "40", "50", "60"])  // Price points
        .add_series(ChartSeries::new("Product Sales", vec![150.0, 120.0, 90.0, 75.0, 60.0, 45.0]))  // Sales volume
        .position(2675890, 1725930)
        .size(6839585, 3959860)
        .build();

    let scatter_slide = SlideContent::new("Scatter Chart: Price vs Sales")
        .add_chart(scatter_chart)
        .add_bullet("Higher price correlates with lower sales volume")
        .add_bullet("Optimal price point appears to be around $10-20");

    let slides = vec![
        title_slide,
        bar_slide,
        line_slide,
        pie_slide,
        scatter_slide,
    ];

    // Generate the PPTX file
    let pptx_data = create_pptx_with_content("Real Charts Demo", slides.clone())?;
    std::fs::write("real_charts_demo.pptx", pptx_data)?;

    println!("✓ Created real_charts_demo.pptx with:");
    println!("  - Title slide");
    println!("  - Bar chart: Quarterly sales comparison");
    println!("  - Line chart: Monthly revenue trend");
    println!("  - Pie chart: Market share distribution");
    println!("  - Scatter chart: Price vs Sales correlation");
    println!("\nTotal slides: {}", slides.len());
    println!("\nNote: Check if charts are visible in the generated PPTX file");

    Ok(())
}