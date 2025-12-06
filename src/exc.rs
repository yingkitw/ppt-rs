//! Exception types for pptx library

use thiserror::Error;

/// Base error type for pptx library
#[derive(Error, Debug)]
pub enum PptxError {
    #[error("Generic pptx error: {0}")]
    Generic(String),

    #[error("Package not found: {0}")]
    PackageNotFound(String),

    #[error("Invalid XML: {0}")]
    InvalidXml(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ZIP error: {0}")]
    Zip(String),

    #[error("XML parse error: {0}")]
    XmlParse(String),

    #[error("Invalid value: {0}")]
    InvalidValue(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

pub type Result<T> = std::result::Result<T, PptxError>;
