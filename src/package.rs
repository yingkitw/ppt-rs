//! Package module

use crate::exc::Result;
use std::io::Read;
use std::path::Path;

/// Represents a PowerPoint package
pub struct Package {
    // Implementation will be added
}

impl Package {
    /// Open a package from a file path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let _path = path.as_ref();
        // TODO: Implement
        Ok(Package {})
    }

    /// Open a package from a reader
    pub fn open_reader<R: Read>(reader: R) -> Result<Self> {
        let _reader = reader;
        // TODO: Implement
        Ok(Package {})
    }
}
