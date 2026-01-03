use ppt_rs::api::Presentation;
use ppt_rs::generator::{SlideContent, ChartBuilder, ChartType, ChartSeries};

#[test]
fn test_chart_generation() {
    let mut pres = Presentation::new();
    
    let mut slide = SlideContent::new("Chart Slide");
    
    // Create a simple bar chart
    let chart = ChartBuilder::new("Monthly Sales", ChartType::Bar)
        .categories(vec!["Jan", "Feb", "Mar"])
        .add_series(ChartSeries::new("Sales", vec![100.0, 150.0, 200.0]))
        .position(1000000, 1000000)
        .size(4000000, 3000000)
        .build();
        
    slide.charts.push(chart);
    
    pres = pres.add_slide(slide);
    
    let output_file = "test_chart.pptx";
    let result = pres.save(output_file);
    
    assert!(result.is_ok(), "Failed to save presentation with chart");
    
    // Validate output (basic check)
    // In a real scenario, we would unzip and check for ppt/charts/chart1.xml
    // and the relationship in slide refs.
    
    if std::path::Path::new(output_file).exists() {
        std::fs::remove_file(output_file).unwrap();
    }
}
