# Codebase Organization Guide

## Overview

This document describes the organization of the ppt-rs codebase and how to navigate and extend it.

## Directory Structure

```
src/
├── generator/          # Core PPTX generation
│   ├── mod.rs         # Public API
│   ├── builder.rs     # ZIP orchestration
│   └── xml.rs         # XML generation
├── integration/        # High-level builders and utilities
│   ├── mod.rs         # Public API
│   ├── builders.rs    # Presentation/Slide builders
│   └── helpers.rs     # Utility functions
├── cli/               # Command-line interface
│   ├── mod.rs         # Module root
│   ├── parser.rs      # Argument parsing
│   └── commands.rs    # Command execution
├── config.rs          # Configuration management
├── constants.rs       # Presentation constants
├── enums/             # Type-safe enumerations
│   ├── mod.rs
│   ├── base.rs
│   ├── action.rs
│   ├── chart.rs
│   ├── dml.rs
│   ├── lang.rs
│   ├── shapes.rs
│   └── text.rs
├── exc.rs             # Error types
├── util.rs            # Utility functions
├── opc/               # Open Packaging Convention
│   ├── mod.rs
│   ├── constants.rs
│   ├── package.rs
│   ├── packuri.rs
│   └── shared.rs
├── oxml/              # Office XML definitions
│   ├── mod.rs
│   ├── ns.rs
│   ├── xmlchemy.rs
│   ├── dml/
│   ├── chart/
│   └── shapes/
├── api.rs             # Public API (placeholder)
├── types.rs           # Type definitions
├── shared.rs          # Shared utilities
└── lib.rs             # Library root
```

## Core Modules

### generator/
**Purpose**: Generates valid PPTX files with proper ZIP structure and XML content.

**Key Components**:
- `builder.rs` - Orchestrates ZIP file creation and manages the generation pipeline
- `xml.rs` - Generates all XML components (slides, layouts, themes, relationships, etc.)
- `SlideContent` - Builder for creating slides with titles and bullet points

**Usage**:
```rust
use ppt_rs::generator::{create_pptx, create_pptx_with_content, SlideContent};

// Simple presentation
let pptx = create_pptx("My Presentation", 5)?;

// Complex presentation with content
let slides = vec![
    SlideContent::new("Title").add_bullet("Point 1"),
];
let pptx = create_pptx_with_content("Title", slides)?;
```

### integration/
**Purpose**: Provides high-level builders and utilities for presentation creation.

**Key Components**:
- `builders.rs` - `PresentationBuilder`, `SlideBuilder`, `PresentationMetadata`
- `helpers.rs` - Utility functions for conversions and formatting

**Usage**:
```rust
use ppt_rs::integration::PresentationBuilder;

let builder = PresentationBuilder::new("My Presentation")
    .with_slides(5);
builder.save_to_file("output.pptx")?;
```

### cli/
**Purpose**: Command-line interface for PPTX operations.

**Key Components**:
- `parser.rs` - Argument parsing and command routing
- `commands.rs` - Command implementations (create, info)

**Usage**:
```bash
cargo run -- create my.pptx --title "My Presentation" --slides 5
```

### Supporting Modules

- **config.rs** - Configuration management and output paths
- **constants.rs** - Presentation defaults and constants
- **enums/** - Type-safe enumeration values for various PPTX properties
- **exc.rs** - Error types and Result type alias
- **util.rs** - Utility functions (EMU conversions, length calculations)
- **opc/** - Open Packaging Convention (ZIP) handling
- **oxml/** - Office XML element definitions

## Module Dependencies

```
lib.rs
├── generator/
│   ├── builder.rs
│   │   └── xml.rs
│   └── xml.rs
├── integration/
│   ├── builders.rs
│   │   ├── generator/
│   │   ├── config.rs
│   │   └── constants.rs
│   └── helpers.rs
│       └── util.rs
├── cli/
│   ├── parser.rs
│   └── commands.rs
│       └── generator/
├── config.rs
├── constants.rs
├── enums/
├── exc.rs
├── util.rs
├── opc/
└── oxml/
```

## Adding New Features

### Adding a New Slide Type

1. Extend `SlideContent` in `src/generator/xml.rs`
2. Add XML generation function in `src/generator/xml.rs`
3. Update `src/generator/builder.rs` to handle the new type
4. Add tests in the module

### Adding a New Builder

1. Create a new builder struct in `src/integration/builders.rs`
2. Implement builder methods
3. Export from `src/integration/mod.rs`
4. Add tests

### Adding a New CLI Command

1. Add command parsing in `src/cli/parser.rs`
2. Implement command in `src/cli/commands.rs`
3. Add help text in `src/bin/pptcli.rs`
4. Add tests

## Testing

Run all tests:
```bash
cargo test
```

Run specific module tests:
```bash
cargo test generator::
cargo test integration::
cargo test cli::
```

Run examples:
```bash
cargo run --example complex_pptx
cargo run --example proper_pptx
```

## Code Style

- Use builder pattern for complex types
- Keep modules focused and single-responsibility
- Document public APIs with doc comments
- Write tests for new functionality
- Use descriptive variable and function names

## Future Expansion

The following modules are placeholders for future expansion:
- `chart/` - Chart creation and manipulation
- `dml/` - Drawing Markup Language
- `media/` - Media embedding
- `parts/` - Package parts management
- `shapes/` - Shape creation and manipulation
- `table/` - Table creation and manipulation
- `text/` - Advanced text handling

When implementing these features:
1. Move implementation from placeholder to actual module
2. Update module documentation
3. Add comprehensive tests
4. Update this guide

## Performance Considerations

- XML generation is done in-memory for simplicity
- ZIP compression is handled by the `zip` crate
- Large presentations may consume significant memory
- Consider streaming for very large files in the future

## Debugging

Enable debug output:
```bash
RUST_LOG=debug cargo run --example complex_pptx
```

Check generated PPTX structure:
```bash
unzip -l output.pptx
```

Validate PPTX format:
```bash
file output.pptx
```
