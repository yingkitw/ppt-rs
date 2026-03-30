# PPTX Examples

This directory contains examples demonstrating different ways to generate PowerPoint presentations using ppt-rs.

## Markdown Examples

### presentation.md
A comprehensive introduction to Rust programming language covering:
- What is Rust and why it matters
- Key features and benefits
- Ownership system
- Getting started guide
- Common use cases
- Resources for learning

**Generate:**
```bash
cargo run -- from-md examples/presentation.md examples/output/rust_presentation.pptx --title "Rust Programming"
```

### business.md
A Q4 2025 business review presentation including:
- Executive summary
- Financial performance metrics
- Market position analysis
- Strategic initiatives
- Team highlights
- Product roadmap for 2026
- Goals and challenges

**Generate:**
```bash
cargo run -- from-md examples/business.md examples/output/business_review.pptx --title "Q4 2025 Business Review"
```

### technical.md
A technical presentation on building scalable web applications:
- Architecture patterns
- Microservices approach
- Database design
- Performance optimization
- Deployment strategies
- Monitoring and observability
- Security best practices
- Cost optimization

**Generate:**
```bash
cargo run -- from-md examples/technical.md examples/output/web_architecture.pptx --title "Building Scalable Web Applications"
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

### Generate All Examples
Run the provided script to generate all markdown examples:

```bash
bash examples/generate_from_md.sh
```

This will create:
- `rust_presentation.pptx` - Rust programming introduction
- `business_review.pptx` - Business performance review
- `web_architecture.pptx` - Technical architecture guide

### Create Your Own

#### From Markdown
1. Create a `.md` file with your content
2. Use `# Heading` for slide titles
3. Use `- Bullet` for bullet points
4. Run: `cargo run -- from-md your_file.md output.pptx`

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
- [API_GUIDE.md](../API_GUIDE.md) - Simplified API reference
- [SPEC.md](../SPEC.md) - Technical specifications
