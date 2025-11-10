//! Image part

use crate::error::Result;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opc::packuri::PackURI;

    #[test]
    fn test_image_part_new() {
        let uri = PackURI::new("/ppt/media/image1.png").unwrap();
        let blob = vec![0x89, 0x50, 0x4E, 0x47]; // PNG header
        let part = ImagePart::new(uri, crate::opc::constants::CONTENT_TYPE::PNG, blob);
        assert!(part.is_ok());
    }

    #[test]
    fn test_image_part_with_filename() {
        let uri = PackURI::new("/ppt/media/image1.png").unwrap();
        let blob = vec![0x89, 0x50, 0x4E, 0x47];
        let part = ImagePart::with_filename(
            uri,
            crate::opc::constants::CONTENT_TYPE::PNG,
            blob,
            "test.png".to_string(),
        );
        assert!(part.is_ok());
        let part = part.unwrap();
        assert_eq!(part.filename(), "test.png");
    }

    #[test]
    fn test_image_part_ext() {
        let uri = PackURI::new("/ppt/media/image1.png").unwrap();
        let blob = vec![0x89, 0x50, 0x4E, 0x47];
        let part = ImagePart::new(uri, crate::opc::constants::CONTENT_TYPE::PNG, blob).unwrap();
        assert_eq!(part.ext(), "png");
    }

    #[test]
    fn test_image_part_filename_default() {
        let uri = PackURI::new("/ppt/media/image1.jpg").unwrap();
        let blob = vec![0xFF, 0xD8, 0xFF];
        let part = ImagePart::new(uri, crate::opc::constants::CONTENT_TYPE::JPEG, blob).unwrap();
        assert_eq!(part.filename(), "image.jpg");
    }

    #[test]
    fn test_image_part_dimensions() {
        // Create a minimal valid PNG (1x1 pixel)
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            0x00, 0x00, 0x00, 0x0D, // IHDR chunk length
            0x49, 0x48, 0x44, 0x52, // IHDR
            0x00, 0x00, 0x00, 0x01, // width: 1
            0x00, 0x00, 0x00, 0x01, // height: 1
            0x08, 0x02, 0x00, 0x00, 0x00, // bit depth, color type, etc.
            0x90, 0x77, 0x53, 0xDE, // CRC
            0x00, 0x00, 0x00, 0x0A, // IDAT chunk length
            0x49, 0x44, 0x41, 0x54, // IDAT
            0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, // compressed data
            0x0D, 0x0A, 0x2D, 0xB4, // CRC
            0x00, 0x00, 0x00, 0x00, // IEND chunk length
            0x49, 0x45, 0x4E, 0x44, // IEND
            0xAE, 0x42, 0x60, 0x82, // CRC
        ];
        
        let uri = PackURI::new("/ppt/media/image1.png").unwrap();
        let part = ImagePart::new(uri, crate::opc::constants::CONTENT_TYPE::PNG, png_data).unwrap();
        
        let dims = part.dimensions();
        assert!(dims.is_ok());
        let (width, height) = dims.unwrap();
        // 1 pixel * 9525 EMU = 9525 EMU
        assert_eq!(width, 9525);
        assert_eq!(height, 9525);
    }

    #[test]
    fn test_image_part_dimensions_fallback() {
        // Invalid image data should return default dimensions
        let uri = PackURI::new("/ppt/media/image1.png").unwrap();
        let blob = vec![0x00, 0x01, 0x02]; // Invalid image data
        let part = ImagePart::new(uri, crate::opc::constants::CONTENT_TYPE::PNG, blob).unwrap();
        
        let dims = part.dimensions();
        assert!(dims.is_ok());
        let (width, height) = dims.unwrap();
        // Should return default size
        assert_eq!(width, 914400);
        assert_eq!(height, 685800);
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

