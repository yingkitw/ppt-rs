//! PowerPoint (.pptx) file manipulation library
//!
//! This is a Rust port of the python-pptx library, providing functionality to create,
//! read, and update PowerPoint 2007+ (.pptx) files.

pub mod api;
pub mod chart;
pub mod cli;
pub mod config;
pub mod constants;
pub mod dml;
pub mod enums;
pub mod exc;
pub mod generator;
pub mod integration;
pub mod media;
pub mod opc;
pub mod oxml;
pub mod package;
pub mod parts;
pub mod presentation;
pub mod shapes;
pub mod shared;
pub mod slide;
pub mod table;
pub mod text;
pub mod types;
pub mod util;

pub use api::Presentation;
pub use exc::PptxError;
pub use api::presentation;

pub const VERSION: &str = "1.0.2";
