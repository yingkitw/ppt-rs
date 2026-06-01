//! CLI module for PPTX tool

#[cfg(feature = "cli")]
pub mod commands;
#[cfg(feature = "cli")]
pub mod markdown;
#[cfg(feature = "cli")]
pub mod parser;
#[cfg(feature = "cli")]
pub mod syntax;

#[cfg(feature = "cli")]
pub use commands::{CreateCommand, FromHtmlCommand, FromMarkdownCommand, InfoCommand, ValidateCommand};
#[cfg(feature = "cli")]
pub use markdown::parse_markdown;
#[cfg(feature = "cli")]
pub use parser::{Cli, Commands, ExportFormat};
#[cfg(feature = "cli")]
pub use syntax::{generate_highlighted_code_xml, highlight_code};
