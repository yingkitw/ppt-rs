//! Simplified API Example
//!
//! This example demonstrates the new simplified API that makes creating
//! presentations easier, more concise, and more consistent.

use ppt_rs::prelude::*;

fn main() -> Result<()> {
    println!("🎨 Creating presentation with simplified API...");

    // =========================================================================
    // Example 1: Simple Shapes with Helper Functions
    // =========================================================================
    
    let slide1 = SlideContent::new("Simplified Shape Creation")
        .add_shape(
            rect(0.5, 1.5, 2.0, 1.0)
                .fill(hex("4F81BD"))
                .text("Rectangle")
        )
        .add_shape(
            circle(3.0, 1.5, 1.5)
                .fill(hex("9BBB59"))
                .text("Circle")
        )
        .add_shape(
            rounded_rect(5.5, 1.5, 2.0, 1.0)
                .fill(hex("C0504D"))
                .text("Rounded")
        )
        .add_shape(
            triangle(1.5, 3.0, 1.5, 1.2)
                .fill(hex("8064A2"))
                .text("Triangle")
        )
        .add_shape(
            diamond(4.0, 3.0, 1.5, 1.2)
                .fill(hex("F79646"))
                .text("Diamond")
        );

    // =========================================================================
    // Example 2: Using Color Helpers
    // =========================================================================
    
    let slide2 = SlideContent::new("Color Helpers - RGB and Hex")
        .add_shape(
            rect(1.0, 1.5, 2.0, 1.0)
                .fill(rgb(79, 129, 189))  // RGB values
                .stroke(rgb(0, 0, 0), 2.0)  // Black border, 2pt
                .text("RGB Color")
        )
        .add_shape(
            rect(4.0, 1.5, 2.0, 1.0)
                .fill(hex("#4CAF50"))  // Hex with #
                .stroke(hex("1B5E20"), 2.0)
                .text("Hex Color")
        )
        .add_shape(
            rect(7.0, 1.5, 1.5, 1.0)
                .fill(hex(colors::MATERIAL_BLUE))  // Color constant
                .text("Constant")
        );

    // =========================================================================
    // Example 3: Images with Simplified API
    // =========================================================================
    
    // Load image from file
    let photo_bytes = std::fs::read("examples/assets/dog.jpg")
        .unwrap_or_else(|_| vec![]);
    
    let slide3 = if !photo_bytes.is_empty() {
        SlideContent::new("Simplified Image API")
            .add_image(
                image(photo_bytes.clone())
                    .size(inches(2.0), inches(2.0))
                    .at(inches(1.0), inches(1.5))
                    .shadow()
                    .build()
            )
            .add_image(
                image(photo_bytes.clone())
                    .size(inches(2.0), inches(2.0))
                    .at(inches(4.0), inches(1.5))
                    .reflection()
                    .build()
            )
            .add_image(
                image(photo_bytes)
                    .size(inches(2.0), inches(2.0))
                    .at(inches(7.0), inches(1.5))
                    .glow()
                    .build()
            )
    } else {
        SlideContent::new("Simplified Image API")
            .add_bullet("No images found in examples/assets/")
    };

    // =========================================================================
    // Example 4: Color Aliases and Adjustments (NEW)
    // =========================================================================
    
    let slide4 = SlideContent::new("Color Aliases & Adjustments")
        .add_shape(
            rect(0.5, 1.5, 1.5, 0.8)
                .fill(red().to_color())
                .text("red()")
        )
        .add_shape(
            rect(2.5, 1.5, 1.5, 0.8)
                .fill(blue().to_color())
                .text("blue()")
        )
        .add_shape(
            rect(4.5, 1.5, 1.5, 0.8)
                .fill(green().to_color())
                .text("green()")
        )
        .add_shape(
            rect(6.5, 1.5, 1.5, 0.8)
                .fill(orange().to_color())
                .text("orange()")
        )
        .add_shape(
            rect(1.5, 2.8, 1.5, 0.8)
                .fill(material_blue().lighter(0.3).to_color())
                .text("lighter()")
        )
        .add_shape(
            rect(3.5, 2.8, 1.5, 0.8)
                .fill(material_blue().darker(0.3).to_color())
                .text("darker()")
        )
        .add_shape(
            rect(5.5, 2.8, 1.5, 0.8)
                .fill(red().mix(&blue(), 0.5).to_color())
                .text("mix()")
        );

    // =========================================================================
    // Example 5: Simple Table Creation (NEW)
    // =========================================================================
    
    let slide5 = SlideContent::new("Simple Table Creation")
        .table(
            QuickTable::new(4)
                .header(&["Name", "Role", "Department", "Status"])
                .row(&["Alice", "Engineer", "Product", "Active"])
                .row(&["Bob", "Designer", "UX", "Active"])
                .row(&["Carol", "Manager", "Ops", "On Leave"])
                .at(1.0, 1.5)
                .build()
        );

    // =========================================================================
    // Example 6: Using Prelude Color Constants
    // =========================================================================
    
    let slide4 = SlideContent::new("Prelude Color Constants")
        .add_shape(
            rect(0.5, 1.5, 1.5, 0.8)
                .fill(hex(colors::RED))
                .text("Red")
        )
        .add_shape(
            rect(2.5, 1.5, 1.5, 0.8)
                .fill(hex(colors::GREEN))
                .text("Green")
        )
        .add_shape(
            rect(4.5, 1.5, 1.5, 0.8)
                .fill(hex(colors::BLUE))
                .text("Blue")
        )
        .add_shape(
            rect(6.5, 1.5, 1.5, 0.8)
                .fill(hex(colors::MATERIAL_PURPLE))
                .text("Purple")
        )
        .add_shape(
            rect(1.5, 3.0, 1.5, 0.8)
                .fill(hex(colors::MATERIAL_ORANGE))
                .text("Orange")
        )
        .add_shape(
            rect(3.5, 3.0, 1.5, 0.8)
                .fill(hex(colors::MATERIAL_TEAL))
                .text("Teal")
        )
        .add_shape(
            rect(5.5, 3.0, 1.5, 0.8)
                .fill(hex(colors::CARBON_BLUE_60))
                .text("Carbon")
        );

    // =========================================================================
    // Example 5: Comparison - Old vs New API
    // =========================================================================
    
    let slide5 = SlideContent::new("API Comparison: Before & After")
        .add_bullet("Old: Shape::new(ShapeType::Rectangle, 500000, 1600000, 2000000, 1000000)")
        .add_bullet("New: rect(0.5, 1.6, 2.0, 1.0)")
        .add_bullet("")
        .add_bullet("Old: .with_fill(ShapeFill::new(\"4F81BD\"))")
        .add_bullet("New: .fill(hex(\"4F81BD\"))")
        .add_bullet("")
        .add_bullet("Old: .with_line(ShapeLine::new(\"FF0000\", 25400))")
        .add_bullet("New: .stroke(hex(\"FF0000\"), 2.0)")
        .add_bullet("")
        .add_bullet("Result: ~40% less code, much more readable!");

    // =========================================================================
    // Example 6: Flowchart with Prelude Shapes
    // =========================================================================
    
    let slide6 = SlideContent::new("Flowchart with Prelude Helpers")
        .add_shape(
            shapes::terminator(1.5, 1.0, 2.0, 0.6, "Start")
                .fill(hex(colors::MATERIAL_GREEN))
        )
        .add_shape(
            shapes::arrow_down(2.3, 1.7, 0.4, 0.4)
                .fill(hex(colors::GRAY))
        )
        .add_shape(
            shapes::process(1.5, 2.2, 2.0, 0.8, "Process Data")
                .fill(hex(colors::MATERIAL_BLUE))
        )
        .add_shape(
            shapes::arrow_down(2.3, 3.1, 0.4, 0.4)
                .fill(hex(colors::GRAY))
        )
        .add_shape(
            shapes::decision(1.75, 3.6, 1.5, "Valid?")
                .fill(hex(colors::MATERIAL_AMBER))
        )
        .add_shape(
            shapes::arrow_right(3.3, 4.2, 0.8, 0.3)
                .fill(hex(colors::GRAY))
        )
        .add_shape(
            shapes::terminator(4.5, 3.9, 2.0, 0.6, "End")
                .fill(hex(colors::MATERIAL_RED))
        );

    // =========================================================================
    // Build Presentation
    // =========================================================================
    
    let slides = vec![slide1, slide2, slide3, slide4, slide5, slide6];
    
    let pptx_data = create_pptx_with_content("Simplified API Demo", slides)?;
    std::fs::write("simplified_api.pptx", pptx_data)?;
    
    println!("✅ Created simplified_api.pptx");
    println!("📊 6 slides demonstrating the new simplified API");
    println!("🎯 Key improvements:");
    println!("   - Helper functions: rect(), circle(), image()");
    println!("   - Color helpers: rgb(), hex()");
    println!("   - Shorter methods: .fill(), .stroke(), .text()");
    println!("   - Readable units: inches() instead of EMU");
    println!("   - Color constants: colors::BLUE, colors::MATERIAL_GREEN");
    
    Ok(())
}
