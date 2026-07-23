//! Export functionality
//!
//! Exports presentations to various formats.

pub mod html;
pub mod md;
pub mod image_export;
pub mod slide_render;

#[cfg(feature = "pdf-native")]
pub mod pdf_export;

pub use md::{export_to_markdown, export_to_markdown_with_options, MarkdownOptions};
pub use image_export::{
    export_to_images, export_slide_to_image, render_thumbnail,
    ImageExportOptions, ImageFormat, ImageExportOptions as ImageOptions
};

#[cfg(feature = "pdf-native")]
pub use pdf_export::{
    export_to_pdf, export_to_pdf_bytes,
    PdfExportOptions, PdfOrientation,
};
