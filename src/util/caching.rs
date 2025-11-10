//! Caching utilities for performance optimization
//!
//! Provides generic caching mechanisms for frequently accessed data
//! with configurable cache sizes and eviction policies.

use std::collections::HashMap;
use std::hash::Hash;

/// Simple LRU (Least Recently Used) cache
pub struct LruCache<K, V> {
    cache: HashMap<K, V>,
    max_size: usize,
    access_order: Vec<K>,
}

impl<K: Clone + Eq + Hash, V: Clone> LruCache<K, V> {
    /// Create a new LRU cache with specified max size
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            access_order: Vec::new(),
        }
    }

    /// Get a value from the cache
    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.cache.get(key) {
            // Move to end (most recently used)
            self.access_order.retain(|k| k != key);
            self.access_order.push(key.clone());
            Some(value.clone())
        } else {
            None
        }
    }

    /// Insert a value into the cache
    pub fn insert(&mut self, key: K, value: V) {
        if self.cache.contains_key(&key) {
            // Update existing
            self.cache.insert(key.clone(), value);
            self.access_order.retain(|k| k != &key);
            self.access_order.push(key);
        } else {
            // Check if we need to evict
            if self.cache.len() >= self.max_size {
                if let Some(lru_key) = self.access_order.first() {
                    let lru_key = lru_key.clone();
                    self.cache.remove(&lru_key);
                    self.access_order.remove(0);
                }
            }
            
            self.cache.insert(key.clone(), value);
            self.access_order.push(key);
        }
    }

    /// Check if key exists in cache
    pub fn contains_key(&self, key: &K) -> bool {
        self.cache.contains_key(key)
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.access_order.clear();
    }

    /// Get cache size
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Get max size
    pub fn max_size(&self) -> usize {
        self.max_size
    }
}

/// Simple statistics for cache performance
#[derive(Clone, Debug)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
    pub evictions: usize,
}

impl CacheStats {
    /// Create new cache statistics
    pub fn new() -> Self {
        Self {
            hits: 0,
            misses: 0,
            evictions: 0,
        }
    }

    /// Get hit rate (0.0 to 1.0)
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Get total accesses
    pub fn total_accesses(&self) -> usize {
        self.hits + self.misses
    }
}

impl Default for CacheStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache_new() {
        let cache: LruCache<String, i32> = LruCache::new(3);
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.max_size(), 3);
    }

    #[test]
    fn test_lru_cache_insert_and_get() {
        let mut cache = LruCache::new(3);
        cache.insert("a".to_string(), 1);
        cache.insert("b".to_string(), 2);
        
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&"a".to_string()), Some(1));
        assert_eq!(cache.get(&"b".to_string()), Some(2));
    }

    #[test]
    fn test_lru_cache_eviction() {
        let mut cache = LruCache::new(2);
        cache.insert("a".to_string(), 1);
        cache.insert("b".to_string(), 2);
        cache.insert("c".to_string(), 3); // Should evict "a"
        
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&"a".to_string()), None);
        assert_eq!(cache.get(&"b".to_string()), Some(2));
        assert_eq!(cache.get(&"c".to_string()), Some(3));
    }

    #[test]
    fn test_lru_cache_update_order() {
        let mut cache = LruCache::new(2);
        cache.insert("a".to_string(), 1);
        cache.insert("b".to_string(), 2);
        
        // Access "a" to make it recently used
        let _ = cache.get(&"a".to_string());
        
        // Insert "c", should evict "b" (least recently used)
        cache.insert("c".to_string(), 3);
        
        assert_eq!(cache.get(&"a".to_string()), Some(1));
        assert_eq!(cache.get(&"b".to_string()), None);
        assert_eq!(cache.get(&"c".to_string()), Some(3));
    }

    #[test]
    fn test_lru_cache_contains_key() {
        let mut cache = LruCache::new(2);
        cache.insert("a".to_string(), 1);
        
        assert!(cache.contains_key(&"a".to_string()));
        assert!(!cache.contains_key(&"b".to_string()));
    }

    #[test]
    fn test_lru_cache_clear() {
        let mut cache = LruCache::new(2);
        cache.insert("a".to_string(), 1);
        cache.insert("b".to_string(), 2);
        
        assert_eq!(cache.len(), 2);
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_stats_new() {
        let stats = CacheStats::new();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.evictions, 0);
        assert_eq!(stats.hit_rate(), 0.0);
    }

    #[test]
    fn test_cache_stats_hit_rate() {
        let mut stats = CacheStats::new();
        stats.hits = 8;
        stats.misses = 2;
        
        assert_eq!(stats.total_accesses(), 10);
        assert_eq!(stats.hit_rate(), 0.8);
    }

    #[test]
    fn test_cache_stats_default() {
        let stats = CacheStats::default();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
    }
}
