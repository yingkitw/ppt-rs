//! Media part (video/audio)

use crate::error::Result;
use crate::opc::part::{BasePart, Part};
use crate::opc::packuri::PackURI;
use crate::opc::relationships::Relationships;

/// Media part - contains video or audio data
pub struct MediaPart {
    base: BasePart,
    media_type: String,
}

impl MediaPart {
    /// Create a new media part
    pub fn new(partname: PackURI, content_type: &str, blob: Vec<u8>) -> Result<Self> {
        let base = BasePart::with_blob(content_type, partname, blob)?;
        Ok(Self {
            base,
            media_type: content_type.to_string(),
        })
    }

    /// Get the media type
    pub fn media_type(&self) -> &str {
        &self.media_type
    }

    /// Check if this is a video
    pub fn is_video(&self) -> bool {
        self.media_type.starts_with("video/")
    }

    /// Check if this is audio
    pub fn is_audio(&self) -> bool {
        self.media_type.starts_with("audio/")
    }
}

impl Part for MediaPart {
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
        // Media parts are binary, not XML
        Ok(String::new())
    }

    fn from_xml<R: std::io::Read>(_reader: R) -> Result<Self> {
        // Media parts are binary, not XML - this is correct behavior
        Err(crate::error::PptError::NotImplemented(
            "MediaPart::from_xml - media files are binary, not XML".to_string(),
        ))
    }
}

