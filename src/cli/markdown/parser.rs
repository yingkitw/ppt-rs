//! Markdown parser state machine
//!
//! Handles parsing of markdown content into slide structures.

use super::mermaid;
use crate::generator::{
    CodeBlock, Shape, ShapeFill, ShapeType, SlideContent, TableBuilder, TableCell, TableRow,
};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Parse markdown content into slides
pub fn parse(content: &str) -> Result<Vec<SlideContent>, String> {
    let mut parser = MarkdownParser::new();
    parser.parse(content)
}

/// State machine for markdown parsing
struct MarkdownParser {
    slides: Vec<SlideContent>,
    current_slide: Option<SlideContent>,
    current_text: String,
    // List state
    in_list: bool,
    list_items: Vec<String>,
    // Table state
    in_table: bool,
    table_rows: Vec<Vec<String>>,
    current_row: Vec<String>,
    current_cell: String,
    in_table_head: bool,
    // Code block state
    in_code_block: bool,
    code_content: String,
    code_language: Option<String>,
    // Formatting state
    is_bold: bool,
    is_italic: bool,
    // Blockquote (speaker notes)
    in_blockquote: bool,
    blockquote_text: String,
    // Image state
    pending_image: Option<(String, String)>,
}

impl MarkdownParser {
    fn new() -> Self {
        Self {
            slides: Vec::new(),
            current_slide: None,
            current_text: String::new(),
            in_list: false,
            list_items: Vec::new(),
            in_table: false,
            table_rows: Vec::new(),
            current_row: Vec::new(),
            current_cell: String::new(),
            in_table_head: false,
            in_code_block: false,
            code_content: String::new(),
            code_language: None,
            is_bold: false,
            is_italic: false,
            in_blockquote: false,
            blockquote_text: String::new(),
            pending_image: None,
        }
    }

    fn parse(&mut self, content: &str) -> Result<Vec<SlideContent>, String> {
        let options =
            Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;

        let parser = Parser::new_ext(content, options);

        for event in parser {
            self.handle_event(event);
        }

        self.finalize_current_slide();

        if self.slides.is_empty() {
            return Err("No slides found in markdown file".to_string());
        }

        Ok(std::mem::take(&mut self.slides))
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            // Headings create new slides
            Event::Start(Tag::Heading { level, .. }) => {
                if level == HeadingLevel::H1 {
                    self.finalize_current_slide();
                }
                self.current_text.clear();
            }
            Event::End(TagEnd::Heading(level)) => {
                let title = std::mem::take(&mut self.current_text).trim().to_string();
                if level == HeadingLevel::H1 {
                    self.current_slide = Some(SlideContent::new(&title));
                } else if let Some(ref mut slide) = self.current_slide {
                    let formatted = format!("**{}**", title);
                    *slide = slide.clone().add_bullet(&formatted);
                }
            }

            // Lists
            Event::Start(Tag::List(_)) => {
                self.in_list = true;
                self.list_items.clear();
            }
            Event::End(TagEnd::List(_)) => {
                self.in_list = false;
                self.flush_list_items();
            }
            Event::Start(Tag::Item) => {
                self.current_text.clear();
            }
            Event::End(TagEnd::Item) => {
                let item = std::mem::take(&mut self.current_text).trim().to_string();
                if !item.is_empty() {
                    self.list_items.push(item);
                }
            }

            // Tables
            Event::Start(Tag::Table(_)) => {
                self.in_table = true;
                self.table_rows.clear();
                self.in_table_head = false;
            }
            Event::End(TagEnd::Table) => {
                self.in_table = false;
                self.flush_table();
            }
            Event::Start(Tag::TableHead) => {
                self.in_table_head = true;
                self.current_row.clear();
            }
            Event::End(TagEnd::TableHead) => {
                self.in_table_head = false;
                if !self.current_row.is_empty() {
                    self.table_rows.push(std::mem::take(&mut self.current_row));
                }
            }
            Event::Start(Tag::TableRow) => {
                self.current_row.clear();
            }
            Event::End(TagEnd::TableRow) => {
                if !self.current_row.is_empty() {
                    self.table_rows.push(std::mem::take(&mut self.current_row));
                }
            }
            Event::Start(Tag::TableCell) => {
                self.current_cell.clear();
            }
            Event::End(TagEnd::TableCell) => {
                self.current_row
                    .push(std::mem::take(&mut self.current_cell).trim().to_string());
            }

            // Code blocks
            Event::Start(Tag::CodeBlock(kind)) => {
                self.in_code_block = true;
                self.code_content.clear();
                self.code_language = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                        let lang_str = lang.to_string();
                        if lang_str.is_empty() {
                            None
                        } else {
                            Some(lang_str)
                        }
                    }
                    _ => None,
                };
            }
            Event::End(TagEnd::CodeBlock) => {
                self.in_code_block = false;
                self.flush_code_block();
            }

            // Blockquotes (speaker notes)
            Event::Start(Tag::BlockQuote) => {
                self.in_blockquote = true;
                self.blockquote_text.clear();
            }
            Event::End(TagEnd::BlockQuote) => {
                self.in_blockquote = false;
                self.flush_blockquote();
            }

            // Inline formatting
            Event::Start(Tag::Strong) => self.is_bold = true,
            Event::End(TagEnd::Strong) => self.is_bold = false,
            Event::Start(Tag::Emphasis) => self.is_italic = true,
            Event::End(TagEnd::Emphasis) => self.is_italic = false,
            Event::Code(code) => {
                let formatted = format!("`{}`", code);
                self.push_text(&formatted);
            }

            // Images
            Event::Start(Tag::Image {
                dest_url, title, ..
            }) => {
                self.pending_image = Some((dest_url.to_string(), title.to_string()));
            }
            Event::End(TagEnd::Image) => {
                if let Some((url, alt)) = self.pending_image.take() {
                    self.add_image_placeholder(&url, &alt);
                }
            }

            // Horizontal rule = slide break
            Event::Rule => {
                self.finalize_current_slide();
                if let Some(last) = self.slides.last() {
                    let title = format!("{} (continued)", last.title);
                    self.current_slide = Some(SlideContent::new(&title));
                }
            }

            // Text content
            Event::Text(text) => {
                self.push_text(&text);
            }
            Event::SoftBreak | Event::HardBreak => {
                self.push_text(" ");
            }

            // Paragraphs
            Event::Start(Tag::Paragraph) => {
                if !self.in_list && !self.in_table && !self.in_blockquote && !self.in_code_block {
                    self.current_text.clear();
                }
            }
            Event::End(TagEnd::Paragraph) => {
                if !self.in_list && !self.in_table && !self.in_blockquote && !self.in_code_block {
                    let text = std::mem::take(&mut self.current_text).trim().to_string();
                    if !text.is_empty() {
                        self.add_paragraph(&text);
                    }
                }
            }

            _ => {}
        }
    }

    fn push_text(&mut self, text: &str) {
        let formatted = if self.is_bold && self.is_italic {
            format!("***{}***", text)
        } else if self.is_bold {
            format!("**{}**", text)
        } else if self.is_italic {
            format!("*{}*", text)
        } else {
            text.to_string()
        };

        if self.in_code_block {
            self.code_content.push_str(text);
        } else if self.in_table {
            self.current_cell.push_str(&formatted);
        } else if self.in_blockquote {
            self.blockquote_text.push_str(&formatted);
        } else {
            self.current_text.push_str(&formatted);
        }
    }

    fn add_paragraph(&mut self, text: &str) {
        if let Some(ref mut slide) = self.current_slide {
            *slide = slide.clone().add_bullet(text);
        } else {
            let mut slide = SlideContent::new("Slide");
            slide = slide.add_bullet(text);
            self.current_slide = Some(slide);
        }
    }

    fn flush_list_items(&mut self) {
        if self.list_items.is_empty() {
            return;
        }

        let items = std::mem::take(&mut self.list_items);

        if let Some(ref mut slide) = self.current_slide {
            for item in items {
                *slide = slide.clone().add_bullet(&item);
            }
        } else {
            let mut slide = SlideContent::new("Slide");
            for item in items {
                slide = slide.add_bullet(&item);
            }
            self.current_slide = Some(slide);
        }
    }

    fn flush_table(&mut self) {
        if self.table_rows.is_empty() {
            return;
        }

        let rows = std::mem::take(&mut self.table_rows);
        let col_count = rows.iter().map(|r| r.len()).max().unwrap_or(1);
        let col_width = 8000000u32 / col_count as u32;
        let col_widths: Vec<u32> = vec![col_width; col_count];

        let mut builder = TableBuilder::new(col_widths);

        for (i, row_data) in rows.iter().enumerate() {
            let cells: Vec<TableCell> = row_data
                .iter()
                .map(|cell_text| {
                    let mut cell = TableCell::new(cell_text);
                    if i == 0 {
                        cell = cell.bold().background_color("4472C4").text_color("FFFFFF");
                    }
                    cell
                })
                .collect();

            let mut cells = cells;
            while cells.len() < col_count {
                cells.push(TableCell::new(""));
            }

            builder = builder.add_row(TableRow::new(cells));
        }

        let table = builder.position(500000, 1800000).build();

        if let Some(ref mut slide) = self.current_slide {
            slide.table = Some(table);
            slide.has_table = true;
        } else {
            let mut slide = SlideContent::new("Data Table");
            slide.table = Some(table);
            slide.has_table = true;
            self.current_slide = Some(slide);
        }
    }

    fn flush_code_block(&mut self) {
        if self.code_content.is_empty() {
            return;
        }

        let code = std::mem::take(&mut self.code_content);
        let lang = self.code_language.take();
        let lang_str = lang.as_deref().unwrap_or("text");

        if lang_str == "mermaid" {
            self.add_mermaid_diagram(&code);
            return;
        }

        let code_block = CodeBlock::new(code.trim(), lang_str);

        if let Some(ref mut slide) = self.current_slide {
            slide.code_blocks.push(code_block);
        } else {
            let mut slide = SlideContent::new("Code");
            slide.code_blocks.push(code_block);
            self.current_slide = Some(slide);
        }
    }

    fn add_mermaid_diagram(&mut self, code: &str) {
        let elements = mermaid::create_diagram_elements(code);
        let diagram_type = mermaid::detect_type(code);
        let (_, _, title, _) = mermaid::get_diagram_style(diagram_type);

        // Center diagram on slide if bounds are available
        // Slide dimensions: 9144000 x 6858000 EMU (standard 16:9)
        let slide_width = 9_144_000u32;
        let slide_height = 6_858_000u32;
        let title_offset = 1_200_000u32; // Leave space for title

        let (offset_x, offset_y) = if let Some(bounds) = &elements.bounds {
            // Calculate offset to center diagram
            let available_height = slide_height - title_offset;
            let center_x = (slide_width.saturating_sub(bounds.width)) / 2;
            let center_y = title_offset + (available_height.saturating_sub(bounds.height)) / 2;

            // Offset from current position to centered position
            (
                center_x.saturating_sub(bounds.x) as i32,
                center_y.saturating_sub(bounds.y) as i32,
            )
        } else {
            (0, 0)
        };

        // Apply offset to shapes
        let shapes: Vec<_> = elements
            .shapes
            .into_iter()
            .map(|mut shape| {
                shape.x = (shape.x as i32 + offset_x).max(0) as u32;
                shape.y = (shape.y as i32 + offset_y).max(0) as u32;
                shape
            })
            .collect();

        // Apply offset to connectors
        let connectors: Vec<_> = elements
            .connectors
            .into_iter()
            .map(|mut conn| {
                conn.start_x = (conn.start_x as i32 + offset_x).max(0) as u32;
                conn.start_y = (conn.start_y as i32 + offset_y).max(0) as u32;
                conn.end_x = (conn.end_x as i32 + offset_x).max(0) as u32;
                conn.end_y = (conn.end_y as i32 + offset_y).max(0) as u32;
                conn
            })
            .collect();

        if let Some(ref mut slide) = self.current_slide {
            for shape in shapes {
                slide.shapes.push(shape);
            }
            for connector in connectors {
                slide.connectors.push(connector);
            }
        } else {
            let mut slide = SlideContent::new(title);
            for shape in shapes {
                slide.shapes.push(shape);
            }
            for connector in connectors {
                slide.connectors.push(connector);
            }
            self.current_slide = Some(slide);
        }
    }

    fn flush_blockquote(&mut self) {
        if self.blockquote_text.is_empty() {
            return;
        }

        let notes = std::mem::take(&mut self.blockquote_text).trim().to_string();

        if let Some(ref mut slide) = self.current_slide {
            slide.notes = Some(notes);
        }
    }

    fn add_image_placeholder(&mut self, url: &str, alt: &str) {
        let label = if alt.is_empty() { url } else { alt };

        let shape = Shape::new(ShapeType::Rectangle, 2000000, 2000000, 5000000, 3000000)
            .with_fill(ShapeFill::new("E0E0E0"))
            .with_text(&format!("[Image: {}]", label));

        if let Some(ref mut slide) = self.current_slide {
            slide.shapes.push(shape);
        } else {
            let mut slide = SlideContent::new("Image");
            slide.shapes.push(shape);
            self.current_slide = Some(slide);
        }
    }

    fn finalize_current_slide(&mut self) {
        self.flush_list_items();

        if let Some(slide) = self.current_slide.take() {
            self.slides.push(slide);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_headings() {
        let md = "# Slide 1\n- Bullet 1\n\n# Slide 2\n- Bullet 2";
        let slides = parse(md).unwrap();
        assert_eq!(slides.len(), 2);
        assert_eq!(slides[0].title, "Slide 1");
        assert_eq!(slides[1].title, "Slide 2");
    }

    #[test]
    fn test_bullets() {
        let md = "# Test\n- Item 1\n- Item 2\n- Item 3";
        let slides = parse(md).unwrap();
        assert_eq!(slides[0].content.len(), 3);
    }

    #[test]
    fn test_table() {
        let md = "# Data\n\n| A | B |\n|---|---|\n| 1 | 2 |";
        let slides = parse(md).unwrap();
        assert!(slides[0].table.is_some());
    }

    #[test]
    fn test_code_block() {
        let md = "# Code\n\n```rust\nfn main() {}\n```";
        let slides = parse(md).unwrap();
        assert!(!slides[0].code_blocks.is_empty());
        assert_eq!(slides[0].code_blocks[0].language, "rust");
    }

    #[test]
    fn test_speaker_notes() {
        let md = "# Slide\n- Content\n\n> Speaker notes here";
        let slides = parse(md).unwrap();
        assert!(slides[0].notes.is_some());
    }

    #[test]
    fn test_formatting() {
        let md = "# Test\n- **Bold** and *italic*";
        let slides = parse(md).unwrap();
        assert!(slides[0].content[0].contains("**Bold**"));
    }

    #[test]
    fn test_mermaid_flowchart() {
        let md = "# Process\n\n```mermaid\nflowchart LR\n    A --> B --> C\n```";
        let slides = parse(md).unwrap();
        assert!(!slides[0].shapes.is_empty());
    }

    #[test]
    fn test_mermaid_sequence() {
        let md = "# Sequence\n\n```mermaid\nsequenceDiagram\n    Alice->>Bob: Hello\n```";
        let slides = parse(md).unwrap();
        assert!(!slides[0].shapes.is_empty());
    }
}
