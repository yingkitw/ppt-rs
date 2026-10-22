# TODO - ppt-rs

**Tests**: 798 inline / 518 integration = 1316 total | **Warnings**: 0 (lib default features) | **Clippy**: clean with `--all-features --all-targets -- -D warnings` (v0.2.36) — lib, bins, tests, examples all green; safe to promote to a CI gate | **Version**: 0.3.0

## Active

- [x] Enhanced HTML & Markdown Features - Image handling, CSS parsing, export navigation (v0.2.14)
- [x] HTML Parser Documentation - Comprehensive parser comparison guide
- [x] Update all examples to use new simplified API
- [x] Update documentation with new API examples
- [x] Lean audit: dead code removal + `escape_xml` hot-path optimization (v0.2.21)
- [x] Complete API documentation with examples (`API_GUIDE.md` expanded)
- [x] Structured validation CLI — severity/category/path report + `--json`

- [x] **Web2MD bridge** — `web2md` feature: HTML→Markdown (via `web2md` crate)→Slides pipeline with `--web2md` CLI flag on `web2ppt`

## Proposed Capabilities

Ideas ranked by impact on real-world decks, round-trip fidelity, and API completeness.
Items here graduate into **Backlog** when scoped for implementation.

### Content & Round-Trip
Extend what can flow in and out without losing meaning.

- [x] **Speaker notes import** — MD/HTML speaker-note syntax → `notesSlides` parts on generation
- [x] **Comments round-trip** — generation + import: `PresentationReader::comment_authors()` / `slide_comments()` → `import_pptx` preserves authors and per-slide comments (v0.2.35)
- [x] **Image accessibility** — alt text / description on images (`descr` / `title` on `cNvPr`)
- [x] **Video & audio embedding** — `add_video`/`add_audio` wired through eager + lazy generation: `ppt/media/mediaN.*` parts, dual link/embed rels per element, poster frames (custom or built-in 1×1 placeholder), content-type Defaults, compression-safe; also fixed lazy-path content-type appends being silently skipped and `remove_unused_media` deleting media referenced only from rels
- [x] **Section-aware export** — `Presentation::add_section` / `with_sections` → `<p14:sectionLst>` in `presentation.xml` (eager + lazy); opt-in section grouping in Markdown/HTML export; section import via `from_path`; CLI `--group-by-sections` and MCP `group_by_sections`

### API & Tooling
Make the library easier to adopt, debug, and automate.

- [x] **Structured validation CLI** — `pptcli validate` reports structured issues (severity/category/path) + `--json`
- [x] **PresentationEditor enhancements** — duplicate slide, reorder slides, insert slide at index, batch replace
- [x] **Chart build-time validation** — category/series length checks via `core::validation` at `ChartBuilder::build()`
- [x] **PPTX semantic diff** — slide-level compare (title, bullets, images, charts, tables, media) for review/CI; `compare_presentations` / `compare_pptx_paths` + `pptcli diff` with `--json`
- [x] **MCP tool expansion** — `compress_pptx`, `repair_pptx`, `apply_theme` (export_markdown/export_html already covered by `export_pptx`)

### Conversion & Platform
Longer-horizon capabilities that broaden deployment options.

- [x] **PDF export without LibreOffice** — `pdfrs`-backed pure-Rust path (`pdf-native` feature, `save_as_pdf_via_pdfrs` / `to_pdf_bytes`)
- [x] **Image export without LibreOffice** — `export_to_images` / `save_slide_as_image` / `save_thumbnail` now render via pure-Rust `pdfrs` PDF + `pdftoppm`; fixes broken multi-slide rasterization (direct `soffice --convert-to png` only ever rasterizes slide 1)
- [~] **WASM target** — out of scope (desktop/server CLI + library focus; not planned)
- [x] **Batch / pipeline API** — `pipeline::run_directory` / `run_batch` + `pptcli batch`; shared theme/template via `BatchOptions`; `batch_generator` example rewritten (v0.2.31)

## Backlog (Prioritized)

### P0 — High Value
- [x] Digital signatures (XML generation done; needs Content_Types + _rels wiring)
- [x] Embedded fonts in output (XML generation done; needs font data parts + rId wiring)
- [x] Complete API documentation with examples
- [x] Wire `core::validation` into CLI validate + MCP `validate_pptx`
- [x] Structured `pptcli validate` output (severity/category/path + `--json`)

### P1 — Medium Value
- [x] Ink annotations (XML generation done; needs ink part + relationship)
- [x] Speaker notes import from MD/HTML
- [x] Image alt text / accessibility metadata
- [x] PresentationEditor: duplicate, reorder, insert slides
- [x] Benchmark suite (Criterion: generation, lazy loading, compression) — `benches/pptx_benchmarks.rs`, `cargo bench` (v0.2.32)
- [x] MCP tools: compress, repair, theme, export

### P2 — Future
- [x] PDF export without LibreOffice
- [~] WASM build target — out of scope
- [x] PPTX semantic diff tool
- [x] REST/HTTP service wrapper (optional `http` feature, `ppt_http` binary) — v0.2.34

### Backlog items seeded by markdown audit + competitive intel (2026-09-30)

- [x] **Clippy `--all-features` cleanup** — `cargo clippy --all-features --all-targets -- -D warnings` exits 0 (lib, bins, tests, examples — v0.2.36). Fixed: 34 `collapsible_if` + 3 `collapsible_match` in src (since TODO seed); 6 `useless_vec` (CLI parser tests using `vec![String;..]` for `Cli::parse_from` — switched to `[String;..]`); 3 `len_zero` (`assert!(x.len() >= 1)` / `len() > 0` → `!x.is_empty()`); 1 `ptr_arg` (`&PathBuf` → `&Path` in `media_deck`); 1 `unused_imports` (`Position` in `image_url_test`). No `too_many_arguments` survived — the original 4 call sites either already had parameter objects, or the lint went away with the v0.2.x builder consolidation.
- [x] **docs_sync.rs extension** — add the loose checks that the current gate doesn't enforce (README install version vs `src/lib.rs::VERSION`, MCP tool count vs `src/mcp/mod.rs` `^    fn ` methods, CLI command set vs `src/cli/parser.rs::Commands`). `tests/docs_sync.rs` excludes itself from the integration-test count (so adding gate tests does not itself drift the count) and walks `#[tool]` attributes with a bracket-depth state machine (multi-line `name = "…", description = "…"` body lines no longer reset the `pending_tool` flag). `cli_command_variants()` walks `pub enum Commands` with a `{`/`/` }` depth counter and only emits variant identifiers whose line ends with `{` (so prose inside `long_about = "…"` strings is not misread). README/ARCHITECTURE updated to mention `pdf2ppt` (previously unmentioned). Success criterion: `cargo test --test docs_sync` exits 0.
- [x] **Placeholder API** — generated slides now emit `<p:ph type="title"/>` / `<p:ph type="body" idx="1"/>` (etc.) so PowerPoint recognizes the shape as filling the layout's intended title/body slot (python-pptx parity: `slide.shapes.title`). Active code path (`src/generator/slide_xml/layouts.rs`) was emitting bare `<p:nvPr/>` for title/body — fixed in v0.2.36 follow-up by extending `generate_title_shape` with a `ph_type` parameter and adding `<p:ph>` to the body shape XML in `create_title_and_content_slide`, `create_title_and_big_content_slide`, and `create_two_column_slide`. Layouts covered: `TitleOnly` (title), `CenteredTitle` (`ctrTitle`), `SectionHeader` (title), `TitleAndContent` (title + body idx="1"), `TitleAndBigContent` (title + body idx="1"), `TwoColumn` (title + body idx="1" + body idx="2"). Tests: 4 inline (`src/generator/slide_xml/mod.rs::tests::title_and_content_emits_title_and_body_placeholders`, `centered_title_uses_ctr_title_placeholder`, `two_column_emits_idx1_and_idx2_body_placeholders`, `title_only_layout_emits_only_title_placeholder`) + 6 integration (`tests/placeholder_api_test.rs` — title/content, title-only, two-column, presentation builder pipeline, plus `package_validation_still_passes_with_placeholders` zero-repair guard). Refactored `generate_title_shape` signature from positional `(text, props, x, y, w, h, align, ph_type)` to parameter-object `(TitleContent, TitlePlacement, align, ph_type)` per MEMORY clippy pattern (avoids `too_many_arguments`). Success criterion met: deck round-trips through `validate_package_bytes` with no errors, slides open in PowerPoint with the layout's title/body placeholders populated.
- [x] **`TextFrame` runtime paragraph API on imported decks** — `PresentationEditor::shape_has_text_frame(slide, shape)`, `set_shape_text(slide, shape, text)`, `add_paragraph(slide, shape, text)`, `add_paragraph_with_format(slide, shape, ParagraphEdit::new(...).with_level(N).with_bold(b).with_font_size(pts))`, `set_title_text(slide, text)`, `shape_paragraphs(slide, shape) -> Vec<String>` (read-back). Plus `ParsedShape::has_text_frame()` predicate on the read side (python-pptx parity: `_BaseShape.has_text_frame`). Implementation: byte-level mutation on `<p:txBody>` via `src/oxml/text_editor.rs` (UTF-8 char-aware `nth_sp_range` / `txbody_range` walkers, depth counter so nested `<p:sp>` are skipped correctly). Tests: 9 inline (`src/oxml/text_editor.rs::tests::paragraph_edit_emits_clean_xml`, `paragraph_edit_escapes_special_chars`, `paragraph_edit_with_level_sets_lvl_attr`, `paragraph_edit_no_level_omits_lvl_attr`, `nth_sp_range_first`, `nth_sp_range_second`, `nth_sp_range_out_of_bounds`, `txbody_range_finds_inner_offsets`, `read_paragraphs_returns_each_paragraph_text`) + 12 integration (`tests/text_editor_test.rs` — `shape_has_text_frame_true_for_title_and_body`, `shape_has_text_frame_errors_for_out_of_range`, `set_shape_text_replaces_paragraphs`, `set_title_text_updates_only_first_paragraph`, `add_paragraph_appends_at_end`, `add_paragraph_with_format_emits_level_and_bold`, `shape_paragraphs_reads_back_what_was_written`, `edit_round_trips_through_save`, `edit_does_not_break_package_validation`, `has_text_frame_predicate_via_parsed_slide`, `edit_returns_error_for_shape_without_txbody`, `edit_handles_unicode_text`). Zero-repair gate confirmed: `edit_does_not_break_package_validation` saves the edited deck and asserts `validate_package_bytes()` reports zero errors. UTF-8 verified: `edit_handles_unicode_text` round-trips `🎉 Unicode round-trip` through `set_shape_text`.
- [ ] **Fuzzing tests for PPTX parsing** — listed in ARCHITECTURE "Future Enhancements"; `cargo-fuzz` target over `oxml::PresentationReader::open`. Success criterion: `cargo +nightly fuzz run read_pptx -- -max_total_time=300` produces no panics on real-world + mutated inputs.
- [ ] **Property-based testing for generation** — `proptest` over `SlideContent` builders to assert that randomly constructed decks always pass `validate_package_bytes` (catches shape-of-the-state regressions the hand-written tests miss). Success criterion: 1000 random deck generations all pass structural validation.

### Backlog items seeded by competitive intel (2026-10-06)

Fresh sweep of python-pptx (`user/quickstart.html`, `user/presentations.html`, `user/charts.html`) + pptxgenjs README/docs + Aspose.Slides Developer Guide surfaced five new gaps. Sources via webfetch (web2md MCP unavailable in this environment; per AGENTS.md Step 8 fallback is fine).

- [x] **Chart data labels + legend + axis customization** — `ChartBuilder::legend(LegendConfig)` (show / position — `Right` / `Left` / `Top` / `TopRight` / `Bottom`), `value_axis(AxisConfig)` / `category_axis(AxisConfig)` (min / max / major_unit / minor_unit / log_scale / number_format / tick_label_size_pt / tick_label_bold / tick_label_rotation_deg), `data_labels(DataLabelsConfig)` (show_value / show_category_name / show_percentage / show_series_name / position — `BestFit` / `Center` / `InsideBase` / `InsideEnd` / `OutsideEnd` / `Left` / `Right` / `Top` / `Bottom`). Convenience presets: `LegendConfig::shown()` / `::hidden()` / `::shown().with_position(Bottom)`; `DataLabelsConfig::show_values()` (InsideEnd) / `::show_percentages_outside()` (OutsideEnd). Defaults preserve backwards compatibility — `ChartBuilder::new(...)` without any `.legend()` / `.data_labels()` emits the same XML as before. Tests: 11 inline (`src/generator/charts/data.rs::tests` — `test_legend_position_str`, `test_label_position_str`, `test_axis_log_scale_rejects_out_of_range`, `test_axis_tick_label_rotation_wraps`, `test_data_labels_presets`, `test_chart_default_legend_is_shown_right`; `src/generator/charts/builder.rs::tests` — `test_chart_builder`, `test_chart_builder_legend_hidden`, `test_chart_builder_axis_min_max`, `test_chart_builder_data_labels_preset`) + 14 integration (`tests/chart_customization_test.rs` — `default_chart_emits_legend_block_at_right`, `legend_hidden_emits_no_legend_block`, `legend_position_bottom_emits_correct_val`, `value_axis_min_max_emits_c_min_max`, `value_axis_log_scale_emits_c_log_scale`, `value_axis_number_format_overrides_general`, `value_axis_major_unit_emits_c_major_unit`, `data_labels_show_values_emits_c_dlbls_with_show_val`, `data_labels_show_percentages_emits_c_show_percent`, `data_labels_off_emits_no_block`, `all_customizations_together_round_trip`, `chart_customization_does_not_break_package_validation`, `label_position_str_renders_all_variants`, `legend_position_str_renders_all_variants`). `validate_package_bytes()` regression guard confirmed. Public re-exports added to `src/generator/charts/mod.rs`.

- [x] **`Shape::has_text_frame()` predicate + `Shape::text()` setter** — python-pptx exposes `_BaseShape.has_text_frame` to query whether a shape can hold text, and `_BaseShape.text` as a writable shortcut that deletes all paragraphs except the first then sets text. ppt-rs has `Shape::with_text()` (builder) but no runtime predicate or short-form setter on the imported shape. Implemented as part of the TextFrame API: `ParsedShape::has_text_frame()` (new `has_text_frame: bool` field on the struct, populated by `SlideParser::parse_shape` only when `<p:txBody>` was found — distinguishes "shape with empty text frame" from "shape with no text frame" like a picture or connector) + `PresentationEditor::set_shape_text(slide, shape, text)` (drops all existing paragraphs, writes a single replacement). 9 inline + 12 integration tests cover both ends (see the TextFrame API entry above).

- [x] **`Presentation::extract_text()` helper** — added `extract_slide_texts_from_bytes(&bytes) -> Result<Vec<Vec<String>>, PptxError>` plus `PresentationReader::open_bytes(&[u8])` for byte-buffer import. Returns per-slide paragraphs; no XML namespaces leak. Use case: RAG/embedding, search/indexing, content extraction for testing. Tests: 3 inline (`oxml/presentation.rs::tests::test_extract_slide_texts_per_slide`, `test_open_bytes_round_trip`, `test_extract_slide_texts_rejects_invalid_bytes`) + 5 integration (`tests/extract_text_test.rs` — returns_one_inner_vec_per_slide, preserves_title_and_bullets_per_slide, does_not_leak_xml_to_caller, reader_api_parity, invalid_bytes_produce_error_not_panic). Docs: README bullet, SPEC "Reading & Editing" snippet, SPEC version-history note.

- [ ] **Table auto-paging** — pptxgenjs has `slide.addTable(rows, {autoPage: true, autoPageRepeatHeader: true, autoPageHeaderRows: 1, autoPageCharWeight, autoPageLineWeight, newSlideStartY})` (docs/api-tables). When a table overflows the slide, pptxgenjs auto-spawns slides with the header repeated. ppt-rs `TableBuilder` always fits the table to its declared size — long tables overflow or clip rather than paginating. Success criterion: `TableBuilder::new(...).auto_page(true).header_rows(1).build()` produces a deck where a 100-row table spans multiple slides, each starting with the header row.

- [ ] **YouTube embeds** — pptxgenjs has `slide.addMedia({type: 'youtube', link: 'youtube-id', ...})` (README "YouTube embeds"). ppt-rs `SlideContent::add_video` accepts a local file path; there's no path for `https://youtu.be/...` → PowerPoint video frame. This is a unique pptxgenjs feature and a common user request ("show a YouTube clip in my deck"). Success criterion: `slide.add_youtube("https://youtu.be/<id>")` produces a deck where PowerPoint offers to play the linked video when the slide is opened.


## Performance Targets

| Metric | Current | Target |
|--------|---------|--------|
| Generation speed | ~1000 slides/sec | Maintain |
| Memory (100 slides) | ~2-5 MB | < 10 MB |
| Memory (1000 slides, lazy) | ~10 MB | < 50 MB |
| Binary size | ~500 KB | < 1 MB |
| Test suite | < 1 sec | Maintain |

## Scorecard (2026-10-06, post v0.2.36 docs_sync gate + extract_text feature)

| Dimension | Score | Evidence |
|---|---|---|
| Correctness & coverage | 5 | `cargo test --all-features --no-fail-fast` green; 797 inline + 513 integration + 45 doctests + 5 tokio::test = 1360 functions; `cargo build --release --all-features` clean; package-validation zero-repair gate on every generated deck (debug builds). New chart customization: 11 inline + 14 integration tests cover legend (show/position), axis (min/max/major_unit/log_scale/number_format/tick_label_font/rotation), and data labels (show_value/show_percentage/position). |
| PowerPoint fidelity | 5 | `validate_package_bytes()` exported in `lib.rs:69` and used as `debug_assert_package_valid` on every generated deck (`src/generator/builder.rs`); 7 layouts, embedded Excel workbooks, slide-master `p:txStyles`, handout master; pdfrs image/PDF export no-LibreOffice path; media + comments round-trip + sections. **Placeholder fix (v0.2.36 follow-up)**: title/body shapes now emit `<p:ph type="title"/>` / `<p:ph type="body" idx="1"/>` so PowerPoint recognizes them as filling the layout's intended slots. **Chart customization (v0.2.36 follow-up)**: every chart now tunable — legend (show/position), axes (min/max/log scale/number format/tick label font/rotation), data labels (show value/percentage/category name + position); defaults preserve prior emission. |
| Maintainability | 5 | `cargo clippy --all-features --all-targets -- -D warnings` exits 0 (lib, bins, tests, examples) — clippy promoted to a CI gate. v0.2.36 closed the 53-warning backlog. No `#[allow(dead_code)]` or `#[allow(unused_*)]` in `src/`. `generate_title_shape` refactored to `(TitleContent, TitlePlacement, align, ph_type)` parameter-object form per MEMORY clippy pattern. `nth_sp_range` uses char-aware iteration so multi-byte UTF-8 (e.g. `•`, `🎉`) doesn't panic. Chart emitter uses parameter blocks (`scaling_block` / `unit_block` / `numfmt_block` / `tick_label_txpr_block` / `generate_data_labels_block`) so each config field is added in one place. |
| Docs alignment | 5 | `tests/docs_sync.rs` enforces 7 drift hotspots — all green after the chart customization work (test counts bumped 1287 → 1310; README/SPEC/ARCHITECTURE/TODO all updated). Chart customization documented in README "Editable charts" bullet, SPEC v0.2.36 changelog row. |
| Wiring & ergonomics | 5 | 18 `pub use` in `src/lib.rs` (incl. `ParagraphEdit`); +5 chart config types in `src/generator/charts/mod.rs` (`AxisConfig`, `DataLabelsConfig`, `LabelPosition`, `LegendConfig`, `LegendPosition`); 12 MCP tools, 11 CLI commands, 7 HTTP routes, 3 binaries, 6 feature flags, 41 example `.rs` files — all wired and re-exported. |
| Footprint | 4 | 17 direct deps (4 required + 13 optional, feature-gated); 5 dev-deps (`criterion`, `http-body-util`, `serde_json`, `serial_test`, `tower`); release profile uses `lto = true` + `strip = true`. No new heavy deps since v0.2.30. Stable at 4. |
| **Total** | **29/30** | |

**Weakest dimension = Footprint (4)**. Stable: minor; revisit only if/when a heavyweight dep enters the graph. TextFrame editing API shipped on the first try against the docs_sync gate (counts + version sync + drift hot-spots all in sync on the same commit).

## Technical Debt

### Code Quality
- [x] Profile memory usage with large presentations (100+ slides)
- [x] Review XML generation patterns — some modules use string concat instead of structured builders
- [x] Consolidate table cell formatting logic (potential DRY opportunity)
- [x] Modularize image effects XML generation

### Refactoring Opportunities
- [x] Extract common validation patterns into `core::validation` module
- [x] Unify error message formatting across modules
- [x] Consider builder pattern consolidation for Shape/Table/Chart builders

## Completed

<details>
<summary>v0.2.36 — Clippy `--all-features --all-targets -D warnings` Green + Docs Sync Hardening + Per-Slide Text Extraction</summary>

- **Mechanically auto-fixed lints (since prior TODO seed)**: 34 `collapsible_if` + 3 `collapsible_match` in `src/`; 6 `useless_vec` (CLI parser tests `vec![String;..]` → `[String;..]`); 3 `len_zero` (`assert!(x.len() >= 1)` / `len() > 0` → `!x.is_empty()`); 1 `ptr_arg` (`&PathBuf` → `&Path` in `tests/media_test.rs`); 1 `unused_imports` (`Position` in `tests/image_url_test.rs`).
- **No `too_many_arguments` survived** — either parameter objects already existed, or builder consolidation eliminated them.
- **Quality gate**: `cargo clippy --all-features --all-targets -- -D warnings` exits 0. Safe to promote `-D warnings` to CI.
- **Tests**: 772 inline / 480 integration = 1252 source-declared + 7 `tokio::test` = 1302 functions; all green under `--all-features`.

### Follow-up — docs_sync gate hardening (2026-10-06)

- `tests/docs_sync.rs` now enforces **7** drift hotspots end-to-end: AGENTS module list, ARCHITECTURE module list, TODO inline count, TODO integration count (gate self-excludes), README install version vs `src/lib.rs::VERSION`, MCP tool count vs MEMORY `### MCP tool count: N`, CLI variant set (case-insensitive) in README/ARCHITECTURE. README/ARCHITECTURE updated to mention `pdf2ppt` (gate caught real drift on first run).

### Follow-up — `extract_slide_texts_from_bytes` (2026-10-06)

- New helper: `ppt_rs::extract_slide_texts_from_bytes(&bytes) -> Result<Vec<Vec<String>>, PptxError>` returns per-slide paragraphs from a PPTX byte buffer without leaking OOXML details to the caller. Use case: RAG/embedding, search/indexing, content extraction for testing.
- `PresentationReader::from_bytes(&[u8])` for byte-buffer import (parallel to the existing path-based `PresentationReader::open`).
- Tests: 3 inline (`oxml/presentation.rs::tests`) + 5 integration (`tests/extract_text_test.rs`).
- Tests: 775 inline / 481 integration = 1256 source-declared (after this work); `cargo test --test docs_sync` exits 0.
- Docs sync: README bullet, SPEC "Reading & Editing" snippet, SPEC version-history note (v0.2.36 row extended).

### Follow-up — Layout placeholder XML (2026-10-06)

- The active slide-XML emitter (`src/generator/slide_xml/layouts.rs`) was producing bare `<p:nvPr/>` for title and body shapes — meaning PowerPoint saw them as regular shapes, not as fills of the layout's intended title/body placeholders. python-pptx's `slide.shapes.title` couldn't find them.
- Fix: `generate_title_shape` extended with a `ph_type` parameter and used `TitleContent` / `TitlePlacement` parameter objects to avoid `too_many_arguments`. Body shape XML in `create_title_and_content_slide`, `create_title_and_big_content_slide`, and `create_two_column_slide` now emits `<p:ph type="title"/>` / `<p:ph type="body" idx="1"/>` (and `idx="2"` for the right column).
- Layouts covered: `TitleOnly` (title), `CenteredTitle` (`ctrTitle`), `SectionHeader` (title), `TitleAndContent` (title + body idx="1"), `TitleAndBigContent` (title + body idx="1"), `TwoColumn` (title + body idx="1" + body idx="2").
- Tests: 4 inline (`generator/slide_xml/mod.rs::tests`) + 6 integration (`tests/placeholder_api_test.rs` — covers all 6 layouts, the `Presentation::build` pipeline, and a `package_validation_still_passes_with_placeholders` zero-repair guard).
- Tests: 779 inline / 487 integration = 1266 source-declared (after this work); `cargo test --test docs_sync` exits 0.
- Docs sync: README layouts bullet, SPEC layout-table footnote, SPEC v0.2.36 changelog row extended.
- Zero-repair gate confirmed: `validate_package_bytes()` still passes after the placeholder XML addition (regression test asserts no errors).

### Follow-up — TextFrame runtime editing (2026-10-06)

- New module `src/oxml/text_editor.rs`: `ParagraphEdit` builder (level / bold / italic / font-size), UTF-8 char-aware `nth_sp_range` / `txbody_range` byte walkers (depth counter for nested `<p:sp>`), `read_paragraphs` for diagnostics. `ParsedShape::has_text_frame: bool` distinguishes "no txBody" from "empty txBody" — `Vec::is_empty()` can't.
- `PresentationEditor`: `shape_has_text_frame(slide, shape) -> bool`, `set_shape_text(slide, shape, text)` (drops all paragraphs, writes replacement), `add_paragraph(slide, shape, text)`, `add_paragraph_with_format(slide, shape, ParagraphEdit)` (level/bold/italic/size), `set_title_text(slide, text)`, `shape_paragraphs(slide, shape) -> Vec<String>` (read-back). python-pptx parity: `_BaseShape.has_text_frame` / `_BaseShape.text` / `TextFrame.add_paragraph()` / `Paragraph.level` / `Paragraph.font.bold`.
- Tests: 9 inline (`src/oxml/text_editor.rs::tests`) + 12 integration (`tests/text_editor_test.rs` — covers all 6 methods, `has_text_frame` predicate, UTF-8 round-trip `🎉 Unicode round-trip`, save-then-reopen, zero-repair gate guard).
- Tests: 788 inline / 499 integration = 1287 source-declared (after this work); `cargo test --test docs_sync` exits 0.
- Docs sync: README bullet (TextFrame editing), SPEC "Reading & Editing" snippet extended, SPEC v0.2.36 changelog row extended.

### Follow-up — Chart customization (2026-10-06)

- New config types in `src/generator/charts/data.rs`: `LegendConfig` (show / position — Right / Left / Top / TopRight / Bottom), `AxisConfig` (min / max / major_unit / minor_unit / log_scale / number_format / tick_label_size_pt / tick_label_bold / tick_label_rotation_deg), `DataLabelsConfig` (show_value / show_category_name / show_percentage / show_series_name / position — BestFit / Center / InsideBase / InsideEnd / OutsideEnd / Left / Right / Top / Bottom). Convenience presets: `LegendConfig::shown()` / `::hidden()`; `DataLabelsConfig::show_values()` (InsideEnd) / `::show_percentages_outside()` (OutsideEnd). Defaults preserve prior emission — `ChartBuilder::new(...)` without `.legend()` / `.data_labels()` emits the same XML as before.
- `ChartBuilder` now exposes `.legend(LegendConfig)` / `.value_axis(AxisConfig)` / `.category_axis(AxisConfig)` / `.data_labels(DataLabelsConfig)`. Public re-exports added to `src/generator/charts/mod.rs`.
- `src/generator/charts/xml.rs` refactored: `chart_part_footer(chart)` takes the chart (was a static); `scaling_block` / `unit_block` / `numfmt_block` / `tick_label_txpr_block` / `generate_data_labels_block` helpers make adding new config fields one-line. Plot-level `<c:dLbls>` block emitted only when at least one data-label flag is on (preserves "all off" default). Per-series `<c:dLbls>` blocks follow the plot-level config.
- Tests: 11 inline (`src/generator/charts/data.rs::tests` + `src/generator/charts/builder.rs::tests`) + 14 integration (`tests/chart_customization_test.rs` — covers legend show/hide/position, axis min/max/log_scale/number_format/major_unit, data labels show_values/show_percentages/all-off, all-together round-trip, `validate_package_bytes` regression guard, `LegendPosition::as_str()` / `LabelPosition::as_str()` mappings).
- Tests: 797 inline / 513 integration = 1310 source-declared (after this work); `cargo test --test docs_sync` exits 0.
- Docs sync: README "Editable charts" bullet, SPEC v0.2.36 changelog row extended.

</details>

<details>
<summary>v0.2.35 — Comment Import Round-Trip</summary>

- **`oxml/comments.rs`**: parse `commentAuthors.xml` and slide comment parts
- **`PresentationReader::comment_authors()` / `slide_comments()`** → `import_pptx` restores authors + per-slide comments
- **Test**: `tests/comments_test.rs::test_comments_import_round_trip`

</details>

<details>
<summary>v0.2.34 — REST/HTTP API</summary>

- **`http` feature** + `ppt_http` binary: axum server with `/health`, `/v1/generate`, `/v1/markdown`, `/v1/validate`, `/v1/export/markdown|html`, `/v1/diff`
- **Tests**: inline router tests + `tests/http_test.rs`

</details>

<details>
<summary>v0.2.33 — CLI Markdown Export & MCP diff_pptx</summary>

- **CLI `export --format md`**: Markdown export + `.md`/`.markdown` extension detection; `--group-by-sections` works for HTML and Markdown
- **MCP `diff_pptx`**: semantic compare with optional JSON report (12 tools total)
- **WASM**: marked out of scope per project direction

</details>

<details>
<summary>v0.2.32 — Criterion Benchmark Suite</summary>

- **`benches/pptx_benchmarks.rs`**: Criterion groups for eager generation, lazy generation, and compression at 10/50/100 slides
- **Smoke test**: `tests/benchmark_smoke_test.rs` verifies generation + compression fixture path
- **Run**: `cargo bench` (not part of `cargo test`)

</details>

<details>
<summary>v0.2.30 — PPTX Semantic Diff</summary>

- **`diff` module**: `compare_presentations`, `compare_pptx_paths`, `PresentationDiff` with slide-level Added/Removed/Modified + field changes (title, bullets, images, charts, tables, media)
- **CLI `pptcli diff old.pptx new.pptx [--json]`**: exit 0 when semantically equal, 1 when different

</details>

<details>
<summary>v0.2.29 — Section Import & CLI Export Grouping</summary>

- **Section import**: `PresentationReader::sections()` parses `<p14:sectionLst>` from `presentation.xml`; `import_pptx` / `from_path` preserve sections in `PresentationSettings`
- **CLI `--group-by-sections`**: `pptcli export … --format html --group-by-sections` uses `HtmlExportOptions::with_section_grouping(true)`

</details>

<details>
<summary>v0.2.28 — Compression Comments & MCP Section Export</summary>

- **`remove_comments` wired**: compression now strips `ppt/comments/*.xml`, `commentAuthors.xml`, slide/presentation rels, and content-type overrides (was documented in SPEC but dead code)
- **MCP `export_pptx`**: optional `group_by_sections` flag for markdown/html export
- **`Presentation::save_as_html_with_options`**: API parity with markdown export options

</details>

<details>
<summary>v0.2.27 — Comments, Media Embedding & Section Export</summary>

- **Comments round-trip**: `SlideContent::add_comment` / `Presentation::add_comment_author` → `ppt/comments/commentN.xml` + `ppt/commentAuthors.xml` (eager + lazy paths, package validation)
- **Video & audio embedding**: `add_video`/`add_audio` wired through eager + lazy generation — `ppt/media/mediaN.*` parts, dual link/embed rels, poster frames, content-type Defaults; compression preserves rel-referenced media
- **Section-aware export**: `Presentation::add_section` → `<p14:sectionLst>`; opt-in section grouping in Markdown export and section labels in HTML export
- **Lazy content-types fix**: append helpers were silent no-ops when `</Types>` was pushed before them
- **Pure-Rust image export**: PNG/JPEG via `pdfrs` PDF render + `pdftoppm` (no LibreOffice)

</details>

<details>
<summary>v0.2.26 — Chart Validation & MCP Tool Expansion</summary>

- **Chart build-time validation**: `ChartBuilder::build()` now returns `Result<Chart>` and calls `core::validation::validate_chart()` — enforces non-empty categories, non-empty series, and series-length == category-count. Surfaced and fixed 6 silently-broken chart call sites in tests/examples.
- **MCP tool expansion**: +3 tools (`compress_pptx`, `repair_pptx`, `apply_theme`) = 11 total
- **rmcp 2.2.0 migration**: fixed pre-existing build break from v0.2.20 upgrade — `Content`→`ContentBlock`, added `server`+`macros` features, `mcp` feature now enables `cli`
- **MEMORY.md**: scaffolded institutional knowledge repository

</details>

<details>
<summary>v0.2.25 — Dependency Updates & URL Validation</summary>

- **Dependency refresh**: crate updates across the dependency tree
- **URL validation simplified**: streamlined validation in `src/web2ppt/fetcher.rs`
- **Markdown parser**: minor cleanup in `src/cli/markdown/parser.rs`

</details>

<details>
<summary>v0.2.24 — Web2MD Bridge (HTML→Markdown→Slides)</summary>

- **`web2md` feature**: HTML→Markdown (via the `web2md` crate)→Slides pipeline
- **`--web2md` CLI flag** on `web2ppt` routes HTML through the Markdown pipeline
- **`src/web2ppt/web2md_bridge.rs`**: bridge implementation (`url_to_pptx_via_web2md`, `html_to_pptx_via_web2md`)
- **`tests/web2md_bridge_test.rs`**: end-to-end bridge tests

</details>

<details>
<summary>v0.2.23 — Pure-Rust PDF Export via pdfrs</summary>

- **`pdf-native` feature**: `pdfrs`-backed PDF generation with no LibreOffice dependency
- **API**: `export_to_pdf` / `export_to_pdf_bytes`, `PdfExportOptions`, `PdfOrientation`
- **Code-style refactor**: `api.rs`, `core::package_validation` rules, examples cleanup

</details>

<details>
<summary>v0.2.22 — API Guide Expansion & CLI/Examples Refactor</summary>

- **`API_GUIDE.md`** expanded with presentation builder, charts, images, themes/templates, export/compression, editor, validation
- **CLI commands** (`src/cli/commands.rs`, `src/bin/pptcli.rs`) enhanced
- **Examples refactored**: `dimension_demo`, `shapes_demo`, `table_generation`, `proper_pptx`, `templates_demo`
- **Dead code removed**: `cli/markdown/mermaid/flowchart` / `types` stubs

</details>

<details>
<summary>v0.2.21 — Lean Audit, API Docs & Structured Validate</summary>

- **Dead code removed**: unused image placeholder / bullet paragraph helpers, unused Mermaid gradient/transparent shape builders, unused `DiagramElements.grouped`, stub `web2ppt` CLI handler, dead example helpers
- **`escape_xml`**: single-pass with fast path for clean strings; new `append_escape_xml` for buffer writers (`XmlWriter` uses it)
- **`XmlWriter`**: dropped unused indent fields
- **Package validation**: removed duplicate unused `slide_parts` computation; `warning_count()` on reports
- **Examples**: migrated to simplified API (`.fill(hex())` / `.stroke()` / `.text()`, `QuickTable`, `table_from_data`)
- **API_GUIDE.md**: Presentation builder, charts, images, themes/templates, export/compression, editor, validation
- **`pptcli validate`**: structured human report + `--json` for CI
- Warnings cleared (lib + shared test helpers)

</details>

<details>
<summary>v0.2.20 — rmcp Upgrade, CLI Validation & Editor Enhancements</summary>

- **rmcp dependency upgraded**; `mcp/mod.rs` tool wiring updated
- **Core validation integrated into CLI** — `pptcli validate` delegates to `core::validation` / `core::package_validation`
- **`PresentationEditor` enhancements** (`src/oxml/editor.rs`): duplicate slide, reorder slides, insert slide at index, batch replace
- **Embedded fonts + ink annotations wiring**: `slide_content/embedded_fonts.rs`, `slide_content/ink_annotations.rs` integrated into the generator
- **`AGENTS.md`** introduced — agent development loop

</details>

<details>
<summary>v0.2.19 — PowerPoint Zero-Repair Compatibility Gate</summary>

Landed the full "Compatibility & Trust" and "Layout & Structure" proposed tracks so generated decks open in PowerPoint without a repair prompt.

- **Package validation engine** (`src/core/package_validation/`):
  - `validate_package()` / `validate_package_bytes()` returning a structured `PackageValidationReport`
  - `PackageValidationIssue` with `ValidationCategory` (MissingPart, Relationship, ContentType, Presentation, SlideMaster, Slide, Chart, Xml, Theme) and `ValidationSeverity` (Warning / Error)
  - `REQUIRED_PACKAGE_PARTS` constant drives missing-part checks
  - Sub-modules: `rules`, `rels`, `context`, `report`
  - Debug builds `debug_assert!` every generated deck passes validation (`builder::debug_assert_package_valid`)
- **Legacy compat wrapper** (`src/core/powerpoint_compat.rs`):
  - `validate_powerpoint_structure()` / `CompatReport` (delegates to package validation)
- **Multiple slide layouts** (`src/generator/layout_parts.rs`):
  - 7 layouts emitted on slide master 1 (`STANDARD_LAYOUT_COUNT = 7`)
  - `SlideLayout::layout_number()` maps each variant to `slideLayoutN.xml`
  - Footer / slide-number / date placeholders on the slide master
- **Per-slide layout selection** — `SlideContent::with_layout()` respected end-to-end; layout index resolves against template layout count
- **Template-based generation** (`src/generator/template.rs`):
  - `PptxTemplate::load()` / `from_package()` clones theme / masters / layouts / tableStyles
  - `create_pptx_with_template()`, `PresentationSettings::template(path)`
  - CLI `pptcli create --template deck.pptx`
- **Chart Excel workbook embedding** (`src/generator/charts/embedding.rs`):
  - Packages `ppt/embeddings/Microsoft_Excel_SheetN.xlsx` + chart `rels` so charts are editable in PowerPoint
- **Handout master packaging** — emits handout master part + rels when `PrintSettings` uses handouts
- **Slide master completeness** — `p:txStyles`, full Office theme template, notes master theme parity
- **Package relationship ordering** — `presentation.xml.rels` ordered master → presProps → viewProps → theme → tableStyles → slides (`create_presentation_rels_xml_full`)
- **Auxiliary generators** — `media_registry`, `package_cache`, `estimate_output_capacity` for reuse / pre-sizing
- **Tests**: `package_validation_test`, `powerpoint_compat_test`, `layouts_packaging_test`, `repair_compare_test`, `tests/common/`

</details>

<details>
<summary>v0.2.18 — Validation, Messages & Builder Placement</summary>

- **`core::validation`** — shared validators, `REQUIRED_PARTS_*` constants, `ValidationIssue`, XML well-formedness checks
- **`exc::messages`** — unified error message formatting (`slide_not_found`, `missing_part`, etc.)
- **`core::placement`** — `ElementPlacement` consolidated into `ChartBuilder`, `TableBuilder`, `ImageBuilder`
- **Migrated consumers** — `oxml/repair`, `api`, `oxml/editor`, `oxml/presentation`, `parts/media`, integration/import tests
- **Tests**: `tests/validation_test.rs`

</details>

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
  - `export_to_images()` - Export slides to PNG/JPEG (pure-Rust `pdfrs` PDF render + `pdftoppm` rasterization, no LibreOffice)
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
  - v0.2.26: +3 tools (`compress_pptx`, `repair_pptx`, `apply_theme`) = 11 total; rmcp 2.2.0 API migration (`Content`→`ContentBlock`, `server`+`macros` features)
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

- **HTML Parser Documentation** (`docs/HTML_PARSERS.md`):
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
