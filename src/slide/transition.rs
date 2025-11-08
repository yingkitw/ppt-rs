//! Slide transition functionality

/// Slide transition type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionType {
    /// No transition
    None,
    /// Fade transition
    Fade,
    /// Push transition
    Push,
    /// Wipe transition
    Wipe,
    /// Cover transition
    Cover,
    /// Uncover transition
    Uncover,
    /// Split transition
    Split,
    /// Reveal transition
    Reveal,
    /// Wheel transition
    Wheel,
    /// Dissolve transition
    Dissolve,
    /// Checkerboard transition
    Checkerboard,
    /// Blinds transition
    Blinds,
    /// Strips transition
    Strips,
    /// Wedge transition
    Wedge,
    /// Zoom transition
    Zoom,
    /// Iris transition
    Iris,
    /// Diamond transition
    Diamond,
    /// Plus transition
    Plus,
    /// Pixelate transition
    Pixelate,
    /// Fly transition
    Fly,
    /// Morph transition
    Morph,
}

/// Slide transition direction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionDirection {
    /// Left direction
    Left,
    /// Right direction
    Right,
    /// Up direction
    Up,
    /// Down direction
    Down,
    /// Horizontal direction
    Horizontal,
    /// Vertical direction
    Vertical,
    /// Diagonal direction
    Diagonal,
}

/// Slide transition - provides access to transition properties
pub struct SlideTransition {
    transition_type: TransitionType,
    duration: u32,  // in milliseconds
    advance_on_click: bool,
    advance_after: Option<u32>,  // in milliseconds
    direction: Option<TransitionDirection>,
}

impl SlideTransition {
    /// Create a new slide transition with no transition
    pub fn new() -> Self {
        Self {
            transition_type: TransitionType::None,
            duration: 500,
            advance_on_click: true,
            advance_after: None,
            direction: None,
        }
    }

    /// Create a transition with a specific type
    pub fn with_type(transition_type: TransitionType) -> Self {
        Self {
            transition_type,
            duration: 500,
            advance_on_click: true,
            advance_after: None,
            direction: None,
        }
    }

    /// Get the transition type
    pub fn transition_type(&self) -> TransitionType {
        self.transition_type
    }

    /// Set the transition type
    pub fn set_transition_type(&mut self, transition_type: TransitionType) {
        self.transition_type = transition_type;
    }

    /// Get the duration in milliseconds
    pub fn duration(&self) -> u32 {
        self.duration
    }

    /// Set the duration in milliseconds
    pub fn set_duration(&mut self, duration: u32) -> crate::error::Result<()> {
        if duration > 10000 {
            return Err(crate::error::PptError::ValueError(
                format!("Duration must be <= 10000ms, got {}", duration)
            ));
        }
        self.duration = duration;
        Ok(())
    }

    /// Check if transition advances on click
    pub fn advance_on_click(&self) -> bool {
        self.advance_on_click
    }

    /// Set whether transition advances on click
    pub fn set_advance_on_click(&mut self, advance: bool) {
        self.advance_on_click = advance;
    }

    /// Get the advance after time in milliseconds
    pub fn advance_after(&self) -> Option<u32> {
        self.advance_after
    }

    /// Set the advance after time in milliseconds
    pub fn set_advance_after(&mut self, time: Option<u32>) -> crate::error::Result<()> {
        if let Some(t) = time {
            if t > 300000 {
                return Err(crate::error::PptError::ValueError(
                    format!("Advance after time must be <= 300000ms, got {}", t)
                ));
            }
        }
        self.advance_after = time;
        Ok(())
    }

    /// Get the transition direction
    pub fn direction(&self) -> Option<TransitionDirection> {
        self.direction
    }

    /// Set the transition direction
    pub fn set_direction(&mut self, direction: Option<TransitionDirection>) {
        self.direction = direction;
    }

    /// Generate transition XML for slide
    pub fn to_xml(&self) -> String {
        if self.transition_type == TransitionType::None {
            return String::new();
        }

        let transition_name = match self.transition_type {
            TransitionType::Fade => "fade",
            TransitionType::Push => "push",
            TransitionType::Wipe => "wipe",
            TransitionType::Cover => "cover",
            TransitionType::Uncover => "uncover",
            TransitionType::Split => "split",
            TransitionType::Reveal => "reveal",
            TransitionType::Wheel => "wheel",
            TransitionType::Dissolve => "dissolve",
            TransitionType::Checkerboard => "checker",
            TransitionType::Blinds => "blinds",
            TransitionType::Strips => "strips",
            TransitionType::Wedge => "wedge",
            TransitionType::Zoom => "zoom",
            TransitionType::Iris => "iris",
            TransitionType::Diamond => "diamond",
            TransitionType::Plus => "plus",
            TransitionType::Pixelate => "pixelate",
            TransitionType::Fly => "fly",
            TransitionType::Morph => "morph",
            TransitionType::None => return String::new(),
        };

        let direction_attr = if let Some(dir) = self.direction {
            let dir_str = match dir {
                TransitionDirection::Left => "l",
                TransitionDirection::Right => "r",
                TransitionDirection::Up => "u",
                TransitionDirection::Down => "d",
                TransitionDirection::Horizontal => "h",
                TransitionDirection::Vertical => "v",
                TransitionDirection::Diagonal => "diag",
            };
            format!(r#" dir="{}""#, dir_str)
        } else {
            String::new()
        };

        let advance_attr = if let Some(time) = self.advance_after {
            format!(r#" advTm="{}""#, time)
        } else {
            String::new()
        };

        let on_click = if self.advance_on_click { "1" } else { "0" };

        format!(
            r#"<p:transition spd="med" dur="{}" advTm="{}" spd="med"{}{}>
  <p:{} />
</p:transition>"#,
            self.duration,
            if self.advance_after.is_some() { self.advance_after.unwrap() } else { 0 },
            direction_attr,
            advance_attr,
            transition_name
        )
    }
}

impl Default for SlideTransition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_new() {
        let trans = SlideTransition::new();
        assert_eq!(trans.transition_type(), TransitionType::None);
        assert_eq!(trans.duration(), 500);
        assert!(trans.advance_on_click());
    }

    #[test]
    fn test_transition_with_type() {
        let trans = SlideTransition::with_type(TransitionType::Fade);
        assert_eq!(trans.transition_type(), TransitionType::Fade);
    }

    #[test]
    fn test_transition_set_type() {
        let mut trans = SlideTransition::new();
        trans.set_transition_type(TransitionType::Push);
        assert_eq!(trans.transition_type(), TransitionType::Push);
    }

    #[test]
    fn test_transition_duration() {
        let mut trans = SlideTransition::new();
        trans.set_duration(1000).unwrap();
        assert_eq!(trans.duration(), 1000);
    }

    #[test]
    fn test_transition_duration_invalid() {
        let mut trans = SlideTransition::new();
        assert!(trans.set_duration(15000).is_err());
    }

    #[test]
    fn test_transition_advance_on_click() {
        let mut trans = SlideTransition::new();
        assert!(trans.advance_on_click());
        
        trans.set_advance_on_click(false);
        assert!(!trans.advance_on_click());
    }

    #[test]
    fn test_transition_advance_after() {
        let mut trans = SlideTransition::new();
        assert!(trans.advance_after().is_none());
        
        trans.set_advance_after(Some(3000)).unwrap();
        assert_eq!(trans.advance_after(), Some(3000));
    }

    #[test]
    fn test_transition_advance_after_invalid() {
        let mut trans = SlideTransition::new();
        assert!(trans.set_advance_after(Some(400000)).is_err());
    }

    #[test]
    fn test_transition_direction() {
        let mut trans = SlideTransition::new();
        assert!(trans.direction().is_none());
        
        trans.set_direction(Some(TransitionDirection::Left));
        assert_eq!(trans.direction(), Some(TransitionDirection::Left));
    }

    #[test]
    fn test_transition_xml_none() {
        let trans = SlideTransition::new();
        assert_eq!(trans.to_xml(), "");
    }

    #[test]
    fn test_transition_xml_fade() {
        let trans = SlideTransition::with_type(TransitionType::Fade);
        let xml = trans.to_xml();
        assert!(xml.contains("<p:transition"));
        assert!(xml.contains("p:fade"));
    }

    #[test]
    fn test_transition_xml_with_direction() {
        let mut trans = SlideTransition::with_type(TransitionType::Wipe);
        trans.set_direction(Some(TransitionDirection::Left));
        let xml = trans.to_xml();
        assert!(xml.contains(r#"dir="l""#));
    }

    #[test]
    fn test_transition_xml_with_advance_after() {
        let mut trans = SlideTransition::with_type(TransitionType::Push);
        trans.set_advance_after(Some(2000)).unwrap();
        let xml = trans.to_xml();
        assert!(xml.contains(r#"advTm="2000""#));
    }

    #[test]
    fn test_all_transition_types() {
        let types = vec![
            TransitionType::Fade,
            TransitionType::Push,
            TransitionType::Wipe,
            TransitionType::Cover,
            TransitionType::Uncover,
            TransitionType::Split,
            TransitionType::Reveal,
            TransitionType::Wheel,
            TransitionType::Dissolve,
            TransitionType::Checkerboard,
            TransitionType::Blinds,
            TransitionType::Strips,
            TransitionType::Wedge,
            TransitionType::Zoom,
            TransitionType::Iris,
            TransitionType::Diamond,
            TransitionType::Plus,
            TransitionType::Pixelate,
            TransitionType::Fly,
            TransitionType::Morph,
        ];
        
        for trans_type in types {
            let trans = SlideTransition::with_type(trans_type);
            assert_eq!(trans.transition_type(), trans_type);
        }
    }

    #[test]
    fn test_all_transition_directions() {
        let directions = vec![
            TransitionDirection::Left,
            TransitionDirection::Right,
            TransitionDirection::Up,
            TransitionDirection::Down,
            TransitionDirection::Horizontal,
            TransitionDirection::Vertical,
            TransitionDirection::Diagonal,
        ];
        
        for dir in directions {
            let mut trans = SlideTransition::new();
            trans.set_direction(Some(dir));
            assert_eq!(trans.direction(), Some(dir));
        }
    }
}
