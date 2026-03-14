# ppt-rs API Guide

Complete guide to the simplified ppt-rs API for creating PowerPoint presentations.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Shape Helpers](#shape-helpers)
3. [Color API](#color-api)
4. [Table API](#table-api)
5. [Extension Methods](#extension-methods)
6. [Complete Examples](#complete-examples)

---

## Quick Start

```rust
use ppt_rs::prelude::*;

fn main() -> Result<()> {
    let slides = vec![
        SlideContent::new("My First Slide")
            .add_shape(
                rect(1.0, 2.0, 3.0, 1.5)
                    .fill(blue().to_color())
                    .text("Hello World")
            ),
    ];
    
    let pptx = create_pptx_with_content("My Presentation", slides)?;
    std::fs::write("output.pptx", pptx)?;
    Ok(())
}
```

---

## Shape Helpers

Create shapes with simple, readable functions. All dimensions are in **inches**.

### Rectangle
```rust
rect(x, y, width, height)
    .fill(color)
    .stroke(color, width)
    .text("Text")
```

**Example:**
```rust
rect(1.0, 2.0, 3.0, 1.5)
    .fill(blue().to_color())
    .stroke(black().to_color(), 2.0)
    .text("Rectangle")
```

### Circle
```rust
circle(x, y, diameter)
```

**Example:**
```rust
circle(2.0, 2.0, 1.5)
    .fill(red().to_color())
    .text("Circle")
```

### Ellipse
```rust
ellipse(x, y, width, height)
```

**Example:**
```rust
ellipse(3.0, 2.0, 2.5, 1.5)
    .fill(green().to_color())
```

### Rounded Rectangle
```rust
rounded_rect(x, y, width, height)
```

**Example:**
```rust
rounded_rect(1.0, 3.0, 2.0, 1.0)
    .fill(orange().to_color())
    .text("Rounded")
```

### Triangle
```rust
triangle(x, y, width, height)
```

### Diamond
```rust
diamond(x, y, width, height)
```

### Arrow Shapes
```rust
shapes::arrow_right(x, y, width, height)
shapes::arrow_left(x, y, width, height)
shapes::arrow_up(x, y, width, height)
shapes::arrow_down(x, y, width, height)
```

---

## Color API

### Color Aliases

Use simple color names instead of hex codes:

```rust
// Basic colors
red()       green()      blue()       yellow()
cyan()      magenta()    white()      black()
gray()      orange()     purple()     pink()
brown()     navy()       teal()       olive()

// Material Design colors
material_red()      material_blue()     material_green()
material_orange()   material_purple()   material_pink()
material_teal()     material_amber()    material_brown()

// Corporate colors
corporate_blue()    corporate_green()   corporate_red()
```

**Example:**
```rust
rect(1.0, 2.0, 2.0, 1.0)
    .fill(material_blue().to_color())
    .text("Material Blue")
```

### Color Adjustments

#### Lighter
Make a color lighter by a percentage (0.0 - 1.0):

```rust
let light_blue = blue().lighter(0.3);  // 30% lighter
```

#### Darker
Make a color darker by a percentage (0.0 - 1.0):

```rust
let dark_blue = blue().darker(0.5);  // 50% darker
```

#### Opacity
Set the opacity (0.0 = transparent, 1.0 = opaque):

```rust
let semi_transparent = red().opacity(0.5);  // 50% opacity
```

#### Transparency
Set transparency as a percentage (0 = opaque, 100 = transparent):

```rust
let transparent = blue().transparent(25);  // 25% transparent
```

### Color Operations

#### Mix Colors
Blend two colors together:

```rust
let purple = red().mix(&blue(), 0.5);  // 50/50 mix
let pink = red().mix(&white(), 0.3);   // 30% white, 70% red
```

#### Grayscale
Convert to grayscale:

```rust
let gray_version = blue().grayscale();
```

#### Invert
Invert the color:

```rust
let inverted = blue().invert();
```

### RGB Colors

Create colors from RGB values:

```rust
ColorValue::rgb(255, 128, 64)
```

### Hex Colors

Create colors from hex strings:

```rust
ColorValue::from_hex("#FF8040")
ColorValue::from_hex("FF8040")  // # is optional
```

### Complete Color Example

```rust
SlideContent::new("Color Showcase")
    .add_shape(
        rect(1.0, 1.5, 1.5, 0.8)
            .fill(blue().to_color())
            .text("Base Blue")
    )
    .add_shape(
        rect(3.0, 1.5, 1.5, 0.8)
            .fill(blue().lighter(0.3).to_color())
            .text("Lighter")
    )
    .add_shape(
        rect(5.0, 1.5, 1.5, 0.8)
            .fill(blue().darker(0.3).to_color())
            .text("Darker")
    )
    .add_shape(
        rect(2.0, 2.8, 1.5, 0.8)
            .fill(red().mix(&blue(), 0.5).to_color())
            .text("Purple Mix")
    )
```

---

## Table API

### Simple Table

Create a table with equal column widths:

```rust
simple_table(rows, cols)
    .position(inches(x), inches(y))
    .build()
```

**Example:**
```rust
let table = simple_table(4, 3)
    .position(inches(1.0), inches(1.5))
    .build();
```

### Table with Custom Widths

Specify column widths in inches:

```rust
table_with_widths(&[2.0, 3.0, 2.5])
    .position(inches(1.0), inches(1.5))
    .build()
```

### Table from Data

Create a table from a 2D array:

```rust
let data = vec![
    vec!["Name", "Age", "City"],
    vec!["Alice", "30", "NYC"],
    vec!["Bob", "25", "LA"],
];

let table = table_from_data(&data, None)
    .position(inches(1.0), inches(1.5))
    .build();
```

With custom column widths:

```rust
let table = table_from_data(&data, Some(vec![2.5, 1.5, 2.0]))
    .position(inches(1.0), inches(1.5))
    .build();
```

### Table with Header

Create a table with an auto-styled header row:

```rust
table_with_header(&["Name", "Role", "Status"], 3)
    .position(inches(1.0), inches(1.5))
    .build()
```

### QuickTable Builder

The most flexible way to create tables:

```rust
QuickTable::new(4)
    .header(&["Name", "Role", "Department", "Status"])
    .row(&["Alice", "Engineer", "Product", "Active"])
    .row(&["Bob", "Designer", "UX", "Active"])
    .at(1.0, 1.5)
    .build()
```

With custom column widths:

```rust
QuickTable::with_widths(&[2.0, 1.5, 1.5, 2.0])
    .header(&["Task", "Priority", "Status", "Owner"])
    .row(&["API Work", "High", "Done", "Team A"])
    .at(0.8, 1.5)
    .build()
```

### Styled Table Cells

Create custom styled cells:

```rust
QuickTable::new(3)
    .styled_row(vec![
        header_cell("Name"),
        header_cell("Status"),
        header_cell("Priority"),
    ])
    .styled_row(vec![
        cell("Task 1"),
        highlight_cell("Done", &material_green().to_hex()),
        highlight_cell("High", &material_red().to_hex()),
    ])
    .at(1.0, 1.5)
    .build()
```

### Cell Helpers

```rust
cell("text")                          // Basic cell
header_cell("Header")                 // Auto-styled header
highlight_cell("Text", "#FF0000")     // Highlighted cell
```

---

## Extension Methods

Shorter, more intuitive method names for common operations.

### Shape Extensions

Instead of `.with_fill()`, `.with_line()`, `.with_text()`:

```rust
// Old API
shape.with_fill(ShapeFill::new("4F81BD"))
     .with_line(ShapeLine::new("000000", 25400))
     .with_text("Hello")

// New API
shape.fill(hex("4F81BD"))
     .stroke(hex("000000"), 2.0)
     .text("Hello")
```

### Available Extensions

- `.fill(color)` - Set fill color
- `.stroke(color, width)` - Set border/stroke
- `.text(text)` - Set text content

---

## Complete Examples

### Example 1: Colorful Dashboard

```rust
use ppt_rs::prelude::*;

fn main() -> Result<()> {
    let slides = vec![
        SlideContent::new("Q1 Dashboard")
            .add_shape(
                rect(0.5, 1.5, 2.0, 1.2)
                    .fill(material_blue().to_color())
                    .text("Revenue\n$2.1M")
            )
            .add_shape(
                rect(3.0, 1.5, 2.0, 1.2)
                    .fill(material_green().to_color())
                    .text("Customers\n12,450")
            )
            .add_shape(
                rect(5.5, 1.5, 2.0, 1.2)
                    .fill(material_orange().to_color())
                    .text("NPS Score\n72")
            ),
    ];
    
    let pptx = create_pptx_with_content("Dashboard", slides)?;
    std::fs::write("dashboard.pptx", pptx)?;
    Ok(())
}
```

### Example 2: Employee Directory Table

```rust
use ppt_rs::prelude::*;

fn main() -> Result<()> {
    let slides = vec![
        SlideContent::new("Employee Directory")
            .table(
                QuickTable::new(4)
                    .header(&["Name", "Role", "Department", "Status"])
                    .row(&["Alice Johnson", "Engineer", "Product", "Active"])
                    .row(&["Bob Smith", "Designer", "UX", "Active"])
                    .row(&["Carol White", "Manager", "Operations", "On Leave"])
                    .row(&["David Brown", "Analyst", "Finance", "Active"])
                    .at(0.5, 1.5)
                    .build()
            ),
    ];
    
    let pptx = create_pptx_with_content("Directory", slides)?;
    std::fs::write("directory.pptx", pptx)?;
    Ok(())
}
```

### Example 3: Color Adjustments

```rust
use ppt_rs::prelude::*;

fn main() -> Result<()> {
    let base_color = material_blue();
    
    let slides = vec![
        SlideContent::new("Color Variations")
            .add_shape(
                rect(1.0, 1.5, 1.5, 0.8)
                    .fill(base_color.darker(0.4).to_color())
                    .text("Darker 40%")
            )
            .add_shape(
                rect(3.0, 1.5, 1.5, 0.8)
                    .fill(base_color.to_color())
                    .text("Base Color")
            )
            .add_shape(
                rect(5.0, 1.5, 1.5, 0.8)
                    .fill(base_color.lighter(0.4).to_color())
                    .text("Lighter 40%")
            ),
    ];
    
    let pptx = create_pptx_with_content("Colors", slides)?;
    std::fs::write("colors.pptx", pptx)?;
    Ok(())
}
```

### Example 4: Task Tracker with Styled Cells

```rust
use ppt_rs::prelude::*;

fn main() -> Result<()> {
    let slides = vec![
        SlideContent::new("Task Tracker")
            .table(
                QuickTable::with_widths(&[2.5, 1.5, 1.5, 1.5])
                    .styled_row(vec![
                        header_cell("Task"),
                        header_cell("Priority"),
                        header_cell("Status"),
                        header_cell("Owner"),
                    ])
                    .styled_row(vec![
                        cell("API Simplification"),
                        highlight_cell("High", &material_red().to_hex()),
                        highlight_cell("Done", &material_green().to_hex()),
                        cell("Team A"),
                    ])
                    .styled_row(vec![
                        cell("Documentation"),
                        highlight_cell("Medium", &material_orange().to_hex()),
                        cell("In Progress"),
                        cell("Team B"),
                    ])
                    .at(0.5, 1.5)
                    .build()
            ),
    ];
    
    let pptx = create_pptx_with_content("Tasks", slides)?;
    std::fs::write("tasks.pptx", pptx)?;
    Ok(())
}
```

---

## API Comparison

### Before (Verbose)

```rust
use ppt_rs::generator::{Shape, ShapeType, ShapeFill, ShapeLine};

let shape = Shape::new(ShapeType::Rectangle, 914400, 1828800, 2743200, 1371600)
    .with_fill(ShapeFill::new("4F81BD"))
    .with_line(ShapeLine::new("000000", 25400))
    .with_text("Hello");
```

### After (Simplified)

```rust
use ppt_rs::prelude::*;

let shape = rect(1.0, 2.0, 3.0, 1.5)
    .fill(blue().to_color())
    .stroke(black().to_color(), 2.0)
    .text("Hello");
```

**Result: ~60% less code, dramatically more readable!**

---

## Tips & Best Practices

1. **Use color aliases** - `red()` is clearer than `hex("FF0000")`
2. **Adjust colors** - Use `.lighter()` and `.darker()` for color variations
3. **QuickTable for flexibility** - Most powerful table creation method
4. **Inches for dimensions** - More intuitive than EMUs
5. **Chain methods** - Use fluent API for cleaner code

---

## See Also

- [Examples](examples/) - Working code examples
- [API Reference](https://docs.rs/ppt-rs) - Complete API documentation
- [SPEC.md](SPEC.md) - Technical specifications
