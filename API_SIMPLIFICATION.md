# API Simplification Proposal

## Current State Analysis

### What's Working Well ✅
- **ImageBuilder**: Simplified with `auto()`, chainable effects, `at()`, `size()`
- **Shape helper**: Local `shape()` function using `inches()`
- **Prelude module**: Provides `inches()`, `colors`, `themes`, `font_sizes`, `Dimension`
- **Fluent API**: Method chaining for builders

### Current Pain Points ⚠️

1. **Inconsistent Construction Patterns**
   - Images: `ImageBuilder::auto(bytes)` ✅
   - Shapes: `Shape::new(type, x, y, w, h)` or `Shape::from_dimensions()` ❌
   - Tables: `TableBuilder::new()` then manual setup ❌
   - Charts: `ChartBuilder::new()` then manual setup ❌

2. **Mixed Unit Systems**
   - Some APIs use raw EMU values
   - Some use `Dimension` enum
   - Helper function `inches()` only in prelude
   - No consistent pattern

3. **Verbose Color Specification**
   - `ShapeFill::new("4F81BD")` - requires knowing it's a hex string
   - No color constants or helpers
   - Inconsistent with `colors::*` in prelude

4. **Builder Inconsistency**
   - Some builders have `build()` method
   - Some don't require `build()`
   - Unclear when to use which pattern

5. **No Global Helper Functions**
   - `shape()` helper only exists in examples
   - No `image()`, `table()`, `chart()` helpers
   - Users must know full builder API

## Proposed Simplification Strategy

### Design Principles

1. **Consistency First**: Same patterns across all types
2. **Sensible Defaults**: Minimize required parameters
3. **Readable Units**: Always use real-world units (inches, cm, pt)
4. **Chainable Everything**: Fluent API throughout
5. **Helper Functions**: Global helpers for common operations
6. **Type Safety**: Maintain Rust's compile-time guarantees

### Proposed API Design

#### 1. Unified Construction Pattern

**Before:**
```rust
// Inconsistent construction
let img = ImageBuilder::auto(bytes).size(2000000, 2000000).at(500000, 1500000).build();
let shape = Shape::new(ShapeType::Rectangle, 500000, 1600000, 2000000, 1000000);
let table = TableBuilder::new().rows(3).cols(4).build();
```

**After:**
```rust
// Consistent construction with helpers
let img = image(bytes).size(2.0, 2.0).at(0.5, 1.5);
let shape = rect(0.5, 1.6, 2.0, 1.0);
let table = table(3, 4); // rows, cols
```

#### 2. Enhanced Prelude Module

Add to `prelude.rs`:
```rust
// Unit helpers
pub use crate::core::units::{inches, cm, mm, pt};

// Shape helpers
pub fn rect(x: f64, y: f64, w: f64, h: f64) -> Shape;
pub fn circle(x: f64, y: f64, diameter: f64) -> Shape;
pub fn ellipse(x: f64, y: f64, w: f64, h: f64) -> Shape;
pub fn rounded_rect(x: f64, y: f64, w: f64, h: f64) -> Shape;
pub fn triangle(x: f64, y: f64, w: f64, h: f64) -> Shape;

// Image helpers
pub fn image<T: Into<Vec<u8>>>(data: T) -> ImageBuilder;
pub fn image_file(path: &str) -> Result<ImageBuilder>;

// Table helpers
pub fn table(rows: usize, cols: usize) -> TableBuilder;

// Chart helpers
pub fn bar_chart() -> ChartBuilder;
pub fn line_chart() -> ChartBuilder;
pub fn pie_chart() -> ChartBuilder;

// Color helpers
pub fn rgb(r: u8, g: u8, b: u8) -> Color;
pub fn hex(color: &str) -> Color;
```

#### 3. Simplified Color API

**Before:**
```rust
.with_fill(ShapeFill::new("4F81BD"))
.with_line(ShapeLine::new("FF0000", 25400))
```

**After:**
```rust
.fill(hex("4F81BD"))  // or .fill(colors::BLUE)
.stroke(hex("FF0000"), 1.0)  // width in points
```

#### 4. Consistent Method Naming

**Before:**
```rust
.with_fill()
.with_line()
.with_text()
.with_gradient()
.add_bullet()
.add_shape()
```

**After:**
```rust
.fill()      // shorter, clearer
.stroke()    // standard graphics term
.text()      // shorter
.gradient()  // shorter
.bullet()    // shorter
.shape()     // shorter
```

#### 5. Smart Defaults

**Before:**
```rust
let shape = Shape::new(ShapeType::Rectangle, 500000, 1600000, 2000000, 1000000)
    .with_fill(ShapeFill::new("4F81BD"))
    .with_text("Hello");
```

**After:**
```rust
// Position defaults to (0, 0), size defaults to reasonable values
let shape = rect(0.5, 1.6, 2.0, 1.0)
    .fill(colors::BLUE)
    .text("Hello");

// Or even simpler with builder
let shape = rect_at(0.5, 1.6)  // auto-size based on text
    .fill(colors::BLUE)
    .text("Hello");
```

#### 6. Dimension Simplification

**Before:**
```rust
Shape::from_dimensions(
    ShapeType::Rectangle,
    Dimension::Inches(0.5),
    Dimension::Inches(1.75),
    Dimension::Inches(2.2),
    Dimension::Inches(1.1)
)
```

**After:**
```rust
// Default unit is inches
rect(0.5, 1.75, 2.2, 1.1)

// Or explicit units when needed
rect(cm(1.0), cm(2.0), cm(5.0), cm(3.0))

// Or mixed units
rect(inches(0.5), percent(20), inches(2.0), percent(10))
```

## Implementation Plan

### Phase 1: Core Helpers (Week 1)
- [ ] Create `src/helpers/mod.rs`
- [ ] Implement shape helpers: `rect()`, `circle()`, `ellipse()`, etc.
- [ ] Implement image helpers: `image()`, `image_file()`
- [ ] Implement color helpers: `rgb()`, `hex()`
- [ ] Add to prelude

### Phase 2: Method Renaming (Week 2)
- [ ] Add shorter method aliases (keep old ones for compatibility)
- [ ] `fill()` as alias for `with_fill()`
- [ ] `stroke()` as alias for `with_line()`
- [ ] `text()` as alias for `with_text()`
- [ ] Update documentation

### Phase 3: Smart Defaults (Week 3)
- [ ] Add default sizes for common shapes
- [ ] Auto-sizing based on text content
- [ ] Default colors (black text, no fill)
- [ ] Position defaults to (0, 0)

### Phase 4: Enhanced Prelude (Week 4)
- [ ] Consolidate all helpers in prelude
- [ ] Add color constants
- [ ] Add common patterns
- [ ] Update examples

### Phase 5: Documentation (Week 5)
- [ ] Update README with new API
- [ ] Create migration guide
- [ ] Update all examples
- [ ] Add API comparison table

## Example Transformations

### Before (Current API)
```rust
use ppt_rs::generator::{
    create_pptx_with_content, SlideContent,
    Shape, ShapeType, ShapeFill, ShapeLine,
    ImageBuilder, Image,
};

let shape = Shape::new(ShapeType::Rectangle, 500000, 1600000, 2000000, 1000000)
    .with_fill(ShapeFill::new("4F81BD"))
    .with_line(ShapeLine::new("FF0000", 25400))
    .with_text("Hello World");

let img = ImageBuilder::auto(image_bytes)
    .size(2000000, 2000000)
    .at(500000, 1500000)
    .shadow()
    .build();

let slide = SlideContent::new("My Slide")
    .add_shape(shape)
    .add_image(img);
```

### After (Simplified API)
```rust
use ppt_rs::prelude::*;

let shape = rect(0.5, 1.6, 2.0, 1.0)
    .fill(colors::BLUE)
    .stroke(colors::RED, 1.0)
    .text("Hello World");

let img = image(image_bytes)
    .size(2.0, 2.0)
    .at(0.5, 1.5)
    .shadow();

let slide = slide("My Slide")
    .shape(shape)
    .image(img);
```

**Reduction**: ~40% less code, much more readable!

## Backward Compatibility

### Strategy
1. **Keep existing APIs**: Don't break existing code
2. **Add new helpers**: Provide simpler alternatives
3. **Deprecation warnings**: Mark old patterns as deprecated
4. **Migration period**: 2-3 releases before removal
5. **Clear documentation**: Show both old and new ways

### Example
```rust
// Old API - still works, but deprecated
#[deprecated(since = "0.3.0", note = "Use `rect()` helper instead")]
pub fn new(shape_type: ShapeType, x: u32, y: u32, w: u32, h: u32) -> Self { ... }

// New API - recommended
pub fn rect(x: f64, y: f64, w: f64, h: f64) -> Shape { ... }
```

## Benefits

### For Users
- ✅ **Faster development**: Less code to write
- ✅ **Easier learning**: Intuitive API
- ✅ **Fewer errors**: Type-safe, sensible defaults
- ✅ **Better readability**: Code is self-documenting
- ✅ **Consistent patterns**: Same approach everywhere

### For Library
- ✅ **Better adoption**: Lower barrier to entry
- ✅ **Fewer support questions**: API is clearer
- ✅ **Competitive advantage**: Simpler than alternatives
- ✅ **Modern Rust**: Follows ecosystem best practices
- ✅ **Maintainability**: Cleaner codebase

## Success Metrics

1. **Code reduction**: 30-50% less code in examples
2. **Learning curve**: New users productive in <30 minutes
3. **API consistency**: 100% of types follow same pattern
4. **Documentation**: All examples use new API
5. **Community feedback**: Positive reception

## Next Steps

1. **Review this proposal** with team/community
2. **Prototype Phase 1** in a feature branch
3. **Get feedback** from early adopters
4. **Iterate** based on feedback
5. **Implement** remaining phases
6. **Release** as v0.3.0 with migration guide

## References

- **PptxGenJS**: Simple JavaScript API inspiration
- **python-pptx**: Pythonic API patterns
- **Rust ecosystem**: Builder patterns, preludes, helper functions
- **Graphics libraries**: Processing, p5.js for naming conventions
