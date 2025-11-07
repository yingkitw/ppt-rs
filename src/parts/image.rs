//! Image part

use crate::error::Result;
use crate::opc::constants::CONTENT_TYPE;
use crate::opc::part::{BasePart, Part};
use crate::opc::packuri::PackURI;
use crate::opc::relationships::Relationships;

/// Image part - contains image data
pub struct ImagePart {
    base: BasePart,
    filename: Option<String>,
}

impl ImagePart {
    /// Create a new image part
    pub fn new(partname: PackURI, content_type: &str, blob: Vec<u8>) -> Result<Self> {
        let base = BasePart::with_blob(content_type, partname, blob)?;
        Ok(Self {
            base,
            filename: None,
        })
    }

    /// Create a new image part with filename
    pub fn with_filename(
        partname: PackURI,
        content_type: &str,
        blob: Vec<u8>,
        filename: String,
    ) -> Result<Self> {
        let base = BasePart::with_blob(content_type, partname, blob)?;
        Ok(Self {
            base,
            filename: Some(filename),
        })
    }

    /// Get the file extension
    pub fn ext(&self) -> String {
        self.base.uri().ext().to_string()
    }

    /// Get the filename or generate a generic one
    pub fn filename(&self) -> String {
        self.filename
            .clone()
            .unwrap_or_else(|| format!("image.{}", self.ext()))
    }

    /// Get image dimensions (width, height) in EMU
    pub fn dimensions(&self) -> Result<(u32, u32)> {
        use crate::opc::part::Part;
        // Try to parse image to get dimensions
        if let Ok(blob) = Part::blob(self) {
            // Try to decode image using image crate - use load_from_memory which works with DynamicImage
            if let Ok(img) = image::load_from_memory(&blob) {
                // DynamicImage implements GenericImage which has dimensions()
                let (width, height) = (img.width(), img.height());
                // Convert pixels to EMU (1 pixel = 9525 EMU at 96 DPI)
                let width_emu = width * 9525;
                let height_emu = height * 9525;
                return Ok((width_emu, height_emu));
            }
        }
        // Fallback: return default size
        Ok((914400, 685800)) // Default size in EMU (1 inch x 0.75 inch)
    }
}

impl Part for ImagePart {
    fn content_type(&self) -> &str {
        self.base.content_type()
    }

    fn uri(&self) -> &PackURI {
        self.base.uri()
    }

    fn relationships(&self) -> &Relationships {
        self.base.relationships()
    }

    fn relationships_mut(&mut self) -> &mut Relationships {
        self.base.relationships_mut()
    }

    fn blob(&self) -> Result<Vec<u8>> {
        self.base.blob()
    }

    fn to_xml(&self) -> Result<String> {
        // Image parts are binary, not XML
        Ok(String::new())
    }

    fn from_xml<R: std::io::Read>(_reader: R) -> Result<Self> {
        Err(crate::error::PptError::NotImplemented(
            "ImagePart::from_xml - images are binary, not XML".to_string(),
        ))
    }
}

