# PPTX Library Architecture

## Overview

The PPTX library is organized into several layers that handle different aspects of PowerPoint file manipulation:

```
┌─────────────────────────────────────────────────────────────┐
│                    Prelude (prelude.rs)                     │
│     pptx!(), shape!(), shapes::, colors::, inches(), cm()   │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                    Public API (lib.rs)                      │
│     Presentation, SlideContent, Table, Chart, Image         │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│              Core Traits (core/)                            │
│         ToXml, Positioned, ElementSized                        │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│              Elements Layer (elements/)                     │
│         Color, Position, Size, Transform                    │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│              Generator Layer (generator/)                   │
│    SlideContent, Tables, Charts, Images, XML generation     │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│              Parts Layer (parts/)                           │
│         SlidePart, ImagePart, ChartPart, etc.               │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│              OPC Layer (opc/)                               │
│         ZIP file handling and Package management            │
└─────────────────────────────────────────────────────────────┘
```

## Design Principles

1. **DRY**: Single source of truth — e.g. `escape_xml` lives only in `core::xml_utils`, image format helpers in `generator::images`
2. **SoC**: Clear boundaries between layers — XML generation in `generator/`, parsing in `oxml/`, packaging in `opc/`
3. **KISS**: Minimal dependencies (12 direct), lightweight header parsing instead of heavy crates, no unnecessary abstractions
4. **Trait-Facing**: Core traits (`ToXml`, `Positioned`, `ElementSized`) implemented on key types for generic dispatch, testability, and polymorphism. Types keep inherent methods for direct callers; trait impls delegate via `Type::method(self)`.
5. **Builder Pattern**: Fluent APIs for constructing complex objects
6. **Flexible Dimensions**: `Dimension` enum supports EMU, inches, cm, points, and ratio (0.0–1.0 of slide). Shapes and images accept `Dimension` via `.at()`, `.with_dimensions()`, and `from_dimensions()` for mixed-unit positioning.

### Trait Coverage

| Trait | Implementors |
|-------|-------------|
| `ToXml` | `Run`, `Paragraph`, `TextFrame`, `BulletStyle`, `TransitionType`, `Relationship`, `Relationships`, `TableCellPart`, `TableRowPart`, `CellBorders`, `RgbColor`, `Position`, `Size`, `Color` |
| `Positioned` | `Shape`, `Image` |
| `ElementSized` | `Shape`, `Image` |

## Module Descriptions

### Prelude Layer (`prelude.rs`)
- **Purpose**: Simplified API for common use cases
- **Key Features**:
  - `pptx!()` macro for quick presentation creation
  - `shape!()` macro for quick shape creation
  - `QuickPptx` builder with fluent API
  - Unit conversion helpers: `inches()`, `cm()`, `pt()`
  - Shape builders: `shapes::rect()`, `shapes::circle()`, `shapes::text_box()`
  - Color constants: `colors::RED`, `colors::BLUE`, `colors::CORPORATE_BLUE`
- **Usage**: `use ppt_rs::prelude::*;`

### API Layer (`api.rs`)
- **Purpose**: High-level presentation builder
- **Key Types**:
  - `Presentation` - Create, build, save, import, and export presentations
  - `Presentation::new()` / `Presentation::with_title()` - Create new presentations
  - `Presentation::from_path()` - Import existing PPTX files
  - `.save()`, `.build()`, `.save_as_html()`, `.save_as_pdf()`, `.save_as_png()` - Output

### Package Layer (`opc/`)
- **Purpose**: Handle .pptx files as ZIP containers
- **Responsibilities**:
  - Open/save ZIP files via `Package` struct
  - Stream writing to any `Write + Seek` target
  - Content type declarations
  - Relationship management
- **Key Types**: `Package`

### Parts Layer (`parts/`)
- **Purpose**: Represent individual package components
- **Part Types**:
  - `PresentationPart` - Main presentation document
  - `SlidePart` - Individual slides
  - `SlideLayoutPart` - Slide layouts
  - `SlideMasterPart` - Slide masters
  - `NotesSlidePart` - Notes pages
  - `ImagePart` - Embedded images
  - `MediaPart` - Embedded media
  - `ChartPart` - Embedded charts
  - `CorePropertiesPart` - Document properties

### OXML Layer (`oxml/`)
- **Purpose**: Parse and manipulate Office XML elements
- **Key Types**:
  - `PresentationReader` / `PresentationInfo` - Read PPTX metadata
  - `PresentationEditor` - Edit existing PPTX files
  - `SlideParser` / `ParsedSlide` - Parse slide content
  - `PptxRepair` / `RepairIssue` / `RepairResult` - Validate and repair PPTX files
  - `XmlElement` / `XmlParser` / `Namespace` - XML parsing utilities

### OPC Layer (`opc/`)
- **Purpose**: Handle Open Packaging Convention (ZIP) specifics
- **Key Types**: `Package` for ZIP file operations

### Utility Layers

#### Helpers (`helpers/`)
- **Purpose**: Simplified helpers for common operations
- **Modules**:
  - `colors.rs` - 40+ color aliases (`red()`, `blue()`, `material_blue()`, etc.), `ColorValue` with `.lighter()`, `.darker()`, `.opacity()`, `.mix()`, `.grayscale()`, `.invert()`
  - `tables.rs` - `simple_table()`, `table_from_data()`, `table_with_header()`, `QuickTable` builder, cell helpers
  - `shapes.rs` - `rect()`, `circle()`, `ellipse()`, `triangle()`, `diamond()` with inch-based dimensions
  - `ext.rs` - `ShapeExt` trait: `.fill()`, `.stroke()`, `.text()` extension methods

#### Core Types (`core/`)
- **Purpose**: Foundational traits and types
- **Key Types**:
  - `ToXml` trait - XML serialization
  - `Positioned` trait - Position interface (`.x()`, `.y()`, `.set_position()`)
  - `ElementSized` trait (re-exported as `Sized`) - Size interface (`.width()`, `.height()`)
  - `Dimension` enum - Flexible dimensions: EMU, Inches, Cm, Pt, Ratio, Percent
  - `FlexPosition` / `FlexSize` - Mixed-unit positioning

#### Element Types (`elements/`)
- **Purpose**: Unified element types used across the library
- **Key Types**: `Color`, `RgbColor`, `SchemeColor`, `Position`, `Size`, `Transform`

#### Exceptions (`exc.rs`)
- **Purpose**: Error types
- **Key Types**:
  - `PptxError` - Main error enum (Generic, Io, Zip, XmlParse, InvalidValue, NotFound, etc.)
  - `Result<T>` - Result type alias

## Data Flow

### Opening a Presentation

```
User calls Presentation::from_path(path)
    ↓
Package reads ZIP via opc::Package
    ↓
OPC layer extracts relationships
    ↓
Parts are loaded and parsed
    ↓
OXML elements are created
    ↓
Presentation object returned to user
```

### Creating a Presentation

```
User calls Presentation::new() or Presentation::with_title()
    ↓
Slides are added via .add_slide() or .add_presentation()
    ↓
User calls .build() or .save(path)
    ↓
Generator creates package structure
    ↓
XML is generated for each part
    ↓
Parts are written to ZIP
    ↓
File is saved
```

### Saving a Presentation

```
User calls presentation.save(path)
    ↓
OXML elements are serialized to XML
    ↓
Parts are written to ZIP (or streamed via Write + Seek)
    ↓
Relationships are written
    ↓
ZIP file is saved
```

## Key Design Patterns

### 1. Builder Pattern
- `SlideContent` uses builder methods (`.add_bullet()`, `.table()`, `.add_shape()`)
- `TableBuilder`, `ChartBuilder`, `ImageBuilder` provide fluent APIs
- `QuickPptx` in prelude for quick presentation creation

### 2. Factory Functions
- `create_pptx()`, `create_pptx_with_content()` create PPTX files
- `rect()`, `circle()`, `triangle()` in helpers create shapes with inch-based dimensions
- `simple_table()`, `QuickTable::new()` create tables

### 3. Trait-Based Dispatch
- `ToXml` for XML serialization across types
- `Positioned` and `ElementSized` for generic positioning/size operations
- `LazySlideSource` trait for on-demand slide generation

### 4. Streaming / Lazy Generation
- `create_pptx_to_writer()` streams directly to `Write + Seek`
- `create_pptx_lazy_to_writer()` generates slides on demand
- Memory-efficient for large presentations

## File Organization

```
src/
├── lib.rs                 # Library root, re-exports public API
├── prelude.rs             # Simplified API: macros, unit helpers, shapes, colors, themes
├── api.rs                 # High-level Presentation builder (new, save, import, export)
├── templates.rs           # Pre-built templates (business_proposal, status_report, etc.)
├── exc.rs                 # Error types (PptxError, Result)
├── core/                  # Core traits and types
│   ├── mod.rs
│   ├── dimension.rs       # Dimension enum (EMU, Inches, Cm, Pt, Ratio)
│   └── xml_utils.rs       # Shared XML utilities (escape_xml)
├── elements/              # Unified element types
│   ├── mod.rs
│   ├── color.rs           # Color, RgbColor, SchemeColor
│   ├── position.rs        # Position, Size, Transform
│   └── ...
├── generator/             # PPTX generation (ZIP + XML)
│   ├── mod.rs
│   ├── builder.rs         # create_pptx(), create_pptx_with_content()
│   ├── slide_xml/         # Modular slide XML generation
│   ├── slide_content/     # SlideContent, bullets, transitions
│   ├── charts/            # Chart types and XML
│   ├── table/             # Table builder and XML
│   ├── shapes.rs          # Shape types
│   ├── shapes_xml.rs      # Shape XML generation
│   ├── images.rs          # Image types, effects, ImageBuilder
│   ├── images_xml.rs      # Image XML generation
│   ├── connectors.rs      # Connector types
│   ├── hyperlinks.rs      # Hyperlink support
│   ├── gradients.rs       # Gradient fills
│   ├── media.rs           # Video/audio embedding
│   ├── package_xml.rs     # Package-level XML
│   ├── theme_xml.rs       # Theme XML
│   ├── props_xml.rs       # Properties XML
│   └── notes_xml.rs       # Notes XML
├── parts/                 # Package parts (SlidePart, ImagePart, etc.)
├── opc/                   # Open Packaging Convention (ZIP handling)
├── oxml/                  # Office XML parsing (reader, editor, parser, repair)
├── helpers/               # Simplified helpers
│   ├── mod.rs
│   ├── colors.rs          # 40+ color aliases, ColorValue
│   ├── tables.rs          # QuickTable, table helpers
│   ├── shapes.rs          # rect(), circle(), etc.
│   └── ext.rs             # ShapeExt extension methods
├── export/                # HTML export
├── import/                # PPTX import
├── cli/                   # CLI commands + Markdown parser + Mermaid renderers
├── web2ppt/               # (Optional) Web-to-PPTX conversion
└── bin/
    └── pptcli.rs          # CLI binary entry point
```

## PPTX Generation Approach

### Overview

The library generates proper Microsoft PowerPoint 2007+ (.pptx) files by creating a complete ZIP-based package structure that conforms to the ECMA-376 Office Open XML standard.

### Generation Process

#### 1. Package Structure Creation
The generator creates a complete PPTX package with the following structure:

```
presentation.pptx (ZIP file)
├── [Content_Types].xml          # Content type declarations
├── _rels/
│   └── .rels                    # Package relationships
├── ppt/
│   ├── presentation.xml         # Main presentation document
│   ├── _rels/
│   │   └── presentation.xml.rels # Presentation relationships
│   ├── slides/
│   │   ├── slide1.xml           # Individual slides
│   │   ├── slide2.xml
│   │   └── _rels/
│   │       ├── slide1.xml.rels  # Slide relationships
│   │       └── slide2.xml.rels
│   ├── slideLayouts/
│   │   ├── slideLayout1.xml     # Slide layout templates
│   │   └── _rels/
│   │       └── slideLayout1.xml.rels
│   ├── slideMasters/
│   │   ├── slideMaster1.xml     # Slide master
│   │   └── _rels/
│   │       └── slideMaster1.xml.rels
│   └── theme/
│       └── theme1.xml           # Color theme definitions
└── docProps/
    ├── core.xml                 # Document properties (title, author, etc.)
    └── app.xml                  # Application-specific properties
```

#### 2. XML Generation

Each component is generated with proper Office Open XML structure:

- **[Content_Types].xml**: Declares MIME types for all package parts
- **Relationships (.rels)**: Define connections between package parts
- **presentation.xml**: Root presentation document with slide references
- **slide*.xml**: Individual slide content with shapes and text
- **slideLayout*.xml**: Layout templates defining slide structure
- **slideMaster*.xml**: Master slide with default formatting
- **theme*.xml**: Color schemes and fonts
- **core.xml**: Document metadata (title, creation date, etc.)
- **app.xml**: Application properties (slide count, etc.)

#### 3. ZIP Packaging

All XML files are compressed into a single ZIP archive with:
- Proper compression levels
- Correct file ordering
- Valid ZIP headers and footers
- No extra metadata

#### 4. Generator Module (`generator.rs`)

The `generator::create_pptx()` function:

```rust
pub fn create_pptx(title: &str, slides: usize) -> Result<Vec<u8>>
```

This function:
1. Creates all required XML documents with proper namespaces
2. Generates the specified number of slides
3. Creates relationships between all parts
4. Packages everything into a ZIP archive
5. Returns the complete PPTX as a byte vector

### Key Features

- **Proper ZIP Structure**: Uses the `zip` crate to create valid ZIP files
- **XML Namespaces**: Correctly declares all Office Open XML namespaces
- **Relationships**: Properly manages part relationships and IDs
- **Metadata**: Includes document properties (title, creation date, etc.)
- **Themes**: Includes default color theme for consistent formatting
- **Layouts**: Provides slide layout templates for proper slide structure

### Validation

Generated PPTX files are validated as:
- Proper ZIP archives (recognized by `file` command as "Microsoft PowerPoint 2007+")
- Readable by Microsoft PowerPoint, LibreOffice, and other Office applications
- Containing all required ECMA-376 compliant components

### Usage

```rust
// Generate a PPTX with 5 slides
let pptx_data = generator::create_pptx("My Presentation", 5)?;

// Write to file
std::fs::write("presentation.pptx", pptx_data)?;
```

## Table XML Structure

Tables in PPTX follow a specific XML structure. The critical ordering is:

```xml
<a:tc>
  <a:txBody>           <!-- TEXT BODY MUST COME FIRST -->
    <a:bodyPr/>
    <a:lstStyle/>
    <a:p>
      <a:r>
        <a:rPr lang="en-US" dirty="0"/>
        <a:t>Cell Text</a:t>
      </a:r>
    </a:p>
  </a:txBody>
  <a:tcPr/>            <!-- CELL PROPERTIES COME SECOND -->
</a:tc>
```

**Key Points:**
- `<a:txBody>` must come BEFORE `<a:tcPr>` (learned from reference PPTX analysis)
- Simple `<a:rPr>` with minimal attributes works best
- Optional formatting (bold, italic, color) added as attributes or child elements

## Generator Module Structure

```
├── generator/          # Generator module
│   ├── mod.rs          # Module exports
│   ├── builder.rs      # PPTX creation functions
│   ├── slide_xml/      # Modular slide XML generation
│   │   ├── mod.rs
│   │   ├── common.rs
│   │   ├── layouts.rs
│   │   └── content.rs
│   ├── slide_content/  # Slide content types (v0.2.x)
│   │   ├── mod.rs
│   │   ├── content.rs
│   │   ├── bullet.rs
│   │   ├── layout.rs
│   │   ├── transition.rs
│   │   └── code_block.rs
│   ├── charts/         # Chart module (v0.2.3)
│   │   ├── mod.rs
│   │   ├── builder.rs
│   │   ├── data.rs
│   │   ├── types.rs
│   │   └── xml.rs
│   ├── table/          # Modular table module
│   │   ├── mod.rs
│   │   ├── cell.rs
│   │   ├── row.rs
│   │   ├── builder.rs
│   │   └── xml.rs
│   ├── shapes.rs       # Shape types
│   ├── shapes_xml.rs   # Shape XML generation
│   ├── images.rs       # Image types
│   ├── images_xml.rs   # Image XML generation
│   ├── connectors.rs   # Connector shapes
│   ├── hyperlinks.rs   # Hyperlink support
│   ├── gradients.rs    # Gradient fills
│   ├── media.rs        # Video/audio
│   ├── package_xml.rs  # Package-level XML
│   ├── theme_xml.rs    # Theme XML
│   ├── props_xml.rs    # Properties XML
│   └── notes_xml.rs    # Notes XML
├── export/             # Export module (v0.2.2)
│   ├── mod.rs
│   ├── html.rs
│   └── html_style.css
├── import/             # Import module (v0.2.2)
│   └── mod.rs
├── web2ppt/            # Web to PPT converter
│   ├── mod.rs
│   ├── converter.rs
│   ├── config.rs
│   ├── fetcher.rs
│   └── parser.rs
├── cli/                # CLI module
```

## CLI Module Structure

```
src/cli/
├── mod.rs              # Module exports
├── commands.rs         # CLI commands
├── syntax.rs           # Syntax highlighting (Solarized Dark)
└── markdown/           # Markdown parsing (v0.1.7)
    ├── mod.rs          # Module exports
    ├── parser.rs       # Markdown parser state machine
    └── mermaid/        # Mermaid diagram rendering
        ├── mod.rs      # Detection and dispatch
        ├── types.rs    # Shared types
        ├── flowchart.rs
        ├── sequence.rs
        ├── pie.rs
        ├── gantt.rs
        ├── class_diagram.rs
        ├── state_diagram.rs
        ├── er_diagram.rs
        ├── mindmap.rs
        └── timeline.rs
```

## Completed Features

- [x] Complete OXML element implementations
- [x] Full ZIP file handling
- [x] XML serialization/deserialization
- [x] Relationship management
- [x] Part factory implementation
- [x] Chart data handling (18+ chart types)
- [x] Media embedding (video, audio)
- [x] Theme support
- [x] Master slide support
- [x] Animation support (50+ effects)
- [x] Transition support (27 effects)
- [x] SmartArt support (25 layouts)
- [x] 3D model support
- [x] VBA macro support
- [x] Custom XML support
- [x] Table cell formatting and text rendering
- [x] Syntax highlighting for code blocks (Solarized Dark)
- [x] Auto-fit font sizing for shapes
- [x] Automatic text color contrast
- [x] Prelude module with simplified API (v0.1.8)
- [x] Gradient fills (linear, multi-stop, custom angles)
- [x] Transparency for solid fills
- [x] Styled connectors (arrows, dash styles)
- [x] 12 Mermaid diagram types
- [x] Table merging (rowspan/colspan)
- [x] HTML/PDF export
- [x] Image effects (shadow, reflection, glow, soft edges, inner shadow, blur, crop)

## Image Effects System (v0.2.10)

### Architecture

```
ImageBuilder
    ↓
Image struct (with effects, crop)
    ↓
generate_image_xml() → OOXML with effects
    ↓
Embedded in slide with relationships
```

### Supported Effects

- **Shadow** (`ImageEffect::Shadow`) - Outer drop shadow with blur and offset
- **Reflection** (`ImageEffect::Reflection`) - Mirror effect below image
- **Glow** (`ImageEffect::Glow`) - Golden aura around image
- **Soft Edges** (`ImageEffect::SoftEdges`) - Feathered borders
- **Inner Shadow** (`ImageEffect::InnerShadow`) - Inset shadow for depth
- **Blur** (`ImageEffect::Blur`) - Artistic defocus effect

### Cropping

- Percentage-based (0.0-1.0) for left, top, right, bottom
- Applied via `<a:srcRect>` in OOXML

### Builder Methods

```rust
// Single effects
.build_with_shadow()
.build_with_reflection()
.build_with_glow()
.build_with_soft_edges()
.build_with_inner_shadow()
.build_with_blur()

// Cropping
.build_with_crop(left, top, right, bottom)

// Combined
.build_with_effects() // shadow + reflection
```

### XML Generation

Effects are rendered in `<a:effectLst>` within `<p:spPr>`:

```xml
<a:effectLst>
  <a:outerShdw blurRad="40000" dist="20000" dir="5400000" rotWithShape="0">
    <a:srgbClr val="000000"><a:alpha val="40000"/></a:srgbClr>
  </a:outerShdw>
  <a:reflection blurRad="6350" stA="50000" endA="300" endPos="35000".../>
</a:effectLst>
```

### Relationship Handling

- Image files written to `ppt/media/imageN.{ext}` with correct extension
- Relationships use actual file extensions (`.jpg`, `.png`, `.gif`)
- Fixed in v0.2.10 to support JPEG images correctly

### Implementation Files

- `src/generator/images.rs` - ImageEffect enum, ImageBuilder methods
- `src/generator/images_xml.rs` - XML generation for effects
- `src/generator/builder.rs` - Image extension collection and passing
- `src/generator/package_xml.rs` - Relationship generation with extensions

## Future Enhancements

- [ ] Advanced theme customization
- [ ] Complete digital signature wiring (XML done, needs Content_Types + _rels)
- [ ] Ink annotations wiring (XML done, needs ink part + relationship)
- [ ] Embedded fonts in output (XML done, needs font data parts + rId wiring)
- [ ] Fuzzing tests for PPTX parsing
- [ ] Property-based testing
- [ ] Benchmark suite
- [ ] Cross-platform testing (Windows, macOS, Linux)
