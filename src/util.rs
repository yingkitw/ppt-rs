//! Utility functions and helpers

pub mod validation;

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

