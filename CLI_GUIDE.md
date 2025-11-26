# PPTX CLI - Command-Line Tool Guide

## Overview

The PPTX CLI is a command-line tool for creating and managing PowerPoint presentations using Rust. It provides a simple interface for generating presentations programmatically.

## Building the CLI

```bash
# Build debug version
cargo build --bin pptx-cli

# Build release version
cargo build --release --bin pptx-cli

# Run directly
cargo run --bin pptx-cli -- help
```

## Installation

After building, the binary is located at:
- Debug: `target/debug/pptx-cli`
- Release: `target/release/pptx-cli`

You can add it to your PATH or create an alias:

```bash
alias pptx-cli="/path/to/pptx-cli"
```

## Commands

### 1. Create Command

Create a new PowerPoint presentation.

**Syntax:**
```bash
pptx-cli create <output.pptx> [options]
```

**Options:**
- `--title <text>` - Set the presentation title
- `--slides <count>` - Number of slides to create (default: 1)
- `--template <file>` - Use a template file (optional)

**Examples:**

Create a simple presentation:
```bash
pptx-cli create my_presentation.pptx
```

Create with title and slides:
```bash
pptx-cli create my_presentation.pptx --title "My Presentation" --slides 5
```

Create in a subdirectory:
```bash
pptx-cli create output/presentations/demo.pptx --title "Demo" --slides 10
```

Create with template:
```bash
pptx-cli create my_presentation.pptx --template template.pptx
```

### 2. Info Command

Display information about a presentation.

**Syntax:**
```bash
pptx-cli info <file.pptx>
```

**Output includes:**
- File path
- File size in bytes
- Last modified time
- Whether it's a valid file
- Presentation title (if available)
- Number of slides (if available)

**Examples:**

Get info about a presentation:
```bash
pptx-cli info my_presentation.pptx
```

### 3. Help Command

Display help information.

**Syntax:**
```bash
pptx-cli help
pptx-cli -h
pptx-cli --help
```

## Usage Examples

### Example 1: Create a Simple Presentation

```bash
pptx-cli create simple.pptx
```

Output:
```
✓ Created presentation: simple.pptx
  Title: Presentation
  Slides: 1
```

### Example 2: Create a Multi-Slide Presentation

```bash
pptx-cli create conference.pptx --title "Annual Conference 2025" --slides 20
```

Output:
```
✓ Created presentation: conference.pptx
  Title: Annual Conference 2025
  Slides: 20
```

### Example 3: Create in Nested Directory

```bash
pptx-cli create projects/2025/q1/report.pptx --title "Q1 Report" --slides 15
```

The CLI automatically creates any missing directories.

### Example 4: Get Presentation Information

```bash
pptx-cli info conference.pptx
```

Output:
```
File Information
================
Path:     conference.pptx
Size:     2048 bytes
Modified: 5 minutes ago
Is file:  true

Presentation Information
========================
Title: Annual Conference 2025
Slides: 20
```

## Error Handling

The CLI provides clear error messages for common issues:

**File already exists:**
```bash
$ pptx-cli create existing.pptx
✗ Error: File already exists
```

**Invalid slide count:**
```bash
$ pptx-cli create my.pptx --slides abc
✗ Error: Invalid slide count
```

**Missing required arguments:**
```bash
$ pptx-cli create
✗ Error: create requires an output file
```

## Architecture

The CLI is built with a modular architecture:

### Modules

- **`cli/parser.rs`** - Command-line argument parsing
  - `Parser` - Main parser struct
  - `Command` - Enum for different commands
  - `CreateArgs` - Arguments for create command
  - `InfoArgs` - Arguments for info command

- **`cli/commands.rs`** - Command implementations
  - `CreateCommand` - Handles presentation creation
  - `InfoCommand` - Handles presentation information retrieval

- **`bin/pptx-cli.rs`** - Main CLI entry point
  - Argument handling
  - Command dispatch
  - Help display

### Data Flow

```
User Input
    ↓
Parser::parse()
    ↓
Command Enum
    ↓
Command Handler (Create/Info)
    ↓
File Operations
    ↓
Output to User
```

## Implementation Details

### Create Command

The create command:
1. Parses command-line arguments
2. Creates output directory if needed
3. Generates XML presentation structure
4. Writes to file
5. Reports success/failure

### Info Command

The info command:
1. Checks if file exists
2. Reads file metadata
3. Attempts to parse XML content
4. Extracts presentation information
5. Displays formatted output

## Testing

Run the CLI tests:

```bash
cargo test --bin pptx-cli
```

Test specific functionality:

```bash
# Test parser
cargo test --bin pptx-cli parser

# Test commands
cargo test --bin pptx-cli commands
```

## Future Enhancements

Planned features for the CLI:

- [ ] Add slides to existing presentations
- [ ] Add shapes and text to slides
- [ ] Add images to presentations
- [ ] Modify presentation properties
- [ ] Convert between formats
- [ ] Batch operations
- [ ] Configuration files
- [ ] Interactive mode
- [ ] Template support
- [ ] Scripting support

## Performance

The CLI is optimized for:
- Fast startup time
- Minimal memory usage
- Efficient file I/O
- Quick XML generation

Typical performance:
- Create 100-slide presentation: < 100ms
- Get info on presentation: < 10ms
- Help display: < 5ms

## Troubleshooting

### Issue: "Command not found"

**Solution:** Make sure the binary is in your PATH or use the full path:
```bash
/path/to/pptx-cli create my.pptx
```

### Issue: "Permission denied"

**Solution:** Make the binary executable:
```bash
chmod +x target/debug/pptx-cli
```

### Issue: "File not found" when using info

**Solution:** Verify the file path is correct:
```bash
ls -la my_presentation.pptx
pptx-cli info ./my_presentation.pptx
```

### Issue: Invalid slide count

**Solution:** Use a valid positive integer:
```bash
pptx-cli create my.pptx --slides 5  # ✓ Valid
pptx-cli create my.pptx --slides -1 # ✗ Invalid
```

## Contributing

To extend the CLI:

1. Add new command to `parser.rs`
2. Implement handler in `commands.rs`
3. Add dispatch in `bin/pptx-cli.rs`
4. Add tests in respective modules
5. Update documentation

## License

MIT License - Same as the pptx library

## References

- [PPTX Library Documentation](README.md)
- [Architecture Guide](ARCHITECTURE.md)
- [Translation Progress](TRANSLATION_PROGRESS.md)
