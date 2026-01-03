//! PowerPoint (.pptx) file manipulation library
//!
//! A comprehensive Rust library for creating, reading, and updating PowerPoint 2007+ (.pptx) files.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use ppt_rs::{create_pptx_with_content, SlideContent};
//!
//! let slides = vec![
//!     SlideContent::new("Welcome")
//!         .add_bullet("First point")
//!         .add_bullet("Second point"),
//! ];
//! let pptx_data = create_pptx_with_content("My Presentation", slides).unwrap();
//! std::fs::write("output.pptx", pptx_data).unwrap();
//! ```
//!
//! # Module Organization
//!
//! - **core** - Core traits (`ToXml`, `Positioned`, `Styled`) and utilities
//! - **elements** - Unified element types (Color, Position, Size, Transform)
//! - **generator** - PPTX file generation with ZIP packaging and XML creation
//! - **parts** - Package parts (SlidePart, ImagePart, ChartPart)
//! - **integration** - High-level builders for presentations
//! - **opc** - Open Packaging Convention (ZIP) handling
//! - **oxml** - Office XML parsing and manipulation
//! - **exc** - Error types

// Core traits and utilities
pub mod core;

// Unified element types
pub mod elements;

// Main functionality
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
pub mod parts;

// Public API
pub mod api;
pub mod types;
pub mod shared;

// Easy-to-use prelude
pub mod prelude;

// Templates for common presentations
pub mod templates;

// Export functionality
pub mod export;

// Import functionality
pub mod import;

// Web2PPT module (optional feature)
#[cfg(feature = "web2ppt")]
pub mod web2ppt;

// Re-exports for convenience
pub use api::Presentation;
pub use core::{ToXml, escape_xml};
pub use elements::{Color, RgbColor, SchemeColor, Position, Size, Transform};
pub use exc::{PptxError, Result};
pub use generator::{
    create_pptx, create_pptx_with_content, SlideContent, SlideLayout,
    TextFormat, FormattedText,
    Table, TableRow, TableCell, TableBuilder,
    Shape, ShapeType, ShapeFill, ShapeLine,
    Image, ImageBuilder, ImageSource,
    Chart, ChartType, ChartSeries, ChartBuilder,
    // Bullet styles
    BulletStyle, BulletPoint,
    // New element types
    Connector, ConnectorType, ConnectorLine, ArrowType, ArrowSize, ConnectionSite, LineDash,
    Hyperlink, HyperlinkAction,
    GradientFill, GradientType, GradientDirection, GradientStop, PresetGradients,
    Video, Audio, VideoFormat, AudioFormat, VideoOptions, AudioOptions,
};
pub use integration::{PresentationBuilder, SlideBuilder, PresentationMetadata};
pub use oxml::repair::{PptxRepair, RepairIssue, RepairResult};

// Parts re-exports
pub use parts::{
    Part, PartType, ContentType,
    PresentationPart, SlidePart, SlideLayoutPart, LayoutType,
    SlideMasterPart, ThemePart, NotesSlidePart,
    ImagePart, MediaPart, MediaFormat, ChartPart,
    TablePart, TableRowPart, TableCellPart,
    CorePropertiesPart, AppPropertiesPart,
    ContentTypesPart, Relationships,
};

// Web2PPT re-exports
#[cfg(feature = "web2ppt")]
pub use web2ppt::{
    Web2Ppt, WebFetcher, WebParser, WebContent, ContentBlock,
    ContentType as WebContentType, // Renamed to avoid conflict with parts::ContentType
    Web2PptConfig, ConversionOptions, Web2PptError,
    html_to_pptx, html_to_pptx_with_options, url_to_pptx, url_to_pptx_with_options,
};

pub const VERSION: &str = "1.0.3";
