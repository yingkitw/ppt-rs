//! CLI module for PPTX tool

pub mod commands;
pub mod markdown;
pub mod parser;
pub mod syntax;

pub use commands::{CreateCommand, FromMarkdownCommand, InfoCommand, ValidateCommand};
pub use markdown::parse_markdown;
pub use parser::{Cli, Commands, ExportFormat};
pub use syntax::{generate_highlighted_code_xml, highlight_code};
