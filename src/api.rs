//! Public API module

pub use crate::presentation::Presentation;

use crate::exc::Result;
use std::io::Read;
use std::path::Path;

pub fn presentation<P: AsRef<Path>>(path: Option<P>) -> Result<Presentation> {
    match path {
        Some(p) => {
            let _path = p.as_ref();
            // TODO: Implement loading from file
            Ok(Presentation::new())
        }
        None => {
            // TODO: Load default template
            Ok(Presentation::new())
        }
    }
}

/// Open a presentation from a reader
pub fn presentation_from_reader<R: Read>(reader: R) -> Result<Presentation> {
    let _reader = reader;
    // TODO: Implement loading from reader
    Ok(Presentation::new())
}
