//! Shape creation support for PPTX generation
//!
//! Provides shape types, fills, lines, and builders for creating shapes in slides.

/// Shape types available in PPTX
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ShapeType {
    // Basic shapes
    Rectangle,
    RoundedRectangle,
    Ellipse,
    Circle, // Alias for Ellipse
    Triangle,
    RightTriangle,
    Diamond,
    Pentagon,
    Hexagon,
    Octagon,
    
    // Arrows
    RightArrow,
    LeftArrow,
    UpArrow,
    DownArrow,
    LeftRightArrow,
    UpDownArrow,
    BentArrow,
    UTurnArrow,
    
    // Stars and banners
    Star4,
    Star5,
    Star6,
    Star8,
    Ribbon,
    Wave,
    
    // Callouts
    WedgeRectCallout,
    WedgeEllipseCallout,
    CloudCallout,
    
    // Flow chart
    FlowChartProcess,
    FlowChartDecision,
    FlowChartTerminator,
    FlowChartDocument,
    FlowChartPredefinedProcess,
    FlowChartInternalStorage,
    FlowChartData,
    FlowChartInputOutput,
    FlowChartManualInput,
    FlowChartManualOperation,
    FlowChartConnector,
    FlowChartOffPageConnector,
    FlowChartPunchedCard,
    FlowChartPunchedTape,
    FlowChartSummingJunction,
    FlowChartOr,
    FlowChartCollate,
    FlowChartSort,
    FlowChartExtract,
    FlowChartMerge,
    FlowChartOnlineStorage,
    FlowChartDelay,
    FlowChartMagneticTape,
    FlowChartMagneticDisk,
    FlowChartMagneticDrum,
    FlowChartDisplay,
    FlowChartPreparation,
    
    // More arrows
    CurvedRightArrow,
    CurvedLeftArrow,
    CurvedUpArrow,
    CurvedDownArrow,
    CurvedLeftRightArrow,
    CurvedUpDownArrow,
    StripedRightArrow,
    NotchedRightArrow,
    PentagonArrow,
    ChevronArrow,
    RightArrowCallout,
    LeftArrowCallout,
    UpArrowCallout,
    DownArrowCallout,
    LeftRightArrowCallout,
    UpDownArrowCallout,
    QuadArrow,
    LeftRightUpArrow,
    CircularArrow,
    
    // More geometric shapes
    Parallelogram,
    Trapezoid,
    NonIsoscelesTrapezoid,
    IsoscelesTrapezoid,
    Cube,
    Can,
    Cone,
    Cylinder,
    Bevel,
    Donut,
    NoSmoking,
    BlockArc,
    FoldedCorner,
    SmileyFace,
    Arc,
    Chord,
    Pie,
    Teardrop,
    Plaque,
    MusicNote,
    PictureFrame,
    
    // More decorative
    Star10,
    Star12,
    Star16,
    Star24,
    Star32,
    Seal,
    Seal4,
    Seal8,
    Seal16,
    Seal32,
    ActionButtonBlank,
    ActionButtonHome,
    ActionButtonHelp,
    ActionButtonInformation,
    ActionButtonForwardNext,
    ActionButtonBackPrevious,
    ActionButtonBeginning,
    ActionButtonEnd,
    ActionButtonReturn,
    ActionButtonDocument,
    ActionButtonSound,
    ActionButtonMovie,
    
    // Other (original shapes)
    Heart,
    Lightning,
    Sun,
    Moon,
    Cloud,
    Brace,
    Bracket,
    Plus,
    Minus,
}

impl ShapeType {
    /// Get the preset geometry name for the shape (OOXML preset name)
    pub fn preset_name(&self) -> &'static str {
        match self {
            ShapeType::Rectangle => "rect",
            ShapeType::RoundedRectangle => "roundRect",
            ShapeType::Ellipse | ShapeType::Circle => "ellipse",
            ShapeType::Triangle => "triangle",
            ShapeType::RightTriangle => "rtTriangle",
            ShapeType::Diamond => "diamond",
            ShapeType::Pentagon => "pentagon",
            ShapeType::Hexagon => "hexagon",
            ShapeType::Octagon => "octagon",
            
            ShapeType::RightArrow => "rightArrow",
            ShapeType::LeftArrow => "leftArrow",
            ShapeType::UpArrow => "upArrow",
            ShapeType::DownArrow => "downArrow",
            ShapeType::LeftRightArrow => "leftRightArrow",
            ShapeType::UpDownArrow => "upDownArrow",
            ShapeType::BentArrow => "bentArrow",
            ShapeType::UTurnArrow => "uturnArrow",
            
            ShapeType::Star4 => "star4",
            ShapeType::Star5 => "star5",
            ShapeType::Star6 => "star6",
            ShapeType::Star8 => "star8",
            ShapeType::Ribbon => "ribbon2",
            ShapeType::Wave => "wave",
            
            ShapeType::WedgeRectCallout => "wedgeRectCallout",
            ShapeType::WedgeEllipseCallout => "wedgeEllipseCallout",
            ShapeType::CloudCallout => "cloudCallout",
            
            ShapeType::FlowChartProcess => "flowChartProcess",
            ShapeType::FlowChartDecision => "flowChartDecision",
            ShapeType::FlowChartTerminator => "flowChartTerminator",
            ShapeType::FlowChartDocument => "flowChartDocument",
            ShapeType::FlowChartPredefinedProcess => "flowChartPredefinedProcess",
            ShapeType::FlowChartInternalStorage => "flowChartInternalStorage",
            ShapeType::FlowChartData => "flowChartData",
            ShapeType::FlowChartInputOutput => "flowChartInputOutput",
            ShapeType::FlowChartManualInput => "flowChartManualInput",
            ShapeType::FlowChartManualOperation => "flowChartManualOperation",
            ShapeType::FlowChartConnector => "flowChartConnector",
            ShapeType::FlowChartOffPageConnector => "flowChartOffPageConnector",
            ShapeType::FlowChartPunchedCard => "flowChartPunchedCard",
            ShapeType::FlowChartPunchedTape => "flowChartPunchedTape",
            ShapeType::FlowChartSummingJunction => "flowChartSummingJunction",
            ShapeType::FlowChartOr => "flowChartOr",
            ShapeType::FlowChartCollate => "flowChartCollate",
            ShapeType::FlowChartSort => "flowChartSort",
            ShapeType::FlowChartExtract => "flowChartExtract",
            ShapeType::FlowChartMerge => "flowChartMerge",
            ShapeType::FlowChartOnlineStorage => "flowChartOnlineStorage",
            ShapeType::FlowChartDelay => "flowChartDelay",
            ShapeType::FlowChartMagneticTape => "flowChartMagneticTape",
            ShapeType::FlowChartMagneticDisk => "flowChartMagneticDisk",
            ShapeType::FlowChartMagneticDrum => "flowChartMagneticDrum",
            ShapeType::FlowChartDisplay => "flowChartDisplay",
            ShapeType::FlowChartPreparation => "flowChartPreparation",
            
            ShapeType::CurvedRightArrow => "curvedRightArrow",
            ShapeType::CurvedLeftArrow => "curvedLeftArrow",
            ShapeType::CurvedUpArrow => "curvedUpArrow",
            ShapeType::CurvedDownArrow => "curvedDownArrow",
            ShapeType::CurvedLeftRightArrow => "curvedLeftRightArrow",
            ShapeType::CurvedUpDownArrow => "curvedUpDownArrow",
            ShapeType::StripedRightArrow => "stripedRightArrow",
            ShapeType::NotchedRightArrow => "notchedRightArrow",
            ShapeType::PentagonArrow => "pentArrow",
            ShapeType::ChevronArrow => "chevron",
            ShapeType::RightArrowCallout => "rightArrowCallout",
            ShapeType::LeftArrowCallout => "leftArrowCallout",
            ShapeType::UpArrowCallout => "upArrowCallout",
            ShapeType::DownArrowCallout => "downArrowCallout",
            ShapeType::LeftRightArrowCallout => "leftRightArrowCallout",
            ShapeType::UpDownArrowCallout => "upDownArrowCallout",
            ShapeType::QuadArrow => "quadArrow",
            ShapeType::LeftRightUpArrow => "leftRightUpArrow",
            ShapeType::CircularArrow => "circularArrow",
            
            ShapeType::Parallelogram => "parallelogram",
            ShapeType::Trapezoid => "trapezoid",
            ShapeType::NonIsoscelesTrapezoid => "nonIsoscelesTrapezoid",
            ShapeType::IsoscelesTrapezoid => "isoTrapezoid",
            ShapeType::Cube => "cube",
            ShapeType::Can => "can",
            ShapeType::Cone => "cone",
            ShapeType::Cylinder => "cylinder",
            ShapeType::Bevel => "bevel",
            ShapeType::Donut => "donut",
            ShapeType::NoSmoking => "noSmoking",
            ShapeType::BlockArc => "blockArc",
            ShapeType::FoldedCorner => "foldedCorner",
            ShapeType::SmileyFace => "smileyFace",
            ShapeType::Arc => "arc",
            ShapeType::Chord => "chord",
            ShapeType::Pie => "pie",
            ShapeType::Teardrop => "teardrop",
            ShapeType::Plaque => "plaque",
            ShapeType::MusicNote => "musicNote",
            ShapeType::PictureFrame => "frame",
            
            ShapeType::Star10 => "star10",
            ShapeType::Star12 => "star12",
            ShapeType::Star16 => "star16",
            ShapeType::Star24 => "star24",
            ShapeType::Star32 => "star32",
            ShapeType::Seal => "seal",
            ShapeType::Seal4 => "seal4",
            ShapeType::Seal8 => "seal8",
            ShapeType::Seal16 => "seal16",
            ShapeType::Seal32 => "seal32",
            ShapeType::ActionButtonBlank => "actionButtonBlank",
            ShapeType::ActionButtonHome => "actionButtonHome",
            ShapeType::ActionButtonHelp => "actionButtonHelp",
            ShapeType::ActionButtonInformation => "actionButtonInformation",
            ShapeType::ActionButtonForwardNext => "actionButtonForwardNext",
            ShapeType::ActionButtonBackPrevious => "actionButtonBackPrevious",
            ShapeType::ActionButtonBeginning => "actionButtonBeginning",
            ShapeType::ActionButtonEnd => "actionButtonEnd",
            ShapeType::ActionButtonReturn => "actionButtonReturn",
            ShapeType::ActionButtonDocument => "actionButtonDocument",
            ShapeType::ActionButtonSound => "actionButtonSound",
            ShapeType::ActionButtonMovie => "actionButtonMovie",
            
            ShapeType::Heart => "heart",
            ShapeType::Lightning => "lightningBolt",
            ShapeType::Sun => "sun",
            ShapeType::Moon => "moon",
            ShapeType::Cloud => "cloud",
            ShapeType::Brace => "leftBrace",
            ShapeType::Bracket => "leftBracket",
            ShapeType::Plus => "mathPlus",
            ShapeType::Minus => "mathMinus",
        }
    }

    /// Get a user-friendly name for the shape
    pub fn display_name(&self) -> &'static str {
        match self {
            ShapeType::Rectangle => "Rectangle",
            ShapeType::RoundedRectangle => "Rounded Rectangle",
            ShapeType::Ellipse => "Ellipse",
            ShapeType::Circle => "Circle",
            ShapeType::Triangle => "Triangle",
            ShapeType::RightTriangle => "Right Triangle",
            ShapeType::Diamond => "Diamond",
            ShapeType::Pentagon => "Pentagon",
            ShapeType::Hexagon => "Hexagon",
            ShapeType::Octagon => "Octagon",
            ShapeType::RightArrow => "Right Arrow",
            ShapeType::LeftArrow => "Left Arrow",
            ShapeType::UpArrow => "Up Arrow",
            ShapeType::DownArrow => "Down Arrow",
            ShapeType::LeftRightArrow => "Left-Right Arrow",
            ShapeType::UpDownArrow => "Up-Down Arrow",
            ShapeType::BentArrow => "Bent Arrow",
            ShapeType::UTurnArrow => "U-Turn Arrow",
            ShapeType::Star4 => "4-Point Star",
            ShapeType::Star5 => "5-Point Star",
            ShapeType::Star6 => "6-Point Star",
            ShapeType::Star8 => "8-Point Star",
            ShapeType::Ribbon => "Ribbon",
            ShapeType::Wave => "Wave",
            ShapeType::WedgeRectCallout => "Rectangle Callout",
            ShapeType::WedgeEllipseCallout => "Oval Callout",
            ShapeType::CloudCallout => "Cloud Callout",
            ShapeType::FlowChartProcess => "Process",
            ShapeType::FlowChartDecision => "Decision",
            ShapeType::FlowChartTerminator => "Terminator",
            ShapeType::FlowChartDocument => "Document",
            ShapeType::FlowChartPredefinedProcess => "Predefined Process",
            ShapeType::FlowChartInternalStorage => "Internal Storage",
            ShapeType::FlowChartData => "Data",
            ShapeType::FlowChartInputOutput => "Input/Output",
            ShapeType::FlowChartManualInput => "Manual Input",
            ShapeType::FlowChartManualOperation => "Manual Operation",
            ShapeType::FlowChartConnector => "Connector",
            ShapeType::FlowChartOffPageConnector => "Off-page Connector",
            ShapeType::FlowChartPunchedCard => "Punched Card",
            ShapeType::FlowChartPunchedTape => "Punched Tape",
            ShapeType::FlowChartSummingJunction => "Summing Junction",
            ShapeType::FlowChartOr => "Or",
            ShapeType::FlowChartCollate => "Collate",
            ShapeType::FlowChartSort => "Sort",
            ShapeType::FlowChartExtract => "Extract",
            ShapeType::FlowChartMerge => "Merge",
            ShapeType::FlowChartOnlineStorage => "Online Storage",
            ShapeType::FlowChartDelay => "Delay",
            ShapeType::FlowChartMagneticTape => "Magnetic Tape",
            ShapeType::FlowChartMagneticDisk => "Magnetic Disk",
            ShapeType::FlowChartMagneticDrum => "Magnetic Drum",
            ShapeType::FlowChartDisplay => "Display",
            ShapeType::FlowChartPreparation => "Preparation",
            
            ShapeType::CurvedRightArrow => "Curved Right Arrow",
            ShapeType::CurvedLeftArrow => "Curved Left Arrow",
            ShapeType::CurvedUpArrow => "Curved Up Arrow",
            ShapeType::CurvedDownArrow => "Curved Down Arrow",
            ShapeType::CurvedLeftRightArrow => "Curved Left-Right Arrow",
            ShapeType::CurvedUpDownArrow => "Curved Up-Down Arrow",
            ShapeType::StripedRightArrow => "Striped Right Arrow",
            ShapeType::NotchedRightArrow => "Notched Right Arrow",
            ShapeType::PentagonArrow => "Pentagon Arrow",
            ShapeType::ChevronArrow => "Chevron Arrow",
            ShapeType::RightArrowCallout => "Right Arrow Callout",
            ShapeType::LeftArrowCallout => "Left Arrow Callout",
            ShapeType::UpArrowCallout => "Up Arrow Callout",
            ShapeType::DownArrowCallout => "Down Arrow Callout",
            ShapeType::LeftRightArrowCallout => "Left-Right Arrow Callout",
            ShapeType::UpDownArrowCallout => "Up-Down Arrow Callout",
            ShapeType::QuadArrow => "Quad Arrow",
            ShapeType::LeftRightUpArrow => "Left-Right-Up Arrow",
            ShapeType::CircularArrow => "Circular Arrow",
            
            ShapeType::Parallelogram => "Parallelogram",
            ShapeType::Trapezoid => "Trapezoid",
            ShapeType::NonIsoscelesTrapezoid => "Non-Isosceles Trapezoid",
            ShapeType::IsoscelesTrapezoid => "Isosceles Trapezoid",
            ShapeType::Cube => "Cube",
            ShapeType::Can => "Can",
            ShapeType::Cone => "Cone",
            ShapeType::Cylinder => "Cylinder",
            ShapeType::Bevel => "Bevel",
            ShapeType::Donut => "Donut",
            ShapeType::NoSmoking => "No Smoking",
            ShapeType::BlockArc => "Block Arc",
            ShapeType::FoldedCorner => "Folded Corner",
            ShapeType::SmileyFace => "Smiley Face",
            ShapeType::Arc => "Arc",
            ShapeType::Chord => "Chord",
            ShapeType::Pie => "Pie",
            ShapeType::Teardrop => "Teardrop",
            ShapeType::Plaque => "Plaque",
            ShapeType::MusicNote => "Music Note",
            ShapeType::PictureFrame => "Picture Frame",
            
            ShapeType::Star10 => "10-Point Star",
            ShapeType::Star12 => "12-Point Star",
            ShapeType::Star16 => "16-Point Star",
            ShapeType::Star24 => "24-Point Star",
            ShapeType::Star32 => "32-Point Star",
            ShapeType::Seal => "Seal",
            ShapeType::Seal4 => "4-Point Seal",
            ShapeType::Seal8 => "8-Point Seal",
            ShapeType::Seal16 => "16-Point Seal",
            ShapeType::Seal32 => "32-Point Seal",
            ShapeType::ActionButtonBlank => "Action Button (Blank)",
            ShapeType::ActionButtonHome => "Action Button (Home)",
            ShapeType::ActionButtonHelp => "Action Button (Help)",
            ShapeType::ActionButtonInformation => "Action Button (Information)",
            ShapeType::ActionButtonForwardNext => "Action Button (Forward/Next)",
            ShapeType::ActionButtonBackPrevious => "Action Button (Back/Previous)",
            ShapeType::ActionButtonBeginning => "Action Button (Beginning)",
            ShapeType::ActionButtonEnd => "Action Button (End)",
            ShapeType::ActionButtonReturn => "Action Button (Return)",
            ShapeType::ActionButtonDocument => "Action Button (Document)",
            ShapeType::ActionButtonSound => "Action Button (Sound)",
            ShapeType::ActionButtonMovie => "Action Button (Movie)",
            
            ShapeType::Heart => "Heart",
            ShapeType::Lightning => "Lightning Bolt",
            ShapeType::Sun => "Sun",
            ShapeType::Moon => "Moon",
            ShapeType::Cloud => "Cloud",
            ShapeType::Brace => "Brace",
            ShapeType::Bracket => "Bracket",
            ShapeType::Plus => "Plus",
            ShapeType::Minus => "Minus",
        }
    }
}

/// Gradient direction for linear gradients
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum GradientDirection {
    /// Left to right (0 degrees)
    Horizontal,
    /// Top to bottom (90 degrees)
    Vertical,
    /// Top-left to bottom-right (45 degrees)
    DiagonalDown,
    /// Bottom-left to top-right (315 degrees)
    DiagonalUp,
    /// Custom angle in degrees (0-360)
    Angle(u32),
}

impl GradientDirection {
    /// Get angle in 60000ths of a degree (OOXML format)
    pub fn to_angle(&self) -> u32 {
        match self {
            GradientDirection::Horizontal => 0,
            GradientDirection::Vertical => 5400000,      // 90 * 60000
            GradientDirection::DiagonalDown => 2700000,  // 45 * 60000
            GradientDirection::DiagonalUp => 18900000,   // 315 * 60000
            GradientDirection::Angle(deg) => deg * 60000,
        }
    }
}

/// A gradient stop (color at a position)
#[derive(Clone, Debug)]
pub struct GradientStop {
    pub color: String,
    pub position: u32,  // 0-100000 (percentage * 1000)
    pub transparency: Option<u32>,
}

impl GradientStop {
    /// Create a gradient stop at a position (0-100%)
    pub fn new(color: &str, position_percent: u32) -> Self {
        GradientStop {
            color: color.trim_start_matches('#').to_uppercase(),
            position: position_percent.min(100) * 1000,
            transparency: None,
        }
    }
    
    /// Set transparency (0-100 percent)
    pub fn with_transparency(mut self, percent: u32) -> Self {
        let alpha = ((100 - percent.min(100)) * 1000) as u32;
        self.transparency = Some(alpha);
        self
    }
}

/// Gradient fill definition
#[derive(Clone, Debug)]
pub struct GradientFill {
    pub stops: Vec<GradientStop>,
    pub direction: GradientDirection,
}

impl GradientFill {
    /// Create a simple two-color gradient
    pub fn linear(start_color: &str, end_color: &str, direction: GradientDirection) -> Self {
        GradientFill {
            stops: vec![
                GradientStop::new(start_color, 0),
                GradientStop::new(end_color, 100),
            ],
            direction,
        }
    }
    
    /// Create a three-color gradient
    pub fn three_color(start: &str, middle: &str, end: &str, direction: GradientDirection) -> Self {
        GradientFill {
            stops: vec![
                GradientStop::new(start, 0),
                GradientStop::new(middle, 50),
                GradientStop::new(end, 100),
            ],
            direction,
        }
    }
    
    /// Add a custom stop
    pub fn with_stop(mut self, stop: GradientStop) -> Self {
        self.stops.push(stop);
        self.stops.sort_by_key(|s| s.position);
        self
    }
}

/// Shape fill type - solid color or gradient
#[derive(Clone, Debug)]
pub enum FillType {
    Solid(ShapeFill),
    Gradient(GradientFill),
    NoFill,
}

/// Shape fill/color properties
#[derive(Clone, Debug)]
pub struct ShapeFill {
    pub color: String, // RGB hex color (e.g., "FF0000")
    pub transparency: Option<u32>, // 0-100000 (100000 = fully transparent)
}

impl ShapeFill {
    /// Create new shape fill with color
    pub fn new(color: &str) -> Self {
        ShapeFill {
            color: color.trim_start_matches('#').to_uppercase(),
            transparency: None,
        }
    }

    /// Set transparency (0-100 percent)
    pub fn with_transparency(mut self, percent: u32) -> Self {
        let alpha = ((100 - percent.min(100)) * 1000) as u32;
        self.transparency = Some(alpha);
        self
    }
    
    /// Set transparency (0-100 percent) - builder style (deprecated, use with_transparency)
    pub fn transparency(self, percent: u32) -> Self {
        self.with_transparency(percent)
    }
}

/// Shape line/border properties
#[derive(Clone, Debug)]
pub struct ShapeLine {
    pub color: String,
    pub width: u32, // in EMU (English Metric Units)
}

impl ShapeLine {
    /// Create new shape line with color and width
    pub fn new(color: &str, width: u32) -> Self {
        ShapeLine {
            color: color.trim_start_matches('#').to_uppercase(),
            width,
        }
    }
}

use crate::core::{Positioned, ElementSized};

/// Shape definition
#[derive(Clone, Debug)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub x: u32,      // Position X in EMU
    pub y: u32,      // Position Y in EMU
    pub width: u32,  // Width in EMU
    pub height: u32, // Height in EMU
    pub fill: Option<ShapeFill>,
    pub gradient: Option<GradientFill>,
    pub line: Option<ShapeLine>,
    pub text: Option<String>,
    /// Optional fixed shape ID for connector anchoring
    pub id: Option<u32>,
    /// Rotation in degrees (0-360)
    pub rotation: Option<i32>,
    /// Optional hyperlink
    pub hyperlink: Option<crate::generator::hyperlinks::Hyperlink>,
}

impl Shape {
    /// Create a new shape
    pub fn new(shape_type: ShapeType, x: u32, y: u32, width: u32, height: u32) -> Self {
        Shape {
            shape_type,
            x,
            y,
            width,
            height,
            fill: None,
            gradient: None,
            line: None,
            text: None,
            id: None,
            rotation: None,
            hyperlink: None,
        }
    }

    /// Set shape ID for connector anchoring
    pub fn with_id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }

    /// Set shape rotation in degrees
    pub fn with_rotation(mut self, degrees: i32) -> Self {
        self.rotation = Some(degrees);
        self
    }

    /// Set shape hyperlink
    pub fn with_hyperlink(mut self, hyperlink: crate::generator::hyperlinks::Hyperlink) -> Self {
        self.hyperlink = Some(hyperlink);
        self
    }

    /// Set shape fill (solid color)
    pub fn with_fill(mut self, fill: ShapeFill) -> Self {
        self.fill = Some(fill);
        self.gradient = None; // Clear gradient if setting solid fill
        self
    }
    
    /// Set gradient fill
    pub fn with_gradient(mut self, gradient: GradientFill) -> Self {
        self.gradient = Some(gradient);
        self.fill = None; // Clear solid fill if setting gradient
        self
    }

    /// Set shape line
    pub fn with_line(mut self, line: ShapeLine) -> Self {
        self.line = Some(line);
        self
    }

    /// Set shape text
    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }
}

impl Positioned for Shape {
    fn x(&self) -> u32 { self.x }
    fn y(&self) -> u32 { self.y }
    fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

impl ElementSized for Shape {
    fn width(&self) -> u32 { self.width }
    fn height(&self) -> u32 { self.height }
    fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

/// Convert EMU (English Metric Units) to inches
pub fn emu_to_inches(emu: u32) -> f64 {
    emu as f64 / 914400.0
}

/// Convert inches to EMU
pub fn inches_to_emu(inches: f64) -> u32 {
    (inches * 914400.0) as u32
}

/// Convert centimeters to EMU
pub fn cm_to_emu(cm: f64) -> u32 {
    (cm * 360000.0) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_type_names() {
        assert_eq!(ShapeType::Rectangle.preset_name(), "rect");
        assert_eq!(ShapeType::Circle.preset_name(), "ellipse");
        assert_eq!(ShapeType::RightArrow.preset_name(), "rightArrow");
        assert_eq!(ShapeType::Star5.preset_name(), "star5");
        assert_eq!(ShapeType::Heart.preset_name(), "heart");
    }

    #[test]
    fn test_shape_fill_builder() {
        let fill = ShapeFill::new("FF0000").transparency(50);
        assert_eq!(fill.color, "FF0000");
        assert_eq!(fill.transparency, Some(50000));
    }

    #[test]
    fn test_shape_builder() {
        let shape = Shape::new(ShapeType::Rectangle, 0, 0, 1000000, 500000)
            .with_fill(ShapeFill::new("0000FF"))
            .with_line(ShapeLine::new("000000", 25400))
            .with_text("Hello");

        assert_eq!(shape.x, 0);
        assert_eq!(shape.width, 1000000);
        assert_eq!(shape.text, Some("Hello".to_string()));
    }

    #[test]
    fn test_emu_conversions() {
        let emu = inches_to_emu(1.0);
        assert_eq!(emu, 914400);
        assert!((emu_to_inches(emu) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_cm_to_emu() {
        let emu = cm_to_emu(2.54); // 1 inch
        assert_eq!(emu, 914400);
    }
}
