//! Shared types for Mermaid diagram parsing

use crate::generator::Shape;
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

/// Result containing shapes and connectors
pub struct DiagramElements {
    pub shapes: Vec<Shape>,
    pub connectors: Vec<Connector>,
}
