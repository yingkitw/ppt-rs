//! Animation Shape - Animation effects for shapes on slides

use crate::slide::{Animation, AnimationType};

/// Animated shape wrapper
#[derive(Clone, Debug)]
pub struct AnimatedShape {
    /// Shape ID
    shape_id: u32,
    /// Animations applied to this shape
    animations: Vec<Animation>,
}

impl AnimatedShape {
    /// Create a new animated shape
    pub fn new(shape_id: u32) -> Self {
        Self {
            shape_id,
            animations: vec![],
        }
    }

    /// Get shape ID
    pub fn shape_id(&self) -> u32 {
        self.shape_id
    }

    /// Add animation
    pub fn add_animation(&mut self, animation: Animation) -> usize {
        self.animations.push(animation);
        self.animations.len() - 1
    }

    /// Get animation by index
    pub fn get_animation(&self, index: usize) -> Option<&Animation> {
        self.animations.get(index)
    }

    /// Get mutable animation by index
    pub fn get_animation_mut(&mut self, index: usize) -> Option<&mut Animation> {
        self.animations.get_mut(index)
    }

    /// Get all animations
    pub fn animations(&self) -> &[Animation] {
        &self.animations
    }

    /// Get animation count
    pub fn animation_count(&self) -> usize {
        self.animations.len()
    }

    /// Check if has animations
    pub fn has_animations(&self) -> bool {
        !self.animations.is_empty()
    }

    /// Generate XML for animations
    pub fn animations_to_xml(&self) -> String {
        if self.animations.is_empty() {
            return String::new();
        }
        
        let mut xml = String::from("<p:timing><p:tnLst>");
        for anim in &self.animations {
            xml.push_str(&anim.to_xml());
        }
        xml.push_str("</p:tnLst></p:timing>");
        xml
    }
}

/// Animation shape manager
#[derive(Clone, Debug)]
pub struct AnimationShapeManager {
    shapes: Vec<AnimatedShape>,
}

impl AnimationShapeManager {
    /// Create a new animation shape manager
    pub fn new() -> Self {
        Self {
            shapes: vec![],
        }
    }

    /// Add an animated shape
    pub fn add_shape(&mut self, shape: AnimatedShape) -> usize {
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    /// Create and add an animated shape
    pub fn create_shape(&mut self, shape_id: u32) -> usize {
        self.add_shape(AnimatedShape::new(shape_id))
    }

    /// Get shape by index
    pub fn get(&self, index: usize) -> Option<&AnimatedShape> {
        self.shapes.get(index)
    }

    /// Get mutable shape by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut AnimatedShape> {
        self.shapes.get_mut(index)
    }

    /// Get shape by shape_id
    pub fn get_by_shape_id(&self, shape_id: u32) -> Option<&AnimatedShape> {
        self.shapes.iter().find(|s| s.shape_id() == shape_id)
    }

    /// Get mutable shape by shape_id
    pub fn get_by_shape_id_mut(&mut self, shape_id: u32) -> Option<&mut AnimatedShape> {
        self.shapes.iter_mut().find(|s| s.shape_id() == shape_id)
    }

    /// Get all shapes
    pub fn all(&self) -> &[AnimatedShape] {
        &self.shapes
    }

    /// Get shape count
    pub fn len(&self) -> usize {
        self.shapes.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    /// Generate XML for all animations
    pub fn to_xml(&self) -> String {
        let mut xml = String::from("<p:timing><p:tnLst>");
        for shape in &self.shapes {
            for anim in shape.animations() {
                xml.push_str(&anim.to_xml());
            }
        }
        xml.push_str("</p:tnLst></p:timing>");
        xml
    }
}

impl Default for AnimationShapeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animated_shape_creation() {
        let shape = AnimatedShape::new(2);
        assert_eq!(shape.shape_id(), 2);
        assert!(!shape.has_animations());
    }


    #[test]
    fn test_animation_shape_manager() {
        let mut manager = AnimationShapeManager::new();
        manager.create_shape(2);
        assert_eq!(manager.len(), 1);
        assert!(manager.get(0).is_some());
    }

    #[test]
    fn test_animation_shape_manager_get_by_id() {
        let mut manager = AnimationShapeManager::new();
        manager.create_shape(2);
        manager.create_shape(3);
        assert!(manager.get_by_shape_id(2).is_some());
        assert!(manager.get_by_shape_id(3).is_some());
        assert!(manager.get_by_shape_id(4).is_none());
    }

}
