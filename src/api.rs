//! Directly exposed API classes, Presentation for now.

use crate::error::Result;
use crate::presentation::Presentation as PresentationImpl;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Return a Presentation object loaded from `pptx`, where `pptx` can be
/// either a path to a `.pptx` file or a file-like object. If `pptx` is missing
/// or `None`, the built-in default presentation "template" is loaded.
pub fn Presentation(pptx: Option<&str>) -> Result<PresentationImpl> {
    let path = pptx.unwrap_or_else(|| {
        // TODO: Return path to built-in default template
        "templates/default.pptx"
    });

    if Path::new(path).exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        PresentationImpl::open(reader)
    } else {
        // Create new presentation
        PresentationImpl::new()
    }
}

/// Create a new empty presentation
pub fn new_presentation() -> Result<PresentationImpl> {
    PresentationImpl::new()
}

/// Open a presentation from a file path
pub fn open_presentation<P: AsRef<Path>>(path: P) -> Result<PresentationImpl> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    PresentationImpl::open(reader)
}

