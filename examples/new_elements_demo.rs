//! Demonstration of new PPT elements in ppt-rs
//!
//! This example showcases:
//! - New chart types (Area, Scatter, Doughnut, Radar, Bubble)
//! - Connectors between shapes
//! - Hyperlinks
//! - Gradient fills
//! - Video/Audio embedding

use ppt_rs::{
    create_pptx_with_content, SlideContent, SlideLayout,
    ChartType,
    Connector, ArrowType, ConnectionSite,
    Hyperlink,
    GradientFill, GradientDirection, PresetGradients,
    Video, VideoOptions,
    Audio, AudioOptions,
};
use ppt_rs::generator::shapes::inches_to_emu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== New PPT Elements Demo ===\n");

    // Create slides demonstrating new features
    let slides = vec![
        // Slide 1: Title slide
        SlideContent::new("New PPT Elements in ppt-rs")
            .add_bullet("18 new chart types")
            .add_bullet("Connectors between shapes")
            .add_bullet("Hyperlinks (URL, slide, email)")
            .add_bullet("Gradient fills")
            .add_bullet("Video/Audio embedding")
            .layout(SlideLayout::TitleAndContent),

        // Slide 2: New Chart Types
        SlideContent::new("New Chart Types")
            .add_bullet("Area charts (standard, stacked, 100% stacked)")
            .add_bullet("Scatter charts (markers, lines, smooth)")
            .add_bullet("Doughnut charts")
            .add_bullet("Radar charts (standard, filled)")
            .add_bullet("Bubble charts")
            .add_bullet("Stock charts (HLC, OHLC)")
            .add_bullet("Combo charts (bar + line)")
            .layout(SlideLayout::TitleAndContent),

        // Slide 3: Connector Types
        SlideContent::new("Connector Types")
            .add_bullet("Straight connectors")
            .add_bullet("Elbow (bent) connectors")
            .add_bullet("Curved connectors")
            .add_bullet("Arrow heads (Triangle, Stealth, Diamond, Oval)")
            .add_bullet("Connection sites (Top, Bottom, Left, Right, Corners)")
            .layout(SlideLayout::TitleAndContent),

        // Slide 4: Hyperlink Types
        SlideContent::new("Hyperlink Support")
            .add_bullet("URL links (external websites)")
            .add_bullet("Slide links (navigate within presentation)")
            .add_bullet("Email links (mailto:)")
            .add_bullet("File links (local files)")
            .add_bullet("Navigation (First, Last, Next, Previous slide)")
            .layout(SlideLayout::TitleAndContent),

        // Slide 5: Gradient Fills
        SlideContent::new("Gradient Fill Support")
            .add_bullet("Linear gradients (horizontal, vertical, diagonal)")
            .add_bullet("Radial gradients")
            .add_bullet("Rectangular gradients")
            .add_bullet("Path gradients")
            .add_bullet("Preset gradients (Blue, Green, Rainbow, etc.)")
            .add_bullet("Custom gradient stops with transparency")
            .layout(SlideLayout::TitleAndContent),

        // Slide 6: Media Support
        SlideContent::new("Video & Audio Support")
            .add_bullet("Video formats: MP4, WMV, AVI, MOV, MKV, WebM")
            .add_bullet("Audio formats: MP3, WAV, WMA, M4A, OGG, FLAC")
            .add_bullet("Playback options: auto-play, loop, mute")
            .add_bullet("Volume control")
            .add_bullet("Start/end time trimming")
            .layout(SlideLayout::TitleAndContent),
    ];

    // Generate the PPTX
    let pptx_data = create_pptx_with_content("New Elements Demo", slides)?;

    // Save to file
    let output_path = "examples/output/new_elements_demo.pptx";
    std::fs::create_dir_all("examples/output")?;
    std::fs::write(output_path, &pptx_data)?;

    println!("✓ Created presentation: {}", output_path);
    println!("✓ File size: {} bytes", pptx_data.len());

    // Demonstrate API usage
    println!("\n=== API Examples ===\n");

    // Chart types
    println!("Chart Types:");
    let chart_types = [
        ChartType::Bar,
        ChartType::BarHorizontal,
        ChartType::BarStacked,
        ChartType::Line,
        ChartType::LineMarkers,
        ChartType::Pie,
        ChartType::Doughnut,
        ChartType::Area,
        ChartType::AreaStacked,
        ChartType::Scatter,
        ChartType::ScatterSmooth,
        ChartType::Bubble,
        ChartType::Radar,
        ChartType::RadarFilled,
        ChartType::StockHLC,
        ChartType::Combo,
    ];
    for ct in &chart_types {
        println!("  - {:?} -> {}", ct, ct.xml_element());
    }

    // Connector example
    println!("\nConnector Example:");
    let connector = Connector::elbow(
        inches_to_emu(1.0), inches_to_emu(1.0),
        inches_to_emu(5.0), inches_to_emu(3.0),
    )
    .with_color("0066CC")
    .with_end_arrow(ArrowType::Triangle)
    .connect_start(1, ConnectionSite::Right)
    .connect_end(2, ConnectionSite::Left);
    println!("  Type: {:?}", connector.connector_type);
    println!("  End Arrow: {:?}", connector.end_arrow);

    // Hyperlink example
    println!("\nHyperlink Examples:");
    let url_link = Hyperlink::url("https://example.com").with_tooltip("Visit Example");
    let slide_link = Hyperlink::slide(3);
    let email_link = Hyperlink::email("test@example.com");
    println!("  URL: {:?}", url_link.action);
    println!("  Slide: {:?}", slide_link.action);
    println!("  Email: {:?}", email_link.action);

    // Gradient example
    println!("\nGradient Examples:");
    let blue_gradient = PresetGradients::blue();
    let rainbow = PresetGradients::rainbow();
    let custom = GradientFill::linear(GradientDirection::DiagonalDown)
        .add_stop(ppt_rs::GradientStop::start("FF0000"))
        .add_stop(ppt_rs::GradientStop::end("0000FF"));
    println!("  Blue gradient: {} stops", blue_gradient.stops.len());
    println!("  Rainbow gradient: {} stops", rainbow.stops.len());
    println!("  Custom gradient: {} stops", custom.stops.len());

    // Video example
    println!("\nVideo Example:");
    if let Some(video) = Video::from_file("video.mp4", 0, 0, inches_to_emu(6.0), inches_to_emu(4.0)) {
        let video = video.with_options(VideoOptions::auto_play().with_loop(true));
        println!("  Format: {:?}", video.format);
        println!("  Auto-play: {}", video.options.auto_play);
        println!("  Loop: {}", video.options.loop_playback);
    }

    // Audio example
    println!("\nAudio Example:");
    if let Some(audio) = Audio::from_file("audio.mp3", 0, 0, inches_to_emu(1.0), inches_to_emu(1.0)) {
        let audio = audio.with_options(AudioOptions::auto_play().with_play_across_slides(true));
        println!("  Format: {:?}", audio.format);
        println!("  Auto-play: {}", audio.options.auto_play);
        println!("  Play across slides: {}", audio.options.play_across_slides);
    }

    println!("\n=== Demo Complete ===");

    Ok(())
}
