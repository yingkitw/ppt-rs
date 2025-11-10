# Fluent API Roadmap

## Overview

The Fluent API is being developed in phases to bring PptxGenJS-like simplicity to ppt-rs while maintaining Rust's type safety.

## Current Status: Phase 2 ✅ COMPLETE

### Phase 1: Builder Pattern & Content Options

**Completed:**
- ✅ `PresentationBuilder` - Fluent presentation creation
- ✅ `TextOptions` - Text configuration with sensible defaults
- ✅ `ShapeOptions` - Shape configuration with sensible defaults
- ✅ `ImageOptions` - Image configuration with sensible defaults
- ✅ Method chaining for all options
- ✅ Comprehensive documentation
- ✅ 9 new tests (491 total, 100% passing)

**What It Provides:**
```rust
let prs = PresentationBuilder::new()
    .title("My Presentation")
    .author("John Doe")
    .build()?;

let text_opts = TextOptions::new()
    .position(1.0, 1.0)
    .font_size(24)
    .color("FF0000")
    .bold(true);

let shape_opts = ShapeOptions::new()
    .position(1.0, 1.0)
    .size(2.0, 1.0)
    .fill_color("4472C4");
```

**Key Features:**
- Builder pattern for configuration
- Sensible defaults for all options
- Method chaining
- Type-safe
- Intuitive API

---

## Phase 2: Fluent Slide API ✅ COMPLETE

### Completed
- ✅ Fluent slide methods: `with_name()`, `with_background()`, `with_transition()`
- ✅ Fluent presentation methods: `with_slide_width()`, `with_slide_height()`
- ✅ Method chaining on Slide and Presentation
- ✅ Example: `examples/03_fluent_slide_api.rs`
- ✅ All tests passing (491 total)

### What It Provides
```rust
let mut prs = PresentationBuilder::new()
    .title("My Presentation")
    .build()?;

let slide_idx = prs.add_slide()?;

// Fluent slide configuration
let slide = Slide::new()
    .with_name("My Slide")
    .with_background(background)
    .with_transition(transition);

// Fluent presentation configuration
let prs = prs
    .with_slide_width(9144000)?
    .with_slide_height(6858000)?;
```

### Key Features
- Fluent methods return `Self` for chaining
- Type-safe configuration
- Intuitive API
- Backward compatible

---

## Phase 3: Simplified Content Methods (PLANNED)

### Objectives
- Add fluent methods to `Slide` struct
- Implement `add_text()`, `add_shape()`, `add_image()` methods
- Enable method chaining on slides
- Return `&mut Slide` for chaining

### Implementation Plan

**1. Extend Slide Struct**
```rust
impl Slide {
    pub fn add_text(&mut self, text: &str, opts: TextOptions) -> Result<&mut Self> {
        // Add text to slide
        Ok(self)
    }
    
    pub fn add_shape(&mut self, shape_type: ShapeType, opts: ShapeOptions) -> Result<&mut Self> {
        // Add shape to slide
        Ok(self)
    }
    
    pub fn add_image(&mut self, path: &str, opts: ImageOptions) -> Result<&mut Self> {
        // Add image to slide
        Ok(self)
    }
}
```

**2. Enable Method Chaining**
```rust
let mut slide = prs.add_slide();
slide
    .add_text("Title", TextOptions::default())?
    .add_text("Subtitle", TextOptions::default())?
    .add_shape(ShapeType::Rectangle, ShapeOptions::default())?;
```

**3. Integration Points**
- Modify `src/slide/slide.rs` - Add fluent methods
- Enhance `src/slide/mod.rs` - Export shape types
- Update `src/shapes/mod.rs` - Ensure all types are public

### Expected Outcome
```rust
let mut prs = PresentationBuilder::new()
    .title("My Presentation")
    .build()?;

let mut slide = prs.add_slide();
slide
    .add_text("Hello, World!", TextOptions::new()
        .position(0.5, 0.5)
        .font_size(44))?
    .add_shape(ShapeType::Rectangle, ShapeOptions::new()
        .position(1.0, 1.0)
        .size(2.0, 1.0))?;

prs.save_to_file("output.pptx")?;
```

### Estimated Effort
- Implementation: 2-3 hours
- Testing: 1-2 hours
- Documentation: 1 hour
- **Total: 4-6 hours**

### Test Count Impact
- Current: 491 tests
- Expected: 510+ tests (+19 tests)

---

## Phase 3: Simplified Content Methods (PLANNED)

### Objectives
- Simplify content addition with preset options
- Add convenience methods for common scenarios
- Reduce boilerplate code

### Implementation Plan

**1. Preset Options**
```rust
impl TextOptions {
    pub fn title() -> Self {
        Self::new()
            .font_size(44)
            .bold(true)
            .color("1F4E78")
    }
    
    pub fn subtitle() -> Self {
        Self::new()
            .font_size(24)
            .color("595959")
    }
    
    pub fn body() -> Self {
        Self::new()
            .font_size(18)
            .color("000000")
    }
}
```

**2. Convenience Methods**
```rust
impl Slide {
    pub fn add_title(&mut self, text: &str) -> Result<&mut Self> {
        self.add_text(text, TextOptions::title())
    }
    
    pub fn add_subtitle(&mut self, text: &str) -> Result<&mut Self> {
        self.add_text(text, TextOptions::subtitle())
    }
    
    pub fn add_body(&mut self, text: &str) -> Result<&mut Self> {
        self.add_text(text, TextOptions::body())
    }
}
```

**3. Usage**
```rust
let mut slide = prs.add_slide();
slide
    .add_title("My Title")?
    .add_subtitle("My Subtitle")?
    .add_body("Body text here")?;
```

### Estimated Effort
- Implementation: 1-2 hours
- Testing: 1 hour
- Documentation: 30 minutes
- **Total: 2.5-3.5 hours**

### Test Count Impact
- Current: 510+ tests
- Expected: 525+ tests (+15 tests)

---

## Phase 4: Advanced Content (PLANNED)

### Objectives
- Add support for tables
- Add support for charts
- Add support for media
- Add support for animations

### Implementation Plan

**1. Tables**
```rust
impl Slide {
    pub fn add_table(&mut self, rows: usize, cols: usize, opts: TableOptions) -> Result<&mut Self> {
        // Add table to slide
        Ok(self)
    }
}
```

**2. Charts**
```rust
impl Slide {
    pub fn add_chart(&mut self, chart_type: ChartType, opts: ChartOptions) -> Result<&mut Self> {
        // Add chart to slide
        Ok(self)
    }
}
```

**3. Media**
```rust
impl Slide {
    pub fn add_video(&mut self, path: &str, opts: MediaOptions) -> Result<&mut Self> {
        // Add video to slide
        Ok(self)
    }
    
    pub fn add_audio(&mut self, path: &str, opts: MediaOptions) -> Result<&mut Self> {
        // Add audio to slide
        Ok(self)
    }
}
```

**4. Animations**
```rust
impl Slide {
    pub fn add_animation(&mut self, shape_id: u32, effect: AnimationEffect) -> Result<&mut Self> {
        // Add animation to shape
        Ok(self)
    }
}
```

### Estimated Effort
- Implementation: 6-8 hours
- Testing: 3-4 hours
- Documentation: 2 hours
- **Total: 11-14 hours**

### Test Count Impact
- Current: 525+ tests
- Expected: 560+ tests (+35 tests)

---

## Phase 5: HTML to PowerPoint (PLANNED)

### Objectives
- Convert HTML to PowerPoint slides
- Support common HTML elements
- Preserve formatting and styling

### Implementation Plan

**1. HTML Parser**
```rust
impl Slide {
    pub fn add_html(&mut self, html: &str) -> Result<&mut Self> {
        // Parse HTML and add content
        Ok(self)
    }
}
```

**2. Supported Elements**
- `<h1>` - Title
- `<h2>` - Subtitle
- `<p>` - Body text
- `<ul>`, `<ol>` - Lists
- `<img>` - Images
- `<table>` - Tables
- `<code>` - Code blocks

**3. Usage**
```rust
let html = r#"
    <h1>My Title</h1>
    <p>Some body text</p>
    <ul>
        <li>Item 1</li>
        <li>Item 2</li>
    </ul>
"#;

let mut slide = prs.add_slide();
slide.add_html(html)?;
```

### Estimated Effort
- Implementation: 8-10 hours
- Testing: 4-5 hours
- Documentation: 2 hours
- **Total: 14-17 hours**

### Test Count Impact
- Current: 560+ tests
- Expected: 600+ tests (+40 tests)

---

## Implementation Timeline

### Recommended Schedule

| Phase | Duration | Start | End | Status |
|-------|----------|-------|-----|--------|
| Phase 1 | 6 hours | Nov 10 | Nov 10 ✅ | COMPLETE |
| Phase 2 | 4-6 hours | Nov 11 | Nov 11 | PLANNED |
| Phase 3 | 2.5-3.5 hours | Nov 12 | Nov 12 | PLANNED |
| Phase 4 | 11-14 hours | Nov 13-14 | Nov 14 | PLANNED |
| Phase 5 | 14-17 hours | Nov 15-17 | Nov 17 | PLANNED |

**Total Estimated Time**: 37.5-46.5 hours (5-6 working days)

---

## Success Criteria

### Phase 1 ✅
- [x] PresentationBuilder implemented
- [x] TextOptions with sensible defaults
- [x] ShapeOptions with sensible defaults
- [x] ImageOptions with sensible defaults
- [x] Method chaining working
- [x] 9 new tests passing
- [x] Documentation complete

### Phase 2 (Planned)
- [ ] Fluent slide methods implemented
- [ ] add_text() working
- [ ] add_shape() working
- [ ] add_image() working
- [ ] Method chaining on slides working
- [ ] 19+ new tests passing
- [ ] Documentation updated

### Phase 3 (Planned)
- [ ] Preset options implemented
- [ ] Convenience methods working
- [ ] 15+ new tests passing
- [ ] Documentation updated

### Phase 4 (Planned)
- [ ] Tables, charts, media, animations supported
- [ ] 35+ new tests passing
- [ ] Documentation updated

### Phase 5 (Planned)
- [ ] HTML to PowerPoint working
- [ ] 40+ new tests passing
- [ ] Documentation updated

---

## Comparison: Before vs After

### Before (Current)
```rust
let mut prs = new_presentation()?;
let mut slides = prs.slides();
let mut slide = slides.add_slide(&layout, &mut prs.package_mut())?;
// Complex shape creation...
prs.save_to_file("output.pptx")?;
```

### After Phase 1 (Current)
```rust
let prs = PresentationBuilder::new()
    .title("My Presentation")
    .build()?;

let text_opts = TextOptions::new()
    .position(1.0, 1.0)
    .font_size(24);
```

### After Phase 2 (Planned)
```rust
let mut prs = PresentationBuilder::new()
    .title("My Presentation")
    .build()?;

let mut slide = prs.add_slide();
slide
    .add_text("Hello", TextOptions::default())?
    .add_shape(ShapeType::Rectangle, ShapeOptions::default())?;

prs.save_to_file("output.pptx")?;
```

### After Phase 5 (Planned)
```rust
let mut prs = PresentationBuilder::new()
    .title("My Presentation")
    .build()?;

let mut slide = prs.add_slide();
slide.add_html(r#"
    <h1>My Title</h1>
    <p>Some content</p>
"#)?;

prs.save_to_file("output.pptx")?;
```

---

## Key Design Decisions

1. **Method Chaining**: Return `&mut Self` for fluent interface
2. **Sensible Defaults**: All options have reasonable defaults
3. **Type Safety**: Strong typing for all enums and options
4. **Backward Compatibility**: Keep existing API intact
5. **Incremental Development**: One phase at a time

---

## Next Steps

1. **Phase 2 Implementation** - Fluent slide API
   - Modify `src/slide/slide.rs`
   - Add `add_text()`, `add_shape()`, `add_image()` methods
   - Enable method chaining
   - Add tests

2. **Documentation Updates**
   - Update FLUENT_API.md with Phase 2 examples
   - Update README.md with new API
   - Add examples for Phase 2

3. **Testing**
   - Add 19+ new tests for Phase 2
   - Ensure all tests pass
   - Verify backward compatibility

---

## Questions & Considerations

1. **Should we support fluent methods on Presentation too?**
   - Yes, for consistency

2. **Should add_* methods return Result or &mut Self?**
   - Return Result<&mut Self> for error handling + chaining

3. **How to handle shape IDs and relationships?**
   - Automatic ID generation and relationship management

4. **Should we support nested shapes?**
   - Future phase (Phase 4+)

5. **How to handle undo/redo?**
   - Not in scope for fluent API

---

## Conclusion

The Fluent API roadmap provides a clear path to making ppt-rs as intuitive as PptxGenJS while maintaining Rust's type safety and performance benefits. Phase 1 is complete, and Phase 2-5 are planned for implementation.

**Current Status**: ✅ Phase 1 Complete
**Next Phase**: Phase 2 (Fluent Slide API)
**Target**: All 5 phases complete by end of November 2025
