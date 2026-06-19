//! Layout, template, and footer capabilities demonstration.
//!
//! Validates end-to-end packaging for:
//! - All seven `SlideLayout` variants via `with_layout()`
//! - Slide master footer / date / page-number placeholders (`PrintSettings`)
//! - Template-based generation (`create_pptx_with_template` / `PptxTemplate`)
//!
//! Run: `cargo run --example layout_template_features`
//!
//! Output: `examples/output/layout_template_features.pptx` and
//!         `examples/output/from_template.pptx`

use ppt_rs::generator::{
    create_pptx_with_content, create_pptx_with_settings, create_pptx_with_template,
    PresentationSettings, PrintSettings, PptxTemplate, SlideContent, SlideLayout,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("examples/output")?;

    println!("1. Building deck with all seven slide layouts + footers...");

    let slides = vec![
        SlideContent::new("Layout & Template Features")
            .title_size(54)
            .title_color("1F4E79")
            .with_layout(SlideLayout::CenteredTitle),
        SlideContent::new("Standard Layout")
            .add_bullet("Title at top, bullets below")
            .add_bullet("Default for most content slides")
            .with_layout(SlideLayout::TitleAndContent),
        SlideContent::new("Two Column")
            .add_bullet("Left column topic")
            .add_bullet("Right column topic")
            .add_bullet("Useful for comparisons")
            .with_layout(SlideLayout::TwoColumn),
        SlideContent::new("Section Two")
            .title_size(48)
            .title_bold(true)
            .with_layout(SlideLayout::SectionHeader),
        SlideContent::new("")
            .with_layout(SlideLayout::Blank),
        SlideContent::new("Title Only Slide")
            .title_size(44)
            .with_layout(SlideLayout::TitleOnly),
        SlideContent::new("Big Content Area")
            .add_bullet("Larger body placeholder")
            .add_bullet("Smaller title band")
            .add_bullet("Good for dense slides")
            .content_size(22)
            .with_layout(SlideLayout::TitleAndBigContent),
        SlideContent::new("Summary")
            .add_bullet("7 packaged slide layouts on slide master 1")
            .add_bullet("Per-slide layout via with_layout()")
            .add_bullet("Footers and page numbers on slide master")
            .add_bullet("Template cloning from an existing .pptx")
            .with_layout(SlideLayout::TitleAndContent),
    ];

    let print = PrintSettings::default()
        .header("ppt-rs layout demo")
        .footer("Confidential — do not distribute")
        .print_date(true)
        .print_page_numbers(true);
    let settings = PresentationSettings::new().print(print);

    let full_deck = create_pptx_with_settings(
        "Layout Template Features",
        &slides,
        Some(settings),
    )?;

    let template_path = "examples/output/layout_template_base.pptx";
    std::fs::write(template_path, &full_deck)?;
    std::fs::write("examples/output/layout_template_features.pptx", &full_deck)?;
    println!("   ✓ examples/output/layout_template_features.pptx (8 slides)");

    println!("2. Loading template and generating a follow-up deck...");

    let tpl = PptxTemplate::load(template_path)?;
    println!(
        "   ✓ Template loaded: {} layout part(s)",
        tpl.layout_count()
    );

    let follow_up = vec![
        SlideContent::new("Generated From Template")
            .title_size(50)
            .with_layout(SlideLayout::CenteredTitle),
        SlideContent::new("Inherited Theme & Layouts")
            .add_bullet("Master and layout parts cloned from base deck")
            .add_bullet("New slides reference template slideLayoutN.xml")
            .with_layout(SlideLayout::TitleAndContent),
    ];

    let from_template = create_pptx_with_template(
        "From Template",
        &follow_up,
        template_path,
        None,
    )?;
    std::fs::write("examples/output/from_template.pptx", &from_template)?;
    println!("   ✓ examples/output/from_template.pptx (2 slides)");

    println!("3. Quick sanity: default layout-only deck...");
    let layout_only = create_pptx_with_content(
        "Layout Only",
        vec![
            SlideContent::new("A").with_layout(SlideLayout::TitleOnly),
            SlideContent::new("B").with_layout(SlideLayout::SectionHeader),
        ],
    )?;
    std::fs::write("examples/output/layout_only.pptx", &layout_only)?;
    println!("   ✓ examples/output/layout_only.pptx (2 slides)");

    println!("\n✅ Layout/template/footer demo complete.");
    println!("   Open the .pptx files in PowerPoint or Keynote to verify layouts and footers.");

    Ok(())
}
