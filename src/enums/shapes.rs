//! Shape-related enumerations

/// Shape types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeType {
    AutoShape,
    Picture,
    GraphicFrame,
    GroupShape,
    Connector,
    Freeform,
}

/// Placeholder types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceholderType {
    Object,
    Chart,
    Body,
    CenterTitle,
    Date,
    Footer,
    Header,
    MediaClip,
    ObjectPlaceholder,
    OrganizationChart,
    Picture,
    SlideNumber,
    Subtitle,
    Table,
    Title,
    VerticalBody,
    VerticalObject,
    VerticalTitle,
}

/// AutoShape types (subset - can be expanded)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoShapeType {
    Rectangle,
    Oval,
    Line,
    RoundedRectangle,
    Triangle,
    RightTriangle,
    Parallelogram,
    Trapezoid,
    Diamond,
    Pentagon,
    Hexagon,
    Octagon,
    Star,
    Arrow,
    // TODO: Add more shape types
}

