//! Slide-related types and functionality

use crate::error::Result;
use crate::parts::presentation::PresentationPart;
use crate::parts::slide::SlidePart;
use crate::shapes::Shape;

/// Collection of slides in a presentation
pub struct Slides<'a> {
    presentation_part: &'a PresentationPart,
}

impl<'a> Slides<'a> {
    pub fn new(presentation_part: &'a PresentationPart) -> Self {
        Self {
            presentation_part,
        }
    }

    /// Add a new slide
    pub fn add_slide(&mut self, _slide_layout: &SlidePart) -> Result<Slide> {
        // TODO: Implement slide addition
        // This should:
        // 1. Create a new SlidePart
        // 2. Add it to the presentation
        // 3. Return a Slide wrapper
        Ok(Slide::new())
    }

    /// Get a slide by index
    pub fn get(&self, _index: usize) -> Option<Slide> {
        // TODO: Implement
        None
    }

    /// Get the number of slides
    pub fn len(&self) -> usize {
        // TODO: Count slides
        0
    }

    /// Check if slides collection is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over slides
    pub fn iter(&self) -> SlideIterator {
        SlideIterator {
            slides: self,
            index: 0,
        }
    }
}

/// Iterator over slides
pub struct SlideIterator<'a> {
    slides: &'a Slides<'a>,
    index: usize,
}

impl<'a> Iterator for SlideIterator<'a> {
    type Item = Slide;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.slides.len() {
            let slide = self.slides.get(self.index);
            self.index += 1;
            slide
        } else {
            None
        }
    }
}

/// Individual slide
pub struct Slide {
    part: Option<SlidePart>,
    name: String,
}

impl Slide {
    pub fn new() -> Self {
        Self {
            part: None,
            name: String::new(),
        }
    }

    pub fn with_part(part: SlidePart) -> Self {
        Self {
            part: Some(part),
            name: String::new(),
        }
    }

    /// Get the slide name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the slide name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get shapes on this slide
    pub fn shapes(&self) -> Vec<Box<dyn Shape>> {
        // TODO: Implement shape collection
        Vec::new()
    }

    /// Add a shape to the slide
    pub fn add_shape(&mut self, _shape: Box<dyn Shape>) -> Result<()> {
        // TODO: Implement
        Ok(())
    }
}

/// Slide masters collection
pub struct SlideMasters {
    // TODO: Implement
}

impl SlideMasters {
    pub fn new() -> Self {
        Self {}
    }
}

/// Slide layouts collection
pub struct SlideLayouts {
    // TODO: Implement
}

impl SlideLayouts {
    pub fn new() -> Self {
        Self {}
    }

    /// Get a layout by name
    pub fn get_by_name(&self, _name: &str) -> Option<()> {
        // TODO: Implement
        None
    }
}

