//! Simple image example demonstrating the simplified API
//!
//! This example shows how easy it is to add images with the new chainable API.

use ppt_rs::generator::{SlideContent, ImageBuilder, create_pptx_with_content};
use ppt_rs::prelude::inches;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating presentation with simplified image API...");
    
    // Create some sample image data (1x1 red PNG)
    let red_png = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53,
        0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41,
        0x54, 0x08, 0xD7, 0x63, 0xF8, 0xCF, 0xC0, 0x00,
        0x00, 0x03, 0x01, 0x01, 0x00, 0x18, 0xDD, 0x8D,
        0xB4, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E,
        0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    
    let slides = vec![
        // Slide 1: Simple image from bytes with auto-detection
        SlideContent::new("Auto-Detection")
            .add_bullet("Image format auto-detected from bytes")
            .add_bullet("Default size (2 inches)")
            .add_bullet("Simple positioning with inches()")
            .with_images(vec![
                ImageBuilder::auto(red_png.clone())
                    .at(inches(1.0), inches(3.0))
                    .build()
            ]),
        
        // Slide 2: Image with shadow effect
        SlideContent::new("Shadow Effect")
            .add_bullet("Chainable shadow() method")
            .add_bullet("No need to call build_with_shadow()")
            .with_images(vec![
                ImageBuilder::auto(red_png.clone())
                    .at(inches(2.0), inches(3.0))
                    .shadow()
                    .build()
            ]),
        
        // Slide 3: Image with multiple effects
        SlideContent::new("Multiple Effects")
            .add_bullet("Chain multiple effects together")
            .add_bullet("Shadow + Reflection + Glow")
            .with_images(vec![
                ImageBuilder::auto(red_png.clone())
                    .at(inches(2.0), inches(3.0))
                    .shadow()
                    .reflection()
                    .glow()
                    .build()
            ]),
        
        // Slide 4: Custom size and cropping
        SlideContent::new("Size & Crop")
            .add_bullet("Custom size with size() method")
            .add_bullet("Crop 10% from each side")
            .with_images(vec![
                ImageBuilder::auto(red_png.clone())
                    .size(inches(3.0), inches(2.0))
                    .at(inches(1.5), inches(3.0))
                    .crop(0.1, 0.1, 0.1, 0.1)
                    .build()
            ]),
        
        // Slide 5: All features combined
        SlideContent::new("All Together")
            .add_bullet("Size, position, effects, and crop")
            .add_bullet("All in one fluent chain")
            .with_images(vec![
                ImageBuilder::auto(red_png)
                    .size(inches(2.5), inches(2.5))
                    .at(inches(2.0), inches(2.5))
                    .shadow()
                    .soft_edges()
                    .crop(0.05, 0.05, 0.05, 0.05)
                    .build()
            ]),
    ];
    
    // Generate the presentation
    let pptx_data = create_pptx_with_content("Simple Images Demo", slides)?;
    std::fs::write("simple_images.pptx", pptx_data)?;
    
    println!("✓ Created simple_images.pptx");
    println!("  - 5 slides demonstrating simplified image API");
    println!("  - Auto-detection, effects, sizing, and cropping");
    
    Ok(())
}
