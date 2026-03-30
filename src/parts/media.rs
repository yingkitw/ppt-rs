//! Media part
//!
//! Represents embedded media (video/audio) in the presentation.

use super::base::{ContentType, Part, PartType};
use crate::exc::PptxError;

/// Media format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaFormat {
    Mp4,
    Webm,
    Avi,
    Wmv,
    Mov,
    Mp3,
    Wav,
    Wma,
    M4a,
    Ogg,
}

impl MediaFormat {
    /// Get file extension
    pub fn extension(&self) -> &'static str {
        match self {
            MediaFormat::Mp4 => "mp4",
            MediaFormat::Webm => "webm",
            MediaFormat::Avi => "avi",
            MediaFormat::Wmv => "wmv",
            MediaFormat::Mov => "mov",
            MediaFormat::Mp3 => "mp3",
            MediaFormat::Wav => "wav",
            MediaFormat::Wma => "wma",
            MediaFormat::M4a => "m4a",
            MediaFormat::Ogg => "ogg",
        }
    }

    /// Get MIME type
    pub fn mime_type(&self) -> &'static str {
        match self {
            MediaFormat::Mp4 => "video/mp4",
            MediaFormat::Webm => "video/webm",
            MediaFormat::Avi => "video/x-msvideo",
            MediaFormat::Wmv => "video/x-ms-wmv",
            MediaFormat::Mov => "video/quicktime",
            MediaFormat::Mp3 => "audio/mpeg",
            MediaFormat::Wav => "audio/wav",
            MediaFormat::Wma => "audio/x-ms-wma",
            MediaFormat::M4a => "audio/mp4",
            MediaFormat::Ogg => "audio/ogg",
        }
    }

    /// Check if this is a video format
    pub fn is_video(&self) -> bool {
        matches!(
            self,
            MediaFormat::Mp4
                | MediaFormat::Webm
                | MediaFormat::Avi
                | MediaFormat::Wmv
                | MediaFormat::Mov
        )
    }

    /// Check if this is an audio format
    pub fn is_audio(&self) -> bool {
        !self.is_video()
    }

    /// Parse from extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "mp4" => Some(MediaFormat::Mp4),
            "webm" => Some(MediaFormat::Webm),
            "avi" => Some(MediaFormat::Avi),
            "wmv" => Some(MediaFormat::Wmv),
            "mov" => Some(MediaFormat::Mov),
            "mp3" => Some(MediaFormat::Mp3),
            "wav" => Some(MediaFormat::Wav),
            "wma" => Some(MediaFormat::Wma),
            "m4a" => Some(MediaFormat::M4a),
            "ogg" => Some(MediaFormat::Ogg),
            _ => None,
        }
    }
}

/// Media part (ppt/media/mediaN.ext)
#[derive(Debug, Clone)]
pub struct MediaPart {
    path: String,
    media_number: usize,
    format: MediaFormat,
    data: Vec<u8>,
    duration_ms: Option<u64>,
}

impl MediaPart {
    /// Create a new media part
    pub fn new(media_number: usize, format: MediaFormat, data: Vec<u8>) -> Self {
        MediaPart {
            path: format!("ppt/media/media{}.{}", media_number, format.extension()),
            media_number,
            format,
            data,
            duration_ms: None,
        }
    }

    /// Create from file
    pub fn from_file(media_number: usize, file_path: &str) -> Result<Self, PptxError> {
        let data = std::fs::read(file_path)?;
        let ext = std::path::Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| PptxError::InvalidValue("No file extension".to_string()))?;

        let format = MediaFormat::from_extension(ext)
            .ok_or_else(|| PptxError::InvalidValue(format!("Unsupported media format: {}", ext)))?;

        Ok(Self::new(media_number, format, data))
    }

    /// Get media number
    pub fn media_number(&self) -> usize {
        self.media_number
    }

    /// Get format
    pub fn format(&self) -> MediaFormat {
        self.format
    }

    /// Get data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Set duration in milliseconds
    pub fn set_duration(&mut self, duration_ms: u64) {
        self.duration_ms = Some(duration_ms);
    }

    /// Get duration in milliseconds
    pub fn duration(&self) -> Option<u64> {
        self.duration_ms
    }

    /// Check if this is video
    pub fn is_video(&self) -> bool {
        self.format.is_video()
    }

    /// Check if this is audio
    pub fn is_audio(&self) -> bool {
        self.format.is_audio()
    }

    /// Get relative path for relationships
    pub fn rel_target(&self) -> String {
        format!(
            "../media/media{}.{}",
            self.media_number,
            self.format.extension()
        )
    }
}

impl Part for MediaPart {
    fn path(&self) -> &str {
        &self.path
    }

    fn part_type(&self) -> PartType {
        PartType::Image // Media uses similar handling to images
    }

    fn content_type(&self) -> ContentType {
        ContentType::Image(self.format.extension().to_string())
    }

    fn to_xml(&self) -> Result<String, PptxError> {
        // Media parts are binary, not XML
        Err(PptxError::InvalidOperation(
            "Media parts are binary, not XML".to_string(),
        ))
    }

    fn from_xml(_xml: &str) -> Result<Self, PptxError> {
        Err(PptxError::InvalidOperation(
            "Media parts cannot be created from XML".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_format_extension() {
        assert_eq!(MediaFormat::Mp4.extension(), "mp4");
        assert_eq!(MediaFormat::Mp3.extension(), "mp3");
    }

    #[test]
    fn test_media_format_mime_type() {
        assert_eq!(MediaFormat::Mp4.mime_type(), "video/mp4");
        assert_eq!(MediaFormat::Mp3.mime_type(), "audio/mpeg");
    }

    #[test]
    fn test_media_format_is_video() {
        assert!(MediaFormat::Mp4.is_video());
        assert!(MediaFormat::Webm.is_video());
        assert!(!MediaFormat::Mp3.is_video());
    }

    #[test]
    fn test_media_format_is_audio() {
        assert!(MediaFormat::Mp3.is_audio());
        assert!(MediaFormat::Wav.is_audio());
        assert!(!MediaFormat::Mp4.is_audio());
    }

    #[test]
    fn test_media_format_from_extension() {
        assert_eq!(MediaFormat::from_extension("mp4"), Some(MediaFormat::Mp4));
        assert_eq!(MediaFormat::from_extension("MP3"), Some(MediaFormat::Mp3));
        assert_eq!(MediaFormat::from_extension("xyz"), None);
    }

    #[test]
    fn test_media_part_new() {
        let media = MediaPart::new(1, MediaFormat::Mp4, vec![0, 1, 2, 3]);
        assert_eq!(media.media_number(), 1);
        assert_eq!(media.format(), MediaFormat::Mp4);
        assert_eq!(media.path(), "ppt/media/media1.mp4");
    }

    #[test]
    fn test_media_part_rel_target() {
        let media = MediaPart::new(2, MediaFormat::Mp3, vec![]);
        assert_eq!(media.rel_target(), "../media/media2.mp3");
    }

    #[test]
    fn test_media_part_duration() {
        let mut media = MediaPart::new(1, MediaFormat::Mp4, vec![]);
        assert_eq!(media.duration(), None);
        media.set_duration(5000);
        assert_eq!(media.duration(), Some(5000));
    }
}
