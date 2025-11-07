//! OPC constants - content types and relationship types

/// Content types for different parts
pub mod CONTENT_TYPE {
    // PresentationML content types
    pub const PML_PRESENTATION_MAIN: &str = "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml";
    pub const PML_PRES_MACRO_MAIN: &str = "application/vnd.ms-powerpoint.presentation.macroEnabled.main+xml";
    pub const PML_TEMPLATE_MAIN: &str = "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml";
    pub const PML_SLIDESHOW_MAIN: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml";
    pub const PML_SLIDE: &str = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml";
    pub const PML_SLIDE_LAYOUT: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml";
    pub const PML_SLIDE_MASTER: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml";
    pub const PML_NOTES_MASTER: &str = "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml";
    pub const PML_NOTES_SLIDE: &str = "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml";
    pub const PML_HANDOUT_MASTER: &str = "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml";
    pub const PML_COMMENTS: &str = "application/vnd.openxmlformats-officedocument.presentationml.comments+xml";
    pub const PML_COMMENT_AUTHORS: &str = "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml";
    pub const PML_PRES_PROPS: &str = "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml";
    pub const PML_VIEW_PROPS: &str = "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml";
    pub const PML_TABLE_STYLES: &str = "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml";
    pub const PML_TAGS: &str = "application/vnd.openxmlformats-officedocument.presentationml.tags+xml";
    pub const PML_SLIDE_UPDATE_INFO: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml";
    pub const PML_PRINTER_SETTINGS: &str = "application/vnd.openxmlformats-officedocument.presentationml.printerSettings";
    
    // DrawingML content types
    pub const DML_CHART: &str = "application/vnd.openxmlformats-officedocument.drawingml.chart+xml";
    pub const DML_CHARTSHAPES: &str = "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml";
    pub const DML_DIAGRAM_COLORS: &str = "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml";
    pub const DML_DIAGRAM_DATA: &str = "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml";
    pub const DML_DIAGRAM_DRAWING: &str = "application/vnd.ms-office.drawingml.diagramDrawing+xml";
    pub const DML_DIAGRAM_LAYOUT: &str = "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml";
    pub const DML_DIAGRAM_STYLE: &str = "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml";
    
    // Office content types
    pub const OFC_CHART_COLORS: &str = "application/vnd.ms-office.chartcolorstyle+xml";
    pub const OFC_CHART_EX: &str = "application/vnd.ms-office.chartex+xml";
    pub const OFC_CHART_STYLE: &str = "application/vnd.ms-office.chartstyle+xml";
    pub const OFC_CUSTOM_PROPERTIES: &str = "application/vnd.openxmlformats-officedocument.custom-properties+xml";
    pub const OFC_CUSTOM_XML_PROPERTIES: &str = "application/vnd.openxmlformats-officedocument.customXmlProperties+xml";
    pub const OFC_DRAWING: &str = "application/vnd.openxmlformats-officedocument.drawing+xml";
    pub const OFC_EXTENDED_PROPERTIES: &str = "application/vnd.openxmlformats-officedocument.extended-properties+xml";
    pub const OFC_OLE_OBJECT: &str = "application/vnd.openxmlformats-officedocument.oleObject";
    pub const OFC_PACKAGE: &str = "application/vnd.openxmlformats-officedocument.package";
    pub const OFC_THEME: &str = "application/vnd.openxmlformats-officedocument.theme+xml";
    pub const OFC_THEME_OVERRIDE: &str = "application/vnd.openxmlformats-officedocument.themeOverride+xml";
    pub const OFC_VML_DRAWING: &str = "application/vnd.openxmlformats-officedocument.vmlDrawing";
    
    // OPC content types
    pub const OPC_CORE_PROPERTIES: &str = "application/vnd.openxmlformats-package.core-properties+xml";
    pub const OPC_DIGITAL_SIGNATURE_CERTIFICATE: &str = "application/vnd.openxmlformats-package.digital-signature-certificate";
    pub const OPC_DIGITAL_SIGNATURE_ORIGIN: &str = "application/vnd.openxmlformats-package.digital-signature-origin";
    pub const OPC_DIGITAL_SIGNATURE_XMLSIGNATURE: &str = "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml";
    pub const OPC_RELATIONSHIPS: &str = "application/vnd.openxmlformats-package.relationships+xml";
    
    // Image types
    pub const PNG: &str = "image/png";
    pub const JPEG: &str = "image/jpeg";
    pub const GIF: &str = "image/gif";
    pub const BMP: &str = "image/bmp";
    pub const TIFF: &str = "image/tiff";
    pub const MS_PHOTO: &str = "image/vnd.ms-photo";
    pub const X_EMF: &str = "image/x-emf";
    pub const X_WMF: &str = "image/x-wmf";
    
    // Media types
    pub const ASF: &str = "video/x-ms-asf";
    pub const AVI: &str = "video/avi";
    pub const MOV: &str = "video/quicktime";
    pub const MP4: &str = "video/mp4";
    pub const MPG: &str = "video/mpeg";
    pub const MS_VIDEO: &str = "video/msvideo";
    pub const VIDEO: &str = "video/unknown";
    pub const WMV: &str = "video/x-ms-wmv";
    pub const X_MS_VIDEO: &str = "video/x-msvideo";
    pub const SWF: &str = "application/x-shockwave-flash";
    
    // Other
    pub const XML: &str = "application/xml";
    pub const INK: &str = "application/inkml+xml";
    pub const X_FONTDATA: &str = "application/x-fontdata";
    pub const X_FONT_TTF: &str = "application/x-font-ttf";
}

/// Relationship types
pub mod RELATIONSHIP_TYPE {
    // Office document relationships
    pub const OFFICE_DOCUMENT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
    
    // Core properties
    pub const CORE_PROPERTIES: &str = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";
    
    // Presentation relationships
    pub const SLIDE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
    pub const SLIDE_LAYOUT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout";
    pub const SLIDE_MASTER: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster";
    pub const NOTES_SLIDE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide";
    pub const NOTES_MASTER: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster";
    pub const HANDOUT_MASTER: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster";
    pub const PRES_PROPS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps";
    pub const VIEW_PROPS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps";
    pub const TABLE_STYLES: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles";
    pub const TAGS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags";
    pub const SLIDE_UPDATE_INFO: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideUpdateInfo";
    pub const PRINTER_SETTINGS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings";
    
    // Drawing relationships
    pub const CHART: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart";
    pub const DRAWING: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing";
    pub const THEME: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";
    pub const THEME_OVERRIDE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride";
    
    // Media relationships
    pub const IMAGE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
    pub const VIDEO: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video";
    pub const AUDIO: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio";
    pub const MEDIA: &str = "http://schemas.microsoft.com/office/2007/relationships/media";
    
    // Other relationships
    pub const HYPERLINK: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink";
    pub const OLE_OBJECT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject";
    pub const PACKAGE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package";
    pub const COMMENTS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments";
    pub const COMMENT_AUTHORS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors";
}

/// Relationship target modes
pub mod RELATIONSHIP_TARGET_MODE {
    pub const INTERNAL: &str = "Internal";
    pub const EXTERNAL: &str = "External";
}

