# ppt-rs API Guide

Complete guide to the simplified ppt-rs API for creating PowerPoint presentations.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Presentation Builder](#presentation-builder)
3. [HTML to PowerPoint](#html-to-powerpoint)
4. [Shape Helpers](#shape-helpers)
5. [Color API](#color-api)
6. [Table API](#table-api)
7. [Charts](#charts)
8. [Images](#images)
9. [Themes & Templates](#themes--templates)
10. [Export & Compression](#export--compression)
11. [Editing Existing Decks](#editing-existing-decks)
12. [Package Validation](#package-validation)
13. [Extension Methods](#extension-methods)
14. [Complete Examples](#complete-examples)
15. [API Comparison](#api-comparison)

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

## Presentation Builder

High-level `Presentation` API for create → save → export workflows:

```rust
use ppt_rs::prelude::*;
use ppt_rs::{Presentation, PresentationTheme};

fn main() -> Result<()> {
    Presentation::with_title("Q1 Review")
        .with_theme(PresentationTheme::corporate())
        .add_slide(
            SlideContent::new("Highlights")
                .add_bullet("Revenue up 15%")
                .add_bullet("NPS at 72")
        )
        .add_slide(
            SlideContent::new("Metrics")
                .layout(SlideLayout::TitleOnly)
                .add_shape(
                    rect(1.0, 2.0, 3.0, 1.5)
                        .fill(hex("1565C0"))
                        .text("KPI")
                )
        )
        .save("q1.pptx")?;
    Ok(())
}
```

Useful methods:

| Method | Purpose |
|--------|---------|
| `Presentation::with_title` / `add_slide` | Build deck |
| `.with_theme(PresentationTheme::…)` | Embed brand theme |
| `.save(path)` | Write PPTX |
| `.from_path(path)` | Open existing PPTX |
| `.save_as_markdown` / `.save_as_html` | Round-trip export |
| `.compress(path, &options)` | Shrink file size |

---

## HTML to PowerPoint

Convert HTML content directly into PowerPoint slides using the `html2ppt` API. No external dependencies required.

### Quick Parse

The simplest way: pass an HTML string to `parse_html()`:

```rust
use ppt_rs::generator::create_pptx_with_content;
use ppt_rs::import::parse_html;

let html = r#"
    <h1>My Presentation</h1>
    <p>This is the first slide.</p>
    <ul>
        <li>Bullet 1</li>
        <li>Bullet 2</li>
    </ul>
    <h1>Data Slide</h1>
    <table>
        <tr><th>Name</th><th>Value</th></tr>
        <tr><td>Alpha</td><td>100</td></tr>
        <tr><td>Beta</td><td>200</td></tr>
    </table>
    <blockquote>Speaker notes for this slide</blockquote>
"#;

let slides = parse_html(html)?;
let pptx = create_pptx_with_content("From HTML", slides)?;
std::fs::write("output.pptx", pptx)?;
```

### Parse with Options

Control parsing behavior with `HtmlParseOptions`:

```rust
use ppt_rs::import::{Html2Ppt, HtmlParseOptions};

let options = HtmlParseOptions::new()
    .max_slides(20)         // limit to 20 slides
    .max_bullets(8)         // max 8 bullets per slide
    .include_code(true)     // include code blocks
    .include_tables(true)   // include tables
    .include_images(false); // skip image placeholders

let slides = Html2Ppt::with_options(options).parse(html)?;
```

### Parse from File

```rust
use ppt_rs::import::Html2Ppt;

let slides = Html2Ppt::new().parse_file("presentation.html")?;
```

### HTML Element Reference

| HTML Element | Creates |
|--------------|---------|
| `<h1>` | New slide with title |
| `<h2>`–`<h6>` | Bold section headers |
| `<p>` | Bullet points |
| `<ul>` / `<ol>` | List items |
| `<table>` | Table with styled header |
| `<pre>` / `<code>` | Code blocks |
| `<img>` | Image placeholder |
| `<blockquote>` | Speaker notes |
| `<hr>` | Slide break |

### CLI Usage

```bash
# Convert HTML file to PPTX
pptcli html2ppt slides.html presentation.pptx

# With custom title
pptcli html2ppt slides.html --title "My Talk"

# Control content extraction
pptcli html2ppt slides.html --no-tables --no-code --max-slides 30
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

## Charts

Charts embed an Excel workbook so values stay editable in PowerPoint.

```rust
use ppt_rs::generator::{ChartSeries, SlideContent, create_pptx_with_content};
use ppt_rs::helpers::bar_chart;

let chart = bar_chart("Quarterly Revenue")
    .categories(vec!["Q1", "Q2", "Q3", "Q4"])
    .add_series(ChartSeries::new("2025", vec![120.0, 145.0, 160.0, 190.0]))
    .add_series(ChartSeries::new("2024", vec![100.0, 110.0, 130.0, 150.0]))
    .build();

let slides = vec![
    SlideContent::new("Revenue")
        .layout(SlideLayout::TitleOnly)
        .add_chart(chart),
];
let pptx = create_pptx_with_content("Charts", slides)?;
```

Helpers: `bar_chart`, `line_chart`, `pie_chart`, `area_chart` (`ppt_rs::helpers`).  
Or use `ChartBuilder::new(title, ChartType::…)` for the full type set.

---

## Images

```rust
use ppt_rs::prelude::*;
use ppt_rs::generator::ImageBuilder;

// From bytes / file, with effects and accessibility alt text
let photo = std::fs::read("photo.jpg")?;
let img = ImageBuilder::auto(photo)
    .at(inches(1.0), inches(1.5))
    .size(inches(4.0), inches(3.0))
    .shadow()
    .build()
    .with_alt_text("Team photo at kickoff");

let slides = vec![SlideContent::new("Gallery").add_image(img)];
```

Prelude shortcuts: `image(bytes)`, `image_file(path)`.  
Effects on `ImageBuilder`: `.shadow()`, `.reflection()`, `.glow()`, `.soft_edges()`, `.blur()`, `.crop(…)`.

---

## Themes & Templates

### Embedded themes

```rust
use ppt_rs::{Presentation, PresentationTheme};

Presentation::with_title("Brand Deck")
    .with_theme(PresentationTheme::modern()) // corporate, vibrant, dark, nature, tech, carbon
    .add_slide(SlideContent::new("Hello"))
    .save("branded.pptx")?;
```

### Clone from an existing deck

```rust
use ppt_rs::generator::{create_pptx_with_template, SlideContent};
use ppt_rs::SlideLayout;

let slides = vec![
    SlideContent::new("Cover").with_layout(SlideLayout::CenteredTitle),
    SlideContent::new("Agenda").with_layout(SlideLayout::TitleAndContent),
];
let pptx = create_pptx_with_template("Review", &slides, "brand.pptx", None)?;
```

CLI: `pptcli create out.pptx --template brand.pptx --title "Review"`

Layouts (7 on slide master 1): `CenteredTitle`, `TitleAndContent`, `TitleOnly`, `TwoColumn`, `Blank`, and related variants via `SlideLayout`.

---

## Export & Compression

```rust
use ppt_rs::{Presentation, CompressionOptions, MarkdownOptions};

let pres = Presentation::from_path("deck.pptx")?;

// Markdown / HTML
pres.save_as_markdown("deck.md")?;
pres.save_as_markdown_with_options("deck.md", &MarkdownOptions::default())?;
pres.save_as_html("deck.html")?;

// Compress (web preset targets ~5MB)
pres.compress("deck-web.pptx", &CompressionOptions::web())?;
```

Image export (PNG/JPEG) and PDF require LibreOffice (`soffice`).

---

## Editing Existing Decks

```rust
use ppt_rs::oxml::PresentationEditor;
use ppt_rs::generator::SlideContent;

let mut editor = PresentationEditor::open("deck.pptx")?;
editor.duplicate_slide(0)?;           // copy first slide after itself
editor.insert_slide(1, SlideContent::new("Inserted"))?;
editor.reorder_slide(2, 0)?;          // move slide to front
editor.save("deck-edited.pptx")?;
```

---

## Package Validation

Structured checks so decks open in PowerPoint without a repair prompt:

```rust
use ppt_rs::core::{validate_package_bytes, ValidationSeverity};

let bytes = std::fs::read("deck.pptx")?;
let report = validate_package_bytes(&bytes);

if !report.is_valid() {
    for issue in &report.issues {
        if issue.severity == ValidationSeverity::Error {
            eprintln!("{:?}: {}", issue.category, issue.message);
        }
    }
}
```

CLI: `pptcli validate deck.pptx`  
JSON: `pptcli validate deck.pptx --json`  
Debug builds also `debug_assert!` every generated package.

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
4. **Inches for dimensions** - More intuitive than EMUs; use `Dimension::Ratio` / `percent` for responsive layouts
5. **Chain methods** - Use fluent API for cleaner code
6. **Validate before ship** - `validate_package_bytes` or `pptcli validate` catches packaging gaps
7. **Theme once** - Prefer `PresentationTheme` / `--template` over hand-styling every shape

---

## See Also

- [Examples](examples/) - Working code (`simplified_api`, `color_and_table_demo`, `dimension_demo`, `theme_customization_test`)
- [examples/README.md](examples/README.md) - Example index
- [API Reference](https://docs.rs/ppt-rs) - Complete rustdoc
- [SPEC.md](SPEC.md) - Technical specifications
- [ARCHITECTURE.md](ARCHITECTURE.md) - Module layout
- [docs/index.html](docs/index.html) - Docs landing page
