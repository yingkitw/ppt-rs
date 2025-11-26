//! Example 5: Generate training material presentation
//!
//! Run with: cargo run --example training_material

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating Rust training material...\n");

    let output_file = "examples/output/rust_training.pptx";
    
    // Create output directory
    fs::create_dir_all("examples/output")?;

    let course = TrainingCourse {
        title: "Introduction to Rust".to_string(),
        duration_hours: 8,
        modules: vec![
            Module {
                name: "Getting Started".to_string(),
                topics: vec![
                    "Installation",
                    "Hello World",
                    "Cargo",
                ],
            },
            Module {
                name: "Ownership & Borrowing".to_string(),
                topics: vec![
                    "Ownership Rules",
                    "References",
                    "Lifetimes",
                ],
            },
            Module {
                name: "Error Handling".to_string(),
                topics: vec![
                    "Result Type",
                    "Option Type",
                    "? Operator",
                ],
            },
            Module {
                name: "Advanced Topics".to_string(),
                topics: vec![
                    "Traits",
                    "Generics",
                    "Macros",
                ],
            },
        ],
    };

    let presentation = course.generate_presentation();
    
    // Write to file
    fs::write(output_file, presentation)?;

    println!("✓ Training material generated: {}", output_file);
    println!("  Title: {}", course.title);
    println!("  Duration: {} hours", course.duration_hours);
    println!("  Modules: {}", course.modules.len());
    println!("  Size: {} bytes", fs::metadata(output_file)?.len());

    Ok(())
}

struct Module {
    name: String,
    topics: Vec<&'static str>,
}

struct TrainingCourse {
    title: String,
    duration_hours: u32,
    modules: Vec<Module>,
}

impl TrainingCourse {
    fn generate_presentation(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<presentation>\n");
        xml.push_str(&format!("  <title>{}</title>\n", escape_xml(&self.title)));

        let total_slides = 2 + self.modules.len() * 2; // Title + Agenda + (Module intro + content) * modules + Conclusion
        xml.push_str(&format!("  <slides count=\"{}\">\n", total_slides));

        let mut slide_num = 1;

        // Slide 1: Title
        xml.push_str(&format!("    <slide number=\"{}\">\n", slide_num));
        xml.push_str(&format!("      <title>{}</title>\n", escape_xml(&self.title)));
        xml.push_str(&format!("      <content>Duration: {} hours</content>\n", self.duration_hours));
        xml.push_str("    </slide>\n");
        slide_num += 1;

        // Slide 2: Agenda
        xml.push_str(&format!("    <slide number=\"{}\">\n", slide_num));
        xml.push_str("      <title>Course Agenda</title>\n");
        let mut agenda = String::new();
        for (i, module) in self.modules.iter().enumerate() {
            if i > 0 {
                agenda.push('\n');
            }
            agenda.push_str(&format!("{}. {}", i + 1, module.name));
        }
        xml.push_str(&format!("      <content>{}</content>\n", escape_xml(&agenda)));
        xml.push_str("    </slide>\n");
        slide_num += 1;

        // Module slides
        for module in &self.modules {
            // Module intro slide
            xml.push_str(&format!("    <slide number=\"{}\">\n", slide_num));
            xml.push_str(&format!("      <title>{}</title>\n", escape_xml(&module.name)));
            xml.push_str("      <content>Module Overview</content>\n");
            xml.push_str("    </slide>\n");
            slide_num += 1;

            // Module content slide
            xml.push_str(&format!("    <slide number=\"{}\">\n", slide_num));
            xml.push_str(&format!("      <title>{} - Topics</title>\n", escape_xml(&module.name)));
            let topics = module
                .topics
                .iter()
                .map(|t| format!("• {}", t))
                .collect::<Vec<_>>()
                .join("\n");
            xml.push_str(&format!("      <content>{}</content>\n", escape_xml(&topics)));
            xml.push_str("    </slide>\n");
            slide_num += 1;
        }

        // Final slide
        xml.push_str(&format!("    <slide number=\"{}\">\n", slide_num));
        xml.push_str("      <title>Thank You!</title>\n");
        xml.push_str("      <content>Questions and Discussion</content>\n");
        xml.push_str("    </slide>\n");

        xml.push_str("  </slides>\n");
        xml.push_str("</presentation>\n");
        xml
    }
}

fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}
