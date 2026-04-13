# ppt-rs

**The Rust library for generating PowerPoint presentations that actually works.**

While other Rust crates for PPTX generation are incomplete, broken, or abandoned, `ppt-rs` generates **valid, production-ready PowerPoint files** that open correctly in PowerPoint, LibreOffice, Google Slides, and other Office applications.

**🎯 Convert Markdown to PowerPoint in seconds** - Write your slides in Markdown, get a professional PPTX file. No PowerPoint needed. **Web:** [bulletpoint.dev](https://bulletpoint.dev).

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

### Library (Simplified API - NEW!)

```rust
use ppt_rs::prelude::*;

fn main() -> Result<()> {
    let slides = vec![
        // Shapes with color aliases
        SlideContent::new("Colorful Shapes")
            .add_shape(
                rect(0.5, 1.5, 2.0, 1.0)
                    .fill(blue().to_color())
                    .text("Blue Rectangle")
            )
            .add_shape(
                circle(3.0, 1.5, 1.5)
                    .fill(red().lighter(0.3).to_color())
                    .text("Light Red Circle")
            ),
        
        // Quick table creation
        SlideContent::new("Employee Directory")
            .table(
                QuickTable::new(3)
                    .header(&["Name", "Role", "Status"])
                    .row(&["Alice", "Engineer", "Active"])
                    .row(&["Bob", "Designer", "Active"])
                    .at(1.0, 1.5)
                    .build()
            ),
    ];
    
    let pptx = create_pptx_with_content("My Presentation", slides)?;
    std::fs::write("output.pptx", pptx)?;
    Ok(())
}
```

**New Features:**
- 🎨 **Color Aliases**: `red()`, `blue()`, `green()`, `orange()`, `material_blue()`, etc.
- 🌈 **Color Adjustments**: `.lighter()`, `.darker()`, `.opacity()`, `.mix()`
- 📊 **Quick Tables**: `QuickTable::new(cols).header().row().build()`
- 🔷 **Shape Helpers**: `rect()`, `circle()`, `ellipse()`, `triangle()`, `diamond()`
- ✨ **Extension Methods**: `.fill()`, `.stroke()`, `.text()` (shorter than `.with_fill()`, etc.)

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
- **Bullet Styles** - Numbered, lettered, Roman numerals, custom characters, hierarchical
- **Text Enhancements** - Strikethrough, highlight, subscript, superscript
- **Tables** - Multi-line cells, styling, positioning
- **Shapes** - 100+ shape types with gradient fills and transparency
- **Connectors** - Straight, elbow, curved with arrows and dash styles
- **Charts** - Bar, line, pie charts with multiple series
- **Images** - Embed from files, bytes, base64, URL, auto-detect format, 8 visual effects
- **Media** - Video (mp4, webm) and audio (mp3, wav) embedding
- **Reading** - Parse and modify existing PPTX files
- **Repair** - Validate and fix damaged PPTX files
- **Export** - Export presentations to HTML and PDF

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
ppt-rs = "0.2"
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

// Create a bar chart
let chart = ChartBuilder::new("Sales", ChartType::Bar)
    .categories(vec!["Q1", "Q2", "Q3"])
    .add_series(ChartSeries::new("2023", vec![100.0, 150.0, 120.0]))
    .add_series(ChartSeries::new("2024", vec![120.0, 180.0, 150.0]))
    .position(1000000, 1000000)
    .size(4000000, 3000000)
    .build();

// Add to slide
let slide = SlideContent::new("Sales Data").add_chart(chart);
```

### Slide Transitions (NEW in v0.2.3)

```rust
use ppt_rs::generator::{SlideContent, TransitionType};

// Create slide with transition
let slide = SlideContent::new("Moving On")
    .with_transition(TransitionType::Push); // Push, Fade, Cut, Cover, etc.
```

### Table Merging (NEW in v0.2.3)

```rust
use ppt_rs::generator::{TableBuilder, TableRow, TableCell};

let table = TableBuilder::new(vec![2000000, 2000000])
    .add_row(TableRow::new(vec![
        // Span 2 columns
        TableCell::new("Header").with_col_span(2),
        // Second cell skipped due to merge
    ]))
    .add_row(TableRow::new(vec![
        // Span 2 rows
        TableCell::new("Row Span").with_row_span(2),
        TableCell::new("Data 1"),
    ]))
    .add_row(TableRow::new(vec![
        // First cell skipped due to merge
        TableCell::new("Data 2"),
    ]))
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

### Bullet Styles (NEW in v0.2.1)

```rust
use ppt_rs::generator::{SlideContent, BulletStyle, BulletPoint};

// Numbered list
let slide = SlideContent::new("Steps")
    .add_numbered("First step")
    .add_numbered("Second step")
    .add_numbered("Third step");

// Lettered list (a, b, c)
let slide = SlideContent::new("Options")
    .add_lettered("Option A")
    .add_lettered("Option B");

// Roman numerals (I, II, III)
let slide = SlideContent::new("Chapters")
    .add_styled_bullet("Introduction", BulletStyle::RomanUpper)
    .add_styled_bullet("Main Content", BulletStyle::RomanUpper)
    .add_styled_bullet("Conclusion", BulletStyle::RomanUpper);

// Custom bullet characters
let slide = SlideContent::new("Custom Bullets")
    .add_styled_bullet("Star bullet", BulletStyle::Custom('★'))
    .add_styled_bullet("Arrow bullet", BulletStyle::Custom('→'))
    .add_styled_bullet("Check bullet", BulletStyle::Custom('✓'));

// Hierarchical (sub-bullets)
let slide = SlideContent::new("Hierarchy")
    .add_bullet("Main point")
    .add_sub_bullet("Supporting detail 1")
    .add_sub_bullet("Supporting detail 2");
```

### Text Enhancements (NEW in v0.2.1)

```rust
use ppt_rs::generator::BulletPoint;
use ppt_rs::prelude::font_sizes;

// Per-bullet formatting
let strikethrough = BulletPoint::new("Deleted text").strikethrough();
let highlighted = BulletPoint::new("Important!").highlight("FFFF00");
let subscript = BulletPoint::new("H₂O").subscript();
let superscript = BulletPoint::new("x²").superscript();
let styled = BulletPoint::new("Bold red text").bold().color("FF0000");

// Per-bullet font sizes
let large_text = BulletPoint::new("Big text").font_size(font_sizes::LARGE);
let small_text = BulletPoint::new("Small text").font_size(font_sizes::SMALL);

// Add to slide
let mut slide = SlideContent::new("Formatted Text");
slide.bullets.push(strikethrough);
slide.bullets.push(highlighted);
slide.bullets.push(large_text);
```

### Font Size Presets (NEW in v0.2.1)

```rust
use ppt_rs::prelude::font_sizes;

// Available presets (in points)
font_sizes::TITLE    // 44pt
font_sizes::SUBTITLE // 32pt
font_sizes::LARGE    // 36pt
font_sizes::HEADING  // 28pt
font_sizes::BODY     // 18pt
font_sizes::SMALL    // 14pt
font_sizes::CAPTION  // 12pt

// Use with slide content
let slide = SlideContent::new("Title")
    .title_size(font_sizes::TITLE)
    .content_size(font_sizes::BODY);
```

### Images from Base64 (NEW in v0.2.1)

```rust
use ppt_rs::generator::{Image, ImageBuilder};
use ppt_rs::prelude::inches;

// From base64 encoded string
let base64_png = "iVBORw0KGgoAAAANSUhEUg...";
let img = Image::from_base64(base64_png, 914400, 914400, "PNG")
    .position(inches(2.0), inches(3.0));

// From raw bytes
let bytes = vec![0x89, 0x50, 0x4E, 0x47, ...]; // PNG data
let img = Image::from_bytes(bytes, 914400, 914400, "PNG");

// Using builder
let img = ImageBuilder::from_base64(base64_png, inches(2.0), inches(2.0), "PNG")
    .position(inches(4.0), inches(3.0))
    .build();
```

### Image Effects (NEW in v0.2.10)

Apply professional visual effects to images with a simple, chainable API:

```rust
use ppt_rs::generator::ImageBuilder;
use ppt_rs::prelude::inches;

// Simple: Load from file with auto-detection
let img = ImageBuilder::from_file("photo.jpg")
    .at(inches(1.0), inches(2.0))
    .build();

// Auto-detect format from bytes
let bytes = std::fs::read("photo.jpg")?;
let img = ImageBuilder::auto(bytes)
    .at(inches(2.0), inches(3.0))
    .build();

// Chainable effects - shadow
let img = ImageBuilder::from_file("photo.jpg")
    .at(inches(1.0), inches(2.0))
    .shadow()
    .build();

// Chainable effects - reflection
let img = ImageBuilder::from_file("photo.jpg")
    .at(inches(3.0), inches(2.0))
    .reflection()
    .build();

// Chainable effects - glow
let img = ImageBuilder::from_file("photo.jpg")
    .at(inches(5.0), inches(2.0))
    .glow()
    .build();

// Multiple effects combined
let img = ImageBuilder::from_file("photo.jpg")
    .at(inches(1.0), inches(4.0))
    .shadow()
    .reflection()
    .build();

// With cropping (10% from each side)
let img = ImageBuilder::from_file("photo.jpg")
    .at(inches(3.0), inches(4.0))
    .crop(0.1, 0.1, 0.1, 0.1)
    .build();

// All together: size, position, effects, crop
let img = ImageBuilder::from_file("photo.jpg")
    .size(inches(3.0), inches(2.0))
    .at(inches(2.0), inches(3.0))
    .shadow()
    .glow()
    .crop(0.05, 0.05, 0.05, 0.05)
    .build();
```

**Supported Effects:**
- **Shadow** - Outer drop shadow with blur and offset
- **Reflection** - Mirror effect below the image
- **Glow** - Golden aura around the image
- **Soft Edges** - Feathered/vignette borders
- **Inner Shadow** - Inset shadow for depth
- **Blur** - Artistic defocus effect
- **Crop** - Trim edges (percentage-based)
- **Combined** - Multiple effects together

**Supported Formats:**
- JPEG/JPG - Full support with all effects
- PNG - Full support with all effects
- GIF - Basic support
- Dynamic loading from `examples/assets/` folder

## What Makes This Different

Unlike other Rust PPTX crates that:
- ❌ Generate invalid files that won't open
- ❌ Have incomplete implementations
- ❌ Are abandoned or unmaintained
- ❌ Lack proper XML structure

`ppt-rs`:
- ✅ Generates **valid PPTX files** from day one
- ✅ **Actively maintained** with comprehensive test coverage (750+ tests)
- ✅ **Complete XML structure** following ECMA-376 standard
- ✅ **Validation tools** - Built-in validation command for quality assurance
- ✅ **Alignment testing** - Framework for ensuring compatibility with python-pptx
- ✅ **Production-ready** - used in real projects

## Quality Assurance

### Validation
- Built-in validation command for ECMA-376 compliance checking
- Comprehensive test suite (750+ tests)
- Integration tests for end-to-end validation

### Alignment Testing
- Framework for comparing output with python-pptx standards
- Alignment testing scripts and documentation
- See `examples/alignment_test.rs` for details

## Technical Details

- **Version**: 0.2.11
- **Format**: Microsoft PowerPoint 2007+ (.pptx)
- **Standard**: ECMA-376 Office Open XML
- **Compatibility**: PowerPoint, LibreOffice, Google Slides, Keynote
- **Architecture**: Modular design with clear separation of concerns
- **Test Coverage**: 750+ tests covering all major features

## Templates

Create presentations quickly with pre-built templates:

```rust
use ppt_rs::templates::{self, ProposalContent, StatusContent};

// Business proposal template
let proposal = templates::business_proposal(
    "Q4 Budget Proposal",
    "Finance Team",
    ProposalContent {
        executive_summary: vec!["Key insight 1", "Key insight 2"],
        problem: vec!["Current challenge"],
        solution: vec!["Our approach"],
        timeline: vec![("Phase 1", "Week 1-2"), ("Phase 2", "Week 3-4")],
        budget: vec![("Development", "$100,000")],
        next_steps: vec!["Approve budget"],
    },
)?;

// Status report template
let status = templates::status_report(
    "Weekly Status",
    "2025-01-01",
    StatusContent {
        summary: vec!["On track for Q1 goals"],
        completed: vec!["Feature A released"],
        in_progress: vec!["Feature B in testing"],
        blocked: vec![],
        next_week: vec!["Release Feature B"],
        metrics: vec![("Velocity", "32 points")],
    },
)?;

// Quick simple presentation
let simple = templates::simple("My Presentation", &[
    ("Introduction", &["Point 1", "Point 2"]),
    ("Conclusion", &["Summary"]),
])?;
```

Available templates: `business_proposal`, `training_material`, `status_report`, `technical_doc`, `simple`

## Themes

Pre-defined color themes for consistent styling:

```rust
use ppt_rs::prelude::{themes, colors};

// Available themes
themes::CORPORATE  // Professional blue/gray
themes::MODERN     // Clean minimalist
themes::VIBRANT    // Bold and colorful
themes::DARK       // Dark mode
themes::NATURE     // Fresh green
themes::TECH       // Technology blue
themes::CARBON     // IBM Carbon Design

// Theme properties
let theme = themes::CORPORATE;
println!("Primary: {}", theme.primary);     // "1565C0"
println!("Background: {}", theme.background); // "FFFFFF"
```

### Extended Color Palettes (NEW in v0.2.1)

```rust
use ppt_rs::prelude::colors;

// Basic colors
colors::RED, colors::GREEN, colors::BLUE, colors::WHITE, colors::BLACK

// Corporate colors
colors::CORPORATE_BLUE, colors::CORPORATE_GREEN, colors::CORPORATE_RED

// Material Design colors
colors::MATERIAL_RED, colors::MATERIAL_BLUE, colors::MATERIAL_GREEN
colors::MATERIAL_PURPLE, colors::MATERIAL_INDIGO, colors::MATERIAL_CYAN
colors::MATERIAL_TEAL, colors::MATERIAL_LIME, colors::MATERIAL_AMBER

// IBM Carbon Design colors
colors::CARBON_BLUE_60, colors::CARBON_BLUE_40
colors::CARBON_GRAY_100, colors::CARBON_GRAY_80, colors::CARBON_GRAY_20
colors::CARBON_GREEN_50, colors::CARBON_RED_60, colors::CARBON_PURPLE_60
```

## Layout Helpers

Position shapes easily with layout helpers:

```rust
use ppt_rs::prelude::layouts;

// Center a shape on the slide
let (x, y) = layouts::center(1000000, 500000);

// Create a grid of positions
let positions = layouts::grid(2, 3, 1000000, 800000); // 2x3 grid

// Stack shapes horizontally
let positions = layouts::stack_horizontal(4, 500000, 100000, 2000000);

// Evenly distribute shapes
let positions = layouts::distribute_horizontal(3, 500000, 2000000);
```

## Advanced Features

- **Prelude Module**: Simplified API with macros (`pptx!`, `shape!`), unit helpers (`inches()`, `cm()`), and color constants
- **Templates**: Pre-built presentation structures (business proposal, status report, training material, technical doc)
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
