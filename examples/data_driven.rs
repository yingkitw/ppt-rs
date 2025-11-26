//! Example 6: Generate presentation from data structures
//!
//! Run with: cargo run --example data_driven

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating data-driven presentation...\n");

    let output_file = "examples/output/data_driven.pptx";
    
    // Create output directory
    fs::create_dir_all("examples/output")?;

    // Define data
    let company_data = CompanyData {
        name: "TechCorp Inc.".to_string(),
        founded: 2015,
        employees: 500,
        offices: vec!["San Francisco", "New York", "London", "Tokyo"],
        products: vec![
            Product {
                name: "Product A".to_string(),
                revenue: 5_000_000.0,
                users: 50_000,
            },
            Product {
                name: "Product B".to_string(),
                revenue: 3_000_000.0,
                users: 30_000,
            },
            Product {
                name: "Product C".to_string(),
                revenue: 2_000_000.0,
                users: 20_000,
            },
        ],
    };

    let presentation = company_data.generate_presentation();
    
    // Write to file
    fs::write(output_file, presentation)?;

    println!("✓ Data-driven presentation generated: {}", output_file);
    println!("  Company: {}", company_data.name);
    println!("  Founded: {}", company_data.founded);
    println!("  Employees: {}", company_data.employees);
    println!("  Offices: {}", company_data.offices.len());
    println!("  Products: {}", company_data.products.len());
    println!("  Size: {} bytes", fs::metadata(output_file)?.len());

    Ok(())
}

struct Product {
    name: String,
    revenue: f64,
    users: u32,
}

struct CompanyData {
    name: String,
    founded: u32,
    employees: u32,
    offices: Vec<&'static str>,
    products: Vec<Product>,
}

impl CompanyData {
    fn generate_presentation(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<presentation>\n");
        xml.push_str(&format!("  <title>{} - Company Overview</title>\n", escape_xml(&self.name)));

        let total_slides = 4 + self.products.len();
        xml.push_str(&format!("  <slides count=\"{}\">\n", total_slides));

        let mut slide_num = 1;

        // Slide 1: Title
        xml.push_str(&format!("    <slide number=\"{}\">\n", slide_num));
        xml.push_str(&format!("      <title>{}</title>\n", escape_xml(&self.name)));
        xml.push_str(&format!("      <content>Founded: {}</content>\n", self.founded));
        xml.push_str("    </slide>\n");
        slide_num += 1;

        // Slide 2: Company Overview
        xml.push_str(&format!("    <slide number=\"{}\">\n", slide_num));
        xml.push_str("      <title>Company Overview</title>\n");
        xml.push_str(&format!(
            "      <content>• Employees: {}\n• Offices: {}\n• Founded: {}</content>\n",
            self.employees,
            self.offices.len(),
            self.founded
        ));
        xml.push_str("    </slide>\n");
        slide_num += 1;

        // Slide 3: Global Presence
        xml.push_str(&format!("    <slide number=\"{}\">\n", slide_num));
        xml.push_str("      <title>Global Presence</title>\n");
        let offices = self.offices.join(", ");
        xml.push_str(&format!("      <content>Offices in: {}</content>\n", escape_xml(&offices)));
        xml.push_str("    </slide>\n");
        slide_num += 1;

        // Product slides
        for product in &self.products {
            xml.push_str(&format!("    <slide number=\"{}\">\n", slide_num));
            xml.push_str(&format!("      <title>{}</title>\n", escape_xml(&product.name)));
            let revenue_str = format!("{:.0}", product.revenue);
            let users_str = format!("{}", product.users);
            xml.push_str(&format!(
                "      <content>• Revenue: ${}\n• Users: {}</content>\n",
                revenue_str, users_str
            ));
            xml.push_str("    </slide>\n");
            slide_num += 1;
        }

        // Final slide
        xml.push_str(&format!("    <slide number=\"{}\">\n", slide_num));
        xml.push_str("      <title>Thank You</title>\n");
        xml.push_str("      <content>Visit us at www.example.com</content>\n");
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
