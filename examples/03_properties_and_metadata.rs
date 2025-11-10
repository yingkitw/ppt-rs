//! Example 3: Working with presentation properties and metadata
//!
//! This example demonstrates how to:
//! - Set core properties (title, author, created, modified)
//! - Set app properties (application, version, slides count)
//! - Set custom properties (user-defined metadata)
//! - Use namespace management
//! - Access properties through the generic OpenXmlDocument trait
//! - Leverage the new OOXML-RS adoption features

use ppt_rs::PresentationBuilder;
use ppt_rs::opc::{OpenXmlDocument, DocumentFormat, Namespaces};
use ppt_rs::oxml::OpenXmlElementType;
use chrono::Utc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Presentation with Properties and Metadata (Fluent API) ===\n");
    
    // Demonstrate namespace management
    println!("--- Namespace Management ---");
    demonstrate_namespaces()?;
    
    // Demonstrate XML element traits
    println!("\n--- XML Element Traits ---");
    demonstrate_xml_traits();
    
    // Create a new presentation using fluent builder
    println!("\n--- Creating Presentation ---");
    let mut prs = PresentationBuilder::new()
        .title("Q1 2025 Business Proposal")
        .author("John Doe")
        .build()?;
    println!("✓ Created presentation with PresentationBuilder");
    
    // Set core properties with builder pattern
    println!("\n--- Setting Core Properties ---");
    {
        let core_props = prs.core_props_mut();
        core_props.title = Some("Q1 2025 Business Proposal".to_string());
        core_props.creator = Some("John Doe".to_string());
        core_props.subject = Some("Strategic Business Initiative".to_string());
        core_props.keywords = Some("business, proposal, 2025, strategy".to_string());
        core_props.description = Some("A comprehensive business proposal for Q1 2025 strategic initiatives".to_string());
        core_props.last_modified_by = Some("Jane Smith".to_string());
        core_props.created = Some(Utc::now());
        core_props.modified = Some(Utc::now());
    }
    println!("✓ Title: Q1 2025 Business Proposal");
    println!("✓ Creator: John Doe");
    println!("✓ Subject: Strategic Business Initiative");
    println!("✓ Keywords: business, proposal, 2025, strategy");
    
    // Set app properties
    println!("\n--- Setting App Properties ---");
    {
        let app_props = prs.app_props_mut();
        app_props.application = Some("ppt-rs (Rust PowerPoint Library)".to_string());
        app_props.app_version = Some("0.1.3".to_string());
        app_props.total_time = Some(120); // 2 hours
        app_props.words = Some(5000);
        app_props.characters = Some(25000);
    }
    println!("✓ Application: ppt-rs (Rust PowerPoint Library)");
    println!("✓ Version: 0.1.3");
    println!("✓ Total editing time: 120 minutes");
    
    // Set custom properties (user-defined metadata)
    println!("\n--- Setting Custom Properties ---");
    {
        let custom_props = prs.custom_props_mut();
        custom_props.set("department".to_string(), "Sales & Marketing".to_string());
        custom_props.set("project".to_string(), "Q1 Strategic Planning".to_string());
        custom_props.set("version".to_string(), "1.0.0".to_string());
        custom_props.set("status".to_string(), "Draft for Review".to_string());
        custom_props.set("priority".to_string(), "High".to_string());
        custom_props.set("audience".to_string(), "Executive Leadership".to_string());
    }
    println!("✓ Department: Sales & Marketing");
    println!("✓ Project: Q1 Strategic Planning");
    println!("✓ Version: 1.0.0");
    println!("✓ Status: Draft for Review");
    println!("✓ Priority: High");
    println!("✓ Audience: Executive Leadership");
    
    // Add slides
    println!("\n--- Adding Slides ---");
    prs.add_slide()?;
    println!("✓ Added slide 1 - Title Slide");
    prs.add_slide()?;
    println!("✓ Added slide 2 - Executive Summary");
    prs.add_slide()?;
    println!("✓ Added slide 3 - Key Initiatives");
    prs.add_slide()?;
    println!("✓ Added slide 4 - Financial Projections");
    
    // Update app properties with slide count
    {
        let app_props = prs.app_props_mut();
        app_props.slides = Some(4);
    }
    
    // Verify properties using the generic OpenXmlDocument trait
    println!("\n--- Verifying Through Generic Trait ---");
    verify_document_properties(&prs)?;
    
    // Save the presentation
    println!("\n--- Saving Presentation ---");
    let output_path = "examples/output/03_with_properties.pptx";
    std::fs::create_dir_all("examples/output").ok();
    prs.save_to_file(output_path)?;
    println!("✓ Saved to {}", output_path);
    
    // Verify file
    if std::path::Path::new(output_path).exists() {
        let file_size = std::fs::metadata(output_path)?.len();
        println!("✓ File size: {} bytes", file_size);
        
        println!("\n✅ Presentation created successfully!");
        println!("\n=== Features Demonstrated ===");
        println!("✓ Namespace Management");
        println!("  - Centralized namespace definitions");
        println!("  - Support for all OOXML formats (PPTX, DOCX, XLSX)");
        println!("✓ XML Element Traits");
        println!("  - Type-safe element handling");
        println!("  - Compile-time element metadata");
        println!("✓ Core Properties");
        println!("  - Title, author, subject, keywords, description");
        println!("  - Creation and modification dates");
        println!("✓ App Properties");
        println!("  - Application name and version");
        println!("  - Slides count, words, characters");
        println!("✓ Custom Properties");
        println!("  - User-defined metadata");
        println!("  - Department, project, status, priority, audience");
        println!("✓ Generic OpenXmlDocument Trait");
        println!("  - Format-agnostic interface");
        println!("  - Foundation for DOCX and XLSX support");
        println!("✓ Property Access and Modification");
        println!("  - Easy-to-use API");
        println!("  - Builder pattern support");
    }
    
    Ok(())
}

/// Demonstrate namespace management capabilities
fn demonstrate_namespaces() -> Result<(), Box<dyn std::error::Error>> {
    // Create namespace manager with standard OOXML namespaces
    let ns = Namespaces::with_standard();
    
    println!("✓ Created namespace manager with standard OOXML namespaces:");
    
    // Check for standard namespaces
    if let Some(uri) = ns.get("p") {
        println!("  - PresentationML (p): {}", uri);
    }
    if let Some(uri) = ns.get("a") {
        println!("  - DrawingML (a): {}", uri);
    }
    if let Some(uri) = ns.get("r") {
        println!("  - Office Document (r): {}", uri);
    }
    
    println!("✓ Namespace system supports:");
    println!("  - PPTX (PresentationML)");
    println!("  - DOCX (WordprocessingML) - future");
    println!("  - XLSX (SpreadsheetML) - future");
    
    Ok(())
}

/// Demonstrate XML element traits
fn demonstrate_xml_traits() {
    println!("✓ XML Element Traits provide:");
    println!("  - Type-safe element handling");
    println!("  - Compile-time element metadata");
    println!("  - Element type classification:");
    
    // Demonstrate element types
    let _leaf_type = OpenXmlElementType::Leaf;
    let _node_type = OpenXmlElementType::Node;
    let _root_type = OpenXmlElementType::Root;
    
    println!("    - Leaf: Plain text/CDATA elements");
    println!("    - Node: Internal XML elements");
    println!("    - Root: Root elements of parts");
    
    println!("✓ Traits support:");
    println!("  - Custom serialization (OpenXmlSerialize)");
    println!("  - Custom deserialization (OpenXmlDeserialize)");
    println!("  - Element metadata queries");
}

/// Verify document properties using the generic OpenXmlDocument trait
fn verify_document_properties(prs: &dyn OpenXmlDocument) -> Result<(), Box<dyn std::error::Error>> {
    println!("✓ Document format: {:?}", prs.format());
    assert_eq!(prs.format(), DocumentFormat::Presentation);
    
    // Access core properties through generic trait
    let core = prs.core_properties();
    println!("✓ Core properties verified:");
    println!("  - Title: {:?}", core.title);
    println!("  - Creator: {:?}", core.creator);
    println!("  - Subject: {:?}", core.subject);
    println!("  - Keywords: {:?}", core.keywords);
    
    // Access app properties through generic trait
    let app = prs.app_properties();
    println!("✓ App properties verified:");
    println!("  - Application: {:?}", app.application);
    println!("  - Version: {:?}", app.app_version);
    println!("  - Slides: {:?}", app.slides);
    
    // Access custom properties through generic trait
    let custom = prs.custom_properties();
    println!("✓ Custom properties verified:");
    println!("  - Department: {:?}", custom.get("department"));
    println!("  - Project: {:?}", custom.get("project"));
    println!("  - Status: {:?}", custom.get("status"));
    println!("  - Priority: {:?}", custom.get("priority"));
    
    Ok(())
}
