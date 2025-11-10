# Open-XML-SDK Implementation Guide for ppt-rs

## Quick Reference: Key Patterns to Adopt

### 1. Element Hierarchy (IMMEDIATE)

**Current ppt-rs:**
```rust
pub trait Shape { ... }
pub struct AutoShape { ... }
pub struct Picture { ... }
```

**Recommended (Open-XML-SDK style):**
```rust
// Base trait for all elements
pub trait OpenXmlElement: Clone + Debug + Send + Sync {
    fn name(&self) -> &str;
    fn namespace(&self) -> &str;
    fn to_xml(&self) -> Result<String>;
    fn from_xml(xml: &str) -> Result<Self>;
    fn parent(&self) -> Option<&dyn OpenXmlElement>;
    fn set_parent(&mut self, parent: Option<Box<dyn OpenXmlElement>>);
}

// Composite elements (can have children)
pub trait CompositeElement: OpenXmlElement {
    fn children(&self) -> &[Box<dyn OpenXmlElement>];
    fn children_mut(&mut self) -> &mut Vec<Box<dyn OpenXmlElement>>;
    fn append(&mut self, child: Box<dyn OpenXmlElement>) -> Result<()>;
    fn insert(&mut self, index: usize, child: Box<dyn OpenXmlElement>) -> Result<()>;
    fn remove(&mut self, index: usize) -> Result<()>;
}

// Leaf elements (no children)
pub trait LeafElement: OpenXmlElement {
    fn text(&self) -> &str;
    fn set_text(&mut self, text: String) -> Result<()>;
}

// Implementation
pub struct Slide {
    name: String,
    namespace: String,
    children: Vec<Box<dyn OpenXmlElement>>,
    parent: Option<Box<dyn OpenXmlElement>>,
}

impl OpenXmlElement for Slide {
    fn name(&self) -> &str { &self.name }
    fn namespace(&self) -> &str { &self.namespace }
    fn to_xml(&self) -> Result<String> { /* ... */ }
    fn from_xml(xml: &str) -> Result<Self> { /* ... */ }
    fn parent(&self) -> Option<&dyn OpenXmlElement> {
        self.parent.as_ref().map(|p| p.as_ref())
    }
    fn set_parent(&mut self, parent: Option<Box<dyn OpenXmlElement>>) {
        self.parent = parent;
    }
}

impl CompositeElement for Slide {
    fn children(&self) -> &[Box<dyn OpenXmlElement>] { &self.children }
    fn children_mut(&mut self) -> &mut Vec<Box<dyn OpenXmlElement>> { &mut self.children }
    fn append(&mut self, child: Box<dyn OpenXmlElement>) -> Result<()> {
        self.children.push(child);
        Ok(())
    }
    fn insert(&mut self, index: usize, child: Box<dyn OpenXmlElement>) -> Result<()> {
        self.children.insert(index, child);
        Ok(())
    }
    fn remove(&mut self, index: usize) -> Result<()> {
        self.children.remove(index);
        Ok(())
    }
}
```

**Benefits:**
- ✅ Unified element handling
- ✅ Type-safe operations
- ✅ Extensible design
- ✅ Easy to test

---

### 2. Relationship Management (HIGH PRIORITY)

**Current ppt-rs:**
```rust
pub struct Relationships {
    relationships: Vec<Relationship>,
}
```

**Recommended (Open-XML-SDK style):**
```rust
// Relationship as first-class object
#[derive(Clone, Debug)]
pub struct Relationship {
    pub id: String,              // rId1, rId2, etc.
    pub rel_type: String,        // Full URI of relationship type
    pub target: String,          // Target URI
    pub is_external: bool,       // Internal vs external
    pub target_mode: TargetMode, // Internal, External, etc.
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum TargetMode {
    Internal,
    External,
}

// Relationship collection with query methods
pub struct RelationshipCollection {
    relationships: Vec<Relationship>,
    next_id: usize,
}

impl RelationshipCollection {
    pub fn new() -> Self {
        Self {
            relationships: Vec::new(),
            next_id: 1,
        }
    }
    
    /// Create a new relationship
    pub fn create(
        &mut self,
        rel_type: String,
        target: String,
        external: bool,
    ) -> String {
        let id = format!("rId{}", self.next_id);
        self.next_id += 1;
        
        self.relationships.push(Relationship {
            id: id.clone(),
            rel_type,
            target,
            is_external: external,
            target_mode: if external {
                TargetMode::External
            } else {
                TargetMode::Internal
            },
        });
        
        id
    }
    
    /// Get relationship by ID
    pub fn get_by_id(&self, id: &str) -> Option<&Relationship> {
        self.relationships.iter().find(|r| r.id == id)
    }
    
    /// Get all relationships of a type
    pub fn get_by_type(&self, rel_type: &str) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| r.rel_type == rel_type)
            .collect()
    }
    
    /// Delete relationship by ID
    pub fn delete(&mut self, id: &str) -> Result<()> {
        if let Some(pos) = self.relationships.iter().position(|r| r.id == id) {
            self.relationships.remove(pos);
            Ok(())
        } else {
            Err(PptError::PartNotFound(format!("Relationship {} not found", id)))
        }
    }
    
    /// Iterate all relationships
    pub fn iter(&self) -> impl Iterator<Item = &Relationship> {
        self.relationships.iter()
    }
    
    /// Generate XML
    pub fn to_xml(&self) -> String {
        let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
        xml.push_str(r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">"#);
        
        for rel in &self.relationships {
            xml.push_str(&format!(
                r#"<Relationship Id="{}" Type="{}" Target="{}""#,
                rel.id, rel.rel_type, rel.target
            ));
            
            if rel.is_external {
                xml.push_str(r#" TargetMode="External""#);
            }
            
            xml.push_str("/>");
        }
        
        xml.push_str("</Relationships>");
        xml
    }
}

// Usage
let mut rels = RelationshipCollection::new();
let slide_id = rels.create(
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide".to_string(),
    "slides/slide1.xml".to_string(),
    false,
);

let slide_rels = rels.get_by_type("http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide");
```

**Benefits:**
- ✅ Type-safe relationship handling
- ✅ Query by type
- ✅ Proper ID generation
- ✅ External/internal distinction

---

### 3. Validation Framework (HIGH PRIORITY)

**Recommended:**
```rust
// Validation error with context
#[derive(Clone, Debug)]
pub struct ValidationError {
    pub error_type: ValidationErrorType,
    pub description: String,
    pub path: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub part_uri: Option<String>,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ValidationErrorType {
    Error,
    Warning,
    Notice,
}

// Validator trait
pub trait Validator {
    fn validate(&self) -> Result<Vec<ValidationError>>;
}

// Schema validator
pub struct SchemaValidator;

impl SchemaValidator {
    pub fn validate(element: &dyn OpenXmlElement) -> Result<Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        // Check required attributes
        // Check element structure
        // Check child elements
        
        Ok(errors)
    }
}

// Semantic validator
pub struct SemanticValidator;

impl SemanticValidator {
    pub fn validate(element: &dyn OpenXmlElement) -> Result<Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        // Check business rules
        // Check relationships
        // Check constraints
        
        Ok(errors)
    }
}

// Document validator
pub struct DocumentValidator;

impl DocumentValidator {
    pub fn validate(part: &dyn Part) -> Result<Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        // Validate all parts
        // Check relationships
        // Check content types
        
        Ok(errors)
    }
}

// Usage
let errors = SchemaValidator::validate(&slide)?;
if !errors.is_empty() {
    for error in errors {
        eprintln!("{:?}: {} at {}", error.error_type, error.description, error.path);
    }
}
```

**Benefits:**
- ✅ Multi-level validation
- ✅ Rich error information
- ✅ Extensible validation
- ✅ Clear error reporting

---

### 4. Part Container Pattern (MEDIUM PRIORITY)

**Recommended:**
```rust
// Generic part container
pub struct PartContainer {
    parts: HashMap<String, Box<dyn Part>>,
    relationships: RelationshipCollection,
}

impl PartContainer {
    pub fn new() -> Self {
        Self {
            parts: HashMap::new(),
            relationships: RelationshipCollection::new(),
        }
    }
    
    /// Create a new part
    pub fn create_part<T: Part + 'static>(
        &mut self,
        uri: String,
        rel_type: String,
    ) -> Result<String> {
        let part = T::new(uri.clone())?;
        self.parts.insert(uri.clone(), Box::new(part));
        
        let rel_id = self.relationships.create(rel_type, uri, false);
        Ok(rel_id)
    }
    
    /// Get part by URI
    pub fn get_part(&self, uri: &str) -> Option<&dyn Part> {
        self.parts.get(uri).map(|p| p.as_ref())
    }
    
    /// Get parts by relationship type
    pub fn get_parts_by_type(&self, rel_type: &str) -> Vec<&dyn Part> {
        self.relationships
            .get_by_type(rel_type)
            .iter()
            .filter_map(|rel| self.parts.get(&rel.target).map(|p| p.as_ref()))
            .collect()
    }
    
    /// Delete part
    pub fn delete_part(&mut self, uri: &str) -> Result<()> {
        self.parts.remove(uri);
        Ok(())
    }
    
    /// Get relationships
    pub fn relationships(&self) -> &RelationshipCollection {
        &self.relationships
    }
}

// Usage
let mut container = PartContainer::new();
let slide_id = container.create_part::<SlidePart>(
    "/ppt/slides/slide1.xml".to_string(),
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide".to_string(),
)?;

let slides = container.get_parts_by_type("http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide");
```

**Benefits:**
- ✅ Generic part management
- ✅ Type-safe operations
- ✅ Relationship tracking
- ✅ Easy to extend

---

### 5. Feature Collection (MEDIUM PRIORITY)

**Recommended:**
```rust
use std::any::{Any, TypeId};
use std::collections::HashMap;

// Feature trait
pub trait Feature: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

// Feature collection
pub struct FeatureCollection {
    features: HashMap<TypeId, Box<dyn Feature>>,
}

impl FeatureCollection {
    pub fn new() -> Self {
        Self {
            features: HashMap::new(),
        }
    }
    
    /// Get feature by type
    pub fn get<T: Feature + 'static>(&self) -> Option<&T> {
        self.features
            .get(&TypeId::of::<T>())
            .and_then(|f| f.as_any().downcast_ref::<T>())
    }
    
    /// Set feature
    pub fn set<T: Feature + 'static>(&mut self, feature: T) {
        self.features.insert(TypeId::of::<T>(), Box::new(feature));
    }
    
    /// Has feature
    pub fn has<T: Feature + 'static>(&self) -> bool {
        self.features.contains_key(&TypeId::of::<T>())
    }
}

// Example features
pub struct ValidationFeature {
    pub enabled: bool,
}

impl Feature for ValidationFeature {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct FormattingFeature {
    pub indent: usize,
}

impl Feature for FormattingFeature {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Usage
let mut features = FeatureCollection::new();
features.set(ValidationFeature { enabled: true });
features.set(FormattingFeature { indent: 2 });

if let Some(validation) = features.get::<ValidationFeature>() {
    if validation.enabled {
        println!("Validation is enabled");
    }
}
```

**Benefits:**
- ✅ Extensible functionality
- ✅ Plugin architecture
- ✅ No tight coupling
- ✅ Type-safe access

---

## Implementation Priority

### Week 1-2: Foundation
- [ ] Implement element hierarchy
- [ ] Formalize relationship management
- [ ] Create validation framework

### Week 3-4: Enhancement
- [ ] Add feature collection
- [ ] Implement part container
- [ ] Add streaming XML support

### Week 5-6: Polish
- [ ] Improve error handling
- [ ] Add builder patterns
- [ ] Implement caching

### Week 7-8: Testing
- [ ] Comprehensive testing
- [ ] Performance benchmarking
- [ ] Documentation updates

---

## Quick Wins (Can do immediately)

1. **Enhance Relationship Management** (2-3 hours)
   - Add query methods to RelationshipCollection
   - Add relationship type constants
   - Add tests

2. **Add Validation Framework** (4-5 hours)
   - Create ValidationError struct
   - Implement SchemaValidator
   - Add validation tests

3. **Create PartContainer** (3-4 hours)
   - Generic part management
   - Relationship tracking
   - Add tests

---

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_relationship_creation() {
        let mut rels = RelationshipCollection::new();
        let id = rels.create(
            "http://example.com/rel".to_string(),
            "target.xml".to_string(),
            false,
        );
        assert_eq!(id, "rId1");
    }
    
    #[test]
    fn test_relationship_query() {
        let mut rels = RelationshipCollection::new();
        rels.create("http://example.com/rel1".to_string(), "target1.xml".to_string(), false);
        rels.create("http://example.com/rel1".to_string(), "target2.xml".to_string(), false);
        rels.create("http://example.com/rel2".to_string(), "target3.xml".to_string(), false);
        
        let rel1_rels = rels.get_by_type("http://example.com/rel1");
        assert_eq!(rel1_rels.len(), 2);
    }
    
    #[test]
    fn test_feature_collection() {
        let mut features = FeatureCollection::new();
        features.set(ValidationFeature { enabled: true });
        
        assert!(features.has::<ValidationFeature>());
        let validation = features.get::<ValidationFeature>().unwrap();
        assert!(validation.enabled);
    }
}
```

---

## Summary

By adopting these Open-XML-SDK patterns, ppt-rs will gain:

1. **Better Architecture** - Clear separation of concerns
2. **Type Safety** - Compile-time guarantees
3. **Extensibility** - Plugin-like architecture
4. **Performance** - Lazy loading and caching
5. **Maintainability** - Clear patterns and conventions
6. **Testability** - Easier to test
7. **Scalability** - Handles large documents
8. **Compatibility** - Better round-trip support

