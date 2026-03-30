//! Comprehensive integrated example using all PPTX modules
//!
//! Run with: cargo run --example integrated_example

use ppt_rs::pptx;
use ppt_rs::{create_pptx_with_content, SlideContent};
use std::fs;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Integrated PPTX Generation Example\n");

    fs::create_dir_all("examples/output")?;

    println!("1. Creating presentation with Presentation API...");
    let pres = ppt_rs::Presentation::with_title("Integrated Example")
        .add_slide(SlideContent::new("Welcome").add_bullet("Point 1"))
        .add_slide(SlideContent::new("Details").add_bullet("Point 2"));
    pres.save("examples/output/integrated_example.pptx")?;
    println!("   Created: examples/output/integrated_example.pptx");

    println!("2. Creating presentation with content API...");
    let slides = vec![
        SlideContent::new("Business Report").add_bullet("Q1 Results"),
        SlideContent::new("Key Metrics").add_bullet("Revenue up 20%"),
    ];
    let data = create_pptx_with_content("Business Report", slides)?;
    fs::write("examples/output/business_report.pptx", data)?;
    println!("   Created: examples/output/business_report.pptx");

    println!("3. Creating presentation with prelude helpers...");
    pptx!("Helper Demo")
        .slide("Shapes", &["Using helpers"])
        .save("examples/output/helper_demo.pptx")?;
    println!("   Created: examples/output/helper_demo.pptx");

    println!("\nDone!");
    Ok(())
}
