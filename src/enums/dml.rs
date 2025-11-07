//! DrawingML-related enumerations

/// Color types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
    Rgb,
    Theme,
    Scheme,
}

/// Fill types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillType {
    NoFill,
    Solid,
    Gradient,
    Pattern,
    Picture,
    Group,
}

