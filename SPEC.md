# PPTX-RS Specification

## Overview

`ppt-rs` is a Rust library for generating, reading, and modifying Microsoft PowerPoint 2007+ (.pptx) files following the ECMA-376 Office Open XML standard.

## File Format Specification

### PPTX Structure

A PPTX file is a ZIP archive containing XML and binary files:

```
presentation.pptx
├── [Content_Types].xml      # MIME type declarations
├── _rels/
│   └── .rels               # Package relationships
├── ppt/
│   ├── presentation.xml    # Main presentation
│   ├── _rels/
│   │   └── presentation.xml.rels
│   ├── slides/             # Individual slides
│   ├── slideLayouts/       # Layout templates
│   ├── slideMasters/       # Master slides
│   ├── theme/              # Theme definitions
│   ├── media/              # Images, videos, audio
│   ├── charts/             # Embedded charts
│   └── notesSlides/        # Speaker notes
└── docProps/
    ├── core.xml            # Document properties
    └── app.xml             # Application properties
```

### XML Namespaces

| Prefix | Namespace URI | Usage |
|--------|--------------|-------|
| `a` | `http://schemas.openxmlformats.org/drawingml/2006/main` | Drawing ML |
| `p` | `http://schemas.openxmlformats.org/presentationml/2006/main` | Presentation ML |
| `r` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships` | Relationships |
| `c` | `http://schemas.openxmlformats.org/drawingml/2006/chart` | Charts |

### Units

The library uses EMU (English Metric Units) internally:

| Unit | EMU Value |
|------|-----------|
| 1 inch | 914,400 EMU |
| 1 cm | 360,000 EMU |
| 1 point | 12,700 EMU |
| 1 mm | 36,000 EMU |

## API Specification

### Prelude Module

The prelude provides a simplified API:

```rust
use ppt_rs::prelude::*;

// Macros
pptx!("Title")                    // Create QuickPptx builder
shape!(rect x, y, w, h)           // Create rectangle shape
shape!(circle x, y, size)         // Create circle shape

// Unit Conversions
inches(1.0) -> 914400             // Convert inches to EMU
cm(2.54) -> 914400                // Convert cm to EMU
pt(72.0) -> 914400                // Convert points to EMU

// Shape Builders
shapes::rect(x, y, w, h)          // Rectangle (inches)
shapes::circle(x, y, d)           // Circle (inches)
shapes::rounded_rect(x, y, w, h)  // Rounded rectangle
shapes::text_box(x, y, w, h, text)// Text box
shapes::colored(shape, fill, line)// Apply colors
shapes::gradient(shape, start, end, dir)// Apply gradient

// Color Constants
colors::RED, colors::BLUE, colors::GREEN
colors::CORPORATE_BLUE, colors::CORPORATE_GREEN
```

### Core Types

#### SlideContent

```rust
SlideContent::new("Title")
    .add_bullet("Text")           // Add bullet point
    .add_notes("Speaker notes")   // Add speaker notes
    .layout(SlideLayout::TwoColumn) // Set layout
    .table(table)                 // Add table
    .with_shapes(shapes)          // Add shapes
    .with_image(image)            // Add image
    .with_chart(chart)            // Add chart
```

#### SlideLayout

| Layout | Description |
|--------|-------------|
| `TitleOnly` | Title at top only |
| `CenteredTitle` | Centered title |
| `TitleAndContent` | Title with bullets (default) |
| `TitleAndBigContent` | Title with large content |
| `TwoColumn` | Two-column layout |
| `Blank` | Empty slide |

#### Shape

```rust
Shape::new(ShapeType::Rectangle, x, y, width, height)
    .with_fill(ShapeFill::new("FF0000"))
    .with_line(ShapeLine::new("000000", 12700))
    .with_text("Text")
    .with_gradient(GradientFill::linear("start", "end", direction))
    .with_transparency(50)
    .with_id(id)                  // Fixed ID for connectors
```

#### ShapeType (100+ types)

| Category | Types |
|----------|-------|
| Basic | Rectangle, Circle, Ellipse, Triangle, Diamond |
| Arrows | ArrowRight, ArrowLeft, ArrowUp, ArrowDown, etc. |
| Stars | Star4, Star5, Star6, Star8, Star12, Star16, Star24, Star32 |
| Callouts | RoundedRectCallout, WedgeCallout, CloudCallout |
| Flowchart | Process, Decision, Data, Document, Terminator |
| Other | Heart, Lightning, Moon, Sun, Cloud |

#### Connector

```rust
Connector::straight(x1, y1, x2, y2)
Connector::elbow(x1, y1, x2, y2)
Connector::curved(x1, y1, x2, y2)
    .with_line(ConnectorLine::new("color", width))
    .with_end_arrow(ArrowType::Triangle)
    .with_start_arrow(ArrowType::Oval)
    .with_arrow_size(ArrowSize::Large)
    .anchored_to(start_shape_id, end_shape_id)
```

#### Table

```rust
TableBuilder::new(vec![col_widths])
    .add_simple_row(vec!["Cell 1", "Cell 2"])
    .add_row(TableRow::new(vec![
        TableCell::new("Text")
            .bold()
            .italic()
            .text_color("FFFFFF")
            .background_color("4472C4")
            .align(CellAlign::Center)
            .valign(CellVAlign::Middle)
    ]))
    .position(x, y)
    .build()
```

#### Chart

```rust
ChartBuilder::new("Title", ChartType::Bar)
    .categories(vec!["Q1", "Q2", "Q3"])
    .add_series(ChartSeries::new("2023", vec![100.0, 150.0, 120.0]))
    .build()
```

| Chart Type | Description |
|------------|-------------|
| `Bar` | Vertical bar chart |
| `BarClustered` | Clustered bar chart |
| `BarStacked` | Stacked bar chart |
| `Line` | Line chart |
| `LineSmooth` | Smooth line chart |
| `Pie` | Pie chart |
| `Pie3D` | 3D pie chart |
| `Doughnut` | Doughnut chart |
| `Area` | Area chart |
| `AreaStacked` | Stacked area chart |
| `Scatter` | Scatter plot |
| `Radar` | Radar chart |
| `Bubble` | Bubble chart |

#### GradientFill

```rust
GradientFill::linear("start_color", "end_color", GradientDirection::Horizontal)
GradientFill::three_color("start", "middle", "end", direction)
GradientFill::custom(vec![
    GradientStop::new("color1", 0, 0),    // position 0%, transparency 0%
    GradientStop::new("color2", 50, 25),  // position 50%, transparency 25%
    GradientStop::new("color3", 100, 0),  // position 100%, transparency 0%
], angle)
```

| Direction | Angle |
|-----------|-------|
| Horizontal | 0° |
| Vertical | 90° |
| DiagonalDown | 45° |
| DiagonalUp | 135° |
| Angle(n) | n° |

### Markdown to PPTX

The CLI converts Markdown to PPTX:

```bash
pptcli md2ppt input.md [output.pptx] [--title "Title"]
```

#### Supported Markdown Syntax

| Syntax | Result |
|--------|--------|
| `# Heading` | New slide with title |
| `## Subheading` | Bold bullet point |
| `- Bullet` | Bullet point |
| `1. Item` | Numbered list |
| `**bold**` | Bold text |
| `*italic*` | Italic text |
| `` `code` `` | Inline code |
| `> Quote` | Speaker notes |
| `\| Table \|` | GFM table |
| ` ```code``` ` | Syntax-highlighted code |
| ` ```mermaid ` | Mermaid diagram |
| `---` | Slide break |

#### Syntax Highlighting

Code blocks use Solarized Dark theme:

| Element | Color |
|---------|-------|
| Keywords | Blue (#268BD2) |
| Functions | Yellow (#B58900) |
| Strings | Cyan (#2AA198) |
| Operators | Green (#859900) |
| Numbers | Violet (#6C71C4) |
| Comments | Gray (#586E75) |

#### Mermaid Diagrams (12 types)

| Type | Description |
|------|-------------|
| `flowchart` | Flowchart with nodes and edges |
| `sequenceDiagram` | Sequence diagram with participants |
| `pie` | Pie chart |
| `gantt` | Gantt chart with tasks |
| `classDiagram` | UML class diagram |
| `stateDiagram` | State machine diagram |
| `erDiagram` | Entity-relationship diagram |
| `mindmap` | Mind map |
| `timeline` | Timeline |
| `journey` | User journey map |
| `quadrantChart` | Quadrant chart |
| `gitGraph` | Git commit graph |

### Repair Capability

```rust
use ppt_rs::PptxRepair;

let mut repair = PptxRepair::open("file.pptx")?;
let issues = repair.validate();
let result = repair.repair();
if result.is_valid {
    repair.save("repaired.pptx")?;
}
```

#### Detectable Issues

- Missing required parts
- Invalid XML structure
- Broken relationships
- Orphan slides
- Invalid content types

### Reading & Editing

```rust
use ppt_rs::oxml::{PresentationReader, PresentationEditor};

// Read presentation info
let reader = PresentationReader::open("file.pptx")?;
let info = reader.presentation_info()?;
println!("Slides: {}", info.slide_count);

// Edit presentation
let mut editor = PresentationEditor::open("file.pptx")?;
editor.add_slide(SlideContent::new("New Slide"))?;
editor.update_slide(0, SlideContent::new("Updated"))?;
editor.remove_slide(1)?;
editor.save("modified.pptx")?;
```

## CLI Commands

```bash
pptcli create <title> [output] [--slides N]      # Create presentation
pptcli md2ppt <input.md> [output.pptx] [--title] # Convert markdown
pptcli validate <file.pptx>                       # Validate PPTX
pptcli info <file.pptx>                           # Show info
pptcli repair <input.pptx> <output.pptx>         # Repair PPTX
```

## Compatibility

| Application | Status |
|-------------|--------|
| Microsoft PowerPoint 2007+ | ✅ Full support |
| LibreOffice Impress | ✅ Full support |
| Google Slides | ✅ Full support |
| Apple Keynote | ✅ Import support |
| WPS Office | ✅ Full support |

## Performance

| Metric | Value |
|--------|-------|
| File size overhead | ~10-15 KB base |
| Generation speed | ~1000 slides/sec |
| Memory usage | ~2 MB + content |
| Test coverage | 650+ tests |

## Error Handling

```rust
use ppt_rs::{PptxError, Result};

// All operations return Result<T, PptxError>
match create_pptx("title", 5) {
    Ok(data) => { /* success */ }
    Err(PptxError::IoError(e)) => { /* I/O error */ }
    Err(PptxError::XmlError(e)) => { /* XML error */ }
    Err(PptxError::ZipError(e)) => { /* ZIP error */ }
    Err(PptxError::InvalidOperation(msg)) => { /* logic error */ }
}
```

## Version History

| Version | Features |
|---------|----------|
| 0.1.8 | Prelude, gradients, transparency, connectors |
| 0.1.7 | 12 Mermaid diagram types |
| 0.1.6 | Syntax highlighting |
| 0.1.5 | Enhanced markdown parsing |
| 0.1.4 | Table text rendering fix |
| 0.1.3 | Modular table module |
| 0.1.2 | Animations, transitions, SmartArt |
| 0.1.1 | Extended parts support |
| 0.1.0 | Initial release |

