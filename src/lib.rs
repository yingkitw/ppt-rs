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
//! - **core** - Core traits (`ToXml`, `Positioned`) and utilities (XML utils, dimensions, validation, package validation)
//! - **elements** - Unified element types (Color, Position, Size, Transform)
//! - **generator** - PPTX file generation with ZIP packaging and XML creation
//! - **parts** - Package parts (SlidePart, ImagePart, ChartPart)
//! - **api** - High-level `Presentation` builder (create, save, import, export)
//! - **prelude** - Simplified API: macros, unit helpers, shapes, colors, themes
//! - **helpers** - Color / table / shape / extension helpers
//! - **templates** - Pre-built presentation templates
//! - **cli** - Feature-gated command-line interface (feature = "cli")
//! - **export** - Export to Markdown, HTML, images
//! - **import** - PPTX import, HTML-to-PPTX
//! - **diff** - Semantic slide-level comparison for review/CI
//! - **pipeline** - Batch Markdown/HTML → `.pptx` conversion with shared theme/template
//! - **http** - REST API for generation, validation, export, and diff (feature = "http")
//! - **mcp** - Model Context Protocol server and tool router (feature = "mcp")
//! - **web2ppt** - Feature-gated HTML/URL-to-PPTX conversion (feature = "web2ppt")
//! - **opc** - Open Packaging Convention (ZIP) handling
//! - **oxml** - Office XML parsing and manipulation
//! - **exc** - Error types

pub mod api;
#[cfg(feature = "cli")]
pub mod cli;
pub mod core;
pub mod diff;
pub mod elements;
pub mod exc;
pub mod export;
pub mod generator;
pub mod helpers;
pub mod import;
pub mod opc;
pub mod oxml;
pub mod parts;
pub mod pipeline;
pub mod prelude;
pub mod templates;

#[cfg(feature = "mcp")]
pub mod mcp;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "web2ppt")]
pub mod web2ppt;

pub use api::Presentation;
pub use core::{
    CompatReport, PackageValidationIssue, PackageValidationReport, REQUIRED_PACKAGE_PARTS,
    ValidationCategory, ValidationSeverity, validate_package, validate_package_bytes,
    validate_powerpoint_structure,
};
pub use core::{ToXml, escape_xml};
pub use diff::{
    FieldChange, PresentationDiff, SlideChange, SlideSemantic, compare_pptx_paths,
    compare_presentations, slide_semantic,
};
pub use elements::{Color, Position, RgbColor, SchemeColor, Size, Transform};
pub use exc::{PptxError, Result, messages};
pub use generator::{
    ArrowSize, ArrowType, Audio, AudioFormat, AudioOptions, BulletPoint, BulletStyle,
    CellMergeState, Chart, ChartBuilder, ChartSeries, ChartType, Comment, CommentAuthor,
    CommentAuthorList, ConnectionSite, Connector, ConnectorLine, ConnectorType, DigitalSignature,
    EmbeddedFont, EmbeddedFontList, FontCharset, FontStyle, FormattedText, GradientDirection,
    GradientFill, GradientStop, GradientType, HandoutLayout, HashAlgorithm, Hyperlink,
    HyperlinkAction, Image, ImageBuilder, ImageSource, InkAnnotations, InkPen, InkPoint, InkStroke,
    LazySlideSource, LineDash, MediaRelIds, MergeRegion, Orientation, PenColor, PenTip,
    PptxTemplate, PresentationSettings, PresentationTheme, PresetGradients, PrintColorMode,
    PrintSettings, PrintWhat, RtlLanguage, RtlTextProps, STANDARD_LAYOUT_COUNT, SectionManager,
    Shape, ShapeFill, ShapeLine, ShapeType, ShowType, SignatureCommitment, SignerInfo,
    SlideComments, SlideContent, SlideLayout, SlideRange, SlideSection, SlideShowSettings, Table,
    TableBuilder, TableCell, TableMergeMap, TableRow, TextDirection, TextFormat, ThemeColorScheme,
    ThemeFonts, Video, VideoFormat, VideoOptions, create_pptx, create_pptx_lazy_to_writer,
    create_pptx_to_writer, create_pptx_with_content, create_pptx_with_content_to_writer,
    create_pptx_with_settings, create_pptx_with_template,
};
pub use pipeline::{
    BatchJob, BatchOptions, BatchResult, InputFormat, convert_file, detect_format, discover_inputs,
    resolve_output_path, run_batch, run_directory,
};

// Export convenience types for new capabilities
#[cfg(feature = "pdf-native")]
pub use export::image_export::{
    ImageExportOptions, ImageFormat, export_slide_to_image, export_to_images, render_thumbnail,
};
pub use export::md::{MarkdownOptions, export_to_markdown, export_to_markdown_with_options};
#[cfg(feature = "pdf-native")]
pub use export::slide_render::{render_to_pdf, render_to_pdf_bytes};
pub use import::html::{Html2Ppt, HtmlParseOptions, parse_html, parse_html_with_options};
pub use oxml::extract_slide_texts_from_bytes;

#[cfg(feature = "pdf-native")]
pub use export::pdf_export::{
    PdfExportOptions, PdfOrientation, export_to_pdf, export_to_pdf_bytes,
};
pub use opc::compress::{
    CompressionLevel, CompressionOptions, CompressionResult, analyze_pptx, compress_pptx,
    compress_pptx_in_memory,
};

pub use parts::{
    AppPropertiesPart, ChartPart, ContentType, ContentTypesPart, CorePropertiesPart, ImagePart,
    LayoutType, MediaFormat, MediaPart, NotesSlidePart, Part, PartType, PresentationPart,
    Relationships, SlideLayoutPart, SlideMasterPart, SlidePart, TableCellPart, TablePart,
    TableRowPart, ThemePart,
};

#[cfg(feature = "web2ppt")]
pub use web2ppt::{
    ContentBlock, ContentType as WebContentType, ConversionOptions, Web2Ppt, Web2PptConfig,
    Web2PptError, WebContent, WebFetcher, WebParser, html_to_pptx, html_to_pptx_with_options,
    url_to_pptx, url_to_pptx_with_options,
};

#[cfg(feature = "web2md")]
pub use web2ppt::{html_to_pptx_via_web2md, url_to_pptx_via_web2md};

pub const VERSION: &str = "0.3.0";
