# XML Generation Patterns

This document reviews how ppt-rs generates OOXML and records the migration path from string concatenation toward structured builders.

## Current Patterns

| Pattern | Location | Use when |
|---------|----------|----------|
| `format!` / `push_str` blobs | `charts/xml.rs`, `shapes_xml.rs`, `slide_xml/layouts.rs`, `package_xml.rs` | Large static templates with few variables |
| `ToXml` trait | `text/run.rs`, `text/paragraph.rs`, `bullet.rs`, `transition.rs` | Composable text and small enum fragments |
| `XmlWriter` | `core/xml_utils.rs`, `layouts/common.rs`, `table/xml.rs` | Repetitive element trees with attributes |
| Dedicated modules | `image_effects.rs`, `table/format.rs`, `gradients.rs` | Domain-specific XML with unit tests |

## Shared Utilities

- **`core::escape_xml`** — XML entity escaping (use everywhere; avoid local duplicates)
- **`core::XmlWriter`** — `start_element`, `end_element`, `empty_element`, `raw`, `text`
- **`text::format::color_to_xml`** — `<a:solidFill>` color fragments
- **`text::format::TextFormat::to_xml_attrs`** — run property attributes (`b`, `i`, `u`, `sz`)
- **`slide_content::table_merge::CellMergeState::to_xml_attrs`** — merge attributes for table cells

## Completed Refactors (v0.2.17)

### Table cell formatting (`generator/table/format.rs`)

- Single `generate_cell_xml()` used by `table/xml.rs`
- Reuses `TextFormat`, `color_to_xml`, and `CellMergeState`
- Emits alignment (`algn`), vertical anchor (`anchor`), and wrap (`wrap`) previously missing from generator path
- `table/style.rs` consolidates header presets and `table_from_string_rows()` for HTML/Markdown import

### Image effects (`generator/image_effects.rs`)

- `generate_effect_xml()`, `generate_effect_list_xml()`, `generate_blip_fill_xml()`
- `images_xml.rs` orchestrates pic frame; effects are independently testable

### Table shell XML (`generator/table/xml.rs`)

- Migrated grid/row shell generation to `XmlWriter` as reference pattern for list-heavy XML

## High-Priority Migration Targets

1. **`charts/xml.rs`** — extract shared series/axis/chartSpace helpers (largest duplication)
2. **`shapes_xml.rs`** — model fill/line/text as `ToXml` sub-structs
3. **`slide_xml/layouts.rs`** — converge with `SlideXmlBuilder` + `TextFrame` composition
4. **`package_xml.rs`** — use `XmlWriter` loops for relationship and slide ID lists
5. **`hyperlinks.rs`**, **`connectors.rs`** — adopt `XmlWriter` for attribute assembly

## Guidelines for New XML Code

1. Prefer **`XmlWriter`** when emitting three or more sibling elements with attributes
2. Prefer **`ToXml`** for types that compose into larger trees (text runs, bullets, colors)
3. Extract domain modules when an effect/type has its own presets and tests (`image_effects.rs` pattern)
4. Never duplicate `escape_xml` — import from `core`
5. Keep orchestration functions thin: delegate fragments to focused generators

## Anti-Patterns to Avoid

- Copy-pasting OOXML snippets across import pipelines (use `table/style.rs` builders)
- Parallel model + XML paths (`parts/table.rs` vs `generator/table/`) — generator path is canonical
- Giant `format!` blocks with inline business logic — extract named functions first, then `XmlWriter`
