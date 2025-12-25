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
//! ```rust
//! use ppt_rs::elements::{Color, RgbColor, Position, Size};
//!
//! // Create an RGB color
//! let red = RgbColor::new(255, 0, 0);
//! assert_eq!(red.to_hex(), "FF0000");
//!
//! // Create a position in EMU (English Metric Units)
//! let pos = Position::new(914400, 914400); // 1 inch x 1 inch
//! ```

mod color;
mod position;

pub use color::{Color, RgbColor, SchemeColor};
pub use position::{Position, Size, Transform, EMU_PER_INCH, EMU_PER_CM, EMU_PER_MM, EMU_PER_PT};

// Re-export core traits
pub use crate::core::{ToXml, Positioned, ElementSized, Styled};
