//! Main presentation object.

use crate::error::{PptError, Result};
use crate::parts::presentation::PresentationPart;
use crate::slide::{Slide, Slides};
use std::io::{Read, Write};

/// PresentationML (PML) presentation.
///
/// Not intended to be constructed directly. Use `ppt_rs::Presentation` to open or
/// create a presentation.
pub struct Presentation {
    part: PresentationPart,
}

impl Presentation {
    /// Create a new empty presentation
    pub fn new() -> Result<Self> {
        let part = PresentationPart::new()?;
        Ok(Self { part })
    }

    /// Open a presentation from a reader
    pub fn open<R: Read>(_reader: R) -> Result<Self> {
        // TODO: Implement package opening
        Err(PptError::NotImplemented("Opening presentations".to_string()))
    }

    /// Save the presentation to a writer
    pub fn save<W: Write>(&self, _writer: W) -> Result<()> {
        // TODO: Implement package saving
        Err(PptError::NotImplemented("Saving presentations".to_string()))
    }

    /// Save the presentation to a file path
    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        self.save(writer)
    }

    /// Get the slides collection
    pub fn slides(&self) -> Slides {
        Slides::new(&self.part)
    }

    /// Get the presentation part
    pub fn part(&self) -> &PresentationPart {
        &self.part
    }

    /// Get mutable presentation part
    pub fn part_mut(&mut self) -> &mut PresentationPart {
        &mut self.part
    }

    /// Get core properties
    pub fn core_properties(&self) -> Result<()> {
        self.part.core_properties()
    }

    /// Get slide width in EMU (English Metric Units)
    pub fn slide_width(&self) -> Option<u32> {
        // TODO: Get from XML element
        Some(9144000) // Default 10 inches
    }

    /// Set slide width in EMU
    pub fn set_slide_width(&mut self, _width: u32) -> Result<()> {
        // TODO: Set in XML element
        Ok(())
    }

    /// Get slide height in EMU
    pub fn slide_height(&self) -> Option<u32> {
        // TODO: Get from XML element
        Some(6858000) // Default 7.5 inches
    }

    /// Set slide height in EMU
    pub fn set_slide_height(&mut self, _height: u32) -> Result<()> {
        // TODO: Set in XML element
        Ok(())
    }
}

