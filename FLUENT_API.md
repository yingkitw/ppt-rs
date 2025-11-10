# Fluent API Guide

## Overview

The Fluent API in `ppt-rs` provides an intuitive, chainable interface for creating PowerPoint presentations, inspired by the simplicity of PptxGenJS while maintaining Rust's type safety.

## Quick Start

### Current Implementation (Phase 1)

The fluent API is currently in **Phase 1**, providing:
- ✅ Fluent presentation builder
- ✅ Content options with sensible defaults
- ✅ Method chaining for options

**Phase 1 Example:**
```rust
use ppt_rs::PresentationBuilder;
use ppt_rs::slide::options::TextOptions;

// Create a presentation with builder
let prs = PresentationBuilder::new()
    .title("My Presentation")
    .author("John Doe")
    .build()?;

// Create content options with sensible defaults
let text_opts = TextOptions::new()
    .position(1.0, 1.0)
    .font_size(24)
    .color("FF0000")
    .bold(true);

// Use options to add content to slides
// (Phase 2 will provide fluent slide API)
```

### Future Implementation (Phase 2+)

**Phase 2 (Planned):**
```rust
// 2. Add a slide with fluent API
let mut slide = prs.add_slide();

// 3. Add content with fluent methods
slide
    .add_text("Hello", TextOptions::default())?
    .add_shape(ShapeType::Rectangle, ShapeOptions::default())?
    .add_image("image.png", ImageOptions::default())?;

// 4. Save
prs.save_to_file("output.pptx")?;
```

## Builder Pattern

### PresentationBuilder

Create presentations with fluent configuration:

```rust
let prs = PresentationBuilder::new()
    .title("My Presentation")
    .author("John Doe")
    .subject("Creating PPTX files")
    .company("Acme Corp")
    .build()?;
```

**Available Methods:**
- `.title(String)` - Set presentation title
- `.author(String)` - Set author name
- `.subject(String)` - Set subject
- `.company(String)` - Set company name
- `.build()` - Create the presentation

## Content Options

### TextOptions

Add text with intuitive configuration:

```rust
let opts = TextOptions::new()
    .position(1.0, 1.0)           // x, y in inches
    .size(4.0, 1.0)               // width, height in inches
    .font_size(24)                // points
    .font_name("Calibri")         // font name
    .color("FF0000")              // hex color
    .bold(true)
    .italic(false);
```

**Default Values:**
- Position: (0.0, 0.0)
- Font size: 12pt
- Font: Calibri
- Color: Black (#000000)
- Bold: false
- Italic: false

**Builder Methods:**
- `.position(x, y)` - Set X, Y position in inches
- `.size(width, height)` - Set width, height in inches
- `.color(hex)` - Set text color (e.g., "FF0000")
- `.font_size(points)` - Set font size
- `.font_name(name)` - Set font name
- `.bold(bool)` - Set bold
- `.italic(bool)` - Set italic

### ShapeOptions

Add shapes with intuitive configuration:

```rust
let opts = ShapeOptions::new()
    .position(1.0, 1.0)           // x, y in inches
    .size(2.0, 1.0)               // width, height in inches
    .fill_color("4472C4")         // hex color
    .line_color("000000")         // hex color
    .line_width(1.5);             // points
```

**Default Values:**
- Position: (0.0, 0.0)
- Size: (1.0, 1.0)
- Fill color: Blue (#4472C4)
- Line color: Black (#000000)
- Line width: 1.0pt

**Builder Methods:**
- `.position(x, y)` - Set X, Y position in inches
- `.size(width, height)` - Set width, height in inches
- `.fill_color(hex)` - Set fill color
- `.line_color(hex)` - Set line color
- `.line_width(points)` - Set line width

### ImageOptions

Add images with intuitive configuration:

```rust
let opts = ImageOptions::new()
    .position(1.0, 1.0)           // x, y in inches
    .size(4.0, 3.0);              // width, height in inches
```

**Default Values:**
- Position: (0.0, 0.0)
- Size: Auto (original dimensions)

**Builder Methods:**
- `.position(x, y)` - Set X, Y position in inches
- `.size(width, height)` - Set width, height in inches

## Color Formats

Colors are specified as hex strings without the '#' prefix:

```rust
// Valid color formats
"FF0000"      // Red
"00FF00"      // Green
"0000FF"      // Blue
"FFFFFF"      // White
"000000"      // Black
"4472C4"      // Office Blue
```

## Measurements

All measurements use **inches** as the default unit:

```rust
// Position: 1 inch from left, 1 inch from top
.position(1.0, 1.0)

// Size: 2 inches wide, 1 inch tall
.size(2.0, 1.0)

// Font size uses points
.font_size(24)  // 24 points

// Line width uses points
.line_width(1.5)  // 1.5 points
```

## Sensible Defaults

All options have sensible defaults, so you only need to specify what you want to change:

```rust
// Minimal configuration - uses all defaults
let opts = TextOptions::default();

// Override just what you need
let opts = TextOptions::default()
    .position(1.0, 1.0)
    .font_size(24);
```

## Complete Example

```rust
use ppt_rs::PresentationBuilder;
use ppt_rs::slide::options::{TextOptions, ShapeOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create presentation
    let mut prs = PresentationBuilder::new()
        .title("Sales Report")
        .author("Sales Team")
        .build()?;
    
    // Add slide
    let mut slides = prs.slides();
    let mut slide = slides.add_slide(&layout, &mut prs.package_mut())?;
    
    // Add title
    let title = TextOptions::new()
        .position(0.5, 0.5)
        .font_size(44)
        .bold(true)
        .color("1F4E78");
    
    // Add subtitle
    let subtitle = TextOptions::new()
        .position(0.5, 1.5)
        .font_size(24)
        .color("595959");
    
    // Add shape
    let shape = ShapeOptions::new()
        .position(1.0, 3.0)
        .size(3.0, 2.0)
        .fill_color("4472C4")
        .line_color("000000");
    
    // Save
    prs.save_to_file("sales_report.pptx")?;
    
    Ok(())
}
```

## Comparison with PptxGenJS

### JavaScript
```javascript
let pres = new PptxGenJS();
let slide = pres.addSlide();
slide.addText("Hello", { x: 1, y: 1, fontSize: 24, color: "FF0000" });
pres.writeFile({ fileName: "output.pptx" });
```

### Rust (Fluent API)
```rust
let mut prs = PresentationBuilder::new().build()?;
let mut slides = prs.slides();
let mut slide = slides.add_slide(&layout, &mut prs.package_mut())?;

let opts = TextOptions::new()
    .position(1.0, 1.0)
    .font_size(24)
    .color("FF0000");

prs.save_to_file("output.pptx")?;
```

## Benefits

✅ **Intuitive** - Familiar pattern from PptxGenJS
✅ **Type-Safe** - Compile-time validation
✅ **Chainable** - Fluent API with method chaining
✅ **Sensible Defaults** - Less boilerplate code
✅ **Discoverable** - IDE autocomplete support
✅ **Flexible** - Override only what you need

## Migration from Old API

### Before
```rust
let mut shape = AutoShape::new(1, "Rectangle", AutoShapeType::Rectangle);
shape.set_left(914400);  // EMU conversion needed
shape.set_top(914400);
shape.set_width(1828800);
shape.set_height(914400);
```

### After
```rust
let opts = ShapeOptions::new()
    .position(1.0, 1.0)
    .size(2.0, 1.0);
```

## Next Steps

- See `examples/02_fluent_api.rs` for complete working example
- Check `API_DESIGN.md` for architectural details
- Read `README.md` for more information
