//! Round-trip support for reading and modifying existing PPTX files

use crate::error::{PptError, Result};
use crate::presentation::Presentation;
use std::io::{Read, Seek, Write};

/// Round-trip handler for PPTX files
pub struct RoundTrip;

impl RoundTrip {
    /// Open an existing PPTX file for modification
    pub fn open<R: Read + Seek>(reader: R) -> Result<Presentation> {
        Presentation::open(reader)
            .map_err(|e| PptError::InvalidPackage(format!("Failed to open PPTX: {}", e)))
    }

    /// Save a presentation to a writer
    pub fn save<W: Write + Seek>(prs: &mut Presentation, writer: W) -> Result<()> {
        prs.save(writer)
            .map_err(|e| PptError::InvalidPackage(format!("Failed to save PPTX: {}", e)))
    }

    /// Modify and save a presentation
    pub fn modify<R, W, F>(reader: R, writer: W, modifier: F) -> Result<()>
    where
        R: Read + Seek,
        W: Write + Seek,
        F: FnOnce(&mut Presentation) -> Result<()>,
    {
        let mut prs = Self::open(reader)?;
        modifier(&mut prs)?;
        Self::save(&mut prs, writer)?;
        Ok(())
    }

    /// Validate that a file can be opened and saved
    pub fn validate_roundtrip<R: Read + Seek + Clone>(reader: R) -> Result<()> {
        // Try to open
        let _prs = Self::open(reader.clone())?;
        
        // Try to save to memory
        let mut cursor = std::io::Cursor::new(Vec::new());
        let mut prs = Self::open(reader)?;
        Self::save(&mut prs, &mut cursor)?;
        
        // Try to reopen
        cursor.set_position(0);
        let _reopened = Self::open(cursor)?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_open_new_presentation() {
        let mut prs = Presentation::new().unwrap();
        let mut cursor = std::io::Cursor::new(Vec::new());
        prs.save(&mut cursor).unwrap();
        
        cursor.set_position(0);
        let result = RoundTrip::open(cursor);
        assert!(result.is_ok());
    }

    #[test]
    fn test_roundtrip_save() {
        let mut prs = Presentation::new().unwrap();
        let mut cursor = std::io::Cursor::new(Vec::new());
        let result = RoundTrip::save(&mut prs, &mut cursor);
        assert!(result.is_ok());
    }

    #[test]
    fn test_roundtrip_modify() {
        let mut prs = Presentation::new().unwrap();
        let mut cursor = std::io::Cursor::new(Vec::new());
        prs.save(&mut cursor).unwrap();
        
        cursor.set_position(0);
        let mut output = std::io::Cursor::new(Vec::new());
        
        let result = RoundTrip::modify(cursor, &mut output, |_prs| {
            // Just modify without doing anything
            Ok(())
        });
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_roundtrip_validate() {
        let mut prs = Presentation::new().unwrap();
        let mut cursor = std::io::Cursor::new(Vec::new());
        prs.save(&mut cursor).unwrap();
        
        cursor.set_position(0);
        let result = RoundTrip::validate_roundtrip(cursor);
        assert!(result.is_ok());
    }
}
