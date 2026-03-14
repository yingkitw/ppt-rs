# Migration Guide: New Simplified API

This guide helps you migrate from the old verbose API to the new simplified API introduced in v0.2.11.

## Overview

The new API reduces boilerplate by ~60% while maintaining full backward compatibility. **Your existing code will continue to work** - this guide shows you how to make it better.

---

## Quick Reference

### Colors

**Before:**
```rust
use ppt_rs::generator::{ShapeFill, ShapeLine};

.with_fill(ShapeFill::new("4F81BD"))
.with_line(ShapeLine::new("FF0000", 25400))
```

**After:**
```rust
use ppt_rs::prelude::*;

.fill(blue().to_color())
.stroke(red().to_color(), 2.0)
```

### Shapes

**Before:**
```rust
use ppt_rs::generator::{Shape, ShapeType};

let shape = Shape::new(ShapeType::Rectangle, 914400, 1828800, 2743200, 1371600)
    .with_fill(ShapeFill::new("4F81BD"))
    .with_text("Hello");
```

**After:**
```rust
use ppt_rs::prelude::*;

let shape = rect(1.0, 2.0, 3.0, 1.5)
    .fill(blue().to_color())
    .text("Hello");
```

### Tables

**Before:**
```rust
use ppt_rs::generator::{TableBuilder, TableRow, TableCell};

let mut builder = TableBuilder::new(vec![2000000, 2000000, 2000000]);
builder = builder.add_row(TableRow::new(vec![
    TableCell::new("Header 1").bold().background_color("1F4E79").text_color("FFFFFF"),
    TableCell::new("Header 2").bold().background_color("1F4E79").text_color("FFFFFF"),
    TableCell::new("Header 3").bold().background_color("1F4E79").text_color("FFFFFF"),
]));
let table = builder.build();
```

**After:**
```rust
use ppt_rs::prelude::*;

let table = QuickTable::new(3)
    .header(&["Header 1", "Header 2", "Header 3"])
    .row(&["Data 1", "Data 2", "Data 3"])
    .at(1.0, 1.5)
    .build();
```

---

## Detailed Migration

### 1. Update Imports

**Before:**
```rust
use ppt_rs::generator::{
    Shape, ShapeType, ShapeFill, ShapeLine,
    TableBuilder, TableRow, TableCell,
};
use ppt_rs::elements::{Color, RgbColor};
```

**After:**
```rust
use ppt_rs::prelude::*;
```

The prelude includes all commonly used types and the new helper functions.

### 2. Color Migration

#### Basic Colors

**Before:**
```rust
.with_fill(ShapeFill::new("FF0000"))  // Red
.with_fill(ShapeFill::new("0000FF"))  // Blue
.with_fill(ShapeFill::new("00FF00"))  // Green
```

**After:**
```rust
.fill(red().to_color())
.fill(blue().to_color())
.fill(green().to_color())
```

#### Color Adjustments

**Before:**
```rust
// Manual RGB calculation for lighter/darker colors
let r = (79 as f32 * 1.3).min(255.0) as u8;
let g = (129 as f32 * 1.3).min(255.0) as u8;
let b = (189 as f32 * 1.3).min(255.0) as u8;
.with_fill(ShapeFill::new(&format!("{:02X}{:02X}{:02X}", r, g, b)))
```

**After:**
```rust
.fill(blue().lighter(0.3).to_color())
.fill(blue().darker(0.3).to_color())
```

#### Material Design Colors

**Before:**
```rust
use ppt_rs::prelude::colors;
.with_fill(ShapeFill::new(colors::MATERIAL_BLUE))
```

**After:**
```rust
.fill(material_blue().to_color())
```

### 3. Shape Migration

#### Rectangle

**Before:**
```rust
let rect = Shape::new(ShapeType::Rectangle, 914400, 1828800, 2743200, 1371600);
```

**After:**
```rust
let rect = rect(1.0, 2.0, 3.0, 1.5);  // All in inches
```

#### Circle

**Before:**
```rust
let circle = Shape::new(ShapeType::Ellipse, 914400, 1828800, 1371600, 1371600);
```

**After:**
```rust
let circle = circle(1.0, 2.0, 1.5);  // x, y, diameter
```

#### Other Shapes

**Before:**
```rust
Shape::new(ShapeType::RoundedRectangle, x, y, w, h)
Shape::new(ShapeType::Triangle, x, y, w, h)
Shape::new(ShapeType::Diamond, x, y, w, h)
```

**After:**
```rust
rounded_rect(x, y, w, h)
triangle(x, y, w, h)
diamond(x, y, w, h)
```

### 4. Extension Methods

**Before:**
```rust
shape
    .with_fill(ShapeFill::new("4F81BD"))
    .with_line(ShapeLine::new("000000", 25400))
    .with_text("Hello World")
```

**After:**
```rust
shape
    .fill(blue().to_color())
    .stroke(black().to_color(), 2.0)
    .text("Hello World")
```

### 5. Table Migration

#### Simple Table

**Before:**
```rust
let col_width = 9144000 / 3;
let mut builder = TableBuilder::new(vec![col_width, col_width, col_width]);
for _ in 0..4 {
    builder = builder.add_row(TableRow::new(vec![
        TableCell::new(""),
        TableCell::new(""),
        TableCell::new(""),
    ]));
}
let table = builder.position(914400, 1371600).build();
```

**After:**
```rust
let table = simple_table(4, 3)
    .position(inches(1.0), inches(1.5))
    .build();
```

#### Table with Header

**Before:**
```rust
let mut builder = TableBuilder::new(vec![2000000, 2000000, 2000000]);
builder = builder.add_row(TableRow::new(vec![
    TableCell::new("Name").bold().background_color("1F4E79").text_color("FFFFFF"),
    TableCell::new("Age").bold().background_color("1F4E79").text_color("FFFFFF"),
    TableCell::new("City").bold().background_color("1F4E79").text_color("FFFFFF"),
]));
builder = builder.add_row(TableRow::new(vec![
    TableCell::new("Alice"),
    TableCell::new("30"),
    TableCell::new("NYC"),
]));
let table = builder.build();
```

**After:**
```rust
let table = QuickTable::new(3)
    .header(&["Name", "Age", "City"])
    .row(&["Alice", "30", "NYC"])
    .at(1.0, 1.5)
    .build();
```

#### Table from Data

**Before:**
```rust
let data = vec![
    vec!["Name", "Age", "City"],
    vec!["Alice", "30", "NYC"],
    vec!["Bob", "25", "LA"],
];
let mut builder = TableBuilder::new(vec![2000000, 2000000, 2000000]);
for row_data in data {
    let cells: Vec<TableCell> = row_data.iter()
        .map(|&text| TableCell::new(text))
        .collect();
    builder = builder.add_row(TableRow::new(cells));
}
let table = builder.build();
```

**After:**
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

---

## Color Reference

### Available Color Aliases

```rust
// Basic colors
red(), green(), blue(), yellow(), cyan(), magenta()
white(), black(), gray(), orange(), purple(), pink(), brown()

// Shades
light_gray(), dark_gray(), silver()

// Web colors
navy(), teal(), olive(), maroon(), lime(), aqua()

// Material Design
material_red(), material_blue(), material_green()
material_orange(), material_purple(), material_pink()
material_teal(), material_amber(), material_brown()
material_cyan(), material_lime(), material_indigo()

// Corporate
corporate_blue(), corporate_green()
corporate_red(), corporate_orange()
```

### Color Operations

```rust
// Adjustments
color.lighter(0.3)      // 30% lighter
color.darker(0.5)       // 50% darker
color.opacity(0.7)      // 70% opacity
color.transparent(25)   // 25% transparent

// Operations
color1.mix(&color2, 0.5)  // 50/50 mix
color.grayscale()         // Convert to grayscale
color.invert()            // Invert colors

// Creation
ColorValue::rgb(255, 128, 64)
ColorValue::from_hex("#FF8040")
```

---

## Table Reference

### Creation Methods

```rust
// Simple table with equal columns
simple_table(rows, cols)

// Custom column widths (in inches)
table_with_widths(&[2.0, 3.0, 2.5])

// From 2D data array
table_from_data(&data, None)
table_from_data(&data, Some(vec![2.0, 1.5, 2.0]))

// With auto-styled header
table_with_header(&["Col1", "Col2", "Col3"], data_rows)

// QuickTable builder (most flexible)
QuickTable::new(cols)
QuickTable::with_widths(&[2.0, 1.5, 2.0])
```

### QuickTable Methods

```rust
QuickTable::new(4)
    .header(&["Name", "Age", "City", "Status"])
    .row(&["Alice", "30", "NYC", "Active"])
    .rows(&[
        vec!["Bob", "25", "LA", "Active"],
        vec!["Carol", "35", "SF", "Away"],
    ])
    .styled_row(vec![
        cell("Custom"),
        header_cell("Styled"),
        highlight_cell("Cell", "#FF0000"),
    ])
    .at(1.0, 1.5)
    .build()
```

### Cell Helpers

```rust
cell("text")                          // Basic cell
header_cell("Header")                 // Auto-styled header (bold, blue bg, white text)
highlight_cell("Text", "#FF0000")     // Highlighted cell with custom color
```

---

## Complete Example Migration

### Before (Old API)

```rust
use ppt_rs::generator::{
    create_pptx_with_content, SlideContent,
    Shape, ShapeType, ShapeFill, ShapeLine,
    TableBuilder, TableRow, TableCell,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut slides = Vec::new();
    
    // Slide 1: Shapes
    let rect = Shape::new(ShapeType::Rectangle, 914400, 1828800, 2743200, 1371600)
        .with_fill(ShapeFill::new("4F81BD"))
        .with_line(ShapeLine::new("000000", 25400))
        .with_text("Rectangle");
    
    slides.push(
        SlideContent::new("Shapes")
            .add_shape(rect)
    );
    
    // Slide 2: Table
    let mut builder = TableBuilder::new(vec![2000000, 2000000, 2000000]);
    builder = builder.add_row(TableRow::new(vec![
        TableCell::new("Name").bold().background_color("1F4E79").text_color("FFFFFF"),
        TableCell::new("Age").bold().background_color("1F4E79").text_color("FFFFFF"),
        TableCell::new("City").bold().background_color("1F4E79").text_color("FFFFFF"),
    ]));
    builder = builder.add_row(TableRow::new(vec![
        TableCell::new("Alice"),
        TableCell::new("30"),
        TableCell::new("NYC"),
    ]));
    
    slides.push(
        SlideContent::new("Table")
            .table(builder.build())
    );
    
    let pptx = create_pptx_with_content("My Presentation", slides)?;
    std::fs::write("output.pptx", pptx)?;
    Ok(())
}
```

### After (New API)

```rust
use ppt_rs::prelude::*;

fn main() -> Result<()> {
    let slides = vec![
        // Slide 1: Shapes
        SlideContent::new("Shapes")
            .add_shape(
                rect(1.0, 2.0, 3.0, 1.5)
                    .fill(blue().to_color())
                    .stroke(black().to_color(), 2.0)
                    .text("Rectangle")
            ),
        
        // Slide 2: Table
        SlideContent::new("Table")
            .table(
                QuickTable::new(3)
                    .header(&["Name", "Age", "City"])
                    .row(&["Alice", "30", "NYC"])
                    .at(1.0, 1.5)
                    .build()
            ),
    ];
    
    let pptx = create_pptx_with_content("My Presentation", slides)?;
    std::fs::write("output.pptx", pptx)?;
    Ok(())
}
```

**Code reduction: 60% fewer lines!**

---

## Backward Compatibility

**Important:** The old API is still fully supported. You can:

1. Keep using the old API
2. Mix old and new APIs in the same file
3. Migrate gradually at your own pace

```rust
// This still works!
let old_shape = Shape::new(ShapeType::Rectangle, 914400, 1828800, 2743200, 1371600)
    .with_fill(ShapeFill::new("4F81BD"));

// And you can use it alongside new API
let new_shape = rect(1.0, 2.0, 3.0, 1.5)
    .fill(blue().to_color());
```

---

## Benefits Summary

✅ **~60% less code** - Dramatically reduced boilerplate  
✅ **More readable** - Color names instead of hex codes  
✅ **Type safe** - Full Rust compile-time checking  
✅ **Backward compatible** - Old code still works  
✅ **Flexible** - Color adjustments and mixing  
✅ **Intuitive** - Follows common naming conventions  

---

## Need Help?

- See [API_GUIDE.md](API_GUIDE.md) for complete API reference
- Check [examples/](examples/) for working code examples
- Run `cargo run --example color_and_table_demo` to see the new API in action

---

## Feedback

Found an issue or have a suggestion? Please open an issue on GitHub!
