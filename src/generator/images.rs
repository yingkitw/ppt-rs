//! Image handling for PPTX presentations
//!
//! Handles image metadata, embedding, and XML generation

use std::path::Path;

/// Image data source
#[derive(Clone, Debug)]
pub enum ImageSource {
    /// Load from file path
    File(String),
    /// Base64 encoded data
    Base64(String),
    /// Raw bytes
    Bytes(Vec<u8>),
}

/// Image metadata and properties
#[derive(Clone, Debug)]
pub struct Image {
    pub filename: String,
    pub width: u32,      // in EMU
    pub height: u32,     // in EMU
    pub x: u32,          // Position X in EMU
    pub y: u32,          // Position Y in EMU
    pub format: String,  // PNG, JPG, GIF, etc.
    /// Image data source (file path, base64, or bytes)
    pub source: Option<ImageSource>,
}

impl Image {
    /// Create a new image
    pub fn new(filename: &str, width: u32, height: u32, format: &str) -> Self {
        Image {
            filename: filename.to_string(),
            width,
            height,
            x: 0,
            y: 0,
            format: format.to_uppercase(),
            source: Some(ImageSource::File(filename.to_string())),
        }
    }
    
    /// Create an image from base64 encoded data
    ///
    /// # Example
    /// ```rust
    /// use ppt_rs::generator::Image;
    ///
    /// let base64_data = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";
    /// let img = Image::from_base64(base64_data, 100, 100, "PNG")
    ///     .position(1000000, 1000000);
    ///
    /// assert_eq!(img.width, 100);
    /// assert_eq!(img.height, 100);
    /// assert_eq!(img.format, "PNG");
    /// ```
    pub fn from_base64(data: &str, width: u32, height: u32, format: &str) -> Self {
        let format_upper = format.to_uppercase();
        let ext = match format_upper.as_str() {
            "JPEG" => "jpg",
            _ => &format_upper.to_lowercase(),
        };
        let filename = format!("image_{}.{}", uuid::Uuid::new_v4(), ext);
        
        Image {
            filename,
            width,
            height,
            x: 0,
            y: 0,
            format: format_upper,
            source: Some(ImageSource::Base64(data.to_string())),
        }
    }
    
    /// Create an image from raw bytes
    pub fn from_bytes(data: Vec<u8>, width: u32, height: u32, format: &str) -> Self {
        let format_upper = format.to_uppercase();
        let ext = match format_upper.as_str() {
            "JPEG" => "jpg",
            _ => &format_upper.to_lowercase(),
        };
        let filename = format!("image_{}.{}", uuid::Uuid::new_v4(), ext);
        
        Image {
            filename,
            width,
            height,
            x: 0,
            y: 0,
            format: format_upper,
            source: Some(ImageSource::Bytes(data)),
        }
    }
    
    /// Get the image data as bytes (decodes base64 if needed)
    pub fn get_bytes(&self) -> Option<Vec<u8>> {
        match &self.source {
            Some(ImageSource::Base64(data)) => {
                // Decode base64
                base64_decode(data).ok()
            }
            Some(ImageSource::Bytes(data)) => Some(data.clone()),
            Some(ImageSource::File(path)) => {
                std::fs::read(path).ok()
            }
            None => None,
        }
    }

    /// Set image position
    pub fn position(mut self, x: u32, y: u32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Get aspect ratio
    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    /// Scale image to width while maintaining aspect ratio
    pub fn scale_to_width(mut self, width: u32) -> Self {
        let ratio = self.aspect_ratio();
        self.width = width;
        self.height = (width as f64 / ratio) as u32;
        self
    }

    /// Scale image to height while maintaining aspect ratio
    pub fn scale_to_height(mut self, height: u32) -> Self {
        let ratio = self.aspect_ratio();
        self.height = height;
        self.width = (height as f64 * ratio) as u32;
        self
    }

    /// Get file extension from filename
    pub fn extension(&self) -> String {
        Path::new(&self.filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_else(|| self.format.to_lowercase())
    }

    /// Get MIME type for the image format
    pub fn mime_type(&self) -> String {
        match self.format.as_str() {
            "PNG" => "image/png".to_string(),
            "JPG" | "JPEG" => "image/jpeg".to_string(),
            "GIF" => "image/gif".to_string(),
            "BMP" => "image/bmp".to_string(),
            "TIFF" => "image/tiff".to_string(),
            "SVG" => "image/svg+xml".to_string(),
            _ => "application/octet-stream".to_string(),
        }
    }
}

/// Decode base64 string to bytes
fn base64_decode(input: &str) -> Result<Vec<u8>, std::io::Error> {
    // Simple base64 decoder
    const DECODE_TABLE: [i8; 128] = [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 62, -1, -1, -1, 63,
        52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -1, -1, -1,
        -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
        15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1,
        -1, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, -1, -1, -1, -1, -1,
    ];
    
    let input = input.trim().replace(['\n', '\r', ' '], "");
    let mut output = Vec::with_capacity(input.len() * 3 / 4);
    let bytes: Vec<u8> = input.bytes().collect();
    
    let mut i = 0;
    while i < bytes.len() {
        let mut buf = [0u8; 4];
        let mut pad = 0;
        
        for j in 0..4 {
            if i + j >= bytes.len() {
                buf[j] = 0;
                pad += 1;
            } else if bytes[i + j] == b'=' {
                buf[j] = 0;
                pad += 1;
            } else if bytes[i + j] < 128 && DECODE_TABLE[bytes[i + j] as usize] >= 0 {
                buf[j] = DECODE_TABLE[bytes[i + j] as usize] as u8;
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid base64 character",
                ));
            }
        }
        
        output.push((buf[0] << 2) | (buf[1] >> 4));
        if pad < 2 {
            output.push((buf[1] << 4) | (buf[2] >> 2));
        }
        if pad < 1 {
            output.push((buf[2] << 6) | buf[3]);
        }
        
        i += 4;
    }
    
    Ok(output)
}

/// Image builder for fluent API
pub struct ImageBuilder {
    filename: String,
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    format: String,
    source: Option<ImageSource>,
}

impl ImageBuilder {
    /// Create a new image builder from file
    pub fn new(filename: &str, width: u32, height: u32) -> Self {
        let format = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_uppercase())
            .unwrap_or_else(|| "PNG".to_string());

        ImageBuilder {
            filename: filename.to_string(),
            width,
            height,
            x: 0,
            y: 0,
            format,
            source: Some(ImageSource::File(filename.to_string())),
        }
    }
    
    /// Create image builder from base64 data
    pub fn from_base64(data: &str, width: u32, height: u32, format: &str) -> Self {
        let format_upper = format.to_uppercase();
        let ext = match format_upper.as_str() {
            "JPEG" => "jpg",
            _ => &format_upper.to_lowercase(),
        };
        
        ImageBuilder {
            filename: format!("image.{}", ext),
            width,
            height,
            x: 0,
            y: 0,
            format: format_upper,
            source: Some(ImageSource::Base64(data.to_string())),
        }
    }
    
    /// Create image builder from bytes
    pub fn from_bytes(data: Vec<u8>, width: u32, height: u32, format: &str) -> Self {
        let format_upper = format.to_uppercase();
        let ext = match format_upper.as_str() {
            "JPEG" => "jpg",
            _ => &format_upper.to_lowercase(),
        };
        
        ImageBuilder {
            filename: format!("image.{}", ext),
            width,
            height,
            x: 0,
            y: 0,
            format: format_upper,
            source: Some(ImageSource::Bytes(data)),
        }
    }

    /// Set image position
    pub fn position(mut self, x: u32, y: u32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set image format
    pub fn format(mut self, format: &str) -> Self {
        self.format = format.to_uppercase();
        self
    }

    /// Scale to width
    pub fn scale_to_width(mut self, width: u32) -> Self {
        let ratio = self.width as f64 / self.height as f64;
        self.width = width;
        self.height = (width as f64 / ratio) as u32;
        self
    }

    /// Scale to height
    pub fn scale_to_height(mut self, height: u32) -> Self {
        let ratio = self.width as f64 / self.height as f64;
        self.height = height;
        self.width = (height as f64 * ratio) as u32;
        self
    }

    /// Build the image
    pub fn build(self) -> Image {
        Image {
            filename: self.filename,
            width: self.width,
            height: self.height,
            x: self.x,
            y: self.y,
            format: self.format,
            source: self.source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_creation() {
        let img = Image::new("test.png", 1920, 1080, "PNG");
        assert_eq!(img.filename, "test.png");
        assert_eq!(img.width, 1920);
        assert_eq!(img.height, 1080);
    }

    #[test]
    fn test_image_position() {
        let img = Image::new("test.png", 1920, 1080, "PNG")
            .position(500000, 1000000);
        assert_eq!(img.x, 500000);
        assert_eq!(img.y, 1000000);
    }

    #[test]
    fn test_image_aspect_ratio() {
        let img = Image::new("test.png", 1920, 1080, "PNG");
        let ratio = img.aspect_ratio();
        assert!((ratio - 1.777).abs() < 0.01);
    }

    #[test]
    fn test_image_scale_to_width() {
        let img = Image::new("test.png", 1920, 1080, "PNG")
            .scale_to_width(960);
        assert_eq!(img.width, 960);
        assert_eq!(img.height, 540);
    }

    #[test]
    fn test_image_scale_to_height() {
        let img = Image::new("test.png", 1920, 1080, "PNG")
            .scale_to_height(540);
        assert_eq!(img.width, 960);
        assert_eq!(img.height, 540);
    }

    #[test]
    fn test_image_extension() {
        let img = Image::new("photo.jpg", 1920, 1080, "JPEG");
        assert_eq!(img.extension(), "jpg");
    }

    #[test]
    fn test_image_mime_types() {
        assert_eq!(
            Image::new("test.png", 100, 100, "PNG").mime_type(),
            "image/png"
        );
        assert_eq!(
            Image::new("test.jpg", 100, 100, "JPG").mime_type(),
            "image/jpeg"
        );
        assert_eq!(
            Image::new("test.gif", 100, 100, "GIF").mime_type(),
            "image/gif"
        );
    }

    #[test]
    fn test_image_builder() {
        let img = ImageBuilder::new("photo.png", 1920, 1080)
            .position(500000, 1000000)
            .scale_to_width(960)
            .build();

        assert_eq!(img.filename, "photo.png");
        assert_eq!(img.width, 960);
        assert_eq!(img.height, 540);
        assert_eq!(img.x, 500000);
        assert_eq!(img.y, 1000000);
    }

    #[test]
    fn test_image_builder_auto_format() {
        let img = ImageBuilder::new("photo.jpg", 1920, 1080).build();
        assert_eq!(img.format, "JPG");
    }
    
    #[test]
    fn test_image_from_base64() {
        // 1x1 PNG image in base64
        let base64_png = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";
        let img = Image::from_base64(base64_png, 100, 100, "PNG");
        
        assert!(img.filename.ends_with(".png"));
        assert_eq!(img.format, "PNG");
        assert!(matches!(img.source, Some(ImageSource::Base64(_))));
    }
    
    #[test]
    fn test_image_from_bytes() {
        let data = vec![0x89, 0x50, 0x4E, 0x47]; // PNG header
        let img = Image::from_bytes(data.clone(), 100, 100, "PNG");
        
        assert_eq!(img.format, "PNG");
        assert!(matches!(img.source, Some(ImageSource::Bytes(_))));
    }
    
    #[test]
    fn test_base64_decode() {
        // Test simple base64 decode
        let result = base64_decode("SGVsbG8=").unwrap();
        assert_eq!(result, b"Hello");
        
        // Test with padding
        let result = base64_decode("SGVsbG8gV29ybGQ=").unwrap();
        assert_eq!(result, b"Hello World");
    }
    
    #[test]
    fn test_image_get_bytes_base64() {
        let base64_png = "SGVsbG8="; // "Hello" in base64
        let img = Image::from_base64(base64_png, 100, 100, "PNG");
        
        let bytes = img.get_bytes().unwrap();
        assert_eq!(bytes, b"Hello");
    }
    
    #[test]
    fn test_image_builder_from_base64() {
        let base64_data = "SGVsbG8=";
        let img = ImageBuilder::from_base64(base64_data, 200, 150, "JPEG")
            .position(1000, 2000)
            .build();
        
        assert_eq!(img.width, 200);
        assert_eq!(img.height, 150);
        assert_eq!(img.x, 1000);
        assert_eq!(img.y, 2000);
        assert_eq!(img.format, "JPEG");
    }
}
