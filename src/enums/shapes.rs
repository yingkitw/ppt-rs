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

// Note: AutoShapeType is defined in shapes/autoshape.rs to avoid duplication

