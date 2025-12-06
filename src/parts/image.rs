//! Image part
//!
//! Represents an image embedded in the presentation.

use super::base::{Part, PartType, ContentType};
use crate::exc::PptxError;
use std::path::Path;

/// Image part (ppt/media/imageN.ext)
#[derive(Debug, Clone)]
pub struct ImagePart {
    path: String,
    image_number: usize,
    format: String,
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl ImagePart {
    /// Create a new image part
    pub fn new(image_number: usize, format: &str, data: Vec<u8>) -> Self {
        let format_lower = format.to_lowercase();
        let ext = match format_lower.as_str() {
            "jpeg" => "jpg",
            other => other,
        };
        
        ImagePart {
            path: format!("ppt/media/image{}.{}", image_number, ext),
            image_number,
            format: format_lower,
            data,
            width: 0,
            height: 0,
        }
    }

    /// Create from file path
    pub fn from_file(image_number: usize, file_path: &str) -> Result<Self, PptxError> {
        let data = std::fs::read(file_path)?;
        let format = Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("png")
            .to_lowercase();
        
        Ok(Self::new(image_number, &format, data))
    }

    /// Get image number
    pub fn image_number(&self) -> usize {
        self.image_number
    }

    /// Get format
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Get image data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Set dimensions
    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Get width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get MIME type
    pub fn mime_type(&self) -> &'static str {
        match self.format.as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "bmp" => "image/bmp",
            "tiff" | "tif" => "image/tiff",
            _ => "application/octet-stream",
        }
    }

    /// Get file extension
    pub fn extension(&self) -> &str {
        match self.format.as_str() {
            "jpeg" => "jpg",
            other => other,
        }
    }

    /// Get relative path for relationships
    pub fn rel_target(&self) -> String {
        format!("../media/image{}.{}", self.image_number, self.extension())
    }
}

impl Part for ImagePart {
    fn path(&self) -> &str {
        &self.path
    }

    fn part_type(&self) -> PartType {
        PartType::Image
    }

    fn content_type(&self) -> ContentType {
        ContentType::Image(self.format.clone())
    }

    fn to_xml(&self) -> Result<String, PptxError> {
        // Images don't have XML content, they're binary
        Err(PptxError::InvalidOperation("Images don't have XML content".to_string()))
    }

    fn from_xml(_xml: &str) -> Result<Self, PptxError> {
        Err(PptxError::InvalidOperation("Cannot create image from XML".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_part_new() {
        let data = vec![0x89, 0x50, 0x4E, 0x47]; // PNG header
        let part = ImagePart::new(1, "png", data);
        
        assert_eq!(part.image_number(), 1);
        assert_eq!(part.format(), "png");
        assert_eq!(part.path(), "ppt/media/image1.png");
    }

    #[test]
    fn test_image_mime_type() {
        let part = ImagePart::new(1, "jpeg", vec![]);
        assert_eq!(part.mime_type(), "image/jpeg");
        
        let part2 = ImagePart::new(2, "png", vec![]);
        assert_eq!(part2.mime_type(), "image/png");
    }

    #[test]
    fn test_image_rel_target() {
        let part = ImagePart::new(3, "png", vec![]);
        assert_eq!(part.rel_target(), "../media/image3.png");
    }
}
