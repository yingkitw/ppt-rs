//! CLI module for PPTX tool

pub mod commands;
pub mod parser;

pub use commands::{CreateCommand, InfoCommand};
pub use parser::{Parser, Command, CreateArgs, InfoArgs};
