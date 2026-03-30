//! Base part trait and types
//!
//! Defines the common interface for all package parts.

use crate::exc::PptxError;

/// Content types for package parts
#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    Presentation,
    Slide,
    SlideLayout,
    SlideMaster,
    Theme,
    NotesSlide,
    NotesMaster,
    Image(String), // format: png, jpeg, gif, etc.
    Media(String), // format: mp4, mp3, etc.
    Chart,
    Table,
    CoreProperties,
    ExtendedProperties,
    ContentTypes,
    Relationships,
    Xml,
}

impl ContentType {
    /// Get the MIME type string
    pub fn mime_type(&self) -> &'static str {
        match self {
            ContentType::Presentation => {
                "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"
            }
            ContentType::Slide => {
                "application/vnd.openxmlformats-officedocument.presentationml.slide+xml"
            }
            ContentType::SlideLayout => {
                "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"
            }
            ContentType::SlideMaster => {
                "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"
            }
            ContentType::Theme => "application/vnd.openxmlformats-officedocument.theme+xml",
            ContentType::NotesSlide => {
                "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"
            }
            ContentType::NotesMaster => {
                "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"
            }
            ContentType::Image(fmt) => match fmt.as_str() {
                "png" => "image/png",
                "jpeg" | "jpg" => "image/jpeg",
                "gif" => "image/gif",
                "bmp" => "image/bmp",
                "tiff" => "image/tiff",
                "svg" => "image/svg+xml",
                _ => "application/octet-stream",
            },
            ContentType::Media(fmt) => match fmt.as_str() {
                "mp4" => "video/mp4",
                "webm" => "video/webm",
                "avi" => "video/x-msvideo",
                "wmv" => "video/x-ms-wmv",
                "mov" => "video/quicktime",
                "mp3" => "audio/mpeg",
                "wav" => "audio/wav",
                "wma" => "audio/x-ms-wma",
                "m4a" => "audio/mp4",
                "ogg" => "audio/ogg",
                _ => "application/octet-stream",
            },
            ContentType::Chart => {
                "application/vnd.openxmlformats-officedocument.drawingml.chart+xml"
            }
            ContentType::Table => {
                "application/vnd.openxmlformats-officedocument.drawingml.table+xml"
            }
            ContentType::CoreProperties => {
                "application/vnd.openxmlformats-package.core-properties+xml"
            }
            ContentType::ExtendedProperties => {
                "application/vnd.openxmlformats-officedocument.extended-properties+xml"
            }
            ContentType::ContentTypes => "application/vnd.openxmlformats-package.content-types+xml",
            ContentType::Relationships => {
                "application/vnd.openxmlformats-package.relationships+xml"
            }
            ContentType::Xml => "application/xml",
        }
    }
}

/// Part types in a PPTX package
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PartType {
    Presentation,
    Slide,
    SlideLayout,
    SlideMaster,
    Theme,
    NotesSlide,
    NotesMaster,
    Image,
    Media,
    Chart,
    Table,
    CoreProperties,
    ExtendedProperties,
    ContentTypes,
    Relationships,
}

impl PartType {
    /// Get the relationship type URI
    pub fn relationship_type(&self) -> &'static str {
        match self {
            PartType::Presentation => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
            PartType::Slide => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
            PartType::SlideLayout => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout",
            PartType::SlideMaster => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster",
            PartType::Theme => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
            PartType::NotesSlide => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide",
            PartType::NotesMaster => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster",
            PartType::Image => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
            PartType::Media => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/media",
            PartType::Chart => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
            PartType::Table => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table",
            PartType::CoreProperties => "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
            PartType::ExtendedProperties => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
            PartType::ContentTypes => "http://schemas.openxmlformats.org/package/2006/content-types",
            PartType::Relationships => "http://schemas.openxmlformats.org/package/2006/relationships",
        }
    }
}

/// Trait for package parts
pub trait Part {
    /// Get the part path within the package
    fn path(&self) -> &str;

    /// Get the part type
    fn part_type(&self) -> PartType;

    /// Get the content type
    fn content_type(&self) -> ContentType;

    /// Generate XML content for this part
    fn to_xml(&self) -> Result<String, PptxError>;

    /// Parse XML content into this part
    fn from_xml(xml: &str) -> Result<Self, PptxError>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_type_mime() {
        assert_eq!(
            ContentType::Slide.mime_type(),
            "application/vnd.openxmlformats-officedocument.presentationml.slide+xml"
        );
        assert_eq!(
            ContentType::Image("png".to_string()).mime_type(),
            "image/png"
        );
        assert_eq!(
            ContentType::Image("jpeg".to_string()).mime_type(),
            "image/jpeg"
        );
    }

    #[test]
    fn test_part_type_relationship() {
        assert!(PartType::Slide.relationship_type().contains("/slide"));
        assert!(PartType::Image.relationship_type().contains("/image"));
    }
}
