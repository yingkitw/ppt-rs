//! Integration tests for Open-XML-SDK architecture adoption
//!
//! Tests that all architectural patterns work together correctly

use ppt_rs::util::{
    ValidationError, ValidationErrorType, SchemaValidator, SemanticValidator,
    DocumentValidator, PackageValidator, Feature, FeatureCollection,
    LazyValue, LazyCollection, LruCache,
};
use ppt_rs::opc::PartContainer;
use ppt_rs::oxml::{StreamingXmlReader, XmlEvent};
use std::io::Cursor;
use std::sync::Arc;
use std::sync::Mutex;

// Test feature for feature collection
struct TestFeature {
    value: i32,
}

impl Feature for TestFeature {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[test]
fn test_validation_framework_integration() {
    // Schema validation
    let schema_errors = SchemaValidator::validate_element_structure(
        "presentation",
        &["id", "name"],
        &["id"],
    );
    assert_eq!(schema_errors.len(), 1);
    assert_eq!(schema_errors[0].error_type, ValidationErrorType::Error);

    // Semantic validation
    let semantic_error = SemanticValidator::validate_range(150, 0, 100, "opacity");
    assert!(semantic_error.is_some());

    // Document validation
    let doc_error = DocumentValidator::validate_slide_count(0);
    assert!(doc_error.is_some());

    // Package validation
    let pkg_errors = PackageValidator::validate_required_parts(
        &["presentation.xml", "theme.xml"],
        &["presentation.xml"],
    );
    assert_eq!(pkg_errors.len(), 1);
}

#[test]
fn test_feature_collection_integration() {
    let mut features = FeatureCollection::new();
    
    // Add multiple features
    features.set(TestFeature { value: 42 });
    
    // Retrieve and verify
    let feature = features.get::<TestFeature>().unwrap();
    assert_eq!(feature.value, 42);
    
    // Verify has
    assert!(features.has::<TestFeature>());
    
    // Remove
    features.remove::<TestFeature>();
    assert!(!features.has::<TestFeature>());
}

#[test]
fn test_lazy_loading_integration() {
    let call_count = Arc::new(Mutex::new(0));
    let call_count_clone = Arc::clone(&call_count);
    
    let lazy = LazyValue::new(move || {
        let mut count = call_count_clone.lock().unwrap();
        *count += 1;
        Ok(vec![1, 2, 3, 4, 5])
    });
    
    // First access
    let _v1 = lazy.get().unwrap();
    assert_eq!(*call_count.lock().unwrap(), 1);
    
    // Second access (cached)
    let _v2 = lazy.get().unwrap();
    assert_eq!(*call_count.lock().unwrap(), 1);
    
    // Reset and access again
    lazy.reset();
    let _v3 = lazy.get().unwrap();
    assert_eq!(*call_count.lock().unwrap(), 2);
}

#[test]
fn test_lazy_collection_integration() {
    let lazy = LazyCollection::new(|| Ok(vec![1, 2, 3, 4, 5]));
    
    assert!(!lazy.is_loaded());
    
    let items = lazy.get_all().unwrap();
    assert_eq!(items.len(), 5);
    assert!(lazy.is_loaded());
    
    let len = lazy.len().unwrap();
    assert_eq!(len, 5);
}

#[test]
fn test_lru_cache_integration() {
    let mut cache = LruCache::new(3);
    
    // Insert items
    cache.insert("a", 1);
    cache.insert("b", 2);
    cache.insert("c", 3);
    assert_eq!(cache.len(), 3);
    
    // Access to update LRU
    let _ = cache.get(&"a");
    
    // Insert new item (should evict "b")
    cache.insert("d", 4);
    assert_eq!(cache.len(), 3);
    assert_eq!(cache.get(&"a"), Some(1));
    assert_eq!(cache.get(&"b"), None);
    assert_eq!(cache.get(&"c"), Some(3));
    assert_eq!(cache.get(&"d"), Some(4));
}

#[test]
fn test_part_container_integration() {
    let mut container = PartContainer::new();
    
    // Create relationships
    let r_id1 = container.create_relationship(
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide".to_string(),
        "/ppt/slides/slide1.xml".to_string(),
        false,
    );
    
    let r_id2 = container.create_relationship(
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide".to_string(),
        "/ppt/slides/slide2.xml".to_string(),
        false,
    );
    
    // Verify relationships
    assert!(container.get_relationship(&r_id1).is_some());
    assert!(container.get_relationship(&r_id2).is_some());
    
    // Remove relationship
    container.remove_relationship(&r_id1);
    assert!(container.get_relationship(&r_id1).is_none());
}

#[test]
fn test_streaming_xml_integration() {
    let reader = StreamingXmlReader::new();
    let xml = b"<?xml version=\"1.0\"?><root><item>test</item></root>";
    let cursor = Cursor::new(xml);
    
    let mut events = Vec::new();
    reader.read(cursor, |event| {
        events.push(event);
        Ok(())
    }).unwrap();
    
    assert!(!events.is_empty());
}

#[test]
fn test_combined_validation_and_features() {
    // Create validation error
    let error = ValidationError::new(
        ValidationErrorType::Error,
        "Test error".to_string(),
        "/test/path".to_string(),
    );
    
    // Store in feature collection
    struct ValidationErrorFeature {
        errors: Vec<ValidationError>,
    }
    
    impl Feature for ValidationErrorFeature {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    }
    
    let mut features = FeatureCollection::new();
    features.set(ValidationErrorFeature {
        errors: vec![error],
    });
    
    let feature = features.get::<ValidationErrorFeature>().unwrap();
    assert_eq!(feature.errors.len(), 1);
}

#[test]
fn test_lazy_loading_with_caching() {
    let call_count = Arc::new(Mutex::new(0));
    let call_count_clone = Arc::clone(&call_count);
    
    let lazy = LazyCollection::new(move || {
        let mut count = call_count_clone.lock().unwrap();
        *count += 1;
        Ok(vec![1, 2, 3, 4, 5])
    });
    
    // Load collection
    let items1 = lazy.get_all().unwrap();
    assert_eq!(items1.len(), 5);
    
    // Get again (cached)
    let items2 = lazy.get_all().unwrap();
    assert_eq!(items2.len(), 5);
    
    // Verify loader called only once
    assert_eq!(*call_count.lock().unwrap(), 1);
}

#[test]
fn test_cache_with_many_items() {
    let mut cache = LruCache::new(10);
    
    // Insert 20 items
    for i in 0..20 {
        cache.insert(i, i * 2);
    }
    
    // Cache should only have 10 items
    assert_eq!(cache.len(), 10);
    
    // First 10 items should be evicted
    assert_eq!(cache.get(&0), None);
    assert_eq!(cache.get(&9), None);
    
    // Last 10 items should be present
    assert_eq!(cache.get(&10), Some(20));
    assert_eq!(cache.get(&19), Some(38));
}

#[test]
fn test_validation_error_display() {
    let error = ValidationError::new(
        ValidationErrorType::Warning,
        "Test warning".to_string(),
        "/test/path".to_string(),
    )
    .with_part_uri("/ppt/presentation.xml".to_string())
    .with_element_id("slide1".to_string());
    
    let display = format!("{}", error);
    assert!(display.contains("Warning"));
    assert!(display.contains("Test warning"));
    assert!(display.contains("/ppt/presentation.xml"));
}

#[test]
fn test_multiple_validators_together() {
    let mut errors = Vec::new();
    
    // Schema errors
    errors.extend(SchemaValidator::validate_element_structure(
        "slide",
        &["id", "name"],
        &["id"],
    ));
    
    // Semantic errors
    if let Some(error) = SemanticValidator::validate_range(150, 0, 100, "opacity") {
        errors.push(error);
    }
    
    // Document errors
    if let Some(error) = DocumentValidator::validate_slide_count(0) {
        errors.push(error);
    }
    
    // Should have at least 3 errors
    assert!(errors.len() >= 3);
}

#[test]
fn test_feature_collection_multiple_types() {
    struct Feature1 { value: i32 }
    impl Feature for Feature1 {
        fn as_any(&self) -> &dyn std::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    }
    
    struct Feature2 { text: String }
    impl Feature for Feature2 {
        fn as_any(&self) -> &dyn std::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    }
    
    let mut features = FeatureCollection::new();
    features.set(Feature1 { value: 42 });
    features.set(Feature2 { text: "test".to_string() });
    
    assert!(features.has::<Feature1>());
    assert!(features.has::<Feature2>());
    assert_eq!(features.len(), 2);
}
