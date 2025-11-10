# API Design: Learning from PptxGenJS

## Overview

This document outlines how the Rust `ppt-rs` library can adopt the intuitive and robust design patterns from the JavaScript `PptxGenJS` library while maintaining Rust's type safety and performance benefits.

## Key Principles from PptxGenJS

### 1. **Simplicity First**
PptxGenJS follows a simple 4-step pattern:
```javascript
// 1. Create
let pres = new PptxGenJS();

// 2. Add Slide
let slide = pres.addSlide();

// 3. Add Content
slide.addText("Hello World", { x: 1, y: 1 });

// 4. Save
pres.writeFile();
```

### 2. **Fluent API**
- Method chaining for intuitive workflows
- Sensible defaults for all parameters
- Optional configuration objects

### 3. **Unified Interface**
- Same methods work across all platforms (Node, Browser)
- Consistent naming conventions
- Predictable behavior

### 4. **Robustness**
- Automatic error handling
- Validation at every step
- Clear error messages

## Rust Implementation Strategy

### Phase 1: Builder Pattern (✅ IMPLEMENTED)

```rust
use ppt_rs::PresentationBuilder;

let mut prs = PresentationBuilder::new()
    .title("My Presentation")
    .author("John Doe")
    .build()?;
```

**Benefits:**
- Type-safe configuration
- Clear intent
- Compile-time validation where possible

### Phase 2: Fluent Slide API (📋 PLANNED)

```rust
let mut slide = prs.add_slide();
slide
    .add_text("Hello, World!", TextOptions::default())?
    .add_shape(ShapeType::Rectangle, ShapeOptions::default())?
    .add_image("image.png", ImageOptions::default())?;
```

**Implementation:**
- `Slide` struct with builder methods
- Return `&mut self` for chaining
- Sensible defaults via `Default` trait

### Phase 3: Simplified Content API (📋 PLANNED)

```rust
// Simple text addition
slide.add_text("Hello", Default::default())?;

// With options
let opts = TextOptions {
    x: 1.0,
    y: 1.0,
    color: "FF0000",
    ..Default::default()
};
slide.add_text("Hello", opts)?;
```

**Implementation:**
- Use `Default` for sensible defaults
- Builder pattern for complex options
- Type-safe color handling

### Phase 4: Save Simplification (📋 PLANNED)

```rust
// Simple save
prs.save_to_file("output.pptx")?;

// Save with options
prs.save_to_file_with_options("output.pptx", SaveOptions {
    compress: true,
    ..Default::default()
})?;
```

## Comparison: JavaScript vs Rust

### Text Addition

**JavaScript:**
```javascript
slide.addText("Hello", { x: 1, y: 1, color: "FF0000" });
```

**Rust (Current):**
```rust
let mut text_frame = slide.text_frame_mut()?;
text_frame.set_text("Hello")?;
```

**Rust (Proposed):**
```rust
slide.add_text("Hello", TextOptions {
    x: 1.0,
    y: 1.0,
    color: "FF0000",
    ..Default::default()
})?;
```

### Shape Addition

**JavaScript:**
```javascript
slide.addShape(pptx.shapes.RECTANGLE, {
    x: 1, y: 1, w: 2, h: 1,
    fill: { color: "FF0000" }
});
```

**Rust (Current):**
```rust
let mut shape = AutoShape::new(1, "Rectangle", AutoShapeType::Rectangle);
shape.set_left(914400); // EMU conversion needed
shape.set_top(914400);
shape.set_width(1828800);
shape.set_height(914400);
```

**Rust (Proposed):**
```rust
slide.add_shape(ShapeType::Rectangle, ShapeOptions {
    x: 1.0,
    y: 1.0,
    width: 2.0,
    height: 1.0,
    fill_color: Some("FF0000"),
    ..Default::default()
})?;
```

## Design Patterns

### 1. **Builder Pattern**
- Used for complex configurations
- Enables method chaining
- Type-safe at compile time

### 2. **Default Trait**
- Provides sensible defaults
- Reduces boilerplate
- Improves readability

### 3. **Fluent API**
- Methods return `&mut self`
- Enables chaining
- Improves ergonomics

### 4. **Result Type**
- Error handling via `Result<T>`
- Clear error messages
- No exceptions/panics

## Implementation Roadmap

### ✅ Completed
- [x] `PresentationBuilder` for fluent presentation creation
- [x] PPTX file saving (valid ZIP structure)
- [x] All 482 tests passing

### 📋 In Progress
- [ ] Simplify slide creation API
- [ ] Add fluent content methods
- [ ] Implement sensible defaults

### 🔮 Future
- [ ] HTML to PowerPoint conversion
- [ ] Advanced animations
- [ ] Custom themes
- [ ] Media handling improvements

## Best Practices

### 1. **Use Builders for Configuration**
```rust
let opts = TextOptions::builder()
    .x(1.0)
    .y(1.0)
    .color("FF0000")
    .build();
```

### 2. **Provide Sensible Defaults**
```rust
impl Default for TextOptions {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            color: "000000",
            size: 12,
            ..
        }
    }
}
```

### 3. **Use Type-Safe Enums**
```rust
enum ShapeType {
    Rectangle,
    Circle,
    Triangle,
    // ...
}
```

### 4. **Clear Error Messages**
```rust
Err(PptError::ValueError(
    "Invalid color format: expected #RRGGBB or RRGGBB".to_string()
))
```

## Example: Complete Workflow

### JavaScript
```javascript
let pres = new PptxGenJS();
let slide = pres.addSlide();
slide.addText("Hello, World!", { x: 1, y: 1, fontSize: 44 });
pres.writeFile({ fileName: "output.pptx" });
```

### Rust (Proposed)
```rust
use ppt_rs::PresentationBuilder;

let mut pres = PresentationBuilder::new()
    .title("My Presentation")
    .build()?;

let mut slide = pres.add_slide();
slide.add_text("Hello, World!", TextOptions {
    x: 1.0,
    y: 1.0,
    font_size: 44,
    ..Default::default()
})?;

pres.save_to_file("output.pptx")?;
```

## Summary

By adopting PptxGenJS design principles while maintaining Rust's type safety:

1. **Simplicity**: Fluent APIs and sensible defaults
2. **Robustness**: Type safety and compile-time checks
3. **Intuitiveness**: Clear method names and patterns
4. **Performance**: Compiled Rust with no GC overhead
5. **Safety**: Memory safety without sacrificing ergonomics

This approach makes `ppt-rs` both powerful and easy to use.
