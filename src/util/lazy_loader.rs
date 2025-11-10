//! Lazy loading utilities for deferred computation
//!
//! Provides lazy-loaded collections and values that are computed
//! only when first accessed, reducing memory footprint and improving startup time.

use crate::error::Result;
use std::sync::{Arc, Mutex};

/// Lazy-loaded value that is computed on first access
pub struct LazyValue<T> {
    value: Arc<Mutex<Option<T>>>,
    loader: Arc<dyn Fn() -> Result<T> + Send + Sync>,
}

impl<T: Clone> LazyValue<T> {
    /// Create a new lazy value with a loader function
    pub fn new<F>(loader: F) -> Self
    where
        F: Fn() -> Result<T> + Send + Sync + 'static,
    {
        Self {
            value: Arc::new(Mutex::new(None)),
            loader: Arc::new(loader),
        }
    }

    /// Get the value, loading it if necessary
    pub fn get(&self) -> Result<T> {
        let mut value = self.value.lock().unwrap();
        
        if let Some(ref v) = *value {
            Ok(v.clone())
        } else {
            let loaded = (self.loader)()?;
            *value = Some(loaded.clone());
            Ok(loaded)
        }
    }

    /// Check if value has been loaded
    pub fn is_loaded(&self) -> bool {
        self.value.lock().unwrap().is_some()
    }

    /// Reset the value (force reload on next access)
    pub fn reset(&self) {
        *self.value.lock().unwrap() = None;
    }
}

impl<T: Clone> Clone for LazyValue<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            loader: Arc::clone(&self.loader),
        }
    }
}

/// Lazy-loaded collection that loads items on first access
pub struct LazyCollection<T> {
    items: Arc<Mutex<Option<Vec<T>>>>,
    loader: Arc<dyn Fn() -> Result<Vec<T>> + Send + Sync>,
}

impl<T: Clone> LazyCollection<T> {
    /// Create a new lazy collection with a loader function
    pub fn new<F>(loader: F) -> Self
    where
        F: Fn() -> Result<Vec<T>> + Send + Sync + 'static,
    {
        Self {
            items: Arc::new(Mutex::new(None)),
            loader: Arc::new(loader),
        }
    }

    /// Get all items, loading if necessary
    pub fn get_all(&self) -> Result<Vec<T>> {
        let mut items = self.items.lock().unwrap();
        
        if let Some(ref v) = *items {
            Ok(v.clone())
        } else {
            let loaded = (self.loader)()?;
            *items = Some(loaded.clone());
            Ok(loaded)
        }
    }

    /// Get item count, loading if necessary
    pub fn len(&self) -> Result<usize> {
        Ok(self.get_all()?.len())
    }

    /// Check if collection is empty
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.get_all()?.is_empty())
    }

    /// Check if items have been loaded
    pub fn is_loaded(&self) -> bool {
        self.items.lock().unwrap().is_some()
    }

    /// Reset the collection (force reload on next access)
    pub fn reset(&self) {
        *self.items.lock().unwrap() = None;
    }
}

impl<T: Clone> Clone for LazyCollection<T> {
    fn clone(&self) -> Self {
        Self {
            items: Arc::clone(&self.items),
            loader: Arc::clone(&self.loader),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_value_creation() {
        let lazy = LazyValue::new(|| Ok(42));
        assert!(!lazy.is_loaded());
    }

    #[test]
    fn test_lazy_value_get() {
        let lazy = LazyValue::new(|| Ok(42));
        let value = lazy.get().unwrap();
        assert_eq!(value, 42);
        assert!(lazy.is_loaded());
    }

    #[test]
    fn test_lazy_value_cached() {
        let call_count = Arc::new(Mutex::new(0));
        let call_count_clone = Arc::clone(&call_count);
        
        let lazy = LazyValue::new(move || {
            let mut count = call_count_clone.lock().unwrap();
            *count += 1;
            Ok(42)
        });
        
        let _v1 = lazy.get().unwrap();
        let _v2 = lazy.get().unwrap();
        
        let count = call_count.lock().unwrap();
        assert_eq!(*count, 1); // Loader called only once
    }

    #[test]
    fn test_lazy_value_reset() {
        let call_count = Arc::new(Mutex::new(0));
        let call_count_clone = Arc::clone(&call_count);
        
        let lazy = LazyValue::new(move || {
            let mut count = call_count_clone.lock().unwrap();
            *count += 1;
            Ok(42)
        });
        
        let _v1 = lazy.get().unwrap();
        lazy.reset();
        let _v2 = lazy.get().unwrap();
        
        let count = call_count.lock().unwrap();
        assert_eq!(*count, 2); // Loader called twice after reset
    }

    #[test]
    fn test_lazy_collection_creation() {
        let lazy = LazyCollection::new(|| Ok(vec![1, 2, 3]));
        assert!(!lazy.is_loaded());
    }

    #[test]
    fn test_lazy_collection_get_all() {
        let lazy = LazyCollection::new(|| Ok(vec![1, 2, 3]));
        let items = lazy.get_all().unwrap();
        assert_eq!(items.len(), 3);
        assert!(lazy.is_loaded());
    }

    #[test]
    fn test_lazy_collection_len() {
        let lazy = LazyCollection::new(|| Ok(vec![1, 2, 3]));
        let len = lazy.len().unwrap();
        assert_eq!(len, 3);
    }

    #[test]
    fn test_lazy_collection_is_empty() {
        let lazy_empty = LazyCollection::new(|| Ok::<Vec<i32>, _>(vec![]));
        assert!(lazy_empty.is_empty().unwrap());
        
        let lazy_full = LazyCollection::new(|| Ok(vec![1, 2, 3]));
        assert!(!lazy_full.is_empty().unwrap());
    }

    #[test]
    fn test_lazy_collection_reset() {
        let call_count = Arc::new(Mutex::new(0));
        let call_count_clone = Arc::clone(&call_count);
        
        let lazy = LazyCollection::new(move || {
            let mut count = call_count_clone.lock().unwrap();
            *count += 1;
            Ok(vec![1, 2, 3])
        });
        
        let _v1 = lazy.get_all().unwrap();
        lazy.reset();
        let _v2 = lazy.get_all().unwrap();
        
        let count = call_count.lock().unwrap();
        assert_eq!(*count, 2); // Loader called twice after reset
    }

    #[test]
    fn test_lazy_value_clone() {
        let lazy1 = LazyValue::new(|| Ok(42));
        let lazy2 = lazy1.clone();
        
        let _v1 = lazy1.get().unwrap();
        assert!(lazy2.is_loaded()); // Cloned value shares state
    }

    #[test]
    fn test_lazy_collection_clone() {
        let lazy1 = LazyCollection::new(|| Ok(vec![1, 2, 3]));
        let lazy2 = lazy1.clone();
        
        let _v1 = lazy1.get_all().unwrap();
        assert!(lazy2.is_loaded()); // Cloned collection shares state
    }
}
