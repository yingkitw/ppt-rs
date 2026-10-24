# PPTX Examples

This directory contains examples demonstrating different ways to generate PowerPoint presentations using ppt-rs.

## Markdown Examples

### markdown_features.md
Demonstrates the enhanced Markdown-to-PowerPoint capabilities:
- Real image handling (local + web URLs, auto-detected PNG/JPEG/GIF/WEBP/SVG)
- Task lists (`- [x]` / `- [ ]`)
- Strikethrough, bold, italic, inline code
- GFM tables and fenced code blocks

**Generate:**
```bash
cargo run --features cli --bin pptcli -- md2ppt examples/markdown_features.md examples/output/markdown_features.pptx --title "Advanced Markdown"
```

### enhanced_markdown_features.rs
Rust API example exercising the same enhanced Markdown features programmatically.

**Run:**
```bash
cargo run --features cli --example enhanced_markdown_features
```

## Rust Examples

### complex_pptx.rs
Demonstrates creating complex presentations programmatically with:
- Business presentation (4 slides)
- Training course (5 slides)
- Project proposal (6 slides)

**Run:**
```bash
cargo run --example complex_pptx
```

### proper_pptx.rs
Demonstrates generating simple presentations with:
- Simple presentation (1 slide)
- Multi-slide presentation (5 slides)
- Report (6 slides)
- Training (10 slides)

**Run:**
```bash
cargo run --example proper_pptx
```

## Quick Start

### Generate the Markdown Example

```bash
cargo run --features cli --bin pptcli -- md2ppt examples/markdown_features.md examples/output/markdown_features.pptx --title "Advanced Markdown"
```

This creates `examples/output/markdown_features.pptx`.

### Create Your Own

#### From Markdown
1. Create a `.md` file with your content
2. Use `# Heading` for slide titles
3. Use `- Bullet` for bullet points
4. Run: `cargo run --features cli --bin pptcli -- md2ppt your_file.md output.pptx`

#### Programmatically
1. Use `SlideContent` builder
2. Create slides with titles and bullets
3. Call `create_pptx_with_content()`

Example:
```rust
use ppt_rs::generator::{SlideContent, create_pptx_with_content};

let slides = vec![
    SlideContent::new("Title").add_bullet("Point 1"),
];
let pptx = create_pptx_with_content("My Presentation", slides)?;
```

## Markdown Format Guide

```markdown
# Slide Title
- Bullet point 1
- Bullet point 2

# Another Slide
- Point A
- Point B
- Point C
```

Features:
- `# Heading` creates a new slide with that title
- `- `, `* `, `+ ` create bullet points
- Empty lines are ignored
- Content is automatically formatted

## Output

All generated presentations are saved to `examples/output/` and are valid Microsoft PowerPoint 2007+ files that can be opened in:
- Microsoft PowerPoint
- LibreOffice Impress
- Google Slides
- Apple Keynote
- Any other compatible presentation software

## Tips

1. **Keep it simple**: One main idea per slide
2. **Use bullet points**: Easier to read and present
3. **Consistent formatting**: Use the same structure throughout
4. **Add titles**: Always give slides meaningful titles
5. **Preview**: Open generated files to verify content

## Troubleshooting

**File not found error:**
- Ensure markdown file exists in the examples directory
- Use correct relative path

**No slides generated:**
- Check markdown format (use `#` for titles, `-` for bullets)
- Ensure at least one slide title exists

**Invalid PPTX:**
- Verify markdown syntax
- Check for special characters that need escaping
- Try a simpler markdown file first

## See Also

- [README.md](../README.md) - Main project documentation
- [ARCHITECTURE.md](../ARCHITECTURE.md) - Architecture overview
- [API_GUIDE.md](../docs/API_GUIDE.md) - Simplified API reference
- [SPEC.md](../SPEC.md) - Technical specifications
