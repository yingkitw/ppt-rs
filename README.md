# PPTX - Rust Port of python-pptx

A Rust library for creating, reading, and updating PowerPoint 2007+ (.pptx) files.

This is a comprehensive port of the popular [python-pptx](https://github.com/scanny/python-pptx) library to Rust, providing similar functionality with the performance and safety benefits of Rust.

## Features

- Create new presentations from scratch
- Read and modify existing .pptx files
- Add slides, shapes, text, and images
- Manipulate presentation properties
- Support for charts, tables, and media
- Full XML manipulation capabilities
- Type-safe enumeration system

## Project Status

This is an active translation of all 101 Python files from python-pptx to Rust. The project is organized into logical modules that mirror the original Python package structure.

### Module Structure

- **`api`** - Public API for creating and opening presentations
- **`enums`** - Enumeration types for various PowerPoint settings
- **`opc`** - Open Packaging Convention (ZIP) handling
- **`oxml`** - Office XML element manipulation
- **`parts`** - Package parts (slides, layouts, masters, etc.)
- **`shapes`** - Shape manipulation
- **`text`** - Text and paragraph handling
- **`chart`** - Chart creation and manipulation
- **`dml`** - Drawing Markup Language (colors, fills, lines, etc.)
- **`util`** - Utility functions and length conversions
- **`shared`** - Shared proxy classes

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## Usage

### Creating a Presentation

#### Using the CLI

```bash
# Create a simple presentation
cargo run -- create my_presentation.pptx

# Create with custom title and slides
cargo run -- create my_presentation.pptx --title "My Title" --slides 5
```

#### Using the Library

```rust
use pptx_rs::generator;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a PPTX with 5 slides
    let pptx_data = generator::create_pptx("My Presentation", 5)?;
    
    // Write to file
    fs::write("presentation.pptx", pptx_data)?;
    
    Ok(())
}
```

### PPTX Generation Approach

The library generates proper Microsoft PowerPoint 2007+ (.pptx) files by:

1. **Creating a complete ZIP package** with all required ECMA-376 compliant components
2. **Generating XML documents** for presentation, slides, layouts, masters, and themes
3. **Managing relationships** between all package parts
4. **Including metadata** (title, creation date, slide count, etc.)
5. **Packaging into ZIP** with proper compression and structure

The generated files are:
- Valid Microsoft PowerPoint 2007+ format (recognized by `file` command)
- Readable by PowerPoint, LibreOffice, Google Slides, and other Office applications
- Fully compliant with ECMA-376 Office Open XML standard

See [ARCHITECTURE.md](ARCHITECTURE.md#pptx-generation-approach) for detailed technical documentation.

## Translation Status

See [TRANSLATION_PROGRESS.md](TRANSLATION_PROGRESS.md) for detailed progress on the translation of all 101 Python files.

## Architecture

The library maintains the same logical architecture as python-pptx:

1. **API Layer** (`api.rs`) - User-facing functions
2. **Package Layer** (`package.rs`) - ZIP file handling
3. **Parts Layer** (`parts/`) - Individual package components
4. **OXML Layer** (`oxml/`) - XML element manipulation
5. **Utility Layer** (`util.rs`, `shared.rs`) - Common utilities

## Dependencies

- `zip` - ZIP file handling
- `xml-rs` - XML parsing
- `image` - Image handling
- `serde` - Serialization
- `thiserror` - Error handling
- `uuid` - Unique identifiers
- `regex` - Regular expressions
- `chrono` - Date/time handling

## License

MIT License - Same as python-pptx

## Contributing

This is an active translation project. Contributions are welcome!

## References

- [python-pptx Documentation](https://python-pptx.readthedocs.io/)
- [ECMA-376 Office Open XML Standard](http://www.ecma-international.org/publications/standards/Ecma-376.htm)
- [Microsoft Office Open XML Formats](https://docs.microsoft.com/en-us/office/open-xml/open-xml-overview)
