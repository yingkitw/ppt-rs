//! OXML (Office XML) element handling
//!
//! Provides XML parsing and content extraction for Office Open XML documents.

pub mod action;
pub mod chart;
pub mod coreprops;
pub mod dml;
pub mod editor;
pub mod ns;
pub mod presentation;
pub mod repair;
pub mod shapes;
pub mod simpletypes;
pub mod slide;
pub mod table;
pub mod text;
pub mod theme;
pub mod xmlchemy;

// Core XML parsing
pub use xmlchemy::{BaseOxmlElement, XmlElement, XmlParser};

// Slide parsing
pub use slide::{
    Paragraph, ParsedImage, ParsedShape, ParsedSlide, ParsedTable, ParsedTableCell, SlideParser,
    TextRun,
};

// Presentation reading
pub use presentation::{PresentationInfo, PresentationReader};

// Presentation editing
pub use editor::PresentationEditor;

// Namespace utilities
pub use ns::Namespace;

// Text elements
pub use text::{
    BodyProperties, ParagraphProperties, RunProperties, TextBody, TextParagraph,
    TextRun as OxmlTextRun,
};

// Table elements
pub use table::{
    GridColumn, Table as OxmlTable, TableCell as OxmlTableCell, TableCellProperties,
    TableRow as OxmlTableRow,
};

// Shape elements
pub use shapes::{
    LineProperties, NonVisualProperties, PresetGeometry, ShapeProperties, SolidFill, Transform2D,
};

// DML elements
pub use dml::{
    Color, DashPattern, EffectExtent, Fill, Glow, GradientFill, GradientStop, LineCap, LineJoin,
    Outline, PatternFill, PictureFill, Point, Reflection, Shadow, Size, TextureFill,
};

// Chart elements
pub use chart::{
    CategoryPoint, ChartAxis, ChartKind, ChartLegend, ChartSeries as OxmlChartSeries, ChartTitle,
    DataPoint, NumericData, StringData,
};

// Repair functionality
pub use repair::{PptxRepair, RepairIssue, RepairResult};
