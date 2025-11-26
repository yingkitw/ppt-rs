//! Example 3: Generate a business report presentation
//!
//! Run with: cargo run --example report_generator

use std::fs;
use chrono::Local;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating quarterly business report...\n");

    let output_file = "examples/output/quarterly_report.pptx";
    
    // Create output directory
    fs::create_dir_all("examples/output")?;

    // Create report with sections
    let report = QuarterlyReport {
        quarter: "Q1",
        year: 2025,
        revenue: 1_500_000.0,
        growth: 15.5,
        employees: 150,
    };

    let presentation = report.generate_presentation();
    
    // Write to file
    fs::write(output_file, presentation)?;

    println!("✓ Report generated: {}", output_file);
    println!("  Title: {} {} Business Report", report.quarter, report.year);
    println!("  Generated: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!("  Size: {} bytes", fs::metadata(output_file)?.len());

    Ok(())
}

struct QuarterlyReport {
    quarter: &'static str,
    year: u32,
    revenue: f64,
    growth: f64,
    employees: u32,
}

impl QuarterlyReport {
    fn generate_presentation(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<presentation>\n");
        xml.push_str(&format!(
            "  <title>{} {} Business Report</title>\n",
            self.quarter, self.year
        ));
        xml.push_str("  <slides count=\"6\">\n");

        // Slide 1: Title
        xml.push_str("    <slide number=\"1\">\n");
        xml.push_str(&format!(
            "      <title>{} {} Quarterly Report</title>\n",
            self.quarter, self.year
        ));
        xml.push_str("      <content>Business Performance Overview</content>\n");
        xml.push_str("    </slide>\n");

        // Slide 2: Executive Summary
        xml.push_str("    <slide number=\"2\">\n");
        xml.push_str("      <title>Executive Summary</title>\n");
        let revenue_str = format!("{:.0}", self.revenue);
        let growth_str = format!("{:.1}", self.growth);
        xml.push_str(&format!(
            "      <content>• Revenue: ${}\n• Growth: {}%\n• Team Size: {}</content>\n",
            revenue_str, growth_str, self.employees
        ));
        xml.push_str("    </slide>\n");

        // Slide 3: Financial Performance
        xml.push_str("    <slide number=\"3\">\n");
        xml.push_str("      <title>Financial Performance</title>\n");
        xml.push_str(&format!(
            "      <content>Total Revenue: ${}\nYear-over-Year Growth: {}%</content>\n",
            revenue_str, growth_str
        ));
        xml.push_str("    </slide>\n");

        // Slide 4: Team Metrics
        xml.push_str("    <slide number=\"4\">\n");
        xml.push_str("      <title>Team Metrics</title>\n");
        let avg_revenue = format!("{:.0}", self.revenue / self.employees as f64);
        xml.push_str(&format!(
            "      <content>Total Employees: {}\nAverage Revenue per Employee: ${}</content>\n",
            self.employees, avg_revenue
        ));
        xml.push_str("    </slide>\n");

        // Slide 5: Key Achievements
        xml.push_str("    <slide number=\"5\">\n");
        xml.push_str("      <title>Key Achievements</title>\n");
        xml.push_str("      <content>• Exceeded growth targets\n• Expanded team\n• Improved efficiency</content>\n");
        xml.push_str("    </slide>\n");

        // Slide 6: Next Quarter Goals
        xml.push_str("    <slide number=\"6\">\n");
        xml.push_str("      <title>Next Quarter Goals</title>\n");
        xml.push_str("      <content>• Increase revenue by 20%\n• Hire 20 new team members\n• Launch new products</content>\n");
        xml.push_str("    </slide>\n");

        xml.push_str("  </slides>\n");
        xml.push_str("</presentation>\n");
        xml
    }
}
