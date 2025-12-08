# ppt-rs

**The Rust library for generating PowerPoint presentations that actually works.**

While other Rust crates for PPTX generation are incomplete, broken, or abandoned, `ppt-rs` generates **valid, production-ready PowerPoint files** that open correctly in PowerPoint, LibreOffice, Google Slides, and other Office applications.

**🎯 Convert Markdown to PowerPoint in seconds** - Write your slides in Markdown, get a professional PPTX file. No PowerPoint needed.

## Why ppt-rs?

- 🚀 **Markdown to PPTX** - Write slides in Markdown, get PowerPoint files. Perfect for developers.
- ✅ **Actually works** - Generates valid PPTX files that open in all major presentation software
- ✅ **Complete implementation** - Full ECMA-376 Office Open XML compliance
- ✅ **Type-safe API** - Rust's type system ensures correctness
- ✅ **Simple & intuitive** - Builder pattern with fluent API

## Quick Start

### Markdown to PowerPoint (Recommended)

The easiest way to create presentations: write Markdown, get PowerPoint.

**1. Create a Markdown file:**

```markdown
# Introduction
- Welcome to the presentation
- Today's agenda

# Key Points
- First important point
- Second important point
- Third important point

# Conclusion
- Summary of takeaways
- Next steps
```

**2. Convert to PPTX:**

```bash
# Auto-generates slides.pptx
pptcli md2ppt slides.md

# Or specify output
pptcli md2ppt slides.md presentation.pptx

# With custom title
pptcli md2ppt slides.md --title "My Presentation"
```

That's it! You now have a valid PowerPoint file that opens in PowerPoint, Google Slides, LibreOffice, and more.

### Library (Simple API)

```rust
use ppt_rs::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Quick presentation with macros
    let pptx = pptx!("My Presentation")
        .title_slide("Welcome", "Introduction to ppt-rs")
        .slide("Key Points", vec!["Point 1", "Point 2", "Point 3"])
        .shapes_slide("Shapes", vec![
            shapes::rect(inches(1.0), inches(2.0), inches(2.0), inches(1.0))
                .with_fill(ShapeFill::new(colors::BLUE))
                .with_text("Hello"),
        ])
        .build()?;
    
    std::fs::write("output.pptx", pptx)?;
    Ok(())
}
```

### Library (Full API)

```rust
use ppt_rs::generator::{SlideContent, create_pptx_with_content};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let slides = vec![
        SlideContent::new("Introduction")
            .add_bullet("Welcome")
            .add_bullet("Agenda"),
        SlideContent::new("Key Points")
            .add_bullet("Point 1")
            .add_bullet("Point 2"),
    ];
    
    let pptx = create_pptx_with_content("My Presentation", slides)?;
    std::fs::write("output.pptx", pptx)?;
    Ok(())
}
```

## Features

### Core Capabilities
- **Slides** - Multiple layouts (title-only, two-column, blank, etc.)
- **Text** - Titles, bullets, formatting (bold, italic, colors, sizes)
- **Tables** - Multi-line cells, styling, positioning
- **Shapes** - 100+ shape types with gradient fills and transparency
- **Connectors** - Straight, elbow, curved with arrows and dash styles
- **Charts** - Bar, line, pie charts with multiple series
- **Images** - Embed and position images
- **Reading** - Parse and modify existing PPTX files
- **Repair** - Validate and fix damaged PPTX files

### Markdown Format

The Markdown format supports rich content:

| Syntax | Result |
|--------|--------|
| `# Heading` | New slide with title |
| `## Subheading` | Bold bullet point |
| `- Bullet` | Bullet points (also `*`, `+`) |
| `1. Item` | Numbered list |
| `**bold**` | Bold text |
| `*italic*` | Italic text |
| `` `code` `` | Inline code |
| `> Quote` | Speaker notes |
| `| Table |` | GFM-style tables |
| ` ```code``` ` | Syntax-highlighted code blocks |
| ` ```mermaid ` | Mermaid diagrams (12 types) |
| `---` | Slide break |

**Code Block Syntax Highlighting:**
Code blocks are rendered with Solarized Dark theme colors:
- **Blue** - Keywords (`fn`, `let`, `def`, `class`)
- **Yellow** - Function names
- **Cyan** - Strings
- **Green** - Operators, macros
- **Violet** - Numbers
- **Orange** - Format specifiers

**Example:**
```markdown
# Introduction
- Welcome to the presentation
- **Key point** with emphasis

# Data Table
| Product | Sales |
|---------|-------|
| Widget  | $100  |

> Speaker notes go here

# Code Example
```python
print("Hello!")
```

# Conclusion
- Summary
- Q&A
```

Convert with: `pptcli md2ppt presentation.md` → `presentation.pptx`

## CLI Commands

### Validate PPTX Files

Validate a PPTX file for ECMA-376 compliance:

```bash
pptcli validate presentation.pptx
```

This checks:
- ZIP archive integrity
- Required XML files presence
- XML validity
- Relationships structure

### Show Presentation Information

```bash
pptcli info presentation.pptx
```

### Repair PPTX Files

Repair damaged or corrupted PPTX files:

```rust
use ppt_rs::oxml::repair::PptxRepair;

// Open and validate
let mut repair = PptxRepair::open("damaged.pptx")?;
let issues = repair.validate();

println!("Found {} issues", issues.len());
for issue in &issues {
    println!("  - {} (severity: {})", issue.description(), issue.severity());
}

// Repair and save
let result = repair.repair();
if result.is_valid {
    repair.save("repaired.pptx")?;
    println!("File repaired successfully!");
}
```

**Detectable Issues:**
- Missing required parts (Content_Types.xml, relationships)
- Invalid or malformed XML
- Broken relationship references
- Missing slide references
- Orphan slides
- Invalid content types

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
ppt-rs = "0.1"
```

## Examples

### Tables

```rust
use ppt_rs::generator::{SlideContent, TableBuilder, TableRow, TableCell, create_pptx_with_content};

// Simple table
let table = TableBuilder::new(vec![2000000, 2000000])
    .add_simple_row(vec!["Name", "Status"])
    .add_simple_row(vec!["Alice", "Active"])
    .build();

// Styled table with formatting
let styled_table = TableBuilder::new(vec![2000000, 2000000, 2000000])
    .add_row(TableRow::new(vec![
        TableCell::new("Header 1").bold().background_color("4472C4").text_color("FFFFFF"),
        TableCell::new("Header 2").bold().background_color("4472C4").text_color("FFFFFF"),
        TableCell::new("Header 3").bold().background_color("4472C4").text_color("FFFFFF"),
    ]))
    .add_row(TableRow::new(vec![
        TableCell::new("Data 1"),
        TableCell::new("Data 2").italic(),
        TableCell::new("Data 3").text_color("2E7D32"),
    ]))
    .position(500000, 1500000)
    .build();

let slides = vec![
    SlideContent::new("Data").table(styled_table),
];
```

### Charts

```rust
use ppt_rs::generator::{ChartBuilder, ChartType, ChartSeries};

let chart = ChartBuilder::new("Sales", ChartType::Bar)
    .categories(vec!["Q1", "Q2", "Q3"])
    .add_series(ChartSeries::new("2023", vec![100.0, 150.0, 120.0]))
    .build();
```

### Shapes

```rust
use ppt_rs::generator::{Shape, ShapeType, ShapeFill, ShapeLine};
use ppt_rs::generator::shapes::{GradientFill, GradientDirection};

// Simple shape with solid fill
let shape = Shape::new(ShapeType::Rectangle, 0, 0, 1000000, 500000)
    .with_fill(ShapeFill::new("FF0000"))
    .with_text("Hello");

// Shape with gradient fill
let gradient_shape = Shape::new(ShapeType::RoundedRectangle, 0, 0, 2000000, 1000000)
    .with_gradient(GradientFill::linear("1565C0", "42A5F5", GradientDirection::Horizontal))
    .with_text("Gradient");

// Shape with transparency
let transparent = Shape::new(ShapeType::Ellipse, 0, 0, 1500000, 1500000)
    .with_fill(ShapeFill::new("4CAF50").with_transparency(50))
    .with_line(ShapeLine::new("1B5E20", 25400));
```

### Connectors

```rust
use ppt_rs::generator::{Connector, ConnectorLine, ArrowType, ArrowSize, LineDash};

// Straight connector with arrow
let conn = Connector::straight(1000000, 1000000, 3000000, 1000000)
    .with_line(ConnectorLine::new("1565C0", 25400))
    .with_end_arrow(ArrowType::Triangle)
    .with_arrow_size(ArrowSize::Large);

// Elbow connector with dashed line
let elbow = Connector::elbow(1000000, 2000000, 3000000, 3000000)
    .with_line(ConnectorLine::new("2E7D32", 19050).with_dash(LineDash::Dash))
    .with_arrows(ArrowType::Oval, ArrowType::Stealth);
```

## What Makes This Different

Unlike other Rust PPTX crates that:
- ❌ Generate invalid files that won't open
- ❌ Have incomplete implementations
- ❌ Are abandoned or unmaintained
- ❌ Lack proper XML structure

`ppt-rs`:
- ✅ Generates **valid PPTX files** from day one
- ✅ **Actively maintained** with comprehensive test coverage (650+ tests)
- ✅ **Complete XML structure** following ECMA-376 standard
- ✅ **Validation tools** - Built-in validation command for quality assurance
- ✅ **Alignment testing** - Framework for ensuring compatibility with python-pptx
- ✅ **Production-ready** - used in real projects

## Quality Assurance

### Validation
- Built-in validation command for ECMA-376 compliance checking
- Comprehensive test suite (650+ tests)
- Integration tests for end-to-end validation

### Alignment Testing
- Framework for comparing output with python-pptx standards
- Alignment testing scripts and documentation
- See [docs/ALIGNMENT.md](docs/ALIGNMENT.md) for details

## Technical Details

- **Format**: Microsoft PowerPoint 2007+ (.pptx)
- **Standard**: ECMA-376 Office Open XML
- **Compatibility**: PowerPoint, LibreOffice, Google Slides, Keynote
- **Architecture**: Layered design with clear separation of concerns
- **Test Coverage**: 650+ tests covering all major features

## Advanced Features

- **Prelude Module**: Simplified API with macros (`pptx!`, `shape!`), unit helpers (`inches()`, `cm()`), and color constants
- **Gradient Fills**: Linear gradients with multiple stops and directions (horizontal, vertical, diagonal, custom angle)
- **Transparency**: Alpha transparency for solid fills (0-100%)
- **Connectors**: Straight, elbow, curved with arrow types (triangle, stealth, diamond, oval, open) and dash styles
- **Tables**: Cell formatting, colors, alignment, borders
- **Charts**: Bar, line, pie, area, scatter, doughnut, radar, and more
- **Shapes**: 100+ shape types with fills, outlines, and text
- **Animations**: 50+ animation effects (fade, fly, zoom, etc.)
- **Transitions**: 27 slide transition effects
- **SmartArt**: 25 SmartArt layouts (lists, processes, cycles)
- **Media**: Video and audio embedding (mp4, webm, mp3, wav)
- **3D Models**: GLB, GLTF, OBJ, FBX, STL formats
- **VBA Macros**: Support for .pptm files with macros
- **Custom XML**: Embed custom data in presentations
- **Themes**: Color schemes and font definitions
- **Speaker Notes**: Add notes to slides

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed documentation.

## License

Apache-2.0

## Contributing

Contributions welcome! See [TODO.md](TODO.md) for current priorities.
