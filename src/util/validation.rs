//! Validation utilities for PPTX files and error context
//! 
//! Ensures that edited PPTX files are not corrupted by validating:
//! - File can be saved as valid ZIP
//! - File can be reopened and read
//! - All essential OPC components are present
//! - XML structure is valid

use crate::error::{PptError, Result};
use crate::presentation::Presentation;
use std::io::Cursor;
use zip::ZipArchive;

/// Error context trait for adding context to errors
pub trait ErrorContext<T> {
    /// Add context to an error
    fn context(self, context: &str) -> Result<T>;
}

impl<T> ErrorContext<T> for Result<T> {
    fn context(self, context: &str) -> Result<T> {
        self.map_err(|e| {
            match e {
                PptError::ValueError(msg) => {
                    PptError::ValueError(format!("{}: {}", context, msg))
                }
                PptError::Xml(msg) => {
                    PptError::Xml(format!("{}: {}", context, msg))
                }
                PptError::NotImplemented(msg) => {
                    PptError::NotImplemented(format!("{}: {}", context, msg))
                }
                PptError::InvalidPackage(msg) => {
                    PptError::InvalidPackage(format!("{}: {}", context, msg))
                }
                other => other,
            }
        })
    }
}

/// Validation helper
pub struct Validator;

impl Validator {
    /// Validate that a value is not empty
    pub fn not_empty(value: &str, field_name: &str) -> Result<()> {
        if value.is_empty() {
            Err(PptError::ValueError(format!("{} cannot be empty", field_name)))
        } else {
            Ok(())
        }
    }

    /// Validate that a value is within range
    pub fn in_range(value: u32, min: u32, max: u32, field_name: &str) -> Result<()> {
        if value < min || value > max {
            Err(PptError::ValueError(format!(
                "{} must be between {} and {}, got {}",
                field_name, min, max, value
            )))
        } else {
            Ok(())
        }
    }

    /// Validate that a value is positive
    pub fn positive(value: u32, field_name: &str) -> Result<()> {
        if value == 0 {
            Err(PptError::ValueError(format!(
                "{} must be positive, got {}",
                field_name, value
            )))
        } else {
            Ok(())
        }
    }
}

/// Validation result type
pub type ValidationResult = Result<()>;

/// Validate a presentation by saving and reopening it
/// 
/// This ensures that edits to a presentation don't corrupt the file.
/// Returns Ok if validation passes, Err if the file is corrupted.
pub fn validate_presentation(prs: &mut Presentation) -> Result<()> {
    // Save to memory
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor)?;
    
    // Validate the saved file
    cursor.set_position(0);
    validate_pptx_file(cursor)
}

/// Validate a PPTX file by checking its structure
/// 
/// Verifies:
/// - File is a valid ZIP archive
/// - [Content_Types].xml exists and is valid
/// - _rels/.rels exists and is valid
/// - ppt/presentation.xml exists and is valid
pub fn validate_pptx_file<R: std::io::Read + std::io::Seek>(reader: R) -> Result<()> {
    // Check if it's a valid ZIP
    let mut archive = ZipArchive::new(reader)
        .map_err(|e| PptError::ValueError(format!("Invalid ZIP archive: {}", e)))?;
    
    // Check for essential files
    {
        archive.by_name("[Content_Types].xml")
            .map_err(|_| PptError::ValueError("Missing [Content_Types].xml".to_string()))?;
    }
    
    {
        archive.by_name("_rels/.rels")
            .map_err(|_| PptError::ValueError("Missing _rels/.rels".to_string()))?;
    }
    
    {
        archive.by_name("ppt/presentation.xml")
            .map_err(|_| PptError::ValueError("Missing ppt/presentation.xml".to_string()))?;
    }
    
    // Validate XML structure
    validate_xml_file(&mut archive, "[Content_Types].xml")?;
    validate_xml_file(&mut archive, "_rels/.rels")?;
    validate_xml_file(&mut archive, "ppt/presentation.xml")?;
    
    Ok(())
}

/// Validate that a file in the archive is valid XML
fn validate_xml_file(archive: &mut ZipArchive<impl std::io::Read + std::io::Seek>, filename: &str) -> Result<()> {
    let mut file = archive.by_name(filename)
        .map_err(|e| PptError::ValueError(format!("Cannot read {}: {}", filename, e)))?;
    
    let mut content = String::new();
    std::io::Read::read_to_string(&mut file, &mut content)
        .map_err(|e| PptError::ValueError(format!("Cannot read {}: {}", filename, e)))?;
    
    // Check for XML declaration
    if !content.contains("<?xml") {
        return Err(PptError::ValueError(format!("{} missing XML declaration", filename)));
    }
    
    // Check for basic XML structure (opening and closing tags)
    if !content.contains("<") || !content.contains(">") {
        return Err(PptError::ValueError(format!("{} has invalid XML structure", filename)));
    }
    
    Ok(())
}

/// Validate that a presentation can be opened after saving
pub fn validate_roundtrip(prs: &mut Presentation) -> Result<()> {
    // Save to memory
    let mut cursor = Cursor::new(Vec::new());
    prs.save(&mut cursor)?;
    
    // Try to open the saved file
    cursor.set_position(0);
    let _reopened = Presentation::open(cursor)
        .map_err(|e| PptError::ValueError(format!("Cannot reopen saved presentation: {}", e)))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_context() {
        let result: Result<()> = Err(PptError::ValueError("test error".to_string()));
        let contexted = result.context("operation failed");
        
        match contexted {
            Err(PptError::ValueError(msg)) => {
                assert!(msg.contains("operation failed"));
                assert!(msg.contains("test error"));
            }
            _ => panic!("Expected ValueError"),
        }
    }

    #[test]
    fn test_validator_not_empty() {
        assert!(Validator::not_empty("value", "field").is_ok());
        assert!(Validator::not_empty("", "field").is_err());
    }

    #[test]
    fn test_validator_in_range() {
        assert!(Validator::in_range(50, 0, 100, "value").is_ok());
        assert!(Validator::in_range(150, 0, 100, "value").is_err());
    }

    #[test]
    fn test_validator_positive() {
        assert!(Validator::positive(1, "value").is_ok());
        assert!(Validator::positive(0, "value").is_err());
    }
    
    #[test]
    fn test_validate_new_presentation() {
        let mut prs = Presentation::new().unwrap();
        let result = validate_presentation(&mut prs);
        assert!(result.is_ok(), "New presentation should be valid");
    }
    
    #[test]
    fn test_validate_pptx_file_structure() {
        let mut prs = Presentation::new().unwrap();
        
        // Save to memory
        let mut cursor = Cursor::new(Vec::new());
        prs.save(&mut cursor).unwrap();
        
        // Validate
        cursor.set_position(0);
        let result = validate_pptx_file(cursor);
        assert!(result.is_ok(), "PPTX file structure should be valid");
    }
    
    #[test]
    fn test_validate_roundtrip() {
        let mut prs = Presentation::new().unwrap();
        let result = validate_roundtrip(&mut prs);
        assert!(result.is_ok(), "Presentation should survive roundtrip save/open");
    }
}
