//! CLI module for PPTX tool

pub mod commands;
pub mod parser;
pub mod markdown;
pub mod syntax;

pub use commands::{CreateCommand, FromMarkdownCommand, InfoCommand, ValidateCommand};
pub use parser::{
    Cli, Commands, Parser, Command, 
    CreateArgs, FromMarkdownArgs, InfoArgs, ValidateArgs, Web2PptArgs,
    ExportFormat,
};
pub use markdown::parse_markdown;
pub use syntax::{highlight_code, generate_highlighted_code_xml};
