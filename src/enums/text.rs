//! Text enumeration types

use super::{BaseEnum, BaseXmlEnum};

/// Specifies the text alignment
pub struct PpParagraphAlignment;

impl PpParagraphAlignment {
    pub const CENTER: BaseXmlEnum = BaseXmlEnum::new("CENTER", 2, Some("ctr"), "Center alignment.");
    pub const DISTRIBUTE: BaseXmlEnum = BaseXmlEnum::new("DISTRIBUTE", 4, Some("dist"), "Distribute alignment.");
    pub const JUSTIFY: BaseXmlEnum = BaseXmlEnum::new("JUSTIFY", 5, Some("just"), "Justify alignment.");
    pub const LEFT: BaseXmlEnum = BaseXmlEnum::new("LEFT", 1, Some("l"), "Left alignment.");
    pub const RIGHT: BaseXmlEnum = BaseXmlEnum::new("RIGHT", 3, Some("r"), "Right alignment.");
}

/// Specifies the font bold setting
pub struct MsoTriState;

impl MsoTriState {
    pub const FALSE: BaseEnum = BaseEnum::new("FALSE", 0, "False.");
    pub const TRUE: BaseEnum = BaseEnum::new("TRUE", -1, "True.");
    pub const MIXED: BaseEnum = BaseEnum::new("MIXED", -2, "Mixed.");
}

/// Specifies the text underline style
pub struct MsoUnderlineStyle;

impl MsoUnderlineStyle {
    pub const NONE: BaseXmlEnum = BaseXmlEnum::new("NONE", 0, None, "No underline.");
    pub const SINGLE: BaseXmlEnum = BaseXmlEnum::new("SINGLE", 1, Some("sng"), "Single underline.");
    pub const DOUBLE: BaseXmlEnum = BaseXmlEnum::new("DOUBLE", 2, Some("dbl"), "Double underline.");
    pub const HEAVY: BaseXmlEnum = BaseXmlEnum::new("HEAVY", 3, Some("heavy"), "Heavy underline.");
    pub const DOTTED: BaseXmlEnum = BaseXmlEnum::new("DOTTED", 17, Some("dot"), "Dotted underline.");
    pub const DASHED: BaseXmlEnum = BaseXmlEnum::new("DASHED", 18, Some("dash"), "Dashed underline.");
}

/// Specifies the font color type
pub struct MsoColorFormat;

impl MsoColorFormat {
    pub const RGB: BaseEnum = BaseEnum::new("RGB", 1, "RGB color.");
    pub const SCHEME: BaseEnum = BaseEnum::new("SCHEME", 2, "Scheme color.");
}
