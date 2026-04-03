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
//! - **core** - Core traits (`ToXml`, `Positioned`) and utilities
//! - **elements** - Unified element types (Color, Position, Size, Transform)
//! - **generator** - PPTX file generation with ZIP packaging and XML creation
//! - **parts** - Package parts (SlidePart, ImagePart, ChartPart)
//! - **integration** - High-level builders for presentations
//! - **opc** - Open Packaging Convention (ZIP) handling
//! - **oxml** - Office XML parsing and manipulation
//! - **exc** - Error types

pub mod core;
pub mod elements;
pub mod generator;
pub mod cli;
pub mod exc;
pub mod opc;
pub mod oxml;
pub mod parts;
pub mod api;
pub mod prelude;
pub mod helpers;
pub mod templates;
pub mod export;
pub mod import;

#[cfg(feature = "web2ppt")]
pub mod web2ppt;

pub use api::Presentation;
pub use core::{ToXml, escape_xml};
pub use elements::{Color, RgbColor, SchemeColor, Position, Size, Transform};
pub use exc::{PptxError, Result};
pub use generator::{
    create_pptx, create_pptx_with_content, create_pptx_with_settings,
    create_pptx_to_writer, create_pptx_with_content_to_writer, create_pptx_lazy_to_writer,
    LazySlideSource,
    SlideContent, SlideLayout,
    TextFormat, FormattedText,
    Table, TableRow, TableCell, TableBuilder,
    Shape, ShapeType, ShapeFill, ShapeLine,
    Image, ImageBuilder, ImageSource,
    Chart, ChartType, ChartSeries, ChartBuilder,
    BulletStyle, BulletPoint,
    TextDirection, RtlLanguage, RtlTextProps,
    Comment, CommentAuthor, CommentAuthorList, SlideComments,
    SlideSection, SectionManager,
    DigitalSignature, SignerInfo, HashAlgorithm, SignatureCommitment,
    InkAnnotations, InkStroke, InkPen, InkPoint, PenTip,
    SlideShowSettings, ShowType, PenColor, SlideRange,
    PrintSettings, HandoutLayout, PrintColorMode, PrintWhat, Orientation,
    TableMergeMap, MergeRegion, CellMergeState,
    EmbeddedFontList, EmbeddedFont, FontStyle, FontCharset,
    PresentationSettings, SlideSize,
    Connector, ConnectorType, ConnectorLine, ArrowType, ArrowSize, ConnectionSite, LineDash,
    Hyperlink, HyperlinkAction,
    GradientFill, GradientType, GradientDirection, GradientStop, PresetGradients,
    Video, Audio, VideoFormat, AudioFormat, VideoOptions, AudioOptions,
};
pub use oxml::repair::{PptxRepair, RepairIssue, RepairResult};

pub use parts::{
    Part, PartType, ContentType,
    PresentationPart, SlidePart, SlideLayoutPart, LayoutType,
    SlideMasterPart, ThemePart, NotesSlidePart,
    ImagePart, MediaPart, MediaFormat, ChartPart,
    TablePart, TableRowPart, TableCellPart,
    CorePropertiesPart, AppPropertiesPart,
    ContentTypesPart, Relationships,
};

#[cfg(feature = "web2ppt")]
pub use web2ppt::{
    Web2Ppt, WebFetcher, WebParser, WebContent, ContentBlock,
    ContentType as WebContentType,
    Web2PptConfig, ConversionOptions, Web2PptError,
    html_to_pptx, html_to_pptx_with_options, url_to_pptx, url_to_pptx_with_options,
};

pub const VERSION: &str = "0.2.7";
