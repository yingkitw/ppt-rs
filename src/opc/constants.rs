//! OPC package constants

/// Content types for various package parts
pub mod CONTENT_TYPE {
    pub const PML_PRESENTATION_MAIN: &str = "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml";
    pub const PML_PRES_MACRO_MAIN: &str = "application/vnd.openxmlformats-officedocument.presentationml.presentation.macroEnabled.main+xml";
    pub const PML_TEMPLATE_MAIN: &str = "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml";
    pub const PML_SLIDESHOW_MAIN: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml";
    pub const OPC_CORE_PROPERTIES: &str = "application/vnd.openxmlformats-package.core-properties+xml";
    pub const PML_NOTES_MASTER: &str = "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml";
    pub const PML_NOTES_SLIDE: &str = "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml";
    pub const PML_SLIDE: &str = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml";
    pub const PML_SLIDE_LAYOUT: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml";
    pub const PML_SLIDE_MASTER: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml";
    pub const DML_CHART: &str = "application/vnd.openxmlformats-officedocument.drawingml.chart+xml";
    pub const BMP: &str = "image/bmp";
    pub const GIF: &str = "image/gif";
    pub const JPEG: &str = "image/jpeg";
    pub const MS_PHOTO: &str = "image/ms-photo";
    pub const PNG: &str = "image/png";
    pub const TIFF: &str = "image/tiff";
    pub const X_EMF: &str = "image/x-emf";
    pub const X_WMF: &str = "image/x-wmf";
    pub const ASF: &str = "video/x-ms-asf";
    pub const AVI: &str = "video/x-msvideo";
    pub const MOV: &str = "video/quicktime";
    pub const MP4: &str = "video/mp4";
    pub const MPG: &str = "video/mpeg";
    pub const MS_VIDEO: &str = "video/x-ms-video";
    pub const SWF: &str = "application/x-shockwave-flash";
    pub const VIDEO: &str = "video/unknown";
    pub const WMV: &str = "video/x-ms-wmv";
    pub const X_MS_VIDEO: &str = "video/x-ms-video";
}

/// Relationship types
pub mod RELATIONSHIP_TYPE {
    pub const SLIDE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
    pub const SLIDE_LAYOUT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout";
    pub const SLIDE_MASTER: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster";
    pub const NOTES_SLIDE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide";
    pub const NOTES_MASTER: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster";
    pub const HANDOUT_MASTER: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster";
    pub const THEME: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";
    pub const IMAGE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
    pub const MEDIA: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/media";
    pub const CHART: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart";
    pub const EMBEDDED_PACKAGE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject";
}

/// XML namespaces
pub mod NAMESPACE {
    pub const PRESENTATION_ML: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
    pub const DRAWING_ML: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
    pub const OFFICE_RELATIONSHIPS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    pub const PACKAGE_RELATIONSHIPS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
    pub const CORE_PROPERTIES: &str = "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
}
