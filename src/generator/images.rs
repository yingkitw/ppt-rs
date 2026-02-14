//! Image handling for PPTX presentations
//!
//! Handles image metadata, embedding, and XML generation

use std::path::Path;
use crate::core::{Positioned, ElementSized, Dimension};

/// Normalize format string and derive file extension
fn format_and_ext(format: &str) -> (String, String) {
    let upper = format.to_uppercase();
    let ext = match upper.as_str() {
        "JPEG" => "jpg".to_string(),
        _ => upper.to_lowercase(),
    };
    (upper, ext)
}

/// Generate a unique image filename from format string
fn generate_image_filename(format: &str) -> (String, String) {
    let (upper, ext) = format_and_ext(format);
    let filename = format!("image_{}.{}", uuid::Uuid::new_v4(), ext);
    (filename, upper)
}

/// Image data source
#[derive(Clone, Debug)]
pub enum ImageSource {
    /// Load from file path
    File(String),
    /// Base64 encoded data
    Base64(String),
    /// Raw bytes
    Bytes(Vec<u8>),
    /// Load from URL
    #[cfg(feature = "web2ppt")]
    Url(String),
}

/// Image crop configuration (values 0.0 to 1.0)
#[derive(Clone, Debug, Default)]
pub struct Crop {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Crop {
    /// Create a new crop configuration
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Self { left, top, right, bottom }
    }
}

/// Image effects
#[derive(Clone, Debug)]
pub enum ImageEffect {
    /// Outer shadow
    Shadow,
    /// Reflection
    Reflection,
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
    /// Image cropping
    pub crop: Option<Crop>,
    /// Image effects
    pub effects: Vec<ImageEffect>,
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
            crop: None,
            effects: Vec::new(),
        }
    }

    /// Create an image from a file path, automatically detecting dimensions
    pub fn from_path<P: AsRef<Path>>(path: P) -> std::result::Result<Self, String> {
        let path = path.as_ref();
        let filename = path.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "image.png".to_string());
        let path_str = path.to_string_lossy().to_string();
        
        let data = std::fs::read(path)
            .map_err(|e| format!("Failed to open image: {e}"))?;
        let (w, h, format) = read_image_dimensions(&data)
            .ok_or_else(|| "Failed to detect image dimensions (unsupported format)".to_string())?;
            
        // Convert pixels to EMU (assuming 96 DPI): 1 pixel = 9525 EMU
        let w_emu = w * 9525;
        let h_emu = h * 9525;
        
        Ok(Image {
            filename,
            width: w_emu,
            height: h_emu,
            x: 0,
            y: 0,
            format,
            source: Some(ImageSource::File(path_str)),
            crop: None,
            effects: Vec::new(),
        })
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
        let (filename, fmt) = generate_image_filename(format);
        Self::with_source(filename, width, height, fmt, ImageSource::Base64(data.to_string()))
    }
    
    /// Create an image from raw bytes
    pub fn from_bytes(data: Vec<u8>, width: u32, height: u32, format: &str) -> Self {
        let (filename, fmt) = generate_image_filename(format);
        Self::with_source(filename, width, height, fmt, ImageSource::Bytes(data))
    }

    /// Create an image from URL
    #[cfg(feature = "web2ppt")]
    pub fn from_url(url: &str, width: u32, height: u32, format: &str) -> Self {
        let (filename, fmt) = generate_image_filename(format);
        Self::with_source(filename, width, height, fmt, ImageSource::Url(url.to_string()))
    }

    /// Internal constructor to avoid repeating struct init
    fn with_source(filename: String, width: u32, height: u32, format: String, source: ImageSource) -> Self {
        Image {
            filename,
            width,
            height,
            x: 0,
            y: 0,
            format,
            source: Some(source),
            crop: None,
            effects: Vec::new(),
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
            #[cfg(feature = "web2ppt")]
            Some(ImageSource::Url(url)) => {
                // Use blocking client to fetch image
                // Set User-Agent to mimic browser to avoid some 403s
                let client = reqwest::blocking::Client::builder()
                    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
                    .build()
                    .ok()?;
                    
                match client.get(url).send() {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            resp.bytes().ok().map(|b| b.to_vec())
                        } else {
                            None
                        }
                    },
                    Err(_) => None,
                }
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

    /// Set image cropping
    pub fn with_crop(mut self, left: f64, top: f64, right: f64, bottom: f64) -> Self {
        self.crop = Some(Crop::new(left, top, right, bottom));
        self
    }

    /// Add an image effect
    pub fn with_effect(mut self, effect: ImageEffect) -> Self {
        self.effects.push(effect);
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

    /// Set position using flexible Dimension units (fluent).
    pub fn at(mut self, x: Dimension, y: Dimension) -> Self {
        self.x = x.to_emu_x();
        self.y = y.to_emu_y();
        self
    }

    /// Set size using flexible Dimension units (fluent).
    pub fn with_dimensions(mut self, width: Dimension, height: Dimension) -> Self {
        self.width = width.to_emu_x();
        self.height = height.to_emu_y();
        self
    }
}

impl Positioned for Image {
    fn x(&self) -> u32 { self.x }
    fn y(&self) -> u32 { self.y }
    fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

impl ElementSized for Image {
    fn width(&self) -> u32 { self.width }
    fn height(&self) -> u32 { self.height }
    fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
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
            if i + j >= bytes.len() || bytes[i + j] == b'=' {
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
        let (upper, ext) = format_and_ext(format);
        ImageBuilder {
            filename: format!("image.{}", ext),
            width, height, x: 0, y: 0,
            format: upper,
            source: Some(ImageSource::Base64(data.to_string())),
        }
    }
    
    /// Create image builder from bytes
    pub fn from_bytes(data: Vec<u8>, width: u32, height: u32, format: &str) -> Self {
        let (upper, ext) = format_and_ext(format);
        ImageBuilder {
            filename: format!("image.{}", ext),
            width, height, x: 0, y: 0,
            format: upper,
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
            crop: None,
            effects: Vec::new(),
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

    #[test]
    fn test_read_png_dimensions() {
        // Minimal 1x1 PNG
        let png: Vec<u8> = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // signature
            0x00, 0x00, 0x00, 0x0D, // IHDR length
            0x49, 0x48, 0x44, 0x52, // "IHDR"
            0x00, 0x00, 0x00, 0x01, // width=1
            0x00, 0x00, 0x00, 0x01, // height=1
            0x08, 0x02, 0x00, 0x00, 0x00, // bit depth, color type, etc.
        ];
        let (w, h, fmt) = read_image_dimensions(&png).unwrap();
        assert_eq!((w, h), (1, 1));
        assert_eq!(fmt, "PNG");
    }

    #[test]
    fn test_read_gif_dimensions() {
        let gif: Vec<u8> = vec![
            0x47, 0x49, 0x46, 0x38, 0x39, 0x61, // "GIF89a"
            0x0A, 0x00, // width=10 (little-endian)
            0x14, 0x00, // height=20
        ];
        let (w, h, fmt) = read_image_dimensions(&gif).unwrap();
        assert_eq!((w, h), (10, 20));
        assert_eq!(fmt, "GIF");
    }

    #[test]
    fn test_read_bmp_dimensions() {
        let mut bmp = vec![0u8; 26];
        bmp[0] = 0x42; bmp[1] = 0x4D; // "BM"
        bmp[18..22].copy_from_slice(&100u32.to_le_bytes()); // width=100
        bmp[22..26].copy_from_slice(&200u32.to_le_bytes()); // height=200
        let (w, h, fmt) = read_image_dimensions(&bmp).unwrap();
        assert_eq!((w, h), (100, 200));
        assert_eq!(fmt, "BMP");
    }
}

/// Read image dimensions from file header bytes (PNG, JPEG, GIF, BMP, WebP).
/// Returns (width, height, format_name) or None if unrecognized.
fn read_image_dimensions(data: &[u8]) -> Option<(u32, u32, String)> {
    if data.len() < 10 {
        return None;
    }
    // PNG: 8-byte signature, then IHDR chunk with width/height as big-endian u32
    if data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) && data.len() >= 24 {
        let w = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
        let h = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
        return Some((w, h, "PNG".into()));
    }
    // JPEG: starts with FF D8, scan for SOF0/SOF2 marker
    if data.starts_with(&[0xFF, 0xD8]) {
        return read_jpeg_dimensions(data);
    }
    // GIF: "GIF87a" or "GIF89a", width/height as little-endian u16 at offset 6
    if data.starts_with(b"GIF8") && data.len() >= 10 {
        let w = u16::from_le_bytes([data[6], data[7]]) as u32;
        let h = u16::from_le_bytes([data[8], data[9]]) as u32;
        return Some((w, h, "GIF".into()));
    }
    // BMP: "BM", width/height as little-endian u32 at offset 18/22
    if data.starts_with(b"BM") && data.len() >= 26 {
        let w = u32::from_le_bytes([data[18], data[19], data[20], data[21]]);
        let h = u32::from_le_bytes([data[22], data[23], data[24], data[25]]);
        return Some((w, h, "BMP".into()));
    }
    // WebP: "RIFF....WEBP", VP8 chunk has dimensions
    if data.len() >= 30 && &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP" {
        // VP8 lossy: width/height at offset 26/28 as little-endian u16
        if &data[12..16] == b"VP8 " && data.len() >= 30 {
            let w = u16::from_le_bytes([data[26], data[27]]) as u32 & 0x3FFF;
            let h = u16::from_le_bytes([data[28], data[29]]) as u32 & 0x3FFF;
            return Some((w, h, "WEBP".into()));
        }
        // VP8L lossless: dimensions encoded at offset 21
        if &data[12..16] == b"VP8L" && data.len() >= 25 {
            let b0 = data[21] as u32;
            let b1 = data[22] as u32;
            let b2 = data[23] as u32;
            let b3 = data[24] as u32;
            let bits = b0 | (b1 << 8) | (b2 << 16) | (b3 << 24);
            let w = (bits & 0x3FFF) + 1;
            let h = ((bits >> 14) & 0x3FFF) + 1;
            return Some((w, h, "WEBP".into()));
        }
    }
    None
}

/// Scan JPEG markers to find SOF0/SOF2 frame with dimensions
fn read_jpeg_dimensions(data: &[u8]) -> Option<(u32, u32, String)> {
    let mut i = 2;
    while i + 1 < data.len() {
        if data[i] != 0xFF {
            i += 1;
            continue;
        }
        let marker = data[i + 1];
        i += 2;
        // SOF0 (0xC0) or SOF2 (0xC2): height at +3, width at +5 (big-endian u16)
        if (marker == 0xC0 || marker == 0xC2) && i + 7 < data.len() {
            let h = u16::from_be_bytes([data[i + 3], data[i + 4]]) as u32;
            let w = u16::from_be_bytes([data[i + 5], data[i + 6]]) as u32;
            return Some((w, h, "JPEG".into()));
        }
        // Skip non-SOF markers by reading segment length
        if marker >= 0xC0 && marker != 0xD9 && marker != 0xDA && i + 1 < data.len() {
            let len = u16::from_be_bytes([data[i], data[i + 1]]) as usize;
            i += len;
        }
    }
    None
}
