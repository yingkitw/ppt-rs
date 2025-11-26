//! Shape enumeration types

use super::{BaseEnum, BaseXmlEnum};

/// Specifies the shape type
pub struct MsoShapeType;

impl MsoShapeType {
    pub const AUTO_SHAPE: BaseEnum = BaseEnum::new("AUTO_SHAPE", 1, "AutoShape.");
    pub const CALLOUT: BaseEnum = BaseEnum::new("CALLOUT", 2, "Callout.");
    pub const CANVAS: BaseEnum = BaseEnum::new("CANVAS", 20, "Canvas.");
    pub const CHART: BaseEnum = BaseEnum::new("CHART", 3, "Chart.");
    pub const COMMENT: BaseEnum = BaseEnum::new("COMMENT", 4, "Comment.");
    pub const FREEFORM: BaseEnum = BaseEnum::new("FREEFORM", 5, "Freeform.");
    pub const GROUP: BaseEnum = BaseEnum::new("GROUP", 6, "Group.");
    pub const EMBEDDED_OLE_CONTROL_OBJECT: BaseEnum = BaseEnum::new("EMBEDDED_OLE_CONTROL_OBJECT", 7, "Embedded OLE control object.");
    pub const FORM_CONTROL: BaseEnum = BaseEnum::new("FORM_CONTROL", 8, "Form control.");
    pub const MEDIA: BaseEnum = BaseEnum::new("MEDIA", 16, "Media.");
    pub const OLE_CONTROL_OBJECT: BaseEnum = BaseEnum::new("OLE_CONTROL_OBJECT", 9, "OLE control object.");
    pub const PICTURE: BaseEnum = BaseEnum::new("PICTURE", 13, "Picture.");
    pub const PLACEHOLDER: BaseEnum = BaseEnum::new("PLACEHOLDER", 14, "Placeholder.");
    pub const SCRIPT_ANCHOR: BaseEnum = BaseEnum::new("SCRIPT_ANCHOR", 10, "Script anchor.");
    pub const TABLE: BaseEnum = BaseEnum::new("TABLE", 19, "Table.");
    pub const TEXT_BOX: BaseEnum = BaseEnum::new("TEXT_BOX", 17, "Text box.");
    pub const THREE_D_MODEL: BaseEnum = BaseEnum::new("THREE_D_MODEL", 21, "3D model.");
}

/// Specifies the text anchor type
pub struct MsoTextAnchorType;

impl MsoTextAnchorType {
    pub const TOP: BaseXmlEnum = BaseXmlEnum::new("TOP", 1, Some("t"), "Top anchor.");
    pub const MIDDLE: BaseXmlEnum = BaseXmlEnum::new("MIDDLE", 3, Some("ctr"), "Middle anchor.");
    pub const BOTTOM: BaseXmlEnum = BaseXmlEnum::new("BOTTOM", 4, Some("b"), "Bottom anchor.");
    pub const TOP_CENTERED: BaseXmlEnum = BaseXmlEnum::new("TOP_CENTERED", 5, Some("t"), "Top centered anchor.");
    pub const MIDDLE_CENTERED: BaseXmlEnum = BaseXmlEnum::new("MIDDLE_CENTERED", 6, Some("ctr"), "Middle centered anchor.");
    pub const BOTTOM_CENTERED: BaseXmlEnum = BaseXmlEnum::new("BOTTOM_CENTERED", 7, Some("b"), "Bottom centered anchor.");
}

/// Specifies the placeholder type
pub struct PpPlaceholderType;

impl PpPlaceholderType {
    pub const BODY: BaseEnum = BaseEnum::new("BODY", 2, "Body placeholder.");
    pub const CHART: BaseEnum = BaseEnum::new("CHART", 8, "Chart placeholder.");
    pub const CENTER_TITLE: BaseEnum = BaseEnum::new("CENTER_TITLE", 3, "Center title placeholder.");
    pub const DATE: BaseEnum = BaseEnum::new("DATE", 16, "Date placeholder.");
    pub const FOOTER: BaseEnum = BaseEnum::new("FOOTER", 15, "Footer placeholder.");
    pub const HEADER: BaseEnum = BaseEnum::new("HEADER", 14, "Header placeholder.");
    pub const OBJECT: BaseEnum = BaseEnum::new("OBJECT", 7, "Object placeholder.");
    pub const PICTURE: BaseEnum = BaseEnum::new("PICTURE", 18, "Picture placeholder.");
    pub const SLIDE_NUMBER: BaseEnum = BaseEnum::new("SLIDE_NUMBER", 13, "Slide number placeholder.");
    pub const TABLE: BaseEnum = BaseEnum::new("TABLE", 12, "Table placeholder.");
    pub const TITLE: BaseEnum = BaseEnum::new("TITLE", 1, "Title placeholder.");
    pub const VERTICAL_BODY: BaseEnum = BaseEnum::new("VERTICAL_BODY", 6, "Vertical body placeholder.");
    pub const VERTICAL_TITLE: BaseEnum = BaseEnum::new("VERTICAL_TITLE", 5, "Vertical title placeholder.");
}
