//! Text-related enumerations

/// Text alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
    Distributed,
    ThaiDistributed,
}

/// Vertical anchor for text
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAnchor {
    Top,
    Middle,
    Bottom,
    TopCentered,
    MiddleCentered,
    BottomCentered,
}

/// Auto size options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoSize {
    None,
    ShapeToFitText,
    TextToFitShape,
}

/// Underline types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Underline {
    None,
    Single,
    Double,
    Heavy,
    Dotted,
    Dashed,
    DotDash,
    DotDotDash,
    Wavy,
    WavyDouble,
    WavyHeavy,
    Words,
}


