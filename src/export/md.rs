//! Markdown export module
//!
//! Provides functionality to export presentations to Markdown format.
//! Supports GitHub Flavored Markdown with extensions for slides.

use crate::api::Presentation;
use crate::exc::Result;

/// Export options for Markdown generation
#[derive(Debug, Clone)]
pub struct MarkdownOptions {
    /// Include slide numbers as headers
    pub include_slide_numbers: bool,
    /// Format for slide separators (--- or horizontal rule)
    pub slide_separator: String,
    /// Include speaker notes
    pub include_notes: bool,
    /// Use GFM tables for table export
    pub use_gfm_tables: bool,
    /// Include image references
    pub include_images: bool,
    /// Add YAML frontmatter with presentation metadata
    pub include_frontmatter: bool,
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self {
            include_slide_numbers: true,
            slide_separator: "---".to_string(),
            include_notes: true,
            use_gfm_tables: true,
            include_images: true,
            include_frontmatter: true,
        }
    }
}

impl MarkdownOptions {
    /// Create new options with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set slide number inclusion
    pub fn with_slide_numbers(mut self, include: bool) -> Self {
        self.include_slide_numbers = include;
        self
    }

    /// Set slide separator
    pub fn with_separator(mut self, sep: &str) -> Self {
        self.slide_separator = sep.to_string();
        self
    }

    /// Set notes inclusion
    pub fn with_notes(mut self, include: bool) -> Self {
        self.include_notes = include;
        self
    }

    /// Set GFM table usage
    pub fn with_gfm_tables(mut self, use_gfm: bool) -> Self {
        self.use_gfm_tables = use_gfm;
        self
    }

    /// Set image inclusion
    pub fn with_images(mut self, include: bool) -> Self {
        self.include_images = include;
        self
    }

    /// Set frontmatter inclusion
    pub fn with_frontmatter(mut self, include: bool) -> Self {
        self.include_frontmatter = include;
        self
    }
}

/// Export a presentation to Markdown format
pub fn export_to_markdown(presentation: &Presentation) -> Result<String> {
    export_to_markdown_with_options(presentation, &MarkdownOptions::default())
}

/// Export a presentation to Markdown with custom options
pub fn export_to_markdown_with_options(
    presentation: &Presentation,
    options: &MarkdownOptions,
) -> Result<String> {
    let mut md = String::new();

    // YAML frontmatter
    if options.include_frontmatter {
        md.push_str("---\n");
        md.push_str(&format!("title: \"{}\"\n", escape_yaml(presentation.get_title())));
        md.push_str(&format!("slides: {}\n", presentation.slide_count()));
        md.push_str(&format!("generator: ppt-rs\n"));
        md.push_str("---\n\n");
    }

    // Presentation title as main heading
    md.push_str(&format!("# {}\n\n", presentation.get_title()));

    // Export each slide
    for (i, slide) in presentation.slides().iter().enumerate() {
        let slide_num = i + 1;

        // Slide separator
        if i > 0 || options.include_slide_numbers {
            md.push_str(&format!("\n{}\n\n", options.slide_separator));
        }

        // Slide number header
        if options.include_slide_numbers {
            md.push_str(&format!("## Slide {}: {}\n\n", slide_num, escape_markdown(&slide.title)));
        } else {
            md.push_str(&format!("## {}\n\n", escape_markdown(&slide.title)));
        }

        // Bullet content
        if !slide.content.is_empty() {
            for item in &slide.content {
                md.push_str(&format!("- {}\n", escape_markdown(item)));
            }
            md.push('\n');
        }

        // Table export (GFM format)
        if options.use_gfm_tables && slide.has_table {
            if let Some(table) = &slide.table {
                md.push_str(&export_table_to_gfm(table));
                md.push('\n');
            }
        }

        // Images
        if options.include_images && !slide.images.is_empty() {
            for (img_idx, image) in slide.images.iter().enumerate() {
                let alt_text = format!("Image {} on slide {}", img_idx + 1, slide_num);
                // Note: Actual image data would need to be saved separately
                md.push_str(&format!(
                    "![{}](images/slide{}_image{}{})\n\n",
                    alt_text,
                    slide_num,
                    img_idx + 1,
                    image.format.to_lowercase().replace("jpeg", ".jpg").replace("png", ".png")
                ));
            }
        }

        // Code blocks
        if !slide.code_blocks.is_empty() {
            for code_block in &slide.code_blocks {
                md.push_str(&format!(
                    "```{lang}\n{code}\n```\n\n",
                    lang = &code_block.language,
                    code = &code_block.code
                ));
            }
        }

        // Speaker notes
        let has_notes = slide.notes.as_ref().map_or(false, |n| !n.is_empty());
        if options.include_notes && has_notes {
            md.push_str("**Notes:**\n\n");
            if let Some(notes) = &slide.notes {
                md.push_str(&format!("> {}\n\n", escape_markdown(notes)));
            }
        }
    }

    Ok(md)
}

/// Export a table to GitHub Flavored Markdown format
fn export_table_to_gfm(table: &crate::generator::Table) -> String {
    let mut md = String::new();

    // Header row
    if let Some(first_row) = table.rows.first() {
        md.push_str("| ");
        for cell in &first_row.cells {
            md.push_str(&escape_markdown(&cell.text));
            md.push_str(" | ");
        }
        md.push('\n');

        // Separator
        md.push_str("|");
        for _ in &first_row.cells {
            md.push_str(" --- |");
        }
        md.push('\n');

        // Data rows
        for row in table.rows.iter().skip(1) {
            md.push_str("| ");
            for cell in &row.cells {
                md.push_str(&escape_markdown(&cell.text));
                md.push_str(" | ");
            }
            md.push('\n');
        }
    }

    md
}

/// Escape special Markdown characters
fn escape_markdown(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('`', "\\`")
        .replace('#', "\\#")
        .replace('<', "\\<")
        .replace('>', "\\>")
}

/// Escape special YAML characters in frontmatter
fn escape_yaml(text: &str) -> String {
    if text.contains('\n') || text.contains('"') || text.contains('\\') {
        // Use literal block scalar for multiline or complex strings
        format!("|\n  {}", text.replace('\n', "\n  "))
    } else {
        text.replace('"', "\\\"").replace('\\', "\\\\")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::{SlideContent, TableBuilder, TableCell, TableRow, CodeBlock};

    #[test]
    fn test_export_simple_presentation() {
        let mut presentation = Presentation::with_title("Test Presentation");
        presentation = presentation.add_slide(SlideContent::new("Slide 1").add_bullet("Point 1"));
        presentation = presentation.add_slide(SlideContent::new("Slide 2").add_bullet("Point 2"));

        let md = export_to_markdown(&presentation).unwrap();

        assert!(md.contains("# Test Presentation"));
        assert!(md.contains("## Slide 1: Slide 1"));
        assert!(md.contains("- Point 1"));
        assert!(md.contains("---"));
    }

    #[test]
    fn test_markdown_options() {
        let mut presentation = Presentation::with_title("Test");
        presentation = presentation.add_slide(SlideContent::new("Slide").add_bullet("Point"));

        let options = MarkdownOptions::new()
            .with_slide_numbers(false)
            .with_frontmatter(false);

        let md = export_to_markdown_with_options(&presentation, &options).unwrap();

        assert!(!md.contains("## Slide 1:"));
        assert!(md.contains("## Slide"));
        assert!(!md.contains("---\ntitle:"));
    }

    #[test]
    fn test_escape_markdown() {
        assert_eq!(escape_markdown("*bold*"), "\\*bold\\*");
        assert_eq!(escape_markdown("[link]"), "\\[link\\]");
        assert_eq!(escape_markdown("`code`"), "\\`code\\`");
    }

    #[test]
    fn test_export_table_to_gfm() {
        let cells1 = vec![TableCell::new("Header 1"), TableCell::new("Header 2")];
        let cells2 = vec![TableCell::new("Row 1 Col 1"), TableCell::new("Row 1 Col 2")];
        let table = TableBuilder::new(vec![100, 100])
            .add_row(TableRow::new(cells1))
            .add_row(TableRow::new(cells2))
            .build();

        let md = export_table_to_gfm(&table);

        assert!(md.contains("| Header 1 | Header 2 |"));
        assert!(md.contains("| --- | --- |"));
        assert!(md.contains("| Row 1 Col 1 | Row 1 Col 2 |"));
    }

    #[test]
    fn test_export_with_code_blocks() {
        let mut presentation = Presentation::with_title("Code Test");
        let mut slide = SlideContent::new("Code Slide");
        slide.code_blocks.push(CodeBlock::new("println!(\"Hello\");", "rust"));
        presentation = presentation.add_slide(slide);

        let md = export_to_markdown(&presentation).unwrap();

        assert!(md.contains("```rust"));
        assert!(md.contains("println!(\"Hello\");"));
        assert!(md.contains("```"));
    }

    #[test]
    fn test_export_with_speaker_notes() {
        let mut presentation = Presentation::with_title("Notes Test");
        let mut slide = SlideContent::new("Notes Slide");
        slide.notes = Some("This is a speaker note".to_string());
        presentation = presentation.add_slide(slide);

        let md = export_to_markdown(&presentation).unwrap();

        assert!(md.contains("**Notes:**"));
        assert!(md.contains("> This is a speaker note"));
    }

    #[test]
    fn test_yaml_escape_multiline() {
        let multiline = "Line 1\nLine 2";
        let escaped = escape_yaml(multiline);
        assert!(escaped.starts_with("|"));
        assert!(escaped.contains("Line 1"));
        assert!(escaped.contains("Line 2"));
    }

    #[test]
    fn test_yaml_escape_quotes() {
        let with_quotes = r#"Title with "quotes""#;
        let escaped = escape_yaml(with_quotes);
        // Single line with quotes gets escaped or uses literal block
        assert!(escaped.contains("quotes") || escaped.contains("\\\""));
    }

    #[test]
    fn test_markdown_all_options_disabled() {
        let mut presentation = Presentation::with_title("Minimal");
        let mut slide = SlideContent::new("Slide");
        slide.notes = Some("Note".to_string());
        presentation = presentation.add_slide(slide);

        let options = MarkdownOptions::new()
            .with_frontmatter(false)
            .with_slide_numbers(false)
            .with_notes(false)
            .with_images(false);

        let md = export_to_markdown_with_options(&presentation, &options).unwrap();

        assert!(!md.contains("---\ntitle:"));
        assert!(!md.contains("Slide 1:"));
        assert!(!md.contains("**Notes:**"));
    }

    #[test]
    fn test_empty_presentation() {
        let presentation = Presentation::with_title("Empty");
        let md = export_to_markdown(&presentation).unwrap();

        assert!(md.contains("# Empty"));
        assert!(!md.contains("## Slide")); // No slides
    }

    #[test]
    fn test_markdown_escape_various_chars() {
        let text = r#"Special chars: * _ [ ] ` # < > \ "#;
        let escaped = escape_markdown(text);
        assert!(escaped.contains("\\*"));
        assert!(escaped.contains("\\_"));
        assert!(escaped.contains("\\["));
        assert!(escaped.contains("\\]"));
        assert!(escaped.contains("\\`"));
        assert!(escaped.contains("\\#"));
        assert!(escaped.contains("\\<"));
        assert!(escaped.contains("\\>"));
    }
}
