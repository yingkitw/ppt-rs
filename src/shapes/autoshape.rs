//! AutoShape functionality

use crate::shapes::base::{BaseShape, Shape};
use crate::shapes::hyperlink::Hyperlink;
use crate::text::TextFrame;

/// AutoShape - predefined shapes like rectangles, circles, etc.
pub struct AutoShape {
    base: BaseShape,
    shape_type: AutoShapeType,
    text_frame: Option<TextFrame>,
    hyperlink: Option<Hyperlink>,
}

/// AutoShape types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoShapeType {
    // Basic shapes
    Rectangle,
    RoundedRectangle,
    Oval,
    Line,
    
    // Triangles
    Triangle,
    RightTriangle,
    IsoscelesTriangle,
    
    // Quadrilaterals
    Parallelogram,
    Trapezoid,
    Diamond,
    
    // Polygons
    Pentagon,
    Hexagon,
    Octagon,
    Decagon,
    Dodecagon,
    
    // Stars and arrows
    Star,
    Star4,
    Star5,
    Star6,
    Star8,
    Star16,
    Star24,
    Star32,
    Arrow,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    LeftRightArrow,
    UpDownArrow,
    QuadArrow,
    LeftRightUpArrow,
    BentArrow,
    BentUpArrow,
    UTurnArrow,
    LeftUpArrow,
    RightUpArrow,
    LeftRightUpArrow2,
    CurvedRightArrow,
    CurvedLeftArrow,
    CurvedUpArrow,
    CurvedDownArrow,
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
    QuadArrowCallout,
    CircularArrow,
    
    // Flowchart shapes
    FlowchartProcess,
    FlowchartAlternateProcess,
    FlowchartDecision,
    FlowchartData,
    FlowchartPredefinedProcess,
    FlowchartInternalStorage,
    FlowchartDocument,
    FlowchartMultidocument,
    FlowchartTerminator,
    FlowchartPreparation,
    FlowchartManualInput,
    FlowchartManualOperation,
    FlowchartConnector,
    FlowchartOffpageConnector,
    FlowchartCard,
    FlowchartPunchedCard,
    FlowchartPunchedTape,
    FlowchartSummingJunction,
    FlowchartOr,
    FlowchartCollate,
    FlowchartSort,
    FlowchartExtract,
    FlowchartMerge,
    FlowchartOfflineStorage,
    FlowchartOnlineStorage,
    FlowchartMagneticTape,
    FlowchartMagneticDisk,
    FlowchartMagneticDrum,
    FlowchartDisplay,
    FlowchartDelay,
    
    // Callouts
    RectangularCallout,
    RoundedRectangularCallout,
    OvalCallout,
    CloudCallout,
    LineCallout1,
    LineCallout2,
    LineCallout3,
    LineCallout4,
    BentLineCallout1,
    BentLineCallout2,
    BentLineCallout3,
    AccentCallout1,
    AccentCallout2,
    AccentCallout3,
    
    // Action buttons
    ActionButtonCustom,
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
    
    // Other shapes
    Arc,
    Bevel,
    BlockArc,
    Can,
    Chord,
    Cube,
    CurvedConnector2,
    CurvedConnector3,
    CurvedConnector4,
    CurvedConnector5,
    Donut,
    DoubleBracket,
    DoubleWave,
    Funnel,
    Heart,
    Hexagon2,
    HomePlate,
    LightningBolt,
    Moon,
    NoSymbol,
    Plaque,
    Plus,
    Ring,
    SmileyFace,
    Snip1Rect,
    Snip2Rect,
    SnipRoundRect,
    Snip2SameRect,
    Sun,
    Teardrop,
    Wave,
    WedgeEllipseCallout,
    WedgeRectCallout,
    WedgeRRectCallout,
}

impl AutoShape {
    /// Create a new AutoShape
    pub fn new(id: u32, name: String, shape_type: AutoShapeType) -> Self {
        Self {
            base: BaseShape::new(id, name),
            shape_type,
            text_frame: None,
            hyperlink: None,
        }
    }
    
    /// Create a new AutoShape with text frame
    pub fn with_text_frame(id: u32, name: String, shape_type: AutoShapeType) -> Self {
        Self {
            base: BaseShape::new(id, name),
            shape_type,
            text_frame: Some(TextFrame::new()),
            hyperlink: None,
        }
    }
    
    /// Get the shape type
    pub fn shape_type(&self) -> AutoShapeType {
        self.shape_type
    }
    
    /// Set the shape type
    pub fn set_shape_type(&mut self, shape_type: AutoShapeType) {
        self.shape_type = shape_type;
    }
}

impl Shape for AutoShape {
    fn id(&self) -> u32 {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }
    
    fn set_name(&mut self, name: String) {
        self.base.set_name(name);
    }
    
    fn left(&self) -> i64 {
        self.base.left()
    }
    
    fn set_left(&mut self, left: i64) {
        self.base.set_left(left);
    }
    
    fn top(&self) -> i64 {
        self.base.top()
    }
    
    fn set_top(&mut self, top: i64) {
        self.base.set_top(top);
    }
    
    fn width(&self) -> u32 {
        self.base.width()
    }
    
    fn set_width(&mut self, width: u32) {
        self.base.set_width(width);
    }
    
    fn height(&self) -> u32 {
        self.base.height()
    }
    
    fn set_height(&mut self, height: u32) {
        self.base.set_height(height);
    }
    
    fn has_text_frame(&self) -> bool {
        self.text_frame.is_some()
    }
    
    fn text_frame(&self) -> Option<&TextFrame> {
        self.text_frame.as_ref()
    }
    
    fn text_frame_mut(&mut self) -> Option<&mut TextFrame> {
        self.text_frame.as_mut()
    }
    
    fn hyperlink(&self) -> Option<&Hyperlink> {
        self.hyperlink.as_ref()
    }
    
    fn hyperlink_mut(&mut self) -> Option<&mut Hyperlink> {
        self.hyperlink.as_mut()
    }
    
    fn set_hyperlink(&mut self, hyperlink: Option<Hyperlink>) {
        self.hyperlink = hyperlink;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autoshape_type_equality() {
        assert_eq!(AutoShapeType::Rectangle, AutoShapeType::Rectangle);
        assert_ne!(AutoShapeType::Rectangle, AutoShapeType::Oval);
    }

    #[test]
    fn test_autoshape_type_copy() {
        let st1 = AutoShapeType::Star;
        let st2 = st1;
        assert_eq!(st1, st2);
    }

    #[test]
    fn test_autoshape_type_debug() {
        let st = AutoShapeType::Triangle;
        let debug_str = format!("{:?}", st);
        assert!(debug_str.contains("Triangle"));
    }

    #[test]
    fn test_autoshape_shape_type() {
        let mut shape = AutoShape::new(1, "Shape".to_string(), AutoShapeType::Rectangle);
        assert_eq!(shape.shape_type(), AutoShapeType::Rectangle);
        
        shape.set_shape_type(AutoShapeType::Oval);
        assert_eq!(shape.shape_type(), AutoShapeType::Oval);
    }

    #[test]
    fn test_autoshape_all_types() {
        let types = vec![
            AutoShapeType::Rectangle,
            AutoShapeType::Oval,
            AutoShapeType::Triangle,
            AutoShapeType::Star,
            AutoShapeType::Arrow,
            AutoShapeType::Heart,
            AutoShapeType::LightningBolt,
            AutoShapeType::FlowchartProcess,
            AutoShapeType::RectangularCallout,
            AutoShapeType::ActionButtonCustom,
        ];
        
        for shape_type in types {
            let shape = AutoShape::new(1, "Test".to_string(), shape_type);
            assert_eq!(shape.shape_type(), shape_type);
        }
    }
}

