//! Test: Create presentation with actual slides
//! 
//! This example creates a presentation with 3 slides to verify
//! that our slide generation matches python-pptx output.

use ppt_rs::new_presentation;
use ppt_rs::parts::slide::SlidePart;
use ppt_rs::opc::packuri::PackURI;
use ppt_rs::opc::part::Part;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating presentation with 3 slides...");
    
    let mut prs = new_presentation()?;
    
    // Add 3 slides
    for i in 1..=3 {
        // Create slide part
        let slide_uri = PackURI::new(&format!("/ppt/slides/slide{}.xml", i))?;
        let slide_part = SlidePart::new(slide_uri, prs.part() as &dyn Part)?;
        
        // Add slide to presentation
        unsafe {
            let prs_ptr: *mut _ = &mut prs;
            let slides = &mut (*prs_ptr).slides();
            let package = &mut (*prs_ptr).package_mut();
            slides.add_slide(&slide_part, package)?;
        }
        
        println!("✓ Added slide {}", i);
    }
    
    println!("✓ Total slides: {}", prs.slides().len());
    
    // Save
    let output_path = "examples/output/05_test_slides.pptx";
    std::fs::create_dir_all("examples/output").ok();
    prs.save_to_file(output_path)?;
    println!("✓ Saved to {}", output_path);
    
    // Verify file
    if std::path::Path::new(output_path).exists() {
        let file_size = std::fs::metadata(output_path)?.len();
        println!("✓ File size: {} bytes", file_size);
    }
    
    Ok(())
}
