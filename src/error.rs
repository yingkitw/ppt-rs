//! Error types for ppt-rs

use thiserror::Error;

/// Result type alias for ppt-rs operations
pub type Result<T> = std::result::Result<T, PptError>;

/// Main error type for ppt-rs
#[derive(Error, Debug)]
pub enum PptError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("XML parsing error: {0}")]
    Xml(String),

    #[error("ZIP archive error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Invalid package: {0}")]
    InvalidPackage(String),

    #[error("Part not found: {0}")]
    PartNotFound(String),

    #[error("Invalid content type: {0}")]
    InvalidContentType(String),

    #[error("Invalid relationship type: {0}")]
    InvalidRelationshipType(String),

    #[error("Value error: {0}")]
    ValueError(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

