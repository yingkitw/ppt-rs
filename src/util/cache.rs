//! Lazy loading cache for parts and shapes

use std::collections::HashMap;

/// Generic lazy-load cache
#[derive(Clone, Debug)]
pub struct LazyCache<K, V> {
    /// Cached values
    cache: HashMap<K, V>,
}

impl<K: Eq + std::hash::Hash + Clone, V: Clone> LazyCache<K, V> {
    /// Create a new lazy cache
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Get or compute a value
    pub fn get_or_compute<F>(&mut self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(value) = self.cache.get(&key) {
            value.clone()
        } else {
            let value = compute();
            self.cache.insert(key, value.clone());
            value
        }
    }

    /// Get cached value
    pub fn get(&self, key: &K) -> Option<V> {
        self.cache.get(key).cloned()
    }

    /// Insert value
    pub fn insert(&mut self, key: K, value: V) {
        self.cache.insert(key, value);
    }

    /// Clear cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

impl<K, V> Default for LazyCache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_cache_creation() {
        let cache: LazyCache<u32, String> = LazyCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_lazy_cache_insert_and_get() {
        let mut cache = LazyCache::new();
        cache.insert(1, "one".to_string());
        assert_eq!(cache.get(&1), Some("one".to_string()));
    }

    #[test]
    fn test_lazy_cache_compute() {
        let mut cache = LazyCache::new();
        let value = cache.get_or_compute(1, || "computed".to_string());
        assert_eq!(value, "computed");
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_lazy_cache_compute_once() {
        let mut cache = LazyCache::new();
        let mut call_count = 0;
        
        cache.get_or_compute(1, || {
            call_count += 1;
            "value".to_string()
        });
        
        cache.get_or_compute(1, || {
            call_count += 1;
            "value".to_string()
        });
        
        assert_eq!(call_count, 1); // Should only compute once
    }

    #[test]
    fn test_lazy_cache_clear() {
        let mut cache = LazyCache::new();
        cache.insert(1, "one".to_string());
        assert_eq!(cache.len(), 1);
        cache.clear();
        assert!(cache.is_empty());
    }
}
