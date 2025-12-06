//! Mermaid diagram parsing and rendering
//!
//! Parses Mermaid diagram code and generates actual PPTX shapes and connectors.
//! 
//! Supported diagram types:
//! - Flowchart (graph/flowchart)
//! - Sequence diagrams
//! - Pie charts
//! - Gantt charts
//! - Class diagrams
//! - State diagrams
//! - ER diagrams
//! - Mindmaps
//! - Timelines
//! - Journey (user journey)
//! - Quadrant charts
//! - Git graphs

mod types;
mod flowchart;
mod sequence;
mod pie;
mod gantt;
mod class_diagram;
mod state_diagram;
mod er_diagram;
mod mindmap;
mod timeline;
mod journey;
mod quadrant;
mod gitgraph;

pub use types::*;

use crate::generator::{Shape, ShapeType, ShapeFill, ShapeLine};

/// Detect the type of Mermaid diagram from code
pub fn detect_type(code: &str) -> MermaidType {
    let first_line = code.lines().next().unwrap_or("").trim().to_lowercase();
    
    if first_line.starts_with("graph") || first_line.starts_with("flowchart") {
        MermaidType::Flowchart
    } else if first_line.starts_with("sequencediagram") || first_line.starts_with("sequence") {
        MermaidType::Sequence
    } else if first_line.starts_with("pie") {
        MermaidType::Pie
    } else if first_line.starts_with("gantt") {
        MermaidType::Gantt
    } else if first_line.starts_with("classdiagram") || first_line.starts_with("class") {
        MermaidType::ClassDiagram
    } else if first_line.starts_with("statediagram") || first_line.starts_with("state") {
        MermaidType::StateDiagram
    } else if first_line.starts_with("erdiagram") || first_line.starts_with("er") {
        MermaidType::ErDiagram
    } else if first_line.starts_with("mindmap") {
        MermaidType::Mindmap
    } else if first_line.starts_with("timeline") {
        MermaidType::Timeline
    } else if first_line.starts_with("journey") {
        MermaidType::Journey
    } else if first_line.starts_with("quadrantchart") || first_line.starts_with("quadrant") {
        MermaidType::Quadrant
    } else if first_line.starts_with("gitgraph") || first_line.starts_with("git") {
        MermaidType::GitGraph
    } else {
        MermaidType::Unknown
    }
}

/// Create shapes and connectors for a Mermaid diagram (main entry point)
pub fn create_diagram_elements(code: &str) -> DiagramElements {
    let diagram_type = detect_type(code);
    
    match diagram_type {
        MermaidType::Flowchart => {
            let fc = flowchart::parse(code);
            flowchart::generate_elements(&fc)
        }
        MermaidType::Pie => {
            let slices = pie::parse(code);
            DiagramElements {
                shapes: pie::generate_shapes(&slices),
                connectors: Vec::new(),
            }
        }
        MermaidType::Sequence => {
            DiagramElements {
                shapes: sequence::generate_shapes(code),
                connectors: Vec::new(),
            }
        }
        MermaidType::Gantt => {
            DiagramElements {
                shapes: gantt::generate_shapes(code),
                connectors: Vec::new(),
            }
        }
        MermaidType::ClassDiagram => {
            class_diagram::generate_elements(code)
        }
        MermaidType::StateDiagram => {
            state_diagram::generate_elements(code)
        }
        MermaidType::ErDiagram => {
            er_diagram::generate_elements(code)
        }
        MermaidType::Mindmap => {
            DiagramElements {
                shapes: mindmap::generate_shapes(code),
                connectors: Vec::new(),
            }
        }
        MermaidType::Timeline => {
            DiagramElements {
                shapes: timeline::generate_shapes(code),
                connectors: Vec::new(),
            }
        }
        MermaidType::Journey => {
            DiagramElements {
                shapes: journey::generate_shapes(code),
                connectors: Vec::new(),
            }
        }
        MermaidType::Quadrant => {
            DiagramElements {
                shapes: quadrant::generate_shapes(code),
                connectors: Vec::new(),
            }
        }
        MermaidType::GitGraph => {
            DiagramElements {
                shapes: gitgraph::generate_shapes(code),
                connectors: Vec::new(),
            }
        }
        _ => {
            // Fallback: create a placeholder
            DiagramElements {
                shapes: vec![
                    Shape::new(ShapeType::Rectangle, 1_000_000, 2_000_000, 7_000_000, 3_000_000)
                        .with_fill(ShapeFill::new("F5F5F5"))
                        .with_line(ShapeLine::new("757575", 1))
                        .with_text(&format!("Diagram: {}", code.lines().next().unwrap_or("Unknown")))
                ],
                connectors: Vec::new(),
            }
        }
    }
}

/// Create shapes for a Mermaid diagram (backward compatibility)
pub fn create_diagram_shapes(code: &str) -> Vec<Shape> {
    create_diagram_elements(code).shapes
}

/// Get diagram style info (for backward compatibility)
pub fn get_diagram_style(diagram_type: MermaidType) -> (&'static str, &'static str, &'static str, &'static str) {
    match diagram_type {
        MermaidType::Flowchart => ("E3F2FD", "1565C0", "Flowchart", ""),
        MermaidType::Sequence => ("F3E5F5", "7B1FA2", "Sequence Diagram", ""),
        MermaidType::Pie => ("FFF8E1", "FF8F00", "Pie Chart", ""),
        MermaidType::Gantt => ("E8F5E9", "2E7D32", "Gantt Chart", ""),
        MermaidType::ClassDiagram => ("FFF3E0", "E65100", "Class Diagram", ""),
        MermaidType::StateDiagram => ("E0F7FA", "00838F", "State Diagram", ""),
        MermaidType::ErDiagram => ("FCE4EC", "C2185B", "ER Diagram", ""),
        MermaidType::Mindmap => ("E8EAF6", "3949AB", "Mind Map", ""),
        MermaidType::Timeline => ("EFEBE9", "5D4037", "Timeline", ""),
        MermaidType::Journey => ("F3E5F5", "7B1FA2", "User Journey", ""),
        MermaidType::Quadrant => ("E3F2FD", "1565C0", "Quadrant Chart", ""),
        MermaidType::GitGraph => ("ECEFF1", "607D8B", "Git Graph", ""),
        MermaidType::Unknown => ("F5F5F5", "757575", "Diagram", ""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_flowchart() {
        assert_eq!(detect_type("flowchart LR"), MermaidType::Flowchart);
        assert_eq!(detect_type("graph TD"), MermaidType::Flowchart);
    }

    #[test]
    fn test_detect_pie() {
        assert_eq!(detect_type("pie"), MermaidType::Pie);
    }

    #[test]
    fn test_detect_sequence() {
        assert_eq!(detect_type("sequenceDiagram"), MermaidType::Sequence);
    }

    #[test]
    fn test_detect_gantt() {
        assert_eq!(detect_type("gantt"), MermaidType::Gantt);
    }

    #[test]
    fn test_detect_class() {
        assert_eq!(detect_type("classDiagram"), MermaidType::ClassDiagram);
    }

    #[test]
    fn test_detect_state() {
        assert_eq!(detect_type("stateDiagram"), MermaidType::StateDiagram);
    }

    #[test]
    fn test_detect_er() {
        assert_eq!(detect_type("erDiagram"), MermaidType::ErDiagram);
    }

    #[test]
    fn test_detect_mindmap() {
        assert_eq!(detect_type("mindmap"), MermaidType::Mindmap);
    }

    #[test]
    fn test_detect_timeline() {
        assert_eq!(detect_type("timeline"), MermaidType::Timeline);
    }
}
