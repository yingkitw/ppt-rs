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

### Library

```rust
use ppt_rs::generator::{SlideContent, create_pptx_with_content};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let slides = vec![
        SlideContent::new("Introduction")
            .add_bullet("Welcome")
            .add_bullet("Agenda"),
        SlideContent::new("Key Points")
            .add_bullet("Point 1")
            .add_bullet("Point 2"),
    ];
    
    let pptx = create_pptx_with_content("My Presentation", slides)?;
    fs::write("output.pptx", pptx)?;
    Ok(())
}
```

## Features

### Core Capabilities
- **Slides** - Multiple layouts (title-only, two-column, blank, etc.)
- **Text** - Titles, bullets, formatting (bold, italic, colors, sizes)
- **Tables** - Multi-line cells, styling, positioning
- **Shapes** - 100+ shape types (flowcharts, arrows, geometric, decorative)
- **Charts** - Bar, line, pie charts with multiple series
- **Images** - Embed and position images
- **Reading** - Parse and modify existing PPTX files

### Markdown Format

The Markdown format is simple and intuitive:

- `# Heading` → Creates a new slide with that title
- `- Bullet` → Adds a bullet point (also supports `*` and `+`)
- Empty lines are ignored
- Each `#` heading starts a new slide

**Example:**
```markdown
# Introduction
- Welcome
- Agenda overview

# Main Content
- Key point 1
- Key point 2
- Key point 3

# Conclusion
- Summary
- Q&A
```

Convert with: `pptcli md2ppt presentation.md` → `presentation.pptx`

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
ppt-rs = "0.1"
```

## Examples

### Tables

```rust
use ppt_rs::generator::{SlideContent, TableBuilder, create_pptx_with_content};

let table = TableBuilder::new(vec![2000000, 2000000])
    .add_simple_row(vec!["Name", "Status"])
    .add_simple_row(vec!["Alice", "Active"])
    .build();

let slides = vec![
    SlideContent::new("Data").table(table),
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
use ppt_rs::generator::{Shape, ShapeType, ShapeFill};

let shape = Shape::new(ShapeType::Rectangle, 0, 0, 1000000, 500000)
    .with_fill(ShapeFill::new("FF0000"))
    .with_text("Hello");
```

## What Makes This Different

Unlike other Rust PPTX crates that:
- ❌ Generate invalid files that won't open
- ❌ Have incomplete implementations
- ❌ Are abandoned or unmaintained
- ❌ Lack proper XML structure

`ppt-rs`:
- ✅ Generates **valid PPTX files** from day one
- ✅ **Actively maintained** with comprehensive test coverage
- ✅ **Complete XML structure** following ECMA-376 standard
- ✅ **Production-ready** - used in real projects

## Technical Details

- **Format**: Microsoft PowerPoint 2007+ (.pptx)
- **Standard**: ECMA-376 Office Open XML
- **Compatibility**: PowerPoint, LibreOffice, Google Slides, Keynote
- **Architecture**: Layered design with clear separation of concerns

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed documentation.

## License

Apache-2.0

## Contributing

Contributions welcome! See [TODO.md](TODO.md) for current priorities.
