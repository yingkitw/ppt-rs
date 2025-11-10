/// Alignment Test Example
/// 
/// This example generates a PPTX file that aligns with python-pptx output.
/// It demonstrates:
/// - Presentation metadata (title, author, subject, keywords, comments)
/// - Multiple slides
/// - Shape creation (rectangles, circles, diamonds)
/// - Text content in shapes
/// - Proper XML structure matching python-pptx

use ppt_rs::PresentationBuilder;
use ppt_rs::shapes::{AutoShape, AutoShapeType, Shape};
use ppt_rs::text::TextFrame;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Alignment Test: ppt-rs vs python-pptx ===\n");
    
    // Create output directory
    fs::create_dir_all("examples/output")?;
    
    // Create presentation with metadata
    let mut prs = PresentationBuilder::new()
        .title("Alignment Test Presentation")
        .author("ppt-rs Team")
        .subject("Testing ppt-rs alignment with python-pptx")
        .keywords("pptx, rust, python-pptx, alignment")
        .comments("This presentation tests alignment between ppt-rs and python-pptx")
        .build()?;
    
    println!("✓ Created presentation");
    println!("  - Title: {}", prs.core_properties().title().unwrap_or(""));
    println!("  - Author: {}", prs.core_properties().author().unwrap_or(""));
    
    // Get slides collection
    let mut slides = prs.slides();
    
    // Slide 1: Title Slide
    println!("\n--- Slide 1: Title Slide ---");
    let mut slide1 = slides.add_slide()?;
    
    // Add title shape
    let mut title_shape = AutoShape::with_text_frame(
        1,
        "Title".to_string(),
        AutoShapeType::Rectangle,
    );
    title_shape.set_left(457200); // 0.5"
    title_shape.set_top(1143000); // 1.25"
    title_shape.set_width(8229600); // 9"
    title_shape.set_height(1371600); // 1.5"
    
    // Configure title text
    if let Some(frame) = title_shape.text_frame_mut() {
        frame.set_text("Alignment Test Presentation");
    }
    
    slide1.add_shape(Box::new(title_shape))?;
    println!("✓ Added title shape");
    
    // Add subtitle shape
    let mut subtitle_shape = AutoShape::with_text_frame(
        2,
        "Subtitle".to_string(),
        AutoShapeType::Rectangle,
    );
    subtitle_shape.set_left(457200); // 0.5"
    subtitle_shape.set_top(2743200); // 3"
    subtitle_shape.set_width(8229600); // 9"
    subtitle_shape.set_height(914400); // 1"
    
    // Configure subtitle text
    if let Some(frame) = subtitle_shape.text_frame_mut() {
        frame.set_text("ppt-rs vs python-pptx Compatibility");
    }
    
    slide1.add_shape(Box::new(subtitle_shape))?;
    println!("✓ Added subtitle shape");
    
    // Slide 2: Content with Shapes
    println!("\n--- Slide 2: Shapes and Formatting ---");
    let mut slide2 = slides.add_slide()?;
    
    // Add slide title
    let mut slide2_title = AutoShape::with_text_frame(
        1,
        "Slide Title".to_string(),
        AutoShapeType::Rectangle,
    );
    slide2_title.set_left(457200); // 0.5"
    slide2_title.set_top(457200); // 0.5"
    slide2_title.set_width(8229600); // 9"
    slide2_title.set_height(728400); // 0.8"
    
    if let Some(frame) = slide2_title.text_frame_mut() {
        frame.set_text("Shapes and Formatting");
    }
    
    slide2.add_shape(Box::new(slide2_title))?;
    println!("✓ Added slide title");
    
    // Add rectangle shape (red)
    let mut rect = AutoShape::with_text_frame(
        2,
        "Rectangle".to_string(),
        AutoShapeType::Rectangle,
    );
    rect.set_left(457200); // 0.5"
    rect.set_top(1628400); // 1.78"
    rect.set_width(2286000); // 2.5"
    rect.set_height(1371600); // 1.5"
    
    if let Some(frame) = rect.text_frame_mut() {
        frame.set_text("Rectangle");
    }
    
    slide2.add_shape(Box::new(rect))?;
    println!("✓ Added rectangle shape");
    
    // Add circle shape (green)
    let mut circle = AutoShape::with_text_frame(
        3,
        "Circle".to_string(),
        AutoShapeType::Oval,
    );
    circle.set_left(3200400); // 3.5"
    circle.set_top(1628400); // 1.78"
    circle.set_width(2286000); // 2.5"
    circle.set_height(1371600); // 1.5"
    
    if let Some(frame) = circle.text_frame_mut() {
        frame.set_text("Circle");
    }
    
    slide2.add_shape(Box::new(circle))?;
    println!("✓ Added circle shape");
    
    // Add diamond shape (blue)
    let mut diamond = AutoShape::with_text_frame(
        4,
        "Diamond".to_string(),
        AutoShapeType::Diamond,
    );
    diamond.set_left(5943600); // 6.5"
    diamond.set_top(1628400); // 1.78"
    diamond.set_width(2286000); // 2.5"
    diamond.set_height(1371600); // 1.5"
    
    if let Some(frame) = diamond.text_frame_mut() {
        frame.set_text("Diamond");
    }
    
    slide2.add_shape(Box::new(diamond))?;
    println!("✓ Added diamond shape");
    
    // Add content text box
    let mut content_box = AutoShape::with_text_frame(
        5,
        "Content".to_string(),
        AutoShapeType::Rectangle,
    );
    content_box.set_left(457200); // 0.5"
    content_box.set_top(3657600); // 4"
    content_box.set_width(8229600); // 9"
    content_box.set_height(2286000); // 2.5"
    
    if let Some(frame) = content_box.text_frame_mut() {
        frame.set_text("Features Tested:\n• Text formatting (bold, colors, sizes)\n• Shape creation and positioning\n• Multiple slides and layouts");
    }
    
    slide2.add_shape(Box::new(content_box))?;
    println!("✓ Added content text box");
    
    // Save presentation
    let output_path = "examples/output/alignment_test_ppt_rs.pptx";
    prs.save_to_file(output_path)?;
    
    println!("\n✓ Presentation saved: {}", output_path);
    println!("  - Slides: {}", slides.len());
    println!("  - File size: {} bytes", fs::metadata(output_path)?.len());
    
    println!("\n=== Comparison Instructions ===");
    println!("1. Generate reference with python-pptx:");
    println!("   python3 generate_reference.py");
    println!("\n2. Compare the two files:");
    println!("   - examples/output/reference_python_pptx.pptx (python-pptx)");
    println!("   - examples/output/alignment_test_ppt_rs.pptx (ppt-rs)");
    println!("\n3. Extract and compare XML:");
    println!("   unzip -l examples/output/reference_python_pptx.pptx");
    println!("   unzip -l examples/output/alignment_test_ppt_rs.pptx");
    
    Ok(())
}
