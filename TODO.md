# TODO - ppt-rs

**Tests**: 850+ passing | **Warnings**: 0 | **Clippy**: clean | **Version**: 0.2.17

## Active

- [x] Enhanced HTML & Markdown Features - Image handling, CSS parsing, export navigation (v0.2.14)
- [x] HTML Parser Documentation - Comprehensive parser comparison guide
- [ ] Update all examples to use new simplified API
- [x] Update documentation with new API examples

## Backlog (Prioritized)

### P1 — High Value
- [ ] Digital signatures (XML generation done; needs Content_Types + _rels wiring)
- [ ] Embedded fonts in output (XML generation done; needs font data parts + rId wiring)
- [ ] Complete API documentation with examples

### P2 — Medium Value
- [ ] Ink annotations (XML generation done; needs ink part + relationship)
- [ ] Benchmark suite


## Performance Targets

| Metric | Current | Target |
|--------|---------|--------|
| Generation speed | ~1000 slides/sec | Maintain |
| Memory (100 slides) | ~2-5 MB | < 10 MB |
| Memory (1000 slides, lazy) | ~10 MB | < 50 MB |
| Binary size | ~500 KB | < 1 MB |
| Test suite | < 1 sec | Maintain |

## Technical Debt

### Code Quality
- [x] Profile memory usage with large presentations (100+ slides)
- [x] Review XML generation patterns — some modules use string concat instead of structured builders
- [x] Consolidate table cell formatting logic (potential DRY opportunity)
- [x] Modularize image effects XML generation

### Refactoring Opportunities
- [ ] Extract common validation patterns into `core::validation` module
- [ ] Unify error message formatting across modules
- [ ] Consider builder pattern consolidation for Shape/Table/Chart builders

## Completed

<details>
<summary>v0.2.17 — Performance Optimizations</summary>

- **Hot-path allocation reductions**:
  - `create_pptx_with_settings()` accepts `&[SlideContent]` — eliminates slide clone in `Presentation::build()`
  - `Presentation::into_bytes()` — consuming build API
  - ZIP output buffer pre-sized from slide count
  - Reusable ZIP path buffers in `write_slides` / `write_slide_relationships`

- **Package XML generation** (`package_xml.rs`):
  - Pre-allocated `String` capacities for content types, relationships, presentation XML
  - `append_usize()` helper replaces per-iteration `format!` in loops

- **Slide content rendering**:
  - `render_additional_content()` pre-reserves XML capacity from element counts

- **Tests**: generation speed assertion for 100 slides (`tests/memory_profile_test.rs`)

</details>

<details>
<summary>v0.2.17 — Technical Debt: Memory Profiling & XML Refactors</summary>

- **Memory profiling** (`src/generator/memory_profile.rs`):
  - `GenerationMetrics`, `profile_eager_generation()`, `profile_lazy_generation()`
  - `sample_slides()` helper for benchmarking
  - Integration tests for 100–500 slide decks (`tests/memory_profile_test.rs`)

- **Table cell formatting consolidation** (`src/generator/table/format.rs`, `style.rs`):
  - Single `generate_cell_xml()` reusing `TextFormat`, `color_to_xml`, `CellMergeState`
  - Alignment, vertical anchor, and wrap now emitted in generator XML
  - `table_from_string_rows()` shared by HTML and Markdown import pipelines

- **Image effects modularization** (`src/generator/image_effects.rs`):
  - `generate_effect_xml()`, `generate_effect_list_xml()`, `generate_blip_fill_xml()`
  - `images_xml.rs` delegates to effects module

- **XML generation review** (`docs/XML_GENERATION.md`):
  - Documents patterns, completed refactors, and migration priorities
  - `table/xml.rs` migrated to `XmlWriter` for grid/row shell generation

</details>

<details>
<summary>v0.2.16 — Advanced Theme Customization</summary>

- **PresentationTheme** (`src/generator/presentation_theme.rs`):
  - `ThemeColorScheme` — ECMA-376 12-slot color scheme with `office()` preset and `from_palette()`
  - `ThemeFonts` — major/minor typeface pair (headings and body)
  - Built-in presets: `corporate()`, `modern()`, `vibrant()`, `dark()`, `nature()`, `tech()`, `carbon()`
  - Fluent builders: `.colors()`, `.fonts()`, `.major_font()`, `.minor_font()`
  - `to_theme_xml()` — generates `ppt/theme/theme1.xml` embedded in output PPTX

- **API integration**:
  - `Presentation::with_theme()` — apply theme via high-level API
  - `PresentationSettings::theme()` — apply theme via generator settings
  - `prelude::themes::Theme::to_presentation_theme()` — bridge prelude presets to embedded themes
  - `create_theme_xml()` delegates to custom theme when settings provide one

- **Tests**: 4 unit tests + 3 integration tests (`tests/theme_customization_test.rs`)

</details>

<details>
<summary>v0.2.12 — Export & Compression: Round-trip Capabilities</summary>

- **Markdown Export** (`src/export/md.rs`):
  - `export_to_markdown()` - Convert presentation to Markdown
  - `MarkdownOptions` - Configure output with slide numbers, frontmatter, GFM tables
  - Speaker notes, code blocks, and image references
  - API: `.save_as_markdown()`, `.save_as_markdown_with_options()`

- **Image Export** (`src/export/image_export.rs`):
  - `export_to_images()` - Export slides to PNG/JPEG via LibreOffice
  - `ImageExportOptions` - Configure format, DPI, quality, dimensions
  - Single slide export and thumbnail generation
  - API: `.save_as_images()`, `.save_slide_as_image()`, `.save_thumbnail()`
  - Presets: `high_quality()` (300 DPI PNG), `web_optimized()` (96 DPI JPEG)

- **PPTX Compression** (`src/opc/compress.rs`):
  - `compress_pptx()` - Optimize file size with configurable levels
  - `CompressionOptions` - Remove unused media, properties, notes, optimize XML
  - `analyze_pptx()` - File size breakdown analysis
  - API: `.compress()`, `.analyze_size()`
  - Presets: `maximum()`, `web()` (5MB target)

- **Tests**: 31 new unit tests, 31 new integration tests for new capabilities
- Full documentation in SPEC.md, ARCHITECTURE.md

</details>

<details>
<summary>v0.2.13 — MCP Server & Documentation Refresh</summary>

- **MCP Server** (`src/mcp/mod.rs`):
  - New `ppt_mcp` binary: Model Context Protocol stdio server via `rmcp`
  - 8 MCP tools: `create_presentation`, `markdown_to_pptx`, `get_pptx_info`, `export_pptx`, `merge_pptx`, `validate_pptx`, `create_presentation_with_tables`, `create_presentation_with_charts`
  - Feature-gated behind `mcp` feature flag
  - Integration test suite (`tests/mcp_integration_test.rs`, 700+ lines)

- **Documentation Refresh**:
  - Updated README version to 0.2.13, test counts to 850+
  - Removed stale "(NEW in v0.2.x)" labels throughout README
  - Added v0.2.13 to SPEC version history
  - Updated ARCHITECTURE with MCP module, core traits
  - Fixed `VERSION` constant in lib.rs (0.2.7 → 0.2.13)

- **Codebase Cleanup**:
  - Removed old `src/enums/`, `src/integration/`, unused parts modules
  - Consolidated to modular architecture with clear separation

</details>

<details>
<summary>v0.2.14 — Enhanced HTML & Markdown Features</summary>

- **Enhanced Markdown Parser** (`src/cli/markdown/parser.rs`):
  - Real image downloading from HTTP/HTTPS URLs
  - Local file path support for images
  - Auto-detection of image formats (PNG, JPEG, GIF, WEBP, SVG)
  - GitHub-style task list support (`- [x]` and `- [ ]`)
  - Strikethrough text support (`~~text~~`)
  - Enhanced nested formatting handling
  - Proper embedding using ImageBuilder API

- **Enhanced HTML Parser** (`src/import/html.rs`):
  - Extended CSS property support (margins, padding, borders, line-height, letter-spacing)
  - Real image downloading from web URLs during HTML parsing
  - Local file support for relative and absolute paths
  - Format auto-detection from magic bytes
  - Anchor tag (`<a href>`) parsing and hyperlink preservation
  - Enhanced color parsing (hex, RGB, named colors)
  - Better style inheritance and cascade

- **Enhanced HTML Export** (`src/export/html.rs`):
  - Interactive navigation controls (Previous/Next buttons)
  - Keyboard navigation (arrow keys, space, Home, End)
  - Touch/swipe support for mobile devices
  - Fullscreen mode support
  - Speaker notes export with toggle functionality
  - Syntax highlighting for code blocks
  - Configurable export options (HtmlExportOptions)
  - Enhanced CSS/JavaScript for better UX

- **HTML Parser Documentation** (`HTML_PARSERS.md`):
  - Comprehensive parser comparison guide
  - Clear use cases for each parser (basic vs web scraper)
  - Migration guide and examples
  - Architecture diagrams and feature comparison tables

- **Testing & Examples**:
  - 19 new integration tests for enhanced features
  - `examples/markdown_features.md` - Demonstrates all new Markdown capabilities
  - `examples/enhanced_markdown_features.rs` - Rust code examples for Markdown
  - `examples/html_features.html` - Comprehensive HTML demonstration
  - `examples/enhanced_html_features.rs` - Rust code examples for HTML
  - `validate_new_features.sh` - Automated validation script

- **Quality Improvements**:
  - All 659 tests passing (638 library + 19 new integration + 2 HTML export)
  - 100% backward compatibility maintained
  - Comprehensive error handling with fallback mechanisms
  - Enhanced documentation with examples and guides

</details>

<details>
<summary>v0.2.11 — API Simplification: Color & Table Helpers</summary>

- **Color Utilities** (`src/helpers/colors.rs`):
  - `ColorValue` struct with RGB/RGBA support
  - 40+ color aliases: `red()`, `blue()`, `green()`, `material_blue()`, `corporate_blue()`, etc.
  - Color adjustments: `.lighter()`, `.darker()`, `.opacity()`, `.transparent()`
  - Color operations: `.mix()`, `.grayscale()`, `.invert()`
  - Hex parsing: `ColorValue::from_hex("#FF8040")`
- **Table Utilities** (`src/helpers/tables.rs`):
  - `simple_table(rows, cols)` - Quick table creation
  - `table_from_data(&data, widths)` - Create from 2D array
  - `table_with_header(&headers, rows)` - Auto-styled headers
  - `QuickTable` builder with fluent API
  - Cell helpers: `cell()`, `header_cell()`, `highlight_cell()`
- **Extension Methods**:
  - `.fill(color)` - Shorter than `.with_fill()`
  - `.stroke(color, width)` - Shorter than `.with_line()`
  - `.text(text)` - Shorter than `.with_text()`
- **Documentation**:
  - Created `API_GUIDE.md` - Complete API reference (600+ lines)
  - Updated `README.md` with new API examples
  - Created `color_and_table_demo.rs` example (10 slides)
  - Updated `simplified_api.rs` with color/table examples
- **Code Reduction**: ~60% less boilerplate for common operations
- All tests passing, backward compatible

</details>

<details>
<summary>v0.2.10 — Image Effects & Dynamic Loading</summary>

- **Image Effects**: 8 professional visual effects for images
  - Shadow (outer drop shadow with blur and offset)
  - Reflection (mirror effect below image)
  - Glow (golden aura around image)
  - Soft Edges (feathered/vignette borders)
  - Inner Shadow (inset shadow for depth)
  - Blur (artistic defocus effect)
  - Crop (percentage-based edge trimming)
  - Combined (multiple effects together)
- **ImageBuilder API**: New builder methods for effects
  - `build_with_shadow()`, `build_with_reflection()`, `build_with_glow()`
  - `build_with_soft_edges()`, `build_with_inner_shadow()`, `build_with_blur()`
  - `build_with_crop(left, top, right, bottom)`, `build_with_effects()`
- **Dynamic Image Loading**: Comprehensive demo auto-loads stock photos from `examples/assets/`
  - Sorted alphabetically for consistent ordering
  - Supports JPEG, PNG, GIF formats
  - Skips non-image files automatically
- **Bug Fix**: Fixed JPEG image relationships to use correct file extensions
  - Updated `create_slide_rels_xml_with_images()` to accept image extensions
  - Relationships now correctly point to `.jpg` files instead of hardcoded `.png`
- **comprehensive_demo.rs**: Enhanced with 8 image effect slides (Slides 13-20)
  - Each effect demonstrated with 3 stock photos
  - Total 24 images with effects across 8 slides
- All tests passing, JPEG and PNG images display correctly in PowerPoint
</details>

<details>
<summary>v0.2.9 — Bug Fix: Compatibility Test Sorting</summary>

- Fixed `test_get_slide_files` to handle alphabetically sorted slide filenames correctly
- Removed unused `Chart` import from compatibility_test.rs
- All 845 tests passing, 0 warnings
</details>

<details>
<summary>v0.2.8 — Compatibility Testing Infrastructure</summary>

- **PptxValidator**: Struct for validating PPTX file structure
  - `validate_zip_structure()` - checks for required files and valid ZIP format
  - `validate_content_types()` - validates Content_Types.xml structure
  - `validate_presentation()` - checks presentation XML and namespaces
  - `validate_slide()` - validates individual slide XML files
  - `get_slide_files()` - lists all slide files in the presentation
- **CompatibilityTestSuite**: Generates test PPTX files for manual verification
  - 8 test files covering: basic, shapes, charts, images, large (100 slides), streaming, and lazy loading
- **Automated validation tests** (6 tests) - all passing
- Provides framework for continuous compatibility validation
- Test files output to `test_output/compatibility/` for manual verification in PowerPoint, LibreOffice, and Google Slides
</details>

<details>
<summary>v0.2.7 — Streaming ZIP & Lazy Loading</summary>

- **Streaming ZIP operations**: Added `create_pptx_to_writer()` and `create_pptx_with_content_to_writer()` APIs for writing PPTX files directly to any `Write + Seek` target (files, streams, etc.)
- **Lazy slide loading**: Added `LazySlideSource` trait and `create_pptx_lazy_to_writer()` for on-demand slide generation
- Benefits:
  - Memory efficiency for large presentations (no need to buffer entire ZIP in memory)
  - Support for dynamically generated slide content
  - Better performance for streaming data sources
- All internal write functions now use generic `W: Write + Seek` instead of hardcoded `Cursor<Vec<u8>>`
- Added comprehensive tests for new APIs (7 new tests)
- Backward compatible - existing `Vec<u8>`-returning APIs unchanged
</details>

<details>
<summary>v0.2.6 — Error Handling & Refactoring</summary>

- Migrated `Box<dyn Error>` returns to `PptxError` in generator/builder.rs
- Added `From<ZipError>` implementation for automatic error conversion
- Refactored `write_package_files` into smaller helper functions:
  - `write_content_types` - handles content types XML
  - `write_presentation_relationships` - handles presentation relationships
  - `write_presentation_properties` - handles presProps.xml
  - `write_notes_master` - handles notes master files
  - `write_theme_and_layouts` - handles theme and layout files
  - `write_document_properties` - handles core and app properties
- Added `ChartInfo` struct and `collect_chart_info` helper
- Removed wasm-bindgen dependency (chrono now uses specific features)
</details>

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
