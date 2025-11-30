//! CLI module for PPTX tool

pub mod commands;
pub mod parser;

pub use commands::{CreateCommand, FromMarkdownCommand, InfoCommand, ValidateCommand};
pub use parser::{
    Cli, Commands, Parser, Command, 
    CreateArgs, FromMarkdownArgs, InfoArgs, ValidateArgs, Web2PptArgs,
};
