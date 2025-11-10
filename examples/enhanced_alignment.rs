/// Enhanced Alignment Test Example
/// 
/// This example generates a PPTX file with slides that aligns with python-pptx output.
/// It demonstrates:
/// - Presentation metadata (title, author, subject, keywords, comments)
/// - Multiple slides with content
/// - Proper XML structure matching python-pptx

use ppt_rs::PresentationBuilder;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Enhanced Alignment Test: ppt-rs vs python-pptx ===\n");
    
    // Create output directory
    fs::create_dir_all("examples/output")?;
    
    // Create presentation with metadata matching python-pptx
    let mut prs = PresentationBuilder::new()
        .title("Enhanced Alignment Test Presentation")
        .author("ppt-rs Team")
        .subject("Testing ppt-rs alignment with python-pptx - with slides")
        .keywords("pptx, rust, python-pptx, alignment, slides")
        .comments("This presentation tests alignment between ppt-rs and python-pptx with slide content")
        .build()?;
    
    println!("✓ Created presentation with metadata");
    
    // Get core properties
    if let Ok(props) = prs.core_properties() {
        println!("  - Title: {}", props.title().unwrap_or(""));
        println!("  - Creator: {}", props.creator().unwrap_or(""));
        println!("  - Subject: {}", props.subject().unwrap_or(""));
    }
    
    // Add slides
    println!("\n--- Adding Slides ---");
    
    // Add slide 1
    match prs.add_slide() {
        Ok(idx) => {
            println!("✓ Added slide {} (Title Slide)", idx + 1);
        }
        Err(e) => {
            println!("⚠ Warning: Could not add slide 1: {}", e);
        }
    }
    
    // Add slide 2
    match prs.add_slide() {
        Ok(idx) => {
            println!("✓ Added slide {} (Content Slide)", idx + 1);
        }
        Err(e) => {
            println!("⚠ Warning: Could not add slide 2: {}", e);
        }
    }
    
    // Save presentation
    let output_path = "examples/output/enhanced_alignment_ppt_rs.pptx";
    prs.save_to_file(output_path)?;
    
    println!("\n✓ Presentation saved: {}", output_path);
    println!("  - File size: {} bytes", fs::metadata(output_path)?.len());
    
    println!("\n=== Comparison Instructions ===");
    println!("1. Compare with reference:");
    println!("   python3 compare_pptx.py");
    println!("\n2. Or manually extract:");
    println!("   unzip -l examples/output/reference_python_pptx.pptx");
    println!("   unzip -l examples/output/enhanced_alignment_ppt_rs.pptx");
    println!("\n3. Verify in PowerPoint:");
    println!("   open examples/output/enhanced_alignment_ppt_rs.pptx");
    
    Ok(())
}
