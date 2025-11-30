//! Command-line argument parser using clap

use clap::{Parser as ClapParser, Subcommand};

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

Each # heading creates a new slide. Bullet points (-, *, +) become slide content.

Markdown Format:
  # Slide Title
  - First bullet point
  - Second bullet point

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
        #[arg(value_name = "OUTPUT", help = "Path to the output PPTX file (default: INPUT.pptx)")]
        output: Option<String>,
        
        /// Presentation title
        #[arg(long, help = "Title of the presentation (overrides Markdown content)")]
        title: Option<String>,
    },
    
    /// Show presentation information
    #[command(
        long_about = "Display information about a PPTX file.

Shows file size, modification date, and basic metadata.

Example:
  pptcli info presentation.pptx"
    )]
    Info {
        /// PPTX file to inspect
        #[arg(value_name = "FILE", help = "Path to the PPTX file to inspect")]
        file: String,
    },
    
    /// Validate a PPTX file for ECMA-376 compliance
    #[command(
        long_about = "Validate a PPTX file for ECMA-376 Office Open XML compliance.

Checks:
  - ZIP archive integrity
  - Required XML files presence
  - XML validity
  - Relationships structure

Example:
  pptcli validate presentation.pptx"
    )]
    Validate {
        /// PPTX file to validate
        #[arg(value_name = "FILE", help = "Path to the PPTX file to validate")]
        file: String,
    },
    
    /// Convert a webpage to PowerPoint (requires web2ppt feature)
    #[command(
        name = "web2ppt",
        alias = "from-url",
        alias = "url2ppt",
        long_about = "Convert a webpage to a PowerPoint presentation.

Fetches the webpage, extracts content (headings, paragraphs, lists, code, tables),
and generates slides grouped by headings.

Requires the 'web2ppt' feature to be enabled.

Examples:
  pptcli web2ppt https://example.com
  pptcli web2ppt https://example.com -o output.pptx
  pptcli web2ppt https://example.com --max-slides 10 --no-images"
    )]
    Web2Ppt {
        /// URL of the webpage to convert
        #[arg(value_name = "URL", help = "URL of the webpage to convert")]
        url: String,
        
        /// Output file path
        #[arg(short, long, default_value = "output.pptx", help = "Path to the output PPTX file")]
        output: String,
        
        /// Custom presentation title (overrides page title)
        #[arg(short, long, help = "Custom title for the presentation")]
        title: Option<String>,
        
        /// Maximum number of slides
        #[arg(long, default_value = "20", help = "Maximum number of slides to generate")]
        max_slides: usize,
        
        /// Maximum bullets per slide
        #[arg(long, default_value = "6", help = "Maximum bullet points per slide")]
        max_bullets: usize,
        
        /// Exclude images from the presentation
        #[arg(long, help = "Don't include images from the webpage")]
        no_images: bool,
        
        /// Exclude tables from the presentation
        #[arg(long, help = "Don't include tables from the webpage")]
        no_tables: bool,
        
        /// Exclude code blocks from the presentation
        #[arg(long, help = "Don't include code blocks from the webpage")]
        no_code: bool,
        
        /// Don't include source URL in presentation
        #[arg(long, help = "Don't add source URL to the title slide")]
        no_source_url: bool,
        
        /// Request timeout in seconds
        #[arg(long, default_value = "30", help = "HTTP request timeout in seconds")]
        timeout: u64,
        
        /// Verbose output
        #[arg(short, long, help = "Show detailed progress")]
        verbose: bool,
    },
}

// Legacy types for backward compatibility with existing command execution code
#[derive(Debug, Clone)]
pub struct CreateArgs {
    pub output: String,
    pub title: Option<String>,
    pub slides: usize,
    pub template: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FromMarkdownArgs {
    pub input: String,
    pub output: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Md2PptArgs {
    pub input: String,
    pub output: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InfoArgs {
    pub file: String,
}

#[derive(Debug, Clone)]
pub struct ValidateArgs {
    pub file: String,
}

#[derive(Debug, Clone)]
pub struct Web2PptArgs {
    pub url: String,
    pub output: String,
    pub title: Option<String>,
    pub max_slides: usize,
    pub max_bullets: usize,
    pub no_images: bool,
    pub no_tables: bool,
    pub no_code: bool,
    pub no_source_url: bool,
    pub timeout: u64,
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub enum Command {
    Create(CreateArgs),
    FromMarkdown(FromMarkdownArgs),
    Md2Ppt(Md2PptArgs),
    Info(InfoArgs),
    Validate(ValidateArgs),
    Web2Ppt(Web2PptArgs),
}

impl From<Commands> for Command {
    fn from(cmd: Commands) -> Self {
        match cmd {
            Commands::Create { output, title, slides, template } => {
                Command::Create(CreateArgs {
                    output,
                    title,
                    slides,
                    template,
                })
            }
            Commands::Md2Ppt { input, output, title } => {
                // If output is not provided, auto-generate it
                let output = output.unwrap_or_else(|| {
                    use std::path::Path;
                    let input_path = Path::new(&input);
                    if let Some(stem) = input_path.file_stem() {
                        if let Some(parent) = input_path.parent() {
                            if parent.as_os_str().is_empty() {
                                format!("{}.pptx", stem.to_string_lossy())
                            } else {
                                format!("{}/{}.pptx", parent.display(), stem.to_string_lossy())
                            }
                        } else {
                            format!("{}.pptx", stem.to_string_lossy())
                        }
                    } else {
                        format!("{}.pptx", input)
                    }
                });
                
                Command::FromMarkdown(FromMarkdownArgs {
                    input,
                    output,
                    title,
                })
            }
            Commands::Info { file } => {
                Command::Info(InfoArgs { file })
            }
            Commands::Validate { file } => {
                Command::Validate(ValidateArgs { file })
            }
            Commands::Web2Ppt { url, output, title, max_slides, max_bullets, no_images, no_tables, no_code, no_source_url, timeout, verbose } => {
                Command::Web2Ppt(Web2PptArgs {
                    url,
                    output,
                    title,
                    max_slides,
                    max_bullets,
                    no_images,
                    no_tables,
                    no_code,
                    no_source_url,
                    timeout,
                    verbose,
                })
            }
        }
    }
}

// Legacy Parser for backward compatibility
pub struct LegacyParser;

impl LegacyParser {
    pub fn parse(args: &[String]) -> Result<Command, String> {
        let cli = Cli::parse_from(std::iter::once(&"pptcli".to_string()).chain(args.iter()));
        Ok(cli.command.into())
    }
}

// Alias for backward compatibility
pub use LegacyParser as Parser;

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
            Commands::Md2Ppt { input, output, title } => {
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
            Commands::Md2Ppt { input, output, title } => {
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
