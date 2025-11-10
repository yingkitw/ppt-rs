/// Simple Alignment Test Example
/// 
/// This example generates a basic PPTX file aligned with python-pptx output.
/// It demonstrates:
/// - Presentation metadata (title, author, subject, keywords, comments)
/// - Basic presentation creation
/// - Proper XML structure matching python-pptx

use ppt_rs::PresentationBuilder;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simple Alignment Test: ppt-rs vs python-pptx ===\n");
    
    // Create output directory
    fs::create_dir_all("examples/output")?;
    
    // Create presentation with metadata matching python-pptx
    let mut prs = PresentationBuilder::new()
        .title("Alignment Test Presentation")
        .author("ppt-rs Team")
        .subject("Testing ppt-rs alignment with python-pptx")
        .keywords("pptx, rust, python-pptx, alignment")
        .comments("This presentation tests alignment between ppt-rs and python-pptx")
        .build()?;
    
    println!("✓ Created presentation with metadata");
    
    // Get core properties
    if let Ok(props) = prs.core_properties() {
        println!("  - Title: {}", props.title().unwrap_or(""));
        println!("  - Creator: {}", props.creator().unwrap_or(""));
        println!("  - Subject: {}", props.subject().unwrap_or(""));
    }
    
    // Save presentation
    let output_path = "examples/output/simple_alignment_ppt_rs.pptx";
    prs.save_to_file(output_path)?;
    
    println!("\n✓ Presentation saved: {}", output_path);
    println!("  - File size: {} bytes", fs::metadata(output_path)?.len());
    
    println!("\n=== Comparison Instructions ===");
    println!("1. Generate reference with python-pptx:");
    println!("   python3 generate_reference.py");
    println!("\n2. Compare the two files:");
    println!("   - examples/output/reference_python_pptx.pptx (python-pptx)");
    println!("   - examples/output/simple_alignment_ppt_rs.pptx (ppt-rs)");
    println!("\n3. Extract and compare XML:");
    println!("   python3 compare_pptx.py");
    println!("\n4. Or manually extract:");
    println!("   unzip -l examples/output/reference_python_pptx.pptx");
    println!("   unzip -l examples/output/simple_alignment_ppt_rs.pptx");
    
    Ok(())
}
