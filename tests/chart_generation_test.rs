use ppt_rs::api::Presentation;
use ppt_rs::generator::{
    SlideContent, ChartBuilder, ChartType, ChartSeries
};
use std::fs::File;
use std::io::Read;

#[test]
fn test_chart_generation() {
    let mut pres = Presentation::new();
    
    // Create a slide with a chart
    let mut slide = SlideContent::new("Chart Slide");
    
    // Create a simple bar chart
    let chart = ChartBuilder::new("Sales Data", ChartType::Bar)
        .position(1000000, 1000000)
        .size(4000000, 3000000)
        .add_series(ChartSeries::new(
            "Series 1", 
            vec![1.0, 2.0, 3.0]
        ))
        .add_series(ChartSeries::new(
            "Series 2", 
            vec![3.0, 2.0, 1.0]
        ))
        .build();
        
    slide = slide.add_chart(chart);
    pres = pres.add_slide(slide);
    
    // Save
    let output = "test_chart_generation.pptx";
    let res = pres.save(output);
    assert!(res.is_ok());
    
    // Verification: Read the file as ZIP and check for chart files
    let file = File::open(output).expect("Failed to open output file");
    let mut archive = zip::ZipArchive::new(file).expect("Failed to open zip archive");
    
    // 1. Check if chart part exists
    {
        let chart_file = archive.by_name("ppt/charts/chart1.xml");
        assert!(chart_file.is_ok(), "ppt/charts/chart1.xml should exist");
    }
    
    // 2. Check content types
    let mut content_types = String::new();
    archive.by_name("[Content_Types].xml").unwrap().read_to_string(&mut content_types).unwrap();
    assert!(content_types.contains("ppt/charts/chart1.xml"), "Content types should contain chart reference");
    assert!(content_types.contains("application/vnd.openxmlformats-officedocument.drawingml.chart+xml"), "Content types should contain chart mime type");
    
    // 3. Check slide relationships
    let mut slide_rels = String::new();
    archive.by_name("ppt/slides/_rels/slide1.xml.rels").unwrap().read_to_string(&mut slide_rels).unwrap();
    assert!(slide_rels.contains("charts/chart1.xml"), "Slide rels should point to chart");
    assert!(slide_rels.contains("http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart"), "Slide rels should have chart relationship type");
    
    // 4. Check slide content
    let mut slide_xml = String::new();
    archive.by_name("ppt/slides/slide1.xml").unwrap().read_to_string(&mut slide_xml).unwrap();
    assert!(slide_xml.contains("c:chart"), "Slide XML should contain chart element");
    assert!(slide_xml.contains("http://schemas.openxmlformats.org/drawingml/2006/chart"), "Slide XML should contain chart namespace");
    
    // Cleanup
    std::fs::remove_file(output).unwrap_or(());
}
