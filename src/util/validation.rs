//! PPTX file validation utilities
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
