//! Markdown to PowerPoint conversion
//!
//! This module provides functionality to parse markdown content
//! and convert it into PowerPoint slide structures.
//!
//! # Supported Features
//!
//! - **Headings**: `#` creates new slides
//! - **Bullet points**: `-`, `*`, `+` create bullet lists
//! - **Numbered lists**: `1.`, `2.` create numbered lists
//! - **Tables**: GFM-style tables with header styling
//! - **Code blocks**: Fenced code blocks with syntax highlighting
//! - **Mermaid diagrams**: Visual placeholders for 12 diagram types
//! - **Inline formatting**: Bold, italic, inline code
//! - **Images**: Placeholder shapes for images
//! - **Horizontal rules**: Create slide breaks
//! - **Speaker notes**: Blockquotes become speaker notes

mod mermaid;
mod parser;

pub use mermaid::MermaidType;
pub use parser::parse;

/// Parse markdown content into slides (convenience re-export)
pub fn parse_markdown(content: &str) -> Result<Vec<crate::generator::SlideContent>, String> {
    parser::parse(content)
}

/// Parse markdown from a file path (resolves relative image URLs)
pub fn parse_markdown_file(
    content: &str,
    path: &std::path::Path,
) -> Result<Vec<crate::generator::SlideContent>, String> {
    parser::parse_with_base(content, path.parent())
}
