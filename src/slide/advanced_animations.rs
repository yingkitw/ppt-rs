//! Advanced Animations Support
//!
//! This module provides comprehensive animation support including:
//! - Entrance animations (Fade, Wipe, Fly In, etc.)
//! - Emphasis animations (Grow/Shrink, Spin, Color Pulse, etc.)
//! - Exit animations (Fade, Wipe, Fly Out, etc.)
//! - Animation timing and sequencing
//! - Animation effects and options

use crate::error::Result;

/// Animation entrance effects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntranceEffect {
    /// Fade in
    Fade,
    /// Wipe in
    Wipe,
    /// Fly in
    FlyIn,
    /// Bounce in
    Bounce,
    /// Zoom in
    Zoom,
    /// Spin in
    Spin,
    /// Split in
    Split,
    /// Wheel in
    Wheel,
    /// Appear
    Appear,
    /// Dissolve
    Dissolve,
}

impl EntranceEffect {
    /// Get the XML preset name
    pub fn preset_name(&self) -> &str {
        match self {
            EntranceEffect::Fade => "fade",
            EntranceEffect::Wipe => "wipe",
            EntranceEffect::FlyIn => "fly",
            EntranceEffect::Bounce => "bounce",
            EntranceEffect::Zoom => "zoom",
            EntranceEffect::Spin => "spin",
            EntranceEffect::Split => "split",
            EntranceEffect::Wheel => "wheel",
            EntranceEffect::Appear => "appear",
            EntranceEffect::Dissolve => "dissolve",
        }
    }
}

/// Animation emphasis effects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmphasisEffect {
    /// Grow/Shrink
    GrowShrink,
    /// Spin
    Spin,
    /// Color Pulse
    ColorPulse,
    /// Bold Flash
    BoldFlash,
    /// Shimmer
    Shimmer,
    /// Darken
    Darken,
    /// Lighten
    Lighten,
    /// Grow With Color
    GrowWithColor,
    /// Shrink With Color
    ShrinkWithColor,
    /// Underline
    Underline,
}

impl EmphasisEffect {
    /// Get the XML preset name
    pub fn preset_name(&self) -> &str {
        match self {
            EmphasisEffect::GrowShrink => "grow",
            EmphasisEffect::Spin => "spin",
            EmphasisEffect::ColorPulse => "colorPulse",
            EmphasisEffect::BoldFlash => "boldFlash",
            EmphasisEffect::Shimmer => "shimmer",
            EmphasisEffect::Darken => "darken",
            EmphasisEffect::Lighten => "lighten",
            EmphasisEffect::GrowWithColor => "growWithColor",
            EmphasisEffect::ShrinkWithColor => "shrinkWithColor",
            EmphasisEffect::Underline => "underline",
        }
    }
}

/// Animation exit effects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitEffect {
    /// Fade out
    Fade,
    /// Wipe out
    Wipe,
    /// Fly out
    FlyOut,
    /// Bounce out
    Bounce,
    /// Zoom out
    Zoom,
    /// Spin out
    Spin,
    /// Split out
    Split,
    /// Wheel out
    Wheel,
    /// Disappear
    Disappear,
    /// Dissolve
    Dissolve,
}

impl ExitEffect {
    /// Get the XML preset name
    pub fn preset_name(&self) -> &str {
        match self {
            ExitEffect::Fade => "fade",
            ExitEffect::Wipe => "wipe",
            ExitEffect::FlyOut => "fly",
            ExitEffect::Bounce => "bounce",
            ExitEffect::Zoom => "zoom",
            ExitEffect::Spin => "spin",
            ExitEffect::Split => "split",
            ExitEffect::Wheel => "wheel",
            ExitEffect::Disappear => "disappear",
            ExitEffect::Dissolve => "dissolve",
        }
    }
}

/// Animation timing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationTiming {
    /// With previous animation
    WithPrevious,
    /// After previous animation
    AfterPrevious,
    /// On click
    OnClick,
}

impl AnimationTiming {
    /// Get the XML representation
    pub fn to_xml_str(&self) -> &str {
        match self {
            AnimationTiming::WithPrevious => "withPrev",
            AnimationTiming::AfterPrevious => "afterPrev",
            AnimationTiming::OnClick => "onClick",
        }
    }
}

/// Advanced animation configuration
#[derive(Debug, Clone)]
pub struct AdvancedAnimation {
    /// Animation name
    name: String,
    /// Animation type (entrance, emphasis, exit)
    animation_type: AnimationType,
    /// Duration in milliseconds
    duration: u32,
    /// Delay in milliseconds
    delay: u32,
    /// Timing mode
    timing: AnimationTiming,
    /// Repeat count (0 = no repeat)
    repeat_count: u32,
    /// Repeat behavior
    repeat_behavior: RepeatBehavior,
    /// Animation speed
    speed: AnimationSpeed,
}

/// Animation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationType {
    /// Entrance animation
    Entrance,
    /// Emphasis animation
    Emphasis,
    /// Exit animation
    Exit,
}

/// Repeat behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatBehavior {
    /// Don't repeat
    None,
    /// Repeat until end of slide
    UntilEndOfSlide,
    /// Repeat specified number of times
    Count,
}

/// Animation speed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationSpeed {
    /// Slow (1000ms)
    Slow,
    /// Medium (500ms)
    Medium,
    /// Fast (250ms)
    Fast,
}

impl AnimationSpeed {
    /// Get duration in milliseconds
    pub fn duration_ms(&self) -> u32 {
        match self {
            AnimationSpeed::Slow => 1000,
            AnimationSpeed::Medium => 500,
            AnimationSpeed::Fast => 250,
        }
    }
}

impl Default for AdvancedAnimation {
    fn default() -> Self {
        Self::new("Animation")
    }
}

impl AdvancedAnimation {
    /// Create a new animation
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            animation_type: AnimationType::Entrance,
            duration: 500,
            delay: 0,
            timing: AnimationTiming::OnClick,
            repeat_count: 0,
            repeat_behavior: RepeatBehavior::None,
            speed: AnimationSpeed::Medium,
        }
    }

    /// Create entrance animation
    pub fn entrance(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            animation_type: AnimationType::Entrance,
            ..Default::default()
        }
    }

    /// Create emphasis animation
    pub fn emphasis(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            animation_type: AnimationType::Emphasis,
            ..Default::default()
        }
    }

    /// Create exit animation
    pub fn exit(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            animation_type: AnimationType::Exit,
            ..Default::default()
        }
    }

    /// Get animation name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set duration in milliseconds
    pub fn set_duration(mut self, duration: u32) -> Self {
        self.duration = duration;
        self
    }

    /// Get duration
    pub fn duration(&self) -> u32 {
        self.duration
    }

    /// Set delay in milliseconds
    pub fn set_delay(mut self, delay: u32) -> Self {
        self.delay = delay;
        self
    }

    /// Get delay
    pub fn delay(&self) -> u32 {
        self.delay
    }

    /// Set timing mode
    pub fn set_timing(mut self, timing: AnimationTiming) -> Self {
        self.timing = timing;
        self
    }

    /// Get timing mode
    pub fn timing(&self) -> AnimationTiming {
        self.timing
    }

    /// Set repeat count
    pub fn set_repeat_count(mut self, count: u32) -> Self {
        self.repeat_count = count;
        self.repeat_behavior = if count > 0 {
            RepeatBehavior::Count
        } else {
            RepeatBehavior::None
        };
        self
    }

    /// Get repeat count
    pub fn repeat_count(&self) -> u32 {
        self.repeat_count
    }

    /// Set repeat until end of slide
    pub fn repeat_until_end(mut self) -> Self {
        self.repeat_behavior = RepeatBehavior::UntilEndOfSlide;
        self
    }

    /// Get repeat behavior
    pub fn repeat_behavior(&self) -> RepeatBehavior {
        self.repeat_behavior
    }

    /// Set animation speed
    pub fn set_speed(mut self, speed: AnimationSpeed) -> Self {
        self.duration = speed.duration_ms();
        self.speed = speed;
        self
    }

    /// Get animation speed
    pub fn speed(&self) -> AnimationSpeed {
        self.speed
    }

    /// Get animation type
    pub fn animation_type(&self) -> AnimationType {
        self.animation_type
    }

    /// Generate XML for animation
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<p:animEffect>");
        xml.push_str(&format!(
            "<p:animProp name=\"{}\" dur=\"{}\" delay=\"{}\"/>",
            self.name, self.duration, self.delay
        ));
        xml.push_str("</p:animEffect>");
        xml
    }
}

/// Animation collection manager
#[derive(Debug, Clone)]
pub struct AnimationCollection {
    /// Animations
    animations: Vec<AdvancedAnimation>,
}

impl Default for AnimationCollection {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationCollection {
    /// Create a new animation collection
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
        }
    }

    /// Add an animation
    pub fn add(&mut self, animation: AdvancedAnimation) {
        self.animations.push(animation);
    }

    /// Get animation by index
    pub fn get(&self, index: usize) -> Option<&AdvancedAnimation> {
        self.animations.get(index)
    }

    /// Get mutable animation by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut AdvancedAnimation> {
        self.animations.get_mut(index)
    }

    /// Get all animations
    pub fn all(&self) -> &[AdvancedAnimation] {
        &self.animations
    }

    /// Get number of animations
    pub fn len(&self) -> usize {
        self.animations.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.animations.is_empty()
    }

    /// Clear all animations
    pub fn clear(&mut self) {
        self.animations.clear();
    }

    /// Generate XML for all animations
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<p:timing>");
        for anim in &self.animations {
            xml.push_str(&anim.to_xml());
        }
        xml.push_str("</p:timing>");
        xml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entrance_effect_fade() {
        let effect = EntranceEffect::Fade;
        assert_eq!(effect.preset_name(), "fade");
    }

    #[test]
    fn test_entrance_effect_fly_in() {
        let effect = EntranceEffect::FlyIn;
        assert_eq!(effect.preset_name(), "fly");
    }

    #[test]
    fn test_emphasis_effect_spin() {
        let effect = EmphasisEffect::Spin;
        assert_eq!(effect.preset_name(), "spin");
    }

    #[test]
    fn test_emphasis_effect_color_pulse() {
        let effect = EmphasisEffect::ColorPulse;
        assert_eq!(effect.preset_name(), "colorPulse");
    }

    #[test]
    fn test_exit_effect_fade() {
        let effect = ExitEffect::Fade;
        assert_eq!(effect.preset_name(), "fade");
    }

    #[test]
    fn test_animation_timing_on_click() {
        let timing = AnimationTiming::OnClick;
        assert_eq!(timing.to_xml_str(), "onClick");
    }

    #[test]
    fn test_animation_speed_slow() {
        let speed = AnimationSpeed::Slow;
        assert_eq!(speed.duration_ms(), 1000);
    }

    #[test]
    fn test_animation_speed_medium() {
        let speed = AnimationSpeed::Medium;
        assert_eq!(speed.duration_ms(), 500);
    }

    #[test]
    fn test_animation_speed_fast() {
        let speed = AnimationSpeed::Fast;
        assert_eq!(speed.duration_ms(), 250);
    }

    #[test]
    fn test_advanced_animation_new() {
        let anim = AdvancedAnimation::new("Test");
        assert_eq!(anim.name(), "Test");
        assert_eq!(anim.duration(), 500);
        assert_eq!(anim.delay(), 0);
    }

    #[test]
    fn test_advanced_animation_entrance() {
        let anim = AdvancedAnimation::entrance("Fade In");
        assert_eq!(anim.animation_type(), AnimationType::Entrance);
    }

    #[test]
    fn test_advanced_animation_emphasis() {
        let anim = AdvancedAnimation::emphasis("Spin");
        assert_eq!(anim.animation_type(), AnimationType::Emphasis);
    }

    #[test]
    fn test_advanced_animation_exit() {
        let anim = AdvancedAnimation::exit("Fade Out");
        assert_eq!(anim.animation_type(), AnimationType::Exit);
    }

    #[test]
    fn test_advanced_animation_duration() {
        let anim = AdvancedAnimation::new("Test").set_duration(1000);
        assert_eq!(anim.duration(), 1000);
    }

    #[test]
    fn test_advanced_animation_delay() {
        let anim = AdvancedAnimation::new("Test").set_delay(500);
        assert_eq!(anim.delay(), 500);
    }

    #[test]
    fn test_advanced_animation_timing() {
        let anim = AdvancedAnimation::new("Test")
            .set_timing(AnimationTiming::AfterPrevious);
        assert_eq!(anim.timing(), AnimationTiming::AfterPrevious);
    }

    #[test]
    fn test_advanced_animation_repeat() {
        let anim = AdvancedAnimation::new("Test").set_repeat_count(3);
        assert_eq!(anim.repeat_count(), 3);
    }

    #[test]
    fn test_advanced_animation_speed() {
        let anim = AdvancedAnimation::new("Test").set_speed(AnimationSpeed::Fast);
        assert_eq!(anim.speed(), AnimationSpeed::Fast);
        assert_eq!(anim.duration(), 250);
    }

    #[test]
    fn test_animation_collection_new() {
        let collection = AnimationCollection::new();
        assert!(collection.is_empty());
        assert_eq!(collection.len(), 0);
    }

    #[test]
    fn test_animation_collection_add() {
        let mut collection = AnimationCollection::new();
        let anim = AdvancedAnimation::new("Test");
        collection.add(anim);
        assert_eq!(collection.len(), 1);
    }

    #[test]
    fn test_animation_collection_get() {
        let mut collection = AnimationCollection::new();
        let anim = AdvancedAnimation::new("Test");
        collection.add(anim);
        assert!(collection.get(0).is_some());
        assert!(collection.get(1).is_none());
    }

    #[test]
    fn test_animation_collection_clear() {
        let mut collection = AnimationCollection::new();
        collection.add(AdvancedAnimation::new("Test"));
        collection.clear();
        assert!(collection.is_empty());
    }

    #[test]
    fn test_animation_collection_to_xml() {
        let mut collection = AnimationCollection::new();
        collection.add(AdvancedAnimation::new("Test"));
        let xml = collection.to_xml();
        assert!(xml.contains("<p:timing>"));
        assert!(xml.contains("</p:timing>"));
    }

    #[test]
    fn test_advanced_animation_to_xml() {
        let anim = AdvancedAnimation::new("Test");
        let xml = anim.to_xml();
        assert!(xml.contains("<p:animEffect>"));
        assert!(xml.contains("</p:animEffect>"));
    }
}
