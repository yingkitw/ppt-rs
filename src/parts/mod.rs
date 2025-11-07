//! Parts module - different types of parts in a PPTX package

pub mod presentation;
pub mod slide;
pub mod image;
pub mod chart;
pub mod coreprops;
pub mod media;

pub use presentation::PresentationPart;
pub use slide::{SlidePart, SlideLayoutPart, SlideMasterPart, NotesMasterPart, NotesSlidePart, BaseSlidePart};
pub use image::ImagePart;
pub use chart::ChartPart;
pub use coreprops::CorePropertiesPart;
pub use media::MediaPart;

