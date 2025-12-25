//! Template demonstration example
//!
//! This example demonstrates all the new template features:
//! - Business proposal template
//! - Status report template
//! - Training material template
//! - Technical documentation template
//! - Theme colors and layout helpers

use ppt_rs::templates::{
    self, ProposalContent, StatusContent, TrainingContent, TechnicalContent,
};
use ppt_rs::prelude::*;
use ppt_rs::pptx;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // 1. Business Proposal Template
    println!("Creating business proposal...");
    let proposal = templates::business_proposal(
        "Q4 Budget Proposal 2025",
        "Finance Team",
        ProposalContent {
            executive_summary: vec![
                "Requesting $500,000 for digital transformation",
                "Expected ROI of 300% within 18 months",
                "Aligns with corporate strategic goals",
            ],
            problem: vec![
                "Legacy systems causing 40% productivity loss",
                "Customer satisfaction declining due to slow response",
                "Competitors gaining market share with modern solutions",
            ],
            solution: vec![
                "Implement cloud-native infrastructure",
                "Deploy AI-powered customer service",
                "Modernize internal workflows with automation",
            ],
            timeline: vec![
                ("Phase 1: Planning", "Q1 2025"),
                ("Phase 2: Development", "Q2-Q3 2025"),
                ("Phase 3: Deployment", "Q4 2025"),
                ("Phase 4: Optimization", "Q1 2026"),
            ],
            budget: vec![
                ("Infrastructure", "$200,000"),
                ("Development", "$150,000"),
                ("Training", "$50,000"),
                ("Contingency", "$100,000"),
            ],
            next_steps: vec![
                "Executive approval by Jan 15",
                "Vendor selection by Feb 1",
                "Team assembly by Feb 15",
                "Kickoff meeting Mar 1",
            ],
        },
    )?;
    std::fs::write("examples/output/proposal.pptx", proposal)?;
    println!("  ✓ Created examples/output/proposal.pptx");

    // 2. Status Report Template
    println!("Creating status report...");
    let status = templates::status_report(
        "Project Alpha - Weekly Status",
        "Week of December 23, 2025",
        StatusContent {
            summary: vec![
                "Sprint 12 completed on schedule",
                "All critical features delivered",
                "On track for Q1 release",
            ],
            completed: vec![
                "User authentication module",
                "Dashboard redesign",
                "API performance optimization",
                "Security audit remediation",
            ],
            in_progress: vec![
                "Mobile responsive layouts",
                "Export functionality",
                "Integration testing",
            ],
            blocked: vec![
                "Third-party API access pending approval",
            ],
            next_week: vec![
                "Complete mobile layouts",
                "Begin UAT testing",
                "Prepare deployment documentation",
            ],
            metrics: vec![
                ("Velocity", "42 story points"),
                ("Bug Count", "3 (down from 12)"),
                ("Test Coverage", "87%"),
                ("Sprint Burndown", "On track"),
            ],
        },
    )?;
    std::fs::write("examples/output/status_report.pptx", status)?;
    println!("  ✓ Created examples/output/status_report.pptx");

    // 3. Training Material Template
    println!("Creating training material...");
    let training = templates::training_material(
        "Introduction to Rust Programming",
        "DevOps Academy",
        TrainingContent {
            objectives: vec![
                "Understand Rust's ownership model",
                "Write safe, concurrent code",
                "Build and test Rust applications",
                "Integrate Rust with existing systems",
            ],
            modules: vec![
                ("Module 1: Getting Started", vec![
                    "Installing Rust toolchain",
                    "Hello World program",
                    "Cargo basics",
                    "VS Code setup",
                ]),
                ("Module 2: Ownership & Borrowing", vec![
                    "Stack vs Heap",
                    "Move semantics",
                    "References and borrowing",
                    "Lifetimes basics",
                ]),
                ("Module 3: Error Handling", vec![
                    "Result and Option types",
                    "Pattern matching",
                    "The ? operator",
                    "Custom error types",
                ]),
            ],
            exercises: vec![
                "Build a CLI calculator",
                "Implement a linked list",
                "Create a REST API client",
                "Write unit tests for all modules",
            ],
            summary: vec![
                "Rust provides memory safety without garbage collection",
                "Ownership model prevents data races at compile time",
                "Cargo makes dependency management easy",
                "Strong type system catches errors early",
            ],
        },
    )?;
    std::fs::write("examples/output/training.pptx", training)?;
    println!("  ✓ Created examples/output/training.pptx");

    // 4. Technical Documentation Template
    println!("Creating technical documentation...");
    let tech_doc = templates::technical_doc(
        "ppt-rs Library Architecture",
        "0.2.0",
        TechnicalContent {
            overview: vec![
                "Rust library for PowerPoint generation",
                "ECMA-376 Office Open XML compliant",
                "Zero runtime dependencies on Microsoft Office",
                "Cross-platform support",
            ],
            architecture: vec![
                "Layered architecture: API → Generator → Parts → OPC",
                "Trait-based design for extensibility",
                "Builder pattern for fluent API",
                "Modular components for maintainability",
            ],
            components: vec![
                ("Generator Module", vec![
                    "SlideContent - Slide builder",
                    "Shape - Shape creation",
                    "Table - Table builder",
                    "Chart - Chart builder",
                ]),
                ("Parts Module", vec![
                    "SlidePart - Individual slides",
                    "ImagePart - Embedded images",
                    "ChartPart - Embedded charts",
                    "MediaPart - Video/audio",
                ]),
                ("CLI Module", vec![
                    "md2ppt - Markdown conversion",
                    "validate - PPTX validation",
                    "create - Quick presentation",
                ]),
            ],
            api_examples: vec![
                ("create_pptx()", "Create basic presentation"),
                ("SlideContent::new()", "Build slide content"),
                ("Shape::new()", "Create shapes"),
                ("TableBuilder::new()", "Build tables"),
            ],
            best_practices: vec![
                "Use prelude module for simplified API",
                "Leverage templates for common use cases",
                "Use layout helpers for consistent positioning",
                "Test presentations with validation command",
            ],
        },
    )?;
    std::fs::write("examples/output/tech_doc.pptx", tech_doc)?;
    println!("  ✓ Created examples/output/tech_doc.pptx");

    // 5. Simple Template
    println!("Creating simple presentation...");
    let simple = templates::simple("Quick Presentation", &[
        ("Welcome", &[
            "Hello, World!",
            "This is a simple presentation",
            "Created with ppt-rs templates",
        ]),
        ("Features", &[
            "Easy to use API",
            "Multiple templates",
            "Theme support",
        ]),
        ("Conclusion", &[
            "Try ppt-rs today!",
            "Visit github.com/yingkitw/ppt-rs",
        ]),
    ])?;
    std::fs::write("examples/output/simple.pptx", simple)?;
    println!("  ✓ Created examples/output/simple.pptx");

    // 6. Theme showcase using prelude
    println!("Creating theme showcase...");
    let theme_pptx = pptx!("Theme Showcase")
        .slide("Available Themes", &[
            &format!("Corporate: Primary {} / Accent {}", 
                themes::CORPORATE.primary, themes::CORPORATE.accent),
            &format!("Modern: Primary {} / Accent {}",
                themes::MODERN.primary, themes::MODERN.accent),
            &format!("Vibrant: Primary {} / Accent {}",
                themes::VIBRANT.primary, themes::VIBRANT.accent),
            &format!("Dark: Primary {} / Accent {}",
                themes::DARK.primary, themes::DARK.accent),
            &format!("Carbon: Primary {} / Accent {}",
                themes::CARBON.primary, themes::CARBON.accent),
        ])
        .slide("Material Colors", &[
            &format!("Red: {}", colors::MATERIAL_RED),
            &format!("Blue: {}", colors::MATERIAL_BLUE),
            &format!("Green: {}", colors::MATERIAL_GREEN),
            &format!("Orange: {}", colors::MATERIAL_ORANGE),
        ])
        .slide("Carbon Colors", &[
            &format!("Blue 60: {}", colors::CARBON_BLUE_60),
            &format!("Green 50: {}", colors::CARBON_GREEN_50),
            &format!("Gray 100: {}", colors::CARBON_GRAY_100),
        ])
        .build()?;
    std::fs::write("examples/output/themes.pptx", theme_pptx)?;
    println!("  ✓ Created examples/output/themes.pptx");

    // 7. Layout helpers demo
    println!("Creating layout demo...");
    let grid_positions = layouts::grid(2, 3, 1500000, 1000000);
    let mut grid_shapes = Vec::new();
    let theme_colors = [
        themes::CORPORATE.primary,
        themes::MODERN.primary,
        themes::VIBRANT.primary,
        themes::DARK.primary,
        themes::NATURE.primary,
        themes::TECH.primary,
    ];
    
    for (i, (x, y)) in grid_positions.iter().enumerate() {
        grid_shapes.push(
            shapes::rect_emu(*x, *y, 1400000, 900000)
                .with_fill(ShapeFill::new(theme_colors[i]))
                .with_text(&format!("Theme {}", i + 1))
        );
    }

    let layout_pptx = pptx!("Layout Helpers Demo")
        .shapes_slide("Grid Layout (2x3)", grid_shapes)
        .slide("Layout Features", &[
            "layouts::grid() - Create grid positions",
            "layouts::center() - Center shapes on slide",
            "layouts::stack_horizontal() - Horizontal stacking",
            "layouts::distribute_horizontal() - Even distribution",
        ])
        .build()?;
    std::fs::write("examples/output/layouts.pptx", layout_pptx)?;
    println!("  ✓ Created examples/output/layouts.pptx");

    println!("\n✅ All template demos created successfully!");
    println!("   Check examples/output/ for the generated files.");
    
    Ok(())
}

