# HTML Parsers in ppt-rs

Two parsers for two jobs:

1. **Basic HTML Parser** (`src/import/html/`) — dependency-light, for HTML strings you already have
2. **Web Scraper Parser** (`src/web2ppt/parser.rs`) — fetches and cleans live web pages

## Basic HTML Parser (`import::html`)

**Location:** `src/import/html/` (`mod.rs` — `HtmlParseOptions` + `Html2Ppt`; `tokenizer.rs`, `style.rs`, `parser.rs` — split in v0.2.37)

State-machine parser, no external dependencies. Handles standard HTML5
elements, inline CSS, tables, images, code blocks, and speaker notes; converts
directly to `SlideContent`.

```rust
use ppt_rs::import::{parse_html, parse_html_with_options, HtmlParseOptions};

let html = r#"
    <h1>My Presentation</h1>
    <ul><li>Item 1</li><li>Item 2</li></ul>
"#;

let slides = parse_html(html)?;
// or: parse_html_with_options(html, HtmlParseOptions::new().max_slides(10))
```

`HtmlParseOptions`: `max_slides`, `max_bullets`, `include_code`,
`include_tables`, `include_images`.

## Web Scraper Parser (`web2ppt::parser`)

**Location:** `src/web2ppt/parser.rs` (`web2ppt` feature — `reqwest` + `scraper`)

Fetches a URL, detects the main content area, strips navigation/ads/footers,
extracts metadata (title, description), links, and images, then groups content
by headings for slide generation.

```rust
use ppt_rs::web2ppt::{url_to_pptx_with_options, ConversionOptions, Web2PptConfig};

let config = Web2PptConfig::new()
    .max_slides(20)
    .group_by_headings(true);
let pptx = url_to_pptx_with_options(
    "https://example.com/article",
    config,
    ConversionOptions::new().with_source_url(true),
)?;
```

`Web2PptConfig`: `max_slides`, `max_bullets_per_slide`, `include_images`,
`include_tables`, `include_code`, `user_agent`, `timeout_secs`,
`group_by_headings`, `extract_links`.

## Comparison

| | Basic Parser | Web Scraper |
|---|---|---|
| Input | HTML strings | Live URLs |
| Dependencies | None | `reqwest` + `scraper` (`web2ppt` feature) |
| Content cleaning | Manual | Automatic (nav/ads removed) |
| Metadata / links | Basic | Advanced |
| Use when | Trusted, well-structured HTML | Messy real-world pages |
