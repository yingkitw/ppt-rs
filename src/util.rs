//! Utility functions and helpers

pub mod cache;
pub mod error_context;
pub mod performance;
pub mod roundtrip;
pub mod shape_content;
pub mod validation;
pub mod media_formats;

pub use cache::LazyCache;
pub use error_context::Validator;
pub use performance::{PerformanceMetrics, Timer, BatchProcessor};
pub use roundtrip::RoundTrip;
pub use shape_content::{ShapeContent, ShapeContentType, PlaceholderType, ShapeContentLoader};
pub use validation::validate_presentation;
pub use media_formats::{MediaFormat, SVGConfig, GIFConfig, YouTubeConfig};

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

