//! Utility functions and helpers

pub mod cache;
pub mod error_context;
pub mod performance;
pub mod roundtrip;
pub mod shape_content;
pub mod validation;
pub mod media_formats;
pub mod thumbnail;
pub mod ole_embedding;
pub mod media_embedding;

pub use cache::LazyCache;
pub use error_context::Validator;
pub use performance::{PerformanceMetrics, Timer, BatchProcessor};
pub use roundtrip::RoundTrip;
pub use shape_content::{ShapeContent, ShapeContentType, PlaceholderType, ShapeContentLoader};
pub use validation::validate_presentation;
pub use media_formats::{MediaFormat, SVGConfig, GIFConfig, YouTubeConfig};
pub use thumbnail::generate_thumbnail_jpeg;
pub use ole_embedding::{OLEObjectType, OLEEmbeddedObject, OLEObjectManager};
pub use media_embedding::{MediaType, MediaPlayback, EmbeddedMedia, MediaManager};

use std::cell::Cell;

/// Lazy property implementation
pub struct LazyProperty<T> {
    cell: Cell<Option<T>>,
    init: Box<dyn Fn() -> T>,
}

impl<T> LazyProperty<T> {
    pub fn new<F>(init: F) -> Self
    where
        F: Fn() -> T + 'static,
    {
        Self {
            cell: Cell::new(None),
            init: Box::new(init),
        }
    }

    pub fn get(&self) -> T
    where
        T: Clone,
    {
        if let Some(ref value) = self.cell.take() {
            let cloned = value.clone();
            self.cell.set(Some(cloned.clone()));
            cloned
        } else {
            let value = (self.init)();
            self.cell.set(Some(value.clone()));
            value
        }
    }
}

