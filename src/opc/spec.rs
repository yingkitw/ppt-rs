//! Default content type mappings

use crate::opc::constants::CONTENT_TYPE;

/// Default content types based on file extension
pub const DEFAULT_CONTENT_TYPES: &[(&str, &str)] = &[
    ("bin", CONTENT_TYPE::PML_PRINTER_SETTINGS),
    ("bmp", CONTENT_TYPE::BMP),
    ("emf", CONTENT_TYPE::X_EMF),
    ("fntdata", CONTENT_TYPE::X_FONTDATA),
    ("gif", CONTENT_TYPE::GIF),
    ("jpe", CONTENT_TYPE::JPEG),
    ("jpeg", CONTENT_TYPE::JPEG),
    ("jpg", CONTENT_TYPE::JPEG),
    ("mov", CONTENT_TYPE::MOV),
    ("mp4", CONTENT_TYPE::MP4),
    ("mpg", CONTENT_TYPE::MPG),
    ("png", CONTENT_TYPE::PNG),
    ("rels", CONTENT_TYPE::OPC_RELATIONSHIPS),
    ("tif", CONTENT_TYPE::TIFF),
    ("tiff", CONTENT_TYPE::TIFF),
    ("vid", CONTENT_TYPE::VIDEO),
    ("wdp", CONTENT_TYPE::MS_PHOTO),
    ("wmf", CONTENT_TYPE::X_WMF),
    ("wmv", CONTENT_TYPE::WMV),
    ("xlsx", CONTENT_TYPE::XML), // Placeholder
    ("xml", CONTENT_TYPE::XML),
];

/// Image content types
pub const IMAGE_CONTENT_TYPES: &[(&str, &str)] = &[
    ("bmp", CONTENT_TYPE::BMP),
    ("emf", CONTENT_TYPE::X_EMF),
    ("gif", CONTENT_TYPE::GIF),
    ("jpe", CONTENT_TYPE::JPEG),
    ("jpeg", CONTENT_TYPE::JPEG),
    ("jpg", CONTENT_TYPE::JPEG),
    ("png", CONTENT_TYPE::PNG),
    ("tif", CONTENT_TYPE::TIFF),
    ("tiff", CONTENT_TYPE::TIFF),
    ("wdp", CONTENT_TYPE::MS_PHOTO),
    ("wmf", CONTENT_TYPE::X_WMF),
];

