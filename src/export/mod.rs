//! Export functionality
//!
//! Exports presentations to various formats.

pub mod html;
#[cfg(feature = "pdf-native")]
pub mod image_export;
pub mod md;
#[cfg(feature = "pdf-native")]
pub mod slide_render;

#[cfg(feature = "pdf-native")]
pub mod pdf_export;

#[cfg(feature = "pdf-native")]
pub use image_export::{
    ImageExportOptions, ImageExportOptions as ImageOptions, ImageFormat, export_slide_to_image,
    export_to_images, render_thumbnail,
};
pub use md::{MarkdownOptions, export_to_markdown, export_to_markdown_with_options};

#[cfg(feature = "pdf-native")]
pub use pdf_export::{PdfExportOptions, PdfOrientation, export_to_pdf, export_to_pdf_bytes};
