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
│         ToXml, Positioned, Styled, XmlWriter                │
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

### API Layer (`lib.rs`)
- **Purpose**: Provide user-friendly functions for common tasks
- **Key Functions**:
  - `presentation()` - Create or open a presentation
  - `presentation_from_reader()` - Load from a reader
- **Exports**: `Presentation` struct and related types

### Package Layer (`package.rs`)
- **Purpose**: Handle .pptx files as ZIP containers
- **Responsibilities**:
  - Open/save ZIP files
  - Manage relationships between parts
  - Handle package structure
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
- **Submodules**:
  - `ns.rs` - XML namespace handling
  - `xmlchemy.rs` - Base element classes
  - `presentation.rs` - Presentation XML elements
  - `slide.rs` - Slide XML elements
  - `shapes/` - Shape XML elements
  - `text.rs` - Text XML elements
  - `chart/` - Chart XML elements
  - `dml/` - Drawing Markup Language elements
  - `table.rs` - Table XML elements

### OPC Layer (`opc/`)
- **Purpose**: Handle Open Packaging Convention (ZIP) specifics
- **Modules**:
  - `constants.rs` - Content types and relationship types
  - `package.rs` - ZIP file operations
  - `packuri.rs` - Package URI handling
  - `shared.rs` - Relationship definitions

### Utility Layers

#### Enumerations (`enums/`)
- **Purpose**: Type-safe enumeration values
- **Modules**:
  - `base.rs` - Base enum types
  - `action.rs` - Click action types
  - `chart.rs` - Chart-related enums
  - `dml.rs` - Drawing markup language enums
  - `shapes.rs` - Shape type enums
  - `text.rs` - Text formatting enums
  - `lang.rs` - Language identifiers

#### Utilities (`util.rs`)
- **Purpose**: Common utility functions
- **Key Types**:
  - `Length` - EMU (English Metric Unit) conversions
  - Conversion functions: `inches()`, `cm()`, `mm()`, `pt()`, `emu()`

#### Shared (`shared.rs`)
- **Purpose**: Shared proxy classes
- **Key Types**:
  - `ElementProxy` - Base proxy for XML elements
  - `ParentedElementProxy` - Proxy with parent reference
  - `PartElementProxy` - Proxy for part root elements

#### Exceptions (`exc.rs`)
- **Purpose**: Error types
- **Key Types**:
  - `PptxError` - Main error enum
  - `Result<T>` - Result type alias

## Data Flow

### Opening a Presentation

```
User calls presentation(path)
    ↓
api::presentation() opens file
    ↓
Package::open() reads ZIP
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
User calls Presentation::new()
    ↓
Default template is loaded
    ↓
Package structure is created
    ↓
Parts are initialized
    ↓
OXML elements are created
    ↓
Presentation object returned
```

### Saving a Presentation

```
User calls presentation.save(path)
    ↓
OXML elements are serialized to XML
    ↓
Parts are written to ZIP
    ↓
Relationships are written
    ↓
ZIP file is saved
```

## Key Design Patterns

### 1. Proxy Pattern
- `ElementProxy` wraps XML elements
- Provides convenient API while maintaining XML structure
- Used throughout for shapes, text, tables, etc.

### 2. Factory Pattern
- `PartFactory` creates appropriate part types based on content type
- Ensures correct part class is instantiated

### 3. Lazy Initialization
- Parts are loaded on demand
- Relationships are resolved lazily
- Improves performance for large presentations

### 4. Type Safety
- Enumerations prevent invalid values
- Traits define capabilities
- Result types for error handling

## File Organization

```
src/
├── lib.rs                 # Library root
├── prelude.rs           # Simplified API (v0.1.8)
├── api.rs               # Public API
├── package.rs           # Package handling
├── presentation.rs      # Presentation type
├── exc.rs              # Exception types
├── util.rs             # Utility functions
├── types.rs            # Type traits
├── shared.rs           # Shared proxy classes
├── enums/              # Enumeration types
│   ├── mod.rs
│   ├── base.rs
│   ├── action.rs
│   ├── chart.rs
│   ├── dml.rs
│   ├── shapes.rs
│   ├── text.rs
│   └── lang.rs
├── opc/                # Open Packaging Convention
│   ├── mod.rs
│   ├── constants.rs
│   ├── package.rs
│   ├── packuri.rs
│   └── shared.rs
├── oxml/               # Office XML
│   ├── mod.rs
│   ├── ns.rs
│   ├── xmlchemy.rs
│   ├── action.rs
│   ├── presentation.rs
│   ├── slide.rs
│   ├── text.rs
│   ├── table.rs
│   ├── theme.rs
│   ├── coreprops.rs
│   ├── simpletypes.rs
│   ├── chart/
│   ├── dml/
│   └── shapes/
├── parts/              # Package parts
├── shapes/             # Shape types
├── text/               # Text handling
├── chart/              # Chart handling
├── dml/                # Drawing markup language
├── slide.rs            # Slide type
├── table.rs            # Table type
└── media.rs            # Media handling
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

## Future Enhancements

- [ ] RTL text support
- [ ] Ink annotations
- [ ] Comments and review features
- [ ] Slide sections
- [ ] Digital signatures
