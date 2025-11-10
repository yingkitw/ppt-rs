# OOXML Integration Plan for PPT-RS

**Date**: November 10, 2025  
**Status**: Ready for Implementation  
**Priority**: HIGH  

---

## Executive Summary

Integrate key capabilities from ooxml-rs into ppt-rs to:
1. Improve code organization and maintainability
2. Prepare foundation for DOCX/XLSX support
3. Better XML compliance and error handling
4. Reduce boilerplate code

---

## Quick Wins (Can be done immediately)

### 1. Add thiserror Crate ⭐ HIGHEST PRIORITY
**Effort**: 30 minutes  
**Impact**: Better error messages  
**Risk**: Very Low  

**Changes**:
```toml
# Cargo.toml
thiserror = "1"
```

**Benefits**:
- Cleaner error definitions
- Better error messages
- Easier to extend
- Standard Rust practice

**Example**:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PptError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("XML error: {0}")]
    Xml(String),
    
    #[error("Invalid part: {0}")]
    InvalidPart(String),
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),
}
```

### 2. Use quick-xml with serde ⭐ HIGH PRIORITY
**Effort**: 2-3 hours  
**Impact**: Cleaner XML handling  
**Risk**: Low  

**Changes**:
```toml
# Cargo.toml
quick-xml = { version = "0.22.0", features = ["serialize"] }
```

**Benefits**:
- Faster XML parsing
- Serde integration
- Less boilerplate
- Better performance

**Example**:
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "p:sld")]
pub struct Slide {
    #[serde(rename = "@xmlns:p")]
    pub xmlns_p: String,
    
    #[serde(rename = "p:cSld")]
    pub common_slide_data: CommonSlideData,
}
```

### 3. Add Namespace Management ⭐ HIGH PRIORITY
**Effort**: 1-2 hours  
**Impact**: Better XML compliance  
**Risk**: Low  

**New File**: `/src/opc/namespace.rs`

```rust
use std::collections::HashMap;

pub struct Namespaces {
    namespaces: HashMap<String, String>,
}

impl Namespaces {
    pub fn new() -> Self {
        Self {
            namespaces: HashMap::new(),
        }
    }
    
    pub fn add(&mut self, prefix: String, uri: String) {
        self.namespaces.insert(prefix, uri);
    }
    
    pub fn get(&self, prefix: &str) -> Option<&str> {
        self.namespaces.get(prefix).map(|s| s.as_str())
    }
    
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.namespaces.iter()
    }
}

// Standard OOXML namespaces
pub mod standard {
    pub const PRESENTATION_ML: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
    pub const DRAWING_ML: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
    pub const OFFICE_DOCUMENT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    pub const PACKAGE_RELATIONSHIPS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
    pub const CONTENT_TYPES: &str = "http://schemas.openxmlformats.org/package/2006/content-types";
}
```

### 4. Enhance Properties Management ⭐ MEDIUM PRIORITY
**Effort**: 1-2 hours  
**Impact**: Better metadata support  
**Risk**: Low  

**New File**: `/src/opc/properties_enhanced.rs`

```rust
use std::collections::HashMap;

pub struct CoreProperties {
    pub title: Option<String>,
    pub subject: Option<String>,
    pub creator: Option<String>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub last_modified_by: Option<String>,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct AppProperties {
    pub application: Option<String>,
    pub app_version: Option<String>,
    pub total_time: Option<i32>,
    pub slides: Option<i32>,
    pub notes: Option<i32>,
}

pub struct CustomProperties {
    properties: HashMap<String, String>,
}

impl CustomProperties {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }
    
    pub fn get(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }
}
```

---

## Phase 1: Foundation (Week 1)

### Step 1.1: Add Dependencies
**Time**: 15 minutes  
**Files**: `Cargo.toml`

```toml
[dependencies]
thiserror = "1"
quick-xml = { version = "0.22.0", features = ["serialize"] }
chrono = { version = "0.4", features = ["serde"] }
linked-hash-map = { version = "0.5", features = ["serde_impl"] }
```

### Step 1.2: Implement Namespace System
**Time**: 1-2 hours  
**Files**: `/src/opc/namespace.rs`, `/src/opc/mod.rs`

```rust
// /src/opc/namespace.rs
pub struct Namespaces { ... }
pub mod standard { ... }

// /src/opc/mod.rs
pub mod namespace;
pub use namespace::Namespaces;
```

### Step 1.3: Implement XML Traits
**Time**: 2-3 hours  
**Files**: `/src/oxml/traits.rs`, `/src/oxml/mod.rs`

```rust
// /src/oxml/traits.rs
pub trait OpenXmlElementInfo {
    fn tag_name() -> &'static str;
    fn element_type() -> OpenXmlElementType;
}

pub trait OpenXmlSerialize {
    fn to_xml(&self) -> Result<String>;
}

pub trait OpenXmlDeserialize: Sized {
    fn from_xml(xml: &str) -> Result<Self>;
}
```

### Step 1.4: Enhance Error Handling
**Time**: 1-2 hours  
**Files**: `/src/error.rs`

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PptError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("XML error: {0}")]
    Xml(String),
    
    #[error("Invalid part: {0}")]
    InvalidPart(String),
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),
}
```

---

## Phase 2: OPC Enhancement (Week 2)

### Step 2.1: Make Package Generic
**Time**: 2-3 hours  
**Files**: `/src/opc/package_generic.rs`

```rust
pub trait OpenXmlDocument {
    fn package(&self) -> &Package;
    fn package_mut(&mut self) -> &mut Package;
    fn format_type(&self) -> DocumentFormat;
}

pub enum DocumentFormat {
    Presentation,
    Spreadsheet,
    Document,
}

impl OpenXmlDocument for Presentation { ... }
```

### Step 2.2: Enhance Properties
**Time**: 1-2 hours  
**Files**: `/src/opc/properties_enhanced.rs`

```rust
pub struct CoreProperties { ... }
pub struct AppProperties { ... }
pub struct CustomProperties { ... }
```

### Step 2.3: Use LinkedHashMap
**Time**: 1 hour  
**Files**: `/src/opc/package.rs`

```rust
use linked_hash_map::LinkedHashMap;

pub struct Package {
    parts: LinkedHashMap<PackURI, Box<dyn Part>>,
    // ... other fields
}
```

---

## Phase 3: XML Serialization (Week 3)

### Step 3.1: Migrate Shapes to Serde
**Time**: 4-6 hours  
**Files**: `/src/shapes/shape.rs`, etc.

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename = "p:sp")]
pub struct Shape {
    #[serde(rename = "@id")]
    pub id: u32,
    
    #[serde(rename = "p:nvSpPr")]
    pub non_visual_props: NonVisualShapeProps,
}
```

### Step 3.2: Migrate Charts to Serde
**Time**: 4-6 hours  
**Files**: `/src/charts/chart.rs`, etc.

### Step 3.3: Migrate Text to Serde
**Time**: 2-3 hours  
**Files**: `/src/text/layout.rs`, etc.

### Step 3.4: Migrate Slides to Serde
**Time**: 2-3 hours  
**Files**: `/src/parts/slide.rs`, etc.

---

## Phase 4: Testing & Validation (Week 4)

### Step 4.1: Update Tests
**Time**: 2-3 hours  
**Files**: All test files

- Update tests for new error handling
- Update tests for serde integration
- Add namespace tests
- Add properties tests

### Step 4.2: Validate PPTX Compatibility
**Time**: 2-3 hours  
**Files**: Integration tests

- Test with python-pptx
- Test with PowerPoint
- Test round-trip (save/load)
- Test with various file sizes

### Step 4.3: Performance Testing
**Time**: 1-2 hours  
**Files**: Benchmark tests

- Compare performance before/after
- Measure XML parsing speed
- Measure serialization speed
- Measure memory usage

---

## Implementation Checklist

### Quick Wins (Do First)
- [ ] Add thiserror crate
- [ ] Add quick-xml crate
- [ ] Add namespace management
- [ ] Enhance properties

### Phase 1: Foundation
- [ ] Add dependencies
- [ ] Implement namespace system
- [ ] Implement XML traits
- [ ] Enhance error handling
- [ ] Update tests

### Phase 2: OPC Enhancement
- [ ] Make package generic
- [ ] Enhance properties
- [ ] Use LinkedHashMap
- [ ] Update tests

### Phase 3: XML Serialization
- [ ] Migrate shapes to serde
- [ ] Migrate charts to serde
- [ ] Migrate text to serde
- [ ] Migrate slides to serde
- [ ] Update tests

### Phase 4: Testing & Validation
- [ ] Update all tests
- [ ] Validate PPTX compatibility
- [ ] Performance testing
- [ ] Documentation

---

## Expected Outcomes

### Code Quality
- ✅ Cleaner error handling
- ✅ Less boilerplate
- ✅ Better XML compliance
- ✅ Improved maintainability

### Performance
- ✅ Faster XML parsing (quick-xml)
- ✅ Better memory usage (LinkedHashMap)
- ✅ Deterministic output

### Extensibility
- ✅ Foundation for DOCX support
- ✅ Foundation for XLSX support
- ✅ Generic OPC implementation
- ✅ Reusable XML traits

### Compatibility
- ✅ Better PowerPoint compatibility
- ✅ Better python-pptx compatibility
- ✅ Better Office compliance

---

## Risk Assessment

### Low Risk
- ✅ Adding thiserror (isolated change)
- ✅ Adding namespace system (new module)
- ✅ Adding XML traits (new module)

### Medium Risk
- ⚠️ Using quick-xml (requires testing)
- ⚠️ Using LinkedHashMap (order matters)
- ⚠️ Migrating to serde (widespread changes)

### Mitigation
- Keep old code as fallback
- Extensive testing at each phase
- Incremental rollout
- Version bump for major changes

---

## Timeline

| Phase | Duration | Start | End |
|-------|----------|-------|-----|
| Quick Wins | 1 day | Week 1 Mon | Week 1 Mon |
| Phase 1 | 3 days | Week 1 Tue | Week 1 Thu |
| Phase 2 | 2 days | Week 2 Mon | Week 2 Tue |
| Phase 3 | 3 days | Week 2 Wed | Week 3 Fri |
| Phase 4 | 2 days | Week 4 Mon | Week 4 Tue |
| **Total** | **~2 weeks** | | |

---

## Success Criteria

✅ **Code Quality**
- All tests passing (359+)
- Zero compilation errors
- No warnings (except allowed)
- Code coverage maintained

✅ **Compatibility**
- PPTX files open in PowerPoint
- PPTX files open in python-pptx
- Round-trip save/load works
- All features preserved

✅ **Performance**
- No performance regression
- XML parsing faster (quick-xml)
- Memory usage optimized
- File size unchanged

✅ **Documentation**
- Updated README
- Updated API docs
- Updated examples
- Migration guide

---

## Conclusion

This integration plan brings best practices from ooxml-rs to ppt-rs while maintaining backward compatibility and production readiness. The phased approach allows for incremental improvements with minimal risk.

**Next Step**: Start with Quick Wins (thiserror, quick-xml)

---

**Status**: ✅ **READY FOR IMPLEMENTATION**  
**Estimated Effort**: 2 weeks  
**Expected ROI**: High (foundation for future formats)  

