//! Generic OpenXML document trait
//!
//! Provides a common interface for all OOXML document types (PPTX, DOCX, XLSX).

use crate::opc::{Package, CoreProperties, AppProperties, CustomProperties};
use crate::error::Result;

/// Document format type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentFormat {
    /// PowerPoint presentation (PPTX)
    Presentation,
    /// Word document (DOCX)
    Document,
    /// Excel spreadsheet (XLSX)
    Spreadsheet,
}

/// Generic trait for all OOXML documents
///
/// Provides a common interface for working with any OOXML format.
pub trait OpenXmlDocument {
    /// Get the document format
    fn format(&self) -> DocumentFormat;

    /// Get the package
    fn package(&self) -> &Package;

    /// Get mutable package
    fn package_mut(&mut self) -> &mut Package;

    /// Get core properties
    fn core_properties(&self) -> &CoreProperties;

    /// Get mutable core properties
    fn core_properties_mut(&mut self) -> &mut CoreProperties;

    /// Get app properties
    fn app_properties(&self) -> &AppProperties;

    /// Get mutable app properties
    fn app_properties_mut(&mut self) -> &mut AppProperties;

    /// Get custom properties
    fn custom_properties(&self) -> &CustomProperties;

    /// Get mutable custom properties
    fn custom_properties_mut(&mut self) -> &mut CustomProperties;

    /// Save the document
    fn save(&mut self) -> Result<Vec<u8>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_format_presentation() {
        assert_eq!(DocumentFormat::Presentation, DocumentFormat::Presentation);
    }

    #[test]
    fn test_document_format_document() {
        assert_eq!(DocumentFormat::Document, DocumentFormat::Document);
    }

    #[test]
    fn test_document_format_spreadsheet() {
        assert_eq!(DocumentFormat::Spreadsheet, DocumentFormat::Spreadsheet);
    }

    #[test]
    fn test_document_format_ne() {
        assert_ne!(DocumentFormat::Presentation, DocumentFormat::Document);
        assert_ne!(DocumentFormat::Document, DocumentFormat::Spreadsheet);
        assert_ne!(DocumentFormat::Spreadsheet, DocumentFormat::Presentation);
    }
}
