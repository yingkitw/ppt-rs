//! CLI module for PPTX tool

#[cfg(feature = "cli")]
pub mod commands;
#[cfg(any(feature = "cli", feature = "python"))]
pub mod markdown;
#[cfg(feature = "cli")]
pub mod parser;
#[cfg(feature = "cli")]
pub mod syntax;

#[cfg(feature = "cli")]
pub use commands::{
    BatchCommand, CreateCommand, DiffCommand, FromHtmlCommand, FromHtmlOptions,
    FromMarkdownCommand, InfoCommand, ValidateCommand,
};
#[cfg(any(feature = "cli", feature = "python"))]
pub use markdown::parse_markdown;
#[cfg(feature = "cli")]
pub use parser::{Cli, Commands, ExportFormat};
#[cfg(feature = "cli")]
pub use syntax::{generate_highlighted_code_xml, highlight_code};
