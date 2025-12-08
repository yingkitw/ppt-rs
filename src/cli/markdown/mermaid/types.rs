//! Shared types for Mermaid diagram parsing

use crate::generator::{Shape, ShapeType, ShapeFill, ShapeLine};
use crate::generator::shapes::{GradientFill, GradientDirection};
use crate::generator::connectors::Connector;

/// Mermaid diagram types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MermaidType {
    Flowchart,
    Sequence,
    Pie,
    Gantt,
    ClassDiagram,
    StateDiagram,
    ErDiagram,
    Mindmap,
    Timeline,
    Journey,
    Quadrant,
    GitGraph,
    Unknown,
}

/// Direction of flowchart layout
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlowDirection {
    LeftToRight,  // LR
    RightToLeft,  // RL
    TopToBottom,  // TB/TD
    BottomToTop,  // BT
}

/// A parsed flowchart node
#[derive(Debug, Clone)]
pub struct FlowNode {
    pub id: String,
    pub label: String,
    pub shape: NodeShape,
}

/// Node shape types in Mermaid
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeShape {
    Rectangle,      // [text]
    RoundedRect,    // (text)
    Stadium,        // ([text])
    Diamond,        // {text}
    Circle,         // ((text))
    Hexagon,        // {{text}}
}

/// A connection between nodes
#[derive(Debug, Clone)]
pub struct FlowConnection {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub arrow_type: ArrowStyle,
}

/// Arrow styles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArrowStyle {
    Arrow,      // -->
    Open,       // ---
    Dotted,     // -.->
    Thick,      // ==>
}

/// A subgraph grouping
#[derive(Debug, Clone)]
pub struct Subgraph {
    pub name: String,
    pub nodes: Vec<String>,
}

/// Parsed flowchart
#[derive(Debug, Clone)]
pub struct Flowchart {
    pub direction: FlowDirection,
    pub nodes: Vec<FlowNode>,
    pub connections: Vec<FlowConnection>,
    pub subgraphs: Vec<Subgraph>,
}

/// Bounding box for diagram positioning
#[derive(Debug, Clone, Copy)]
pub struct DiagramBounds {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl DiagramBounds {
    /// Calculate bounds from a set of positions and dimensions
    pub fn from_elements(positions: &[(u32, u32, u32, u32)]) -> Option<Self> {
        if positions.is_empty() {
            return None;
        }
        
        let mut min_x = u32::MAX;
        let mut min_y = u32::MAX;
        let mut max_x = 0u32;
        let mut max_y = 0u32;
        
        for &(x, y, w, h) in positions {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x + w);
            max_y = max_y.max(y + h);
        }
        
        Some(Self {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        })
    }
}

/// Result containing shapes and connectors
pub struct DiagramElements {
    pub shapes: Vec<Shape>,
    pub connectors: Vec<Connector>,
    /// Bounding box of the diagram for positioning
    pub bounds: Option<DiagramBounds>,
    /// Whether elements should be grouped (for future `<p:grpSp>` support)
    #[allow(dead_code)]
    pub grouped: bool,
}

impl DiagramElements {
    /// Create from shapes only (calculates bounds automatically)
    pub fn from_shapes(shapes: Vec<Shape>) -> Self {
        let element_bounds: Vec<(u32, u32, u32, u32)> = shapes
            .iter()
            .map(|s| (s.x, s.y, s.width, s.height))
            .collect();
        let bounds = DiagramBounds::from_elements(&element_bounds);
        
        Self {
            shapes,
            connectors: Vec::new(),
            bounds,
            grouped: true,
        }
    }
    
    /// Create from shapes and connectors
    pub fn from_shapes_and_connectors(shapes: Vec<Shape>, connectors: Vec<Connector>) -> Self {
        let element_bounds: Vec<(u32, u32, u32, u32)> = shapes
            .iter()
            .map(|s| (s.x, s.y, s.width, s.height))
            .collect();
        let bounds = DiagramBounds::from_elements(&element_bounds);
        
        Self {
            shapes,
            connectors,
            bounds,
            grouped: true,
        }
    }
}

/// Position for label relative to shape
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum LabelPosition {
    Above,
    Below,
    Right,
    Inside,
}

/// Helper to create a shape with a separate label
/// Returns (background_shape, label_shape)
pub fn create_labeled_shape(
    shape_type: ShapeType,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    fill_color: Option<&str>,
    line_color: Option<&str>,
    text: &str,
    label_pos: LabelPosition,
) -> Vec<Shape> {
    let mut shapes = Vec::new();
    
    // Background shape (no text)
    let mut bg = Shape::new(shape_type, x, y, width, height);
    if let Some(color) = fill_color {
        bg = bg.with_fill(ShapeFill::new(color));
    }
    if let Some(color) = line_color {
        bg = bg.with_line(ShapeLine::new(color, 12700));
    }
    shapes.push(bg);
    
    // Label shape
    let label_height = 200_000u32;
    let label_width = width.max(800_000);
    
    let (lx, ly) = match label_pos {
        LabelPosition::Above => (
            x + width / 2 - label_width / 2,
            y.saturating_sub(label_height + 50_000)
        ),
        LabelPosition::Below => (
            x + width / 2 - label_width / 2,
            y + height + 50_000
        ),
        LabelPosition::Right => (
            x + width + 50_000,
            y + height / 2 - label_height / 2
        ),
        LabelPosition::Inside => (
            x + 50_000,
            y + 50_000
        ),
    };
    
    let lw = if matches!(label_pos, LabelPosition::Inside) { width - 100_000 } else { label_width };
    let lh = if matches!(label_pos, LabelPosition::Inside) { height - 100_000 } else { label_height };
    
    let label = Shape::new(ShapeType::Rectangle, lx, ly, lw, lh)
        .with_text(text);
    shapes.push(label);
    
    shapes
}

/// Helper to create a circle/dot with label
pub fn create_labeled_dot(
    x: u32,
    y: u32,
    size: u32,
    fill_color: &str,
    line_color: Option<&str>,
    text: &str,
    label_pos: LabelPosition,
) -> Vec<Shape> {
    let mut shapes = Vec::new();
    
    // Dot shape
    let mut dot = Shape::new(ShapeType::Circle, x, y, size, size)
        .with_fill(ShapeFill::new(fill_color));
    if let Some(color) = line_color {
        dot = dot.with_line(ShapeLine::new(color, 25400));
    }
    shapes.push(dot);
    
    // Label
    let label_width = 800_000u32;
    let label_height = 200_000u32;
    
    let (lx, ly) = match label_pos {
        LabelPosition::Above => (
            x + size / 2 - label_width / 2,
            y.saturating_sub(label_height + 50_000)
        ),
        LabelPosition::Below => (
            x + size / 2 - label_width / 2,
            y + size + 50_000
        ),
        LabelPosition::Right => (
            x + size + 50_000,
            y + size / 2 - label_height / 2
        ),
        LabelPosition::Inside => (
            x, y
        ),
    };
    
    if !matches!(label_pos, LabelPosition::Inside) {
        let label = Shape::new(ShapeType::Rectangle, lx, ly, label_width, label_height)
            .with_text(text);
        shapes.push(label);
    }
    
    shapes
}

/// Helper to create a shape with gradient fill and separate label
#[allow(dead_code)]
pub fn create_gradient_shape(
    shape_type: ShapeType,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    start_color: &str,
    end_color: &str,
    direction: GradientDirection,
    line_color: Option<&str>,
    text: &str,
    label_pos: LabelPosition,
) -> Vec<Shape> {
    let mut shapes = Vec::new();
    
    // Background shape with gradient (no text)
    let gradient = GradientFill::linear(start_color, end_color, direction);
    let mut bg = Shape::new(shape_type, x, y, width, height)
        .with_gradient(gradient);
    if let Some(color) = line_color {
        bg = bg.with_line(ShapeLine::new(color, 12700));
    }
    shapes.push(bg);
    
    // Label shape
    let label_height = 200_000u32;
    let label_width = width.max(800_000);
    
    let (lx, ly) = match label_pos {
        LabelPosition::Above => (
            x + width / 2 - label_width / 2,
            y.saturating_sub(label_height + 50_000)
        ),
        LabelPosition::Below => (
            x + width / 2 - label_width / 2,
            y + height + 50_000
        ),
        LabelPosition::Right => (
            x + width + 50_000,
            y + height / 2 - label_height / 2
        ),
        LabelPosition::Inside => (
            x + 50_000,
            y + 50_000
        ),
    };
    
    let lw = if matches!(label_pos, LabelPosition::Inside) { width - 100_000 } else { label_width };
    let lh = if matches!(label_pos, LabelPosition::Inside) { height - 100_000 } else { label_height };
    
    let label = Shape::new(ShapeType::Rectangle, lx, ly, lw, lh)
        .with_text(text);
    shapes.push(label);
    
    shapes
}

/// Helper to create a shape with transparency
#[allow(dead_code)]
pub fn create_transparent_shape(
    shape_type: ShapeType,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    fill_color: &str,
    transparency_percent: u32,
    line_color: Option<&str>,
    text: Option<&str>,
) -> Shape {
    let fill = ShapeFill::new(fill_color).with_transparency(transparency_percent);
    let mut shape = Shape::new(shape_type, x, y, width, height)
        .with_fill(fill);
    
    if let Some(color) = line_color {
        shape = shape.with_line(ShapeLine::new(color, 12700));
    }
    if let Some(t) = text {
        shape = shape.with_text(t);
    }
    
    shape
}
