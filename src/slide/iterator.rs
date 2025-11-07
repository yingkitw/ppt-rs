//! Iterator over slides

use super::{Slide, Slides};

/// Iterator over slides
pub struct SlideIterator<'a, 'b> {
    pub(super) slides: &'a mut Slides<'b>,
    pub(super) package: &'a mut crate::opc::package::Package,
    pub(super) index: usize,
}

impl<'a, 'b> Iterator for SlideIterator<'a, 'b> {
    type Item = Slide;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.slides.len() {
            let slide = self.slides.get(self.index, self.package);
            self.index += 1;
            slide
        } else {
            None
        }
    }
}

