//! Extensible feature collection system inspired by Open-XML-SDK
//!
//! Provides a type-based plugin architecture for adding functionality
//! without tight coupling to core classes.

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Marker trait for features
pub trait Feature: Send + Sync {
    /// Convert to Any for downcasting
    fn as_any(&self) -> &dyn Any;
    
    /// Convert to mutable Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Type-based feature collection
/// 
/// Allows storing and retrieving features by type without tight coupling.
/// Features are stored in a HashMap keyed by TypeId.
#[derive(Default)]
pub struct FeatureCollection {
    features: HashMap<TypeId, Box<dyn Feature>>,
}

impl FeatureCollection {
    /// Create a new empty feature collection
    pub fn new() -> Self {
        Self {
            features: HashMap::new(),
        }
    }

    /// Get a feature by type
    pub fn get<T: Feature + 'static>(&self) -> Option<&T> {
        self.features
            .get(&TypeId::of::<T>())
            .and_then(|f| f.as_any().downcast_ref::<T>())
    }

    /// Get a mutable reference to a feature by type
    pub fn get_mut<T: Feature + 'static>(&mut self) -> Option<&mut T> {
        self.features
            .get_mut(&TypeId::of::<T>())
            .and_then(|f| f.as_any_mut().downcast_mut::<T>())
    }

    /// Set a feature
    pub fn set<T: Feature + 'static>(&mut self, feature: T) {
        self.features.insert(TypeId::of::<T>(), Box::new(feature));
    }

    /// Check if a feature exists
    pub fn has<T: Feature + 'static>(&self) -> bool {
        self.features.contains_key(&TypeId::of::<T>())
    }

    /// Remove a feature
    pub fn remove<T: Feature + 'static>(&mut self) -> Option<Box<dyn Feature>> {
        self.features.remove(&TypeId::of::<T>())
    }

    /// Clear all features
    pub fn clear(&mut self) {
        self.features.clear();
    }

    /// Get the number of features
    pub fn len(&self) -> usize {
        self.features.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }
}

impl Clone for FeatureCollection {
    fn clone(&self) -> Self {
        // Note: Features are not cloned as they may contain non-cloneable data
        Self {
            features: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test feature implementations
    struct TestFeature1 {
        value: i32,
    }

    impl Feature for TestFeature1 {
        fn as_any(&self) -> &dyn Any {
            self
        }
        
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    struct TestFeature2 {
        name: String,
    }

    impl Feature for TestFeature2 {
        fn as_any(&self) -> &dyn Any {
            self
        }
        
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn test_feature_collection_new() {
        let collection = FeatureCollection::new();
        assert!(collection.is_empty());
        assert_eq!(collection.len(), 0);
    }

    #[test]
    fn test_feature_collection_set_and_get() {
        let mut collection = FeatureCollection::new();
        let feature = TestFeature1 { value: 42 };
        
        collection.set(feature);
        assert!(!collection.is_empty());
        assert_eq!(collection.len(), 1);
        
        let retrieved = collection.get::<TestFeature1>().unwrap();
        assert_eq!(retrieved.value, 42);
    }

    #[test]
    fn test_feature_collection_multiple_features() {
        let mut collection = FeatureCollection::new();
        
        collection.set(TestFeature1 { value: 42 });
        collection.set(TestFeature2 { name: "test".to_string() });
        
        assert_eq!(collection.len(), 2);
        assert!(collection.has::<TestFeature1>());
        assert!(collection.has::<TestFeature2>());
    }

    #[test]
    fn test_feature_collection_get_mut() {
        let mut collection = FeatureCollection::new();
        collection.set(TestFeature1 { value: 42 });
        
        if let Some(feature) = collection.get_mut::<TestFeature1>() {
            feature.value = 100;
        }
        
        let retrieved = collection.get::<TestFeature1>().unwrap();
        assert_eq!(retrieved.value, 100);
    }

    #[test]
    fn test_feature_collection_remove() {
        let mut collection = FeatureCollection::new();
        collection.set(TestFeature1 { value: 42 });
        
        assert!(collection.has::<TestFeature1>());
        collection.remove::<TestFeature1>();
        assert!(!collection.has::<TestFeature1>());
        assert!(collection.is_empty());
    }

    #[test]
    fn test_feature_collection_clear() {
        let mut collection = FeatureCollection::new();
        collection.set(TestFeature1 { value: 42 });
        collection.set(TestFeature2 { name: "test".to_string() });
        
        assert_eq!(collection.len(), 2);
        collection.clear();
        assert!(collection.is_empty());
    }

    #[test]
    fn test_feature_collection_get_nonexistent() {
        let collection = FeatureCollection::new();
        let result = collection.get::<TestFeature1>();
        assert!(result.is_none());
    }

    #[test]
    fn test_feature_collection_clone() {
        let mut collection = FeatureCollection::new();
        collection.set(TestFeature1 { value: 42 });
        
        let cloned = collection.clone();
        assert!(cloned.is_empty()); // Features not cloned
    }
}
