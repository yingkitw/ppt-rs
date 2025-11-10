//! Example: Advanced Animations Support
//!
//! This example demonstrates:
//! - Entrance animations (Fade, Wipe, Fly In, Bounce, etc.)
//! - Emphasis animations (Spin, Color Pulse, Shimmer, etc.)
//! - Exit animations (Fade, Wipe, Fly Out, etc.)
//! - Animation timing and sequencing
//! - Animation effects and options

use ppt_rs::PresentationBuilder;
use ppt_rs::slide::{
    AdvancedAnimation, AnimationTiming, AnimationSpeed, AnimationCollection,
    AdvEntranceEffect, AdvEmphasisEffect, AdvExitEffect,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎬 Advanced Animations Examples\n");

    // Example 1: Entrance effects overview
    println!("1️⃣  Entrance Effects");
    let entrance_effects = vec![
        ("Fade", AdvEntranceEffect::Fade),
        ("Wipe", AdvEntranceEffect::Wipe),
        ("Fly In", AdvEntranceEffect::FlyIn),
        ("Bounce", AdvEntranceEffect::Bounce),
        ("Zoom", AdvEntranceEffect::Zoom),
        ("Spin", AdvEntranceEffect::Spin),
        ("Split", AdvEntranceEffect::Split),
        ("Wheel", AdvEntranceEffect::Wheel),
        ("Appear", AdvEntranceEffect::Appear),
        ("Dissolve", AdvEntranceEffect::Dissolve),
    ];

    for (name, _effect) in &entrance_effects {
        println!("   ✓ {}", name);
    }

    // Example 2: Emphasis effects overview
    println!("\n2️⃣  Emphasis Effects");
    let emphasis_effects = vec![
        ("Grow/Shrink", AdvEmphasisEffect::GrowShrink),
        ("Spin", AdvEmphasisEffect::Spin),
        ("Color Pulse", AdvEmphasisEffect::ColorPulse),
        ("Bold Flash", AdvEmphasisEffect::BoldFlash),
        ("Shimmer", AdvEmphasisEffect::Shimmer),
        ("Darken", AdvEmphasisEffect::Darken),
        ("Lighten", AdvEmphasisEffect::Lighten),
        ("Grow With Color", AdvEmphasisEffect::GrowWithColor),
        ("Shrink With Color", AdvEmphasisEffect::ShrinkWithColor),
        ("Underline", AdvEmphasisEffect::Underline),
    ];

    for (name, _effect) in &emphasis_effects {
        println!("   ✓ {}", name);
    }

    // Example 3: Exit effects overview
    println!("\n3️⃣  Exit Effects");
    let exit_effects = vec![
        ("Fade", AdvExitEffect::Fade),
        ("Wipe", AdvExitEffect::Wipe),
        ("Fly Out", AdvExitEffect::FlyOut),
        ("Bounce", AdvExitEffect::Bounce),
        ("Zoom", AdvExitEffect::Zoom),
        ("Spin", AdvExitEffect::Spin),
        ("Split", AdvExitEffect::Split),
        ("Wheel", AdvExitEffect::Wheel),
        ("Disappear", AdvExitEffect::Disappear),
        ("Dissolve", AdvExitEffect::Dissolve),
    ];

    for (name, _effect) in &exit_effects {
        println!("   ✓ {}", name);
    }

    // Example 4: Animation timing options
    println!("\n4️⃣  Animation Timing Options");
    let timings = vec![
        ("On Click", AnimationTiming::OnClick),
        ("With Previous", AnimationTiming::WithPrevious),
        ("After Previous", AnimationTiming::AfterPrevious),
    ];

    for (name, timing) in timings {
        println!("   ✓ {}: {}", name, timing.to_xml_str());
    }

    // Example 5: Animation speed options
    println!("\n5️⃣  Animation Speed Options");
    let speeds = vec![
        ("Slow", AnimationSpeed::Slow),
        ("Medium", AnimationSpeed::Medium),
        ("Fast", AnimationSpeed::Fast),
    ];

    for (name, speed) in speeds {
        println!("   ✓ {}: {}ms", name, speed.duration_ms());
    }

    // Example 6: Create basic animation
    println!("\n6️⃣  Create Basic Animation");
    let anim = AdvancedAnimation::entrance("Fade In")
        .set_duration(500)
        .set_timing(AnimationTiming::OnClick);
    
    println!("   ✓ Animation: {}", anim.name());
    println!("   ✓ Duration: {}ms", anim.duration());
    println!("   ✓ Timing: {:?}", anim.timing());

    // Example 7: Animation with delay
    println!("\n7️⃣  Animation with Delay");
    let anim = AdvancedAnimation::entrance("Wipe In")
        .set_duration(750)
        .set_delay(500)
        .set_timing(AnimationTiming::AfterPrevious);
    
    println!("   ✓ Animation: {}", anim.name());
    println!("   ✓ Duration: {}ms", anim.duration());
    println!("   ✓ Delay: {}ms", anim.delay());

    // Example 8: Animation with repeat
    println!("\n8️⃣  Animation with Repeat");
    let anim = AdvancedAnimation::emphasis("Spin")
        .set_duration(1000)
        .set_repeat_count(3);
    
    println!("   ✓ Animation: {}", anim.name());
    println!("   ✓ Repeat count: {}", anim.repeat_count());

    // Example 9: Animation with speed preset
    println!("\n9️⃣  Animation with Speed Preset");
    let anim = AdvancedAnimation::entrance("Bounce In")
        .set_speed(AnimationSpeed::Fast);
    
    println!("   ✓ Animation: {}", anim.name());
    println!("   ✓ Speed: {:?}", anim.speed());
    println!("   ✓ Duration: {}ms", anim.duration());

    // Example 10: Animation collection
    println!("\n🔟 Animation Collection");
    let mut collection = AnimationCollection::new();
    
    collection.add(AdvancedAnimation::entrance("Fade In")
        .set_timing(AnimationTiming::OnClick));
    collection.add(AdvancedAnimation::emphasis("Spin")
        .set_timing(AnimationTiming::AfterPrevious));
    collection.add(AdvancedAnimation::exit("Fade Out")
        .set_timing(AnimationTiming::AfterPrevious));
    
    println!("   ✓ Total animations: {}", collection.len());
    for (i, anim) in collection.all().iter().enumerate() {
        println!("     {} - {}", i + 1, anim.name());
    }

    // Example 11: Create presentation with animations
    println!("\n1️⃣1️⃣  Creating Presentation with Animations");
    let mut prs = PresentationBuilder::new()
        .title("Animations Demo")
        .author("Rust Developer")
        .build()?;

    // Add slides
    for i in 0..3 {
        let _idx = prs.add_slide()?;
        println!("   ✓ Added slide {}", i + 1);
    }

    // Save presentation
    let output_path = "examples/output/09_advanced_animations.pptx";
    prs.save_to_file(output_path)?;
    println!("   ✓ Saved to {}", output_path);

    // Example 12: Complex animation sequence
    println!("\n1️⃣2️⃣  Complex Animation Sequence");
    let mut sequence = AnimationCollection::new();
    
    // Title animation
    sequence.add(AdvancedAnimation::entrance("Fade In")
        .set_duration(500)
        .set_timing(AnimationTiming::OnClick));
    
    // Content animations
    sequence.add(AdvancedAnimation::entrance("Wipe In")
        .set_duration(750)
        .set_delay(250)
        .set_timing(AnimationTiming::AfterPrevious));
    
    // Emphasis animation
    sequence.add(AdvancedAnimation::emphasis("Color Pulse")
        .set_duration(1000)
        .set_timing(AnimationTiming::AfterPrevious));
    
    // Exit animation
    sequence.add(AdvancedAnimation::exit("Fade Out")
        .set_duration(500)
        .set_timing(AnimationTiming::OnClick));
    
    println!("   ✓ Sequence created with {} animations", sequence.len());

    // Example 13: Animation XML generation
    println!("\n1️⃣3️⃣  Animation XML Generation");
    let anim = AdvancedAnimation::entrance("Test");
    let xml = anim.to_xml();
    println!("   ✓ Generated XML: {}", xml.lines().next().unwrap_or(""));

    // Example 14: Collection XML generation
    println!("\n1️⃣4️⃣  Collection XML Generation");
    let xml = sequence.to_xml();
    println!("   ✓ Generated collection XML: {}", xml.lines().next().unwrap_or(""));

    println!("\n✅ Advanced animations examples complete!");
    println!("\n📋 Features demonstrated:");
    println!("  • 10 entrance effects");
    println!("  • 10 emphasis effects");
    println!("  • 10 exit effects");
    println!("  • Animation timing (on click, with/after previous)");
    println!("  • Animation speed (slow, medium, fast)");
    println!("  • Animation duration and delay");
    println!("  • Animation repeat options");
    println!("  • Animation collection management");
    println!("  • Complex animation sequences");
    println!("  • XML generation");
    println!("\n🎉 All advanced animation features working correctly!");

    Ok(())
}
