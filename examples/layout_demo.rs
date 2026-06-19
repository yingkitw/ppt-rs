use ppt_rs::generator::{create_pptx_with_content, SlideContent, SlideLayout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let slides = vec![
        // Slide 1: Title Only
        SlideContent::new("Welcome to Layout Demo")
            .layout(SlideLayout::TitleOnly),

        // Slide 2: Centered Title (good for cover slides)
        SlideContent::new("Centered Title Slide")
            .layout(SlideLayout::CenteredTitle)
            .title_size(60)
            .title_color("4F81BD"),

        // Slide 3: Title and Content (standard layout)
        SlideContent::new("Standard Layout")
            .add_bullet("Point 1: Title at top")
            .add_bullet("Point 2: Content below")
            .add_bullet("Point 3: Most common layout")
            .layout(SlideLayout::TitleAndContent),

        // Slide 4: Title and Big Content
        SlideContent::new("Big Content Area")
            .add_bullet("More space for content")
            .add_bullet("Smaller title area")
            .add_bullet("Good for detailed slides")
            .add_bullet("Maximizes content space")
            .layout(SlideLayout::TitleAndBigContent),

        // Slide 5: Section Header (divider slide)
        SlideContent::new("Section Two")
            .title_size(48)
            .title_bold(true)
            .with_layout(SlideLayout::SectionHeader),

        // Slide 6: Two Column Layout
        SlideContent::new("Two Column Layout")
            .add_bullet("Left column content")
            .add_bullet("Organized side by side")
            .add_bullet("Great for comparisons")
            .with_layout(SlideLayout::TwoColumn),

        // Slide 7: Blank Slide
        SlideContent::new("")
            .with_layout(SlideLayout::Blank),

        // Slide 8: Summary with different formatting
        SlideContent::new("Summary")
            .title_size(48)
            .title_bold(true)
            .title_color("C0504D")
            .add_bullet("Layout types implemented:")
            .add_bullet("• TitleOnly - Just title")
            .add_bullet("• CenteredTitle - Title centered")
            .add_bullet("• TitleAndContent - Standard")
            .add_bullet("• TitleAndBigContent - Large content")
            .add_bullet("• TwoColumn - Side by side")
            .add_bullet("• SectionHeader - Section divider")
            .add_bullet("• Blank - Empty slide")
            .content_size(20)
            .with_layout(SlideLayout::TitleAndContent),
    ];

    let pptx_data = create_pptx_with_content("Layout Demo Presentation", slides)?;
    std::fs::write("layout_demo.pptx", pptx_data)?;
    println!("✓ Created layout_demo.pptx with 8 slides demonstrating all layout types");

    Ok(())
}
