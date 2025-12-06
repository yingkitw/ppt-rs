//! Unified PPTX element types
//!
//! This module provides a single source of truth for all PPTX element types,
//! consolidating what was previously split between `generator/` and `oxml/`.
//!
//! # Architecture
//!
//! Each element type implements:
//! - `ToXml` trait for XML generation
//! - Builder pattern for construction
//! - Parse methods for reading from XML
//!
//! # Example
//!
//! ```rust,ignore
//! use ppt_rs::elements::{Table, TableRow, TableCell};
//!
//! let table = Table::builder()
//!     .add_row(TableRow::new(vec![
//!         TableCell::new("Header 1").bold(),
//!         TableCell::new("Header 2").bold(),
//!     ]))
//!     .build();
//! ```

mod color;
mod position;

pub use color::{Color, RgbColor, SchemeColor};
pub use position::{Position, Size, Transform, EMU_PER_INCH, EMU_PER_CM, EMU_PER_MM, EMU_PER_PT};

// Re-export core traits
pub use crate::core::{ToXml, Positioned, ElementSized, Styled};
