# TODO - ppt-rs

**Tests**: 816 passing | **Warnings**: 0 | **Clippy**: clean

## Active

- [ ] Migrate `Box<dyn Error>` returns in generator to `PptxError` for consistency
- [ ] Review and refactor large functions

## Backlog

### Code Quality
- [ ] Profile memory usage with large presentations
- [ ] Lazy loading for slide content
- [ ] Streaming ZIP operations

### Features
- [ ] Digital signatures (XML generation done; needs Content_Types + _rels wiring)
- [ ] Ink annotations (XML generation done; needs ink part + relationship)
- [ ] Embedded fonts in output (XML generation done; needs font data parts + rId wiring)
- [ ] Advanced theme customization

### Testing
- [ ] Fuzzing tests for PPTX parsing
- [ ] Property-based testing
- [ ] Benchmark suite
- [ ] Cross-platform testing (Windows, macOS, Linux)
- [ ] Compatibility testing with Office 2007+, LibreOffice, Google Slides

### Documentation
- [ ] Complete API documentation with examples
- [ ] Tutorial: Building your first presentation
- [ ] Tutorial: Markdown to PPTX workflow

## Completed

<details>
<summary>v0.2.5 — Codebase Cleanup</summary>

- Merged dual table implementations (`tables.rs`+`tables_xml.rs` → `table/` module)
- Removed `generator/xml.rs` re-export shim
- Fixed all compiler warnings (0 warnings, 0 clippy issues)
- Removed stale docs/, scripts/, LEARNING_ANALYSIS.md
- Updated `.gitignore`, trimmed TODO.md
- Created `docs/index.html` landing page
</details>

<details>
<summary>v0.2.4 — Dimension API, Trait Refactor, DRY Cleanup</summary>

- Flexible `Dimension` API: EMU, Inches, Cm, Pt, Ratio, percent
- `FlexPosition` / `FlexSize` structs
- Fluent `.at()` / `.with_dimensions()` on Shape and Image
- Prelude: `shapes::dim()`, `shapes::rect_ratio()`, `shapes::text_box_ratio()`
- `impl ToXml` for Run, Paragraph, TextFrame, BulletStyle, TransitionType, Relationship, etc.
- `impl Positioned` / `ElementSized` for Shape, Image
- Consolidated `escape_xml`, removed unused deps, replaced `image` crate with header parser
</details>

<details>
<summary>v0.2.3 — Visual Polish</summary>

- Slide transitions (Fade, Push, Wipe, Split, etc.)
- Shape rotation, hyperlinks, gradient fills, connectors
- Cell merging (rowspan/colspan)
- 10 chart types with ChartBuilder
</details>

<details>
<summary>v0.2.1 — Content Enhancements</summary>

- Bullet styles (Number, Letter, Roman, Custom)
- Text enhancements (subscript, superscript, strikethrough, highlight)
- Image from URL/base64/bytes, cropping, effects
</details>

<details>
<summary>v0.2.0 — Templates & Prelude</summary>

- Template system (business_proposal, training_material, status_report, etc.)
- Theme presets (Corporate, Modern, Vibrant, Dark, Nature, Tech, Carbon)
- Prelude shapes, layout helpers, color constants
</details>

<details>
<summary>v0.1.x — Foundation</summary>

- PPTX generation with valid ZIP/XML structure
- 40+ shape types, tables, charts, images
- Reading/modifying existing PPTX files
- Mermaid diagram rendering (12 types)
- Syntax highlighting, md2ppt CLI
- Parts API, OXML parsing, DML
- 6 slide layouts, speaker notes
- SmartArt, 3D models, VBA, animations
</details>
