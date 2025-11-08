//! Animation Effects - Animation support for slides and shapes

/// Animation type enumeration
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnimationType {
    /// Entrance animation
    Entrance,
    /// Exit animation
    Exit,
    /// Emphasis animation
    Emphasis,
    /// Motion path animation
    MotionPath,
}

impl AnimationType {
    /// Get animation type string
    pub fn type_str(&self) -> &str {
        match self {
            AnimationType::Entrance => "entrance",
            AnimationType::Exit => "exit",
            AnimationType::Emphasis => "emphasis",
            AnimationType::MotionPath => "motionPath",
        }
    }
}

/// Entrance animation effects
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntranceEffect {
    /// Appear effect
    Appear,
    /// Fade effect
    Fade,
    /// Fly in effect
    FlyIn,
    /// Wipe effect
    Wipe,
    /// Bounce effect
    Bounce,
    /// Zoom effect
    Zoom,
    /// Spin effect
    Spin,
    /// Swivel effect
    Swivel,
}

impl EntranceEffect {
    /// Get effect string
    pub fn effect_str(&self) -> &str {
        match self {
            EntranceEffect::Appear => "appear",
            EntranceEffect::Fade => "fade",
            EntranceEffect::FlyIn => "flyIn",
            EntranceEffect::Wipe => "wipe",
            EntranceEffect::Bounce => "bounce",
            EntranceEffect::Zoom => "zoom",
            EntranceEffect::Spin => "spin",
            EntranceEffect::Swivel => "swivel",
        }
    }
}

/// Exit animation effects
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExitEffect {
    /// Disappear effect
    Disappear,
    /// Fade effect
    Fade,
    /// Fly out effect
    FlyOut,
    /// Wipe effect
    Wipe,
    /// Shrink effect
    Shrink,
    /// Zoom effect
    Zoom,
    /// Spin effect
    Spin,
}

impl ExitEffect {
    /// Get effect string
    pub fn effect_str(&self) -> &str {
        match self {
            ExitEffect::Disappear => "disappear",
            ExitEffect::Fade => "fade",
            ExitEffect::FlyOut => "flyOut",
            ExitEffect::Wipe => "wipe",
            ExitEffect::Shrink => "shrink",
            ExitEffect::Zoom => "zoom",
            ExitEffect::Spin => "spin",
        }
    }
}

/// Emphasis animation effects
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EmphasisEffect {
    /// Bold effect
    Bold,
    /// Underline effect
    Underline,
    /// Italic effect
    Italic,
    /// Color effect
    Color,
    /// Grow effect
    Grow,
    /// Shrink effect
    Shrink,
    /// Rotate effect
    Rotate,
}

impl EmphasisEffect {
    /// Get effect string
    pub fn effect_str(&self) -> &str {
        match self {
            EmphasisEffect::Bold => "bold",
            EmphasisEffect::Underline => "underline",
            EmphasisEffect::Italic => "italic",
            EmphasisEffect::Color => "color",
            EmphasisEffect::Grow => "grow",
            EmphasisEffect::Shrink => "shrink",
            EmphasisEffect::Rotate => "rotate",
        }
    }
}

/// Animation effect
#[derive(Clone, Debug)]
pub struct Animation {
    /// Animation ID
    id: u32,
    /// Animation type
    animation_type: AnimationType,
    /// Effect (stored as string for flexibility)
    effect: String,
    /// Duration in milliseconds
    duration: u32,
    /// Delay in milliseconds
    delay: u32,
    /// Repeat count (0 = no repeat)
    repeat_count: u32,
}

impl Animation {
    /// Create a new animation
    pub fn new(id: u32, animation_type: AnimationType, effect: String) -> Self {
        Self {
            id,
            animation_type,
            effect,
            duration: 500,
            delay: 0,
            repeat_count: 0,
        }
    }

    /// Create entrance animation
    pub fn entrance(id: u32, effect: EntranceEffect) -> Self {
        Self::new(id, AnimationType::Entrance, effect.effect_str().to_string())
    }

    /// Create exit animation
    pub fn exit(id: u32, effect: ExitEffect) -> Self {
        Self::new(id, AnimationType::Exit, effect.effect_str().to_string())
    }

    /// Create emphasis animation
    pub fn emphasis(id: u32, effect: EmphasisEffect) -> Self {
        Self::new(id, AnimationType::Emphasis, effect.effect_str().to_string())
    }

    /// Set duration
    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    /// Get duration
    pub fn duration(&self) -> u32 {
        self.duration
    }

    /// Set delay
    pub fn set_delay(&mut self, delay: u32) {
        self.delay = delay;
    }

    /// Get delay
    pub fn delay(&self) -> u32 {
        self.delay
    }

    /// Set repeat count
    pub fn set_repeat_count(&mut self, repeat_count: u32) {
        self.repeat_count = repeat_count;
    }

    /// Get repeat count
    pub fn repeat_count(&self) -> u32 {
        self.repeat_count
    }

    /// Get animation ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get animation type
    pub fn animation_type(&self) -> &AnimationType {
        &self.animation_type
    }

    /// Get effect
    pub fn effect(&self) -> &str {
        &self.effect
    }

    /// Generate XML for animation
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(&format!(
            r#"<p:animEffect id="{}" type="{}" effect="{}" dur="{}" delay="{}" repeatCount="{}">"#,
            self.id, self.animation_type.type_str(), self.effect, self.duration, self.delay, self.repeat_count
        ));
        xml.push_str(r#"</p:animEffect>"#);
        xml
    }
}

/// Animation Manager
#[derive(Clone, Debug)]
pub struct AnimationManager {
    /// Animations
    animations: Vec<Animation>,
}

impl AnimationManager {
    /// Create a new animation manager
    pub fn new() -> Self {
        Self {
            animations: vec![],
        }
    }

    /// Add an animation
    pub fn add_animation(&mut self, animation: Animation) -> usize {
        self.animations.push(animation);
        self.animations.len() - 1
    }

    /// Add entrance animation
    pub fn add_entrance(&mut self, effect: EntranceEffect) -> usize {
        let id = self.animations.len() as u32;
        self.add_animation(Animation::entrance(id, effect))
    }

    /// Add exit animation
    pub fn add_exit(&mut self, effect: ExitEffect) -> usize {
        let id = self.animations.len() as u32;
        self.add_animation(Animation::exit(id, effect))
    }

    /// Add emphasis animation
    pub fn add_emphasis(&mut self, effect: EmphasisEffect) -> usize {
        let id = self.animations.len() as u32;
        self.add_animation(Animation::emphasis(id, effect))
    }

    /// Get animation by index
    pub fn get(&self, index: usize) -> Option<&Animation> {
        self.animations.get(index)
    }

    /// Get mutable animation by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Animation> {
        self.animations.get_mut(index)
    }

    /// Get all animations
    pub fn all(&self) -> &[Animation] {
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
        xml.push_str(r#"<p:timing><p:tnLst>"#);
        xml.push('\n');

        for animation in &self.animations {
            xml.push_str(&animation.to_xml());
            xml.push('\n');
        }

        xml.push_str(r#"</p:tnLst></p:timing>"#);
        xml
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_type_str() {
        assert_eq!(AnimationType::Entrance.type_str(), "entrance");
        assert_eq!(AnimationType::Exit.type_str(), "exit");
        assert_eq!(AnimationType::Emphasis.type_str(), "emphasis");
    }

    #[test]
    fn test_entrance_effect_str() {
        assert_eq!(EntranceEffect::Appear.effect_str(), "appear");
        assert_eq!(EntranceEffect::Fade.effect_str(), "fade");
        assert_eq!(EntranceEffect::FlyIn.effect_str(), "flyIn");
    }

    #[test]
    fn test_exit_effect_str() {
        assert_eq!(ExitEffect::Disappear.effect_str(), "disappear");
        assert_eq!(ExitEffect::Fade.effect_str(), "fade");
        assert_eq!(ExitEffect::FlyOut.effect_str(), "flyOut");
    }

    #[test]
    fn test_emphasis_effect_str() {
        assert_eq!(EmphasisEffect::Bold.effect_str(), "bold");
        assert_eq!(EmphasisEffect::Color.effect_str(), "color");
    }

    #[test]
    fn test_animation_creation() {
        let anim = Animation::new(1, AnimationType::Entrance, "appear".to_string());
        assert_eq!(anim.id(), 1);
        assert_eq!(anim.duration(), 500);
        assert_eq!(anim.delay(), 0);
    }

    #[test]
    fn test_animation_entrance() {
        let anim = Animation::entrance(1, EntranceEffect::Fade);
        assert_eq!(anim.animation_type(), &AnimationType::Entrance);
        assert_eq!(anim.effect(), "fade");
    }

    #[test]
    fn test_animation_exit() {
        let anim = Animation::exit(1, ExitEffect::Zoom);
        assert_eq!(anim.animation_type(), &AnimationType::Exit);
        assert_eq!(anim.effect(), "zoom");
    }

    #[test]
    fn test_animation_emphasis() {
        let anim = Animation::emphasis(1, EmphasisEffect::Grow);
        assert_eq!(anim.animation_type(), &AnimationType::Emphasis);
        assert_eq!(anim.effect(), "grow");
    }

    #[test]
    fn test_animation_properties() {
        let mut anim = Animation::new(1, AnimationType::Entrance, "appear".to_string());
        anim.set_duration(1000);
        anim.set_delay(500);
        anim.set_repeat_count(2);

        assert_eq!(anim.duration(), 1000);
        assert_eq!(anim.delay(), 500);
        assert_eq!(anim.repeat_count(), 2);
    }

    #[test]
    fn test_animation_to_xml() {
        let anim = Animation::new(1, AnimationType::Entrance, "appear".to_string());
        let xml = anim.to_xml();
        assert!(xml.contains(r#"<p:animEffect"#));
        assert!(xml.contains(r#"id="1""#));
        assert!(xml.contains(r#"type="entrance""#));
        assert!(xml.contains(r#"effect="appear""#));
    }

    #[test]
    fn test_animation_manager_creation() {
        let manager = AnimationManager::new();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_animation_manager_add() {
        let mut manager = AnimationManager::new();
        manager.add_entrance(EntranceEffect::Fade);
        manager.add_exit(ExitEffect::Zoom);

        assert_eq!(manager.len(), 2);
    }

    #[test]
    fn test_animation_manager_get() {
        let mut manager = AnimationManager::new();
        manager.add_entrance(EntranceEffect::Fade);

        let anim = manager.get(0);
        assert!(anim.is_some());
        assert_eq!(anim.unwrap().effect(), "fade");
    }

    #[test]
    fn test_animation_manager_get_mut() {
        let mut manager = AnimationManager::new();
        manager.add_entrance(EntranceEffect::Fade);

        if let Some(anim) = manager.get_mut(0) {
            anim.set_duration(2000);
        }

        assert_eq!(manager.get(0).unwrap().duration(), 2000);
    }

    #[test]
    fn test_animation_manager_clear() {
        let mut manager = AnimationManager::new();
        manager.add_entrance(EntranceEffect::Fade);
        manager.add_exit(ExitEffect::Zoom);
        assert_eq!(manager.len(), 2);

        manager.clear();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_animation_manager_to_xml() {
        let mut manager = AnimationManager::new();
        manager.add_entrance(EntranceEffect::Fade);

        let xml = manager.to_xml();
        assert!(xml.contains(r#"<p:timing>"#));
        assert!(xml.contains(r#"<p:tnLst>"#));
        assert!(xml.contains(r#"<p:animEffect"#));
        assert!(xml.contains(r#"</p:tnLst>"#));
        assert!(xml.contains(r#"</p:timing>"#));
    }

    #[test]
    fn test_animation_manager_default() {
        let manager = AnimationManager::default();
        assert!(manager.is_empty());
    }
}
