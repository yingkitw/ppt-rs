//! PowerPoint (.pptx) file manipulation library
//!
//! This is a Rust port of the python-pptx library, providing functionality to create,
//! read, and update PowerPoint 2007+ (.pptx) files.
//!
//! # Core Modules
//!
//! - **generator** - PPTX file generation with ZIP packaging and XML creation
//! - **integration** - High-level builders and utilities for presentations
//! - **cli** - Command-line interface for PPTX operations
//!
//! # Supporting Modules
//!
//! - **config** - Configuration management
//! - **constants** - Presentation constants and defaults
//! - **enums** - Type-safe enumeration values
//! - **exc** - Error types and handling
//! - **util** - Utility functions (length conversions, etc.)
//! - **opc** - Open Packaging Convention (ZIP) handling
//! - **oxml** - Office XML element definitions

// Core modules
pub mod generator;
pub mod integration;
pub mod cli;

// Supporting modules
pub mod config;
pub mod constants;
pub mod enums;
pub mod exc;
pub mod util;
pub mod opc;
pub mod oxml;

// Placeholder modules (for future expansion)
pub mod api;
pub mod types;
pub mod shared;

// Deprecated/stub modules (kept for compatibility)
#[deprecated(since = "0.1.0", note = "Use generator module instead")]
pub mod chart {
    //! Placeholder for chart functionality
}

#[deprecated(since = "0.1.0", note = "Use generator module instead")]
pub mod dml {
    //! Placeholder for drawing markup language
}

#[deprecated(since = "0.1.0", note = "Use generator module instead")]
pub mod media {
    //! Placeholder for media handling
}

#[deprecated(since = "0.1.0", note = "Use generator module instead")]
pub mod package {
    //! Placeholder for package handling
}

#[deprecated(since = "0.1.0", note = "Use generator module instead")]
pub mod parts {
    //! Placeholder for package parts
}

#[deprecated(since = "0.1.0", note = "Use generator module instead")]
pub mod presentation {
    //! Placeholder for presentation handling
}

#[deprecated(since = "0.1.0", note = "Use generator module instead")]
pub mod shapes {
    //! Placeholder for shape handling
}

#[deprecated(since = "0.1.0", note = "Use generator module instead")]
pub mod slide {
    //! Placeholder for slide handling
}

#[deprecated(since = "0.1.0", note = "Use generator module instead")]
pub mod table {
    //! Placeholder for table handling
}

#[deprecated(since = "0.1.0", note = "Use generator module instead")]
pub mod text {
    //! Placeholder for text handling
}

// Public API exports
pub use api::Presentation;
pub use generator::{create_pptx, create_pptx_with_content, SlideContent};
pub use integration::{PresentationBuilder, SlideBuilder, PresentationMetadata};
pub use exc::PptxError;

pub const VERSION: &str = "1.0.2";
