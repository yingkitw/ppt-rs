# Markdown → PowerPoint
- **ppt-rs** — pure Rust `.pptx` from Git-friendly Markdown
- Opens in PowerPoint **without repair** (`validate_package_bytes`)
- API: `markdown_to_pptx_bytes` · CLI: `pptcli md2ppt`

# Rich elements
| Feature | In the deck |
|---------|-------------|
| Tables | GFM pipe syntax |
| Code | Syntax-highlighted blocks |
| Tasks | `- [x]` checklists |

```rust
use ppt_rs::markdown_to_pptx_bytes;

fn main() -> ppt_rs::Result<()> {
    let md = std::fs::read_to_string("slides.md")?;
    std::fs::write("deck.pptx", markdown_to_pptx_bytes(&md, "Deck")?)?;
    Ok(())
}
```

- [x] ECMA-376 packaging
- [x] Title + body placeholders
- [ ] Your content next

# Export and polish
- PDF via **pdfrs** — no LibreOffice
- Themes, charts, templates, MCP `ppt_mcp`
- Round-trip import and text-frame editing

![Flower (comprehensive demo asset)](../examples/assets/flower.jpg)
