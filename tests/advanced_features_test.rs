use ppt_rs::Presentation;
use ppt_rs::generator::{
    SlideContent, Shape, ShapeType,
    Connector, ConnectorType, ArrowType, ConnectionSite,
    ShapeGradientFill, ShapeGradientDirection,
};
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;

#[test]
fn test_connector_generation() {
    let mut pres = Presentation::new();
    let mut slide = SlideContent::new("Connector Test");

    // Create two shapes
    let shape1 = Shape::new(ShapeType::Rectangle, 1000000, 1000000, 1000000, 1000000)
        .with_id(101);
    let shape2 = Shape::new(ShapeType::Rectangle, 3000000, 1000000, 1000000, 1000000)
        .with_id(102);

    slide.shapes.push(shape1);
    slide.shapes.push(shape2);

    // Create connector between them
    // Connector::new(type, sx, sy, ex, ey)
    let connector = Connector::new(ConnectorType::Curved, 1500000, 1500000, 3500000, 1500000)
        .connect_start(101, ConnectionSite::Right)
        .connect_end(102, ConnectionSite::Left)
        .with_start_arrow(ArrowType::Oval)
        .with_end_arrow(ArrowType::Stealth);
        
    slide.connectors.push(connector);
    
    pres = pres.add_slide(slide);
    
    let output = "test_connector.pptx";
    let _ = pres.save(output);
    
    // Verification
    let file = File::open(output).expect("Failed to open output file");
    let mut archive = ZipArchive::new(file).expect("Failed to open zip archive");
    
    let mut slide_xml = String::new();
    archive.by_name("ppt/slides/slide1.xml").unwrap().read_to_string(&mut slide_xml).unwrap();
    
    assert!(slide_xml.contains("cxnSp"), "Should contain connection shape");
    assert!(slide_xml.contains("curvedConnector3"), "Should contain curved connector geometry");
    assert!(slide_xml.contains("stCxn"), "Should contain start connection");
    assert!(slide_xml.contains("endCxn"), "Should contain end connection");
    assert!(slide_xml.contains("oval"), "Should contain start arrow type");
    assert!(slide_xml.contains("stealth"), "Should contain end arrow type");
    
    std::fs::remove_file(output).unwrap_or(());
}

#[test]
fn test_gradient_generation() {
    let mut pres = Presentation::new();
    let mut slide = SlideContent::new("Gradient Test");

    // Create shape with gradient
    let mut shape = Shape::new(ShapeType::Rectangle, 1000000, 1000000, 2000000, 2000000);
    
    // Use ShapeGradientFill (from shapes.rs)
    let gradient = ShapeGradientFill::linear("FF0000", "0000FF", ShapeGradientDirection::Vertical);
        
    shape.gradient = Some(gradient);
    
    slide.shapes.push(shape);
    pres = pres.add_slide(slide);
    
    let output = "test_gradient.pptx";
    let _ = pres.save(output);
    
    // Verification
    let file = File::open(output).expect("Failed to open output file");
    let mut archive = ZipArchive::new(file).expect("Failed to open zip archive");
    
    let mut slide_xml = String::new();
    archive.by_name("ppt/slides/slide1.xml").unwrap().read_to_string(&mut slide_xml).unwrap();
    
    assert!(slide_xml.contains("gradFill"), "Should contain gradient fill");
    assert!(slide_xml.contains("lin"), "Should contain linear gradient");
    assert!(slide_xml.contains("FF0000"), "Should contain start color");
    assert!(slide_xml.contains("0000FF"), "Should contain end color");
    
    std::fs::remove_file(output).unwrap_or(());
}
