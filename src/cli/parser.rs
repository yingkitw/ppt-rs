//! Command-line argument parser using clap

use clap::{Parser as ClapParser, Subcommand, ValueEnum};

#[derive(ClapParser, Debug)]
#[command(name = "pptcli")]
#[command(about = "PowerPoint Generator - Create, read, and update PowerPoint 2007+ (.pptx) files")]
#[command(
    long_about = "pptcli - A command-line tool for generating PowerPoint presentations from Markdown, webpages, or programmatically.

Examples:
  # Create a simple presentation
  pptcli create output.pptx --title \"My Presentation\" --slides 5

  # Convert Markdown to PowerPoint
  pptcli md2ppt slides.md presentation.pptx

  # Auto-generate output filename from Markdown
  pptcli md2ppt slides.md

  # Convert webpage to PowerPoint (requires --features web2ppt)
  pptcli web2ppt https://example.com -o output.pptx

  # Validate a PPTX file
  pptcli validate presentation.pptx

  # Show presentation information
  pptcli info presentation.pptx"
)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new presentation
    #[command(
        long_about = "Create a new PowerPoint presentation with the specified number of slides.

Examples:
  pptcli create output.pptx --title \"My Presentation\" --slides 5
  pptcli create report.pptx --slides 10"
    )]
    Create {
        /// Output file path (.pptx)
        #[arg(value_name = "FILE", help = "Path to the output PPTX file")]
        output: String,

        /// Presentation title
        #[arg(long, help = "Title of the presentation (stored in metadata)")]
        title: Option<String>,

        /// Number of slides to create
        #[arg(long, default_value_t = 1, help = "Number of blank slides to create")]
        slides: usize,

        /// Template file to use
        #[arg(long, help = "Template PPTX file to use as base (not yet implemented)")]
        template: Option<String>,
    },

    /// Generate PPTX from Markdown file
    #[command(
        name = "md2ppt",
        alias = "from-md",
        alias = "from-markdown",
        long_about = "Convert a Markdown file to a PowerPoint presentation.

Supported Markdown Features:
  # Heading      → New slide with title
  ## Subheading  → Bold bullet point
  - Bullet       → Bullet points (also *, +)
  1. Numbered    → Numbered list items
  **bold**       → Bold text
  *italic*       → Italic text
  `code`         → Inline code
  > Blockquote   → Speaker notes
  | Table |      → Tables (GFM style)
  ```code```     → Code blocks (as shapes)
  ```mermaid     → Mermaid diagrams (12 types)
  ---            → Slide break (continuation)

Example Markdown:
  # Introduction
  - Welcome to the presentation
  - **Key point** with emphasis

  # Data
  | Name | Value |
  |------|-------|
  | A    | 100   |

  > Speaker notes go here

Examples:
  pptcli md2ppt slides.md presentation.pptx
  pptcli md2ppt slides.md --title \"My Presentation\"
  pptcli md2ppt slides.md  # Auto-generates slides.pptx"
    )]
    Md2Ppt {
        /// Input markdown file
        #[arg(value_name = "INPUT", help = "Path to the input Markdown file")]
        input: String,

        /// Output PPTX file (optional: auto-generated from input if not provided)
        #[arg(
            value_name = "OUTPUT",
            help = "Path to the output PPTX file (default: INPUT.pptx)"
        )]
        output: Option<String>,

        /// Presentation title
        #[arg(long, help = "Title of the presentation (overrides Markdown content)")]
        title: Option<String>,
    },

    /// Show presentation information
    #[command(long_about = "Display information about a PPTX file.

Shows file size, modification date, and basic metadata.

Example:
  pptcli info presentation.pptx")]
    Info {
        /// PPTX file to inspect
        #[arg(value_name = "FILE", help = "Path to the PPTX file to inspect")]
        file: String,
    },

    /// Validate a PPTX file
    #[command(long_about = "Validate a PPTX file structure and content.
        
Checks for:
- Valid ZIP structure
- Required parts (presentation.xml, slide masters, etc.)
- Content types
- Relationships")]
    Validate {
        /// PPTX file to validate
        #[arg(value_name = "FILE")]
        file: String,
    },

    /// Export presentation to other formats
    #[command(long_about = "Export PPTX to PDF, HTML, or images.

Formats:
- pdf:  Requires LibreOffice installed
- html: Self-contained HTML slideshow
- png:  Requires LibreOffice and pdftoppm")]
    Export {
        /// Input PPTX file
        #[arg(value_name = "INPUT")]
        input: String,

        /// Output file path
        #[arg(value_name = "OUTPUT")]
        output: String,

        /// Output format (overrides extension)
        #[arg(long, value_enum)]
        format: Option<ExportFormat>,
    },

    /// Merge multiple presentations
    #[command(long_about = "Merge multiple PPTX files into one.
        
Slides from all input files will be appended in order.")]
    Merge {
        /// Output PPTX file
        #[arg(short, long)]
        output: String,

        /// Input PPTX files
        #[arg(value_name = "INPUTS", required = true, num_args = 1..)]
        inputs: Vec<String>,
    },

    /// Convert PDF to PowerPoint
    #[command(
        name = "pdf2ppt",
        long_about = "Convert PDF pages to PowerPoint slides.
        
Requires `pdftoppm` (poppler) installed.
Each page becomes a slide with the page image."
    )]
    Pdf2Ppt {
        /// Input PDF file
        #[arg(value_name = "INPUT")]
        input: String,

        /// Output PPTX file
        #[arg(value_name = "OUTPUT")]
        output: Option<String>,
    },

    /// Generate PPTX from webpage (requires web2ppt feature)
    #[cfg(feature = "web2ppt")]
    #[command(
        name = "web2ppt",
        long_about = "Convert a webpage to a PowerPoint presentation.
        
Extracts:
- Title and headings
- Text content
- Images
- Tables
- Code blocks"
    )]
    Web2Ppt {
        /// URL to convert
        #[arg(value_name = "URL")]
        url: String,

        /// Output file path (.pptx)
        #[arg(short, long, default_value = "output.pptx")]
        output: String,

        /// Presentation title (overrides page title)
        #[arg(long)]
        title: Option<String>,

        /// Maximum number of slides to generate
        #[arg(long, default_value_t = 20)]
        max_slides: usize,

        /// Maximum bullet points per slide
        #[arg(long, default_value_t = 7)]
        max_bullets: usize,

        /// Disable image extraction
        #[arg(long)]
        no_images: bool,

        /// Disable table extraction
        #[arg(long)]
        no_tables: bool,

        /// Disable code block extraction
        #[arg(long)]
        no_code: bool,

        /// Don't add source URL to last slide
        #[arg(long)]
        no_source_url: bool,

        /// Request timeout in seconds
        #[arg(long, default_value_t = 30)]
        timeout: u64,

        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ExportFormat {
    Pdf,
    Html,
    Png,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_create() {
        let args = vec![
            "pptcli".to_string(),
            "create".to_string(),
            "test.pptx".to_string(),
            "--title".to_string(),
            "My Presentation".to_string(),
        ];
        let cli = Cli::parse_from(args.iter());
        match cli.command {
            Commands::Create { output, title, .. } => {
                assert_eq!(output, "test.pptx");
                assert_eq!(title, Some("My Presentation".to_string()));
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_parse_md2ppt_with_output() {
        let args = vec![
            "pptcli".to_string(),
            "md2ppt".to_string(),
            "input.md".to_string(),
            "output.pptx".to_string(),
            "--title".to_string(),
            "From Markdown".to_string(),
        ];
        let cli = Cli::parse_from(args.iter());
        match cli.command {
            Commands::Md2Ppt {
                input,
                output,
                title,
            } => {
                assert_eq!(input, "input.md");
                assert_eq!(output, Some("output.pptx".to_string()));
                assert_eq!(title, Some("From Markdown".to_string()));
            }
            _ => panic!("Expected Md2Ppt command"),
        }
    }

    #[test]
    fn test_parse_md2ppt_auto_output() {
        let args = vec![
            "pptcli".to_string(),
            "md2ppt".to_string(),
            "input.md".to_string(),
            "--title".to_string(),
            "From Markdown".to_string(),
        ];
        let cli = Cli::parse_from(args.iter());
        match cli.command {
            Commands::Md2Ppt {
                input,
                output,
                title,
            } => {
                assert_eq!(input, "input.md");
                assert_eq!(output, None);
                assert_eq!(title, Some("From Markdown".to_string()));
            }
            _ => panic!("Expected Md2Ppt command"),
        }
    }

    #[test]
    fn test_parse_from_md_alias() {
        let args = vec![
            "pptcli".to_string(),
            "from-md".to_string(),
            "input.md".to_string(),
            "output.pptx".to_string(),
        ];
        let cli = Cli::parse_from(args.iter());
        match cli.command {
            Commands::Md2Ppt { input, output, .. } => {
                assert_eq!(input, "input.md");
                assert_eq!(output, Some("output.pptx".to_string()));
            }
            _ => panic!("Expected Md2Ppt command via from-md alias"),
        }
    }

    #[test]
    fn test_parse_info() {
        let args = vec![
            "pptcli".to_string(),
            "info".to_string(),
            "test.pptx".to_string(),
        ];
        let cli = Cli::parse_from(args.iter());
        match cli.command {
            Commands::Info { file } => {
                assert_eq!(file, "test.pptx");
            }
            _ => panic!("Expected Info command"),
        }
    }
}
