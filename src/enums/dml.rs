//! Drawing Markup Language (DML) enumeration types

use super::{BaseEnum, BaseXmlEnum};

/// Specifies the fill type
pub struct MsoFillType;

impl MsoFillType {
    pub const BACKGROUND: BaseEnum = BaseEnum::new("BACKGROUND", 5, "Background fill.");
    pub const GRADIENT: BaseEnum = BaseEnum::new("GRADIENT", 3, "Gradient fill.");
    pub const PATTERNED: BaseEnum = BaseEnum::new("PATTERNED", 2, "Patterned fill.");
    pub const PICTURE: BaseEnum = BaseEnum::new("PICTURE", 6, "Picture fill.");
    pub const SOLID: BaseEnum = BaseEnum::new("SOLID", 1, "Solid fill.");
    pub const TEXTURED: BaseEnum = BaseEnum::new("TEXTURED", 4, "Textured fill.");
}

/// Specifies the line dash style
pub struct MsoLineDashStyle;

impl MsoLineDashStyle {
    pub const DASH: BaseXmlEnum = BaseXmlEnum::new("DASH", 2, Some("dash"), "Dashed line.");
    pub const DASH_DOT: BaseXmlEnum = BaseXmlEnum::new("DASH_DOT", 4, Some("dashDot"), "Dash-dot line.");
    pub const LONG_DASH: BaseXmlEnum = BaseXmlEnum::new("LONG_DASH", 3, Some("lgDash"), "Long dashed line.");
    pub const SOLID: BaseXmlEnum = BaseXmlEnum::new("SOLID", 1, Some("solid"), "Solid line.");
    pub const SQUARE_DOT: BaseXmlEnum = BaseXmlEnum::new("SQUARE_DOT", 5, Some("sysDot"), "Square dot line.");
}

/// Specifies the line width
pub struct MsoLineWidth;

impl MsoLineWidth {
    pub const HAIRLINE: BaseEnum = BaseEnum::new("HAIRLINE", 1, "Hairline width.");
    pub const THIN: BaseEnum = BaseEnum::new("THIN", 2, "Thin width.");
    pub const MEDIUM: BaseEnum = BaseEnum::new("MEDIUM", 3, "Medium width.");
    pub const THICK: BaseEnum = BaseEnum::new("THICK", 4, "Thick width.");
}

/// Specifies the color scheme
pub struct MsoColorType;

impl MsoColorType {
    pub const RGB: BaseEnum = BaseEnum::new("RGB", 1, "RGB color.");
    pub const SCHEME: BaseEnum = BaseEnum::new("SCHEME", 2, "Scheme color.");
    pub const SYSTEM_COLOR: BaseEnum = BaseEnum::new("SYSTEM_COLOR", 3, "System color.");
}
