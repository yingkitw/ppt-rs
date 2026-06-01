# HTML Parsers in ppt-rs

This document explains the two HTML parsers available in ppt-rs and when to use each one.

## Overview

ppt-rs includes two distinct HTML parsers designed for different use cases:

1. **Basic HTML Parser** (`src/import/html.rs`) - Simple, dependency-light parsing
2. **Web Scraper Parser** (`src/web2ppt/parser.rs`) - Advanced web content extraction

## Basic HTML Parser (`import::html`)

**Location:** `src/import/html.rs`

**Purpose:** Convert simple HTML strings directly to PowerPoint slides with minimal dependencies.

**Use Cases:**
- Converting HTML strings from user input
- Processing simple HTML files
- Embedded HTML content in applications
- When you want zero external dependencies for HTML parsing

**Features:**
- ✅ No external dependencies (state-machine parser)
- ✅ Handles standard HTML5 elements
- ✅ Inline CSS style parsing
- ✅ Direct conversion to SlideContent
- ✅ Fast and lightweight
- ✅ Supports tables, images, code blocks, speaker notes

**Limitations:**
- ❌ No web fetching capabilities
- ❌ Limited content extraction heuristics
- ❌ No automatic content cleaning
- ❌ No JavaScript execution
- ❌ Manual DOM navigation

**Example:**
```rust
use ppt_rs::import::{parse_html, HtmlParseOptions};

let html = r#"
    <h1>My Presentation</h1>
    <p>First bullet point</p>
    <ul>
        <li>Item 1</li>
        <li>Item 2</li>
    </ul>
"#;

let options = HtmlParseOptions::new()
    .max_slides(10)
    .include_images(true);

let slides = parse_html_with_options(html, options)?;
```

## Web Scraper Parser (`web2ppt::parser`)

**Location:** `src/web2ppt/parser.rs`

**Purpose:** Extract meaningful content from web pages with intelligent content detection and cleaning.

**Use Cases:**
- Converting live web pages to presentations
- Extracting content from complex websites
- When you need automatic content detection
- Processing web pages with navigation, ads, etc.

**Features:**
- ✅ Intelligent content extraction
- ✅ Automatic detection of main content areas
- ✅ Removes navigation, ads, footers
- ✅ Meta data extraction (title, description)
- ✅ Content grouping by headings
- ✅ Image extraction with alt text
- ✅ Link discovery and extraction
- ✅ Responsive design handling

**Requirements:**
- Requires `scraper` crate dependency
- Works with `Web2PptConfig` for configuration
- Designed for use with `WebFetcher`

**Example:**
```rust
use ppt_rs::web2ppt::{Web2Ppt, Web2PptConfig, ConversionOptions};

// Fetch and convert a web page
let config = Web2PptConfig::new()
    .max_slides(20)
    .group_by_headings(true);

let converter = Web2Ppt::with_config(config);
let options = ConversionOptions::new()
    .with_source_url(true);

// Requires web2ppt feature
#[cfg(feature = "web2ppt")]
{
    use ppt_rs::web2ppt::url_to_pptx_with_options;
    let pptx = url_to_pptx_with_options(
        "https://example.com/article",
        config,
        options
    )?;
}
```

## When to Use Which Parser

### Use Basic HTML Parser (`import::html`) when:
- ✅ You have HTML strings from trusted sources
- ✅ You need fast, simple conversion
- ✅ You want to avoid external dependencies
- ✅ The HTML is well-structured and predictable
- ✅ You're building an embedded solution

### Use Web Scraper Parser (`web2ppt::parser`) when:
- ✅ You're processing live web pages
- ✅ The HTML contains navigation, ads, or clutter
- ✅ You need intelligent content extraction
- ✅ You want automatic content cleaning
- ✅ You need meta data extraction
- ✅ You're building a web scraping tool

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     HTML Input                          │
└─────────────────────┬───────────────────────────────────┘
                      │
        ┌─────────────┴─────────────┐
        │                           │
        ▼                           ▼
┌───────────────────┐     ┌─────────────────────┐
│  Basic Parser     │     │  Web Scraper        │
│  (import::html)   │     │  (web2ppt::parser)  │
└─────────┬─────────┘     └──────────┬──────────┘
          │                           │
          │                           │
          ▼                           ▼
  ┌───────────────┐           ┌─────────────────┐
  │ Simple HTML   │           │ Web Fetcher     │
  │ Strings       │           │ + Config        │
  └───────┬───────┘           └────────┬────────┘
          │                            │
          └────────────┬───────────────┘
                       │
                       ▼
            ┌─────────────────────┐
            │  SlideContent       │
            │  (generator)        │
            └──────────┬──────────┘
                       │
                       ▼
              ┌─────────────────┐
              │  PowerPoint      │
              │  Presentation   │
              └─────────────────┘
```

## Comparison Table

| Feature | Basic Parser | Web Scraper |
|---------|-------------|-------------|
| Dependencies | None | scraper crate |
| Speed | Fast | Moderate |
| Content Detection | Manual | Automatic |
| Web Fetching | No | Yes |
| CSS Support | Inline styles | Smart extraction |
| Ad Removal | No | Yes |
| Navigation Handling | No | Yes |
| Meta Extraction | Basic | Advanced |
| Best For | Simple HTML | Web Pages |

## Migration Guide

If you're currently using one parser and want to switch to the other:

### From Basic to Web Scraper:
```rust
// Before (Basic Parser)
let slides = parse_html(html_string)?;

// After (Web Scraper)
let parser = WebParser::new();
let content = parser.parse(html_string, "https://example.com")?;
let converter = Web2Ppt::new();
let pptx = converter.convert(&content, &ConversionOptions::default())?;
```

### From Web Scraper to Basic:
```rust
// Before (Web Scraper)
let content = parser.parse(html, url)?;
let pptx = converter.convert(&content, &options)?;

// After (Basic Parser)
let slides = parse_html_with_options(html, HtmlParseOptions::default())?;
let pptx = create_pptx_with_content("Title", slides)?;
```

## Configuration Options

### Basic Parser (`HtmlParseOptions`)
- `max_slides`: Maximum slides to generate
- `max_bullets`: Maximum bullets per slide
- `include_code`: Include code blocks
- `include_tables`: Include tables
- `include_images`: Include images

### Web Scraper (`Web2PptConfig`)
- `max_slides`: Maximum slides to generate
- `max_bullets_per_slide`: Maximum bullets per slide
- `include_images`: Include images
- `include_tables`: Include tables
- `include_code`: Include code blocks
- `user_agent`: HTTP user agent
- `timeout_secs`: Request timeout
- `group_by_headings`: Group content by headings
- `extract_links`: Extract hyperlinks

## Performance Considerations

- **Basic Parser**: ~2-5x faster for simple HTML
- **Web Scraper**: Slower but produces cleaner output from complex pages
- **Memory**: Basic parser uses less memory
- **Network**: Only Web Scraper requires network access

## Conclusion

Both parsers serve different purposes:
- Use the **Basic Parser** for simple, fast HTML conversion
- Use the **Web Scraper** for intelligent web content extraction

Choose based on your specific use case and requirements.