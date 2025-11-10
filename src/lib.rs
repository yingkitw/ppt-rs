//! # ppt-rs
//!
//! A Rust library for creating, reading, and updating PowerPoint (.pptx) files.
//!
//! This is a Rust port of the [python-pptx](https://github.com/scanny/python-pptx) library.
//!
//! ## Features
//!
//! - Create new PowerPoint presentations
//! - Read existing .pptx files
//! - Modify slides, shapes, text, images, and charts
//! - Full support for OpenXML format (ISO/IEC 29500)
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use ppt_rs::new_presentation;
//!
//! // Create a new presentation
//! let mut prs = new_presentation().unwrap();
//!
//! // Save the presentation
//! prs.save_to_file("output.pptx").unwrap();
//! ```
//!
//! ## Status
//!
//! 🚧 **Work in Progress** - This library is currently under active development.
//!
//! See [MIGRATION_STATUS.md](../MIGRATION_STATUS.md) for detailed migration progress.

pub mod api;
pub mod builder;
pub mod chart;
pub mod dml;
pub mod enums;
pub mod error;
pub mod opc;
pub mod oxml;
pub mod parts;
pub mod presentation;
pub mod shapes;
pub mod slide;
pub mod table;
pub mod text;
pub mod util;

pub use api::{new_presentation, open_presentation, Presentation};
pub use builder::PresentationBuilder;
pub use error::{PptError, Result};
pub use presentation::Presentation as PresentationStruct;

