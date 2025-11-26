//! Chart enumeration types

use super::{BaseEnum, BaseXmlEnum};

/// Specifies the point on an axis where the other axis crosses
pub struct XlAxisCrosses;

impl XlAxisCrosses {
    pub const AUTOMATIC: BaseXmlEnum = BaseXmlEnum::new(
        "AUTOMATIC",
        -4105,
        Some("autoZero"),
        "The axis crossing point is set automatically, often at zero.",
    );
    pub const CUSTOM: BaseXmlEnum = BaseXmlEnum::new(
        "CUSTOM",
        -4114,
        None,
        "The .crosses_at property specifies the axis crossing point.",
    );
    pub const MAXIMUM: BaseXmlEnum = BaseXmlEnum::new(
        "MAXIMUM",
        2,
        Some("max"),
        "The axis crosses at the maximum value.",
    );
    pub const MINIMUM: BaseXmlEnum = BaseXmlEnum::new(
        "MINIMUM",
        4,
        Some("min"),
        "The axis crosses at the minimum value.",
    );
}

/// Specifies the type of the category axis
pub struct XlCategoryType;

impl XlCategoryType {
    pub const AUTOMATIC_SCALE: BaseEnum = BaseEnum::new(
        "AUTOMATIC_SCALE",
        -4105,
        "The application controls the axis type.",
    );
    pub const CATEGORY_SCALE: BaseEnum = BaseEnum::new(
        "CATEGORY_SCALE",
        2,
        "Axis groups data by an arbitrary set of categories",
    );
    pub const TIME_SCALE: BaseEnum = BaseEnum::new(
        "TIME_SCALE",
        3,
        "Axis groups data on a time scale of days, months, or years.",
    );
}

/// Specifies the type of a chart
pub struct XlChartType;

impl XlChartType {
    pub const THREE_D_AREA: BaseEnum = BaseEnum::new("THREE_D_AREA", -4098, "3D Area.");
    pub const THREE_D_AREA_STACKED: BaseEnum = BaseEnum::new("THREE_D_AREA_STACKED", 78, "3D Stacked Area.");
    pub const THREE_D_AREA_STACKED_100: BaseEnum = BaseEnum::new("THREE_D_AREA_STACKED_100", 79, "100% Stacked Area.");
    pub const THREE_D_BAR_CLUSTERED: BaseEnum = BaseEnum::new("THREE_D_BAR_CLUSTERED", 60, "3D Clustered Bar.");
    pub const THREE_D_BAR_STACKED: BaseEnum = BaseEnum::new("THREE_D_BAR_STACKED", 61, "3D Stacked Bar.");
    pub const THREE_D_BAR_STACKED_100: BaseEnum = BaseEnum::new("THREE_D_BAR_STACKED_100", 62, "3D 100% Stacked Bar.");
    pub const THREE_D_COLUMN: BaseEnum = BaseEnum::new("THREE_D_COLUMN", -4100, "3D Column.");
    pub const THREE_D_COLUMN_CLUSTERED: BaseEnum = BaseEnum::new("THREE_D_COLUMN_CLUSTERED", 54, "3D Clustered Column.");
    pub const THREE_D_COLUMN_STACKED: BaseEnum = BaseEnum::new("THREE_D_COLUMN_STACKED", 55, "3D Stacked Column.");
    pub const BAR_CLUSTERED: BaseEnum = BaseEnum::new("BAR_CLUSTERED", 57, "Clustered Bar.");
    pub const BAR_STACKED: BaseEnum = BaseEnum::new("BAR_STACKED", 58, "Stacked Bar.");
    pub const BAR_STACKED_100: BaseEnum = BaseEnum::new("BAR_STACKED_100", 59, "100% Stacked Bar.");
    pub const COLUMN_CLUSTERED: BaseEnum = BaseEnum::new("COLUMN_CLUSTERED", 51, "Clustered Column.");
    pub const COLUMN_STACKED: BaseEnum = BaseEnum::new("COLUMN_STACKED", 52, "Stacked Column.");
    pub const COLUMN_STACKED_100: BaseEnum = BaseEnum::new("COLUMN_STACKED_100", 53, "100% Stacked Column.");
    pub const LINE: BaseEnum = BaseEnum::new("LINE", 4, "Line.");
    pub const PIE: BaseEnum = BaseEnum::new("PIE", 5, "Pie.");
    pub const SCATTER: BaseEnum = BaseEnum::new("SCATTER", 74, "Scatter.");
    pub const AREA: BaseEnum = BaseEnum::new("AREA", 1, "Area.");
    pub const AREA_STACKED: BaseEnum = BaseEnum::new("AREA_STACKED", 76, "Stacked Area.");
    pub const AREA_STACKED_100: BaseEnum = BaseEnum::new("AREA_STACKED_100", 77, "100% Stacked Area.");
    pub const BUBBLE: BaseEnum = BaseEnum::new("BUBBLE", 15, "Bubble.");
    pub const DOUGHNUT: BaseEnum = BaseEnum::new("DOUGHNUT", -4120, "Doughnut.");
    pub const RADAR: BaseEnum = BaseEnum::new("RADAR", -4151, "Radar.");
    pub const STOCK: BaseEnum = BaseEnum::new("STOCK", 88, "Stock.");
    pub const SURFACE: BaseEnum = BaseEnum::new("SURFACE", 83, "Surface.");
}
