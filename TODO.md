# TODO - ppt-rs

**Tests**: 845 passing | **Warnings**: 0 | **Clippy**: clean

## Active

- [x] API Simplification - Color & Table helpers (v0.2.11)
- [ ] Update all examples to use new simplified API
- [x] Update documentation with new API examples

## Backlog (Prioritized)

### P1 — High Value
- [ ] Digital signatures (XML generation done; needs Content_Types + _rels wiring)
- [ ] Embedded fonts in output (XML generation done; needs font data parts + rId wiring)
- [ ] Complete API documentation with examples

### P2 — Medium Value
- [ ] Advanced theme customization
- [ ] Ink annotations (XML generation done; needs ink part + relationship)
- [ ] Benchmark suite
- [ ] Tutorial: Building your first presentation
- [ ] Tutorial: Markdown to PPTX workflow

### P3 — Future Work
- [ ] Fuzzing tests for PPTX parsing
- [ ] Property-based testing
- [ ] Cross-platform testing (Windows, macOS, Linux)

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
- [ ] Profile memory usage with large presentations (100+ slides)
- [ ] Review XML generation patterns — some modules use string concat instead of structured builders
- [ ] Consolidate table cell formatting logic (potential DRY opportunity)
- [ ] Modularize image effects XML generation

### Refactoring Opportunities
- [ ] Extract common validation patterns into `core::validation` module
- [ ] Unify error message formatting across modules
- [ ] Consider builder pattern consolidation for Shape/Table/Chart builders

## Completed

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
