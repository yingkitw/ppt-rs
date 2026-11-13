# Samples

| File | Purpose |
|------|---------|
| `readme-slides.md` | Markdown used in the README workflow example |
| `readme-slides.pptx` | Generated deck (regenerate with `pptcli md2ppt`) |

Three-slide showcase: title bullets, table + syntax-highlighted Rust + task list, bullets + photo from `examples/assets/` (same stock images as `comprehensive_demo`).

Regenerate README previews (**use this** — renders from Markdown in memory so tables + code show correctly; `pptcli export` re-imports and drops code blocks):

```bash
cargo run --release --example readme_assets --features markdown
```

CLI-only path (`.pptx` only):

```bash
pptcli md2ppt samples/readme-slides.md samples/readme-slides.pptx --title "ppt-rs sample"
```
