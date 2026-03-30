//! Flowchart diagram parsing and rendering

use super::types::{DiagramBounds, *};
use crate::generator::connectors::{
    ArrowType, ConnectionSite, Connector, ConnectorLine, ConnectorType, LineDash,
};
use crate::generator::{Shape, ShapeFill, ShapeLine, ShapeType};
use std::collections::HashMap;

/// Parse flowchart direction from first line
fn parse_direction(first_line: &str) -> FlowDirection {
    let line = first_line.to_uppercase();
    if line.contains("LR") {
        FlowDirection::LeftToRight
    } else if line.contains("RL") {
        FlowDirection::RightToLeft
    } else if line.contains("BT") {
        FlowDirection::BottomToTop
    } else {
        FlowDirection::TopToBottom
    }
}

/// Parse a flowchart from Mermaid code
pub fn parse(code: &str) -> Flowchart {
    let mut lines = code.lines();
    let first_line = lines.next().unwrap_or("");
    let direction = parse_direction(first_line);

    let mut nodes: HashMap<String, FlowNode> = HashMap::new();
    let mut connections: Vec<FlowConnection> = Vec::new();
    let mut subgraphs: Vec<Subgraph> = Vec::new();
    let mut current_subgraph: Option<Subgraph> = None;

    for line in lines {
        let line = line.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }

        // Handle subgraph start
        if line.starts_with("subgraph") {
            let name = line
                .strip_prefix("subgraph")
                .unwrap_or("")
                .trim()
                .to_string();
            current_subgraph = Some(Subgraph {
                name,
                nodes: Vec::new(),
            });
            continue;
        }

        // Handle subgraph end
        if line == "end" {
            if let Some(sg) = current_subgraph.take() {
                subgraphs.push(sg);
            }
            continue;
        }

        // Parse connections: A --> B, A --> B[Label], A[Text] --> B[Text]
        if let Some((from_part, rest)) = split_connection(line) {
            let (arrow_type, to_part) = parse_arrow_and_rest(&rest);

            // Parse from node
            let (from_id, from_node) = parse_node_def(&from_part);
            if let Some(node) = from_node {
                nodes.entry(from_id.clone()).or_insert(node);
                if let Some(ref mut sg) = current_subgraph {
                    if !sg.nodes.contains(&from_id) {
                        sg.nodes.push(from_id.clone());
                    }
                }
            }

            // Parse to node (may have label on arrow)
            let (to_part_clean, arrow_label) = extract_arrow_label(&to_part);
            let (to_id, to_node) = parse_node_def(&to_part_clean);
            if let Some(node) = to_node {
                nodes.entry(to_id.clone()).or_insert(node);
                if let Some(ref mut sg) = current_subgraph {
                    if !sg.nodes.contains(&to_id) {
                        sg.nodes.push(to_id.clone());
                    }
                }
            }

            connections.push(FlowConnection {
                from: from_id,
                to: to_id,
                label: arrow_label,
                arrow_type,
            });
        } else {
            // Standalone node definition
            let (id, node) = parse_node_def(line);
            if let Some(n) = node {
                nodes.entry(id.clone()).or_insert(n);
                if let Some(ref mut sg) = current_subgraph {
                    if !sg.nodes.contains(&id) {
                        sg.nodes.push(id);
                    }
                }
            }
        }
    }

    Flowchart {
        direction,
        nodes: nodes.into_values().collect(),
        connections,
        subgraphs,
    }
}

/// Split line at connection arrow
fn split_connection(line: &str) -> Option<(String, String)> {
    for arrow in ["==>", "-.->", "-->", "---", "->"] {
        if let Some(pos) = line.find(arrow) {
            let from = line[..pos].trim().to_string();
            let rest = line[pos..].to_string();
            return Some((from, rest));
        }
    }
    None
}

/// Parse arrow type and get the rest of the string
fn parse_arrow_and_rest(s: &str) -> (ArrowStyle, String) {
    if s.starts_with("==>") {
        (ArrowStyle::Thick, s[3..].trim().to_string())
    } else if s.starts_with("-.->") {
        (ArrowStyle::Dotted, s[4..].trim().to_string())
    } else if s.starts_with("-->") {
        (ArrowStyle::Arrow, s[3..].trim().to_string())
    } else if s.starts_with("---") {
        (ArrowStyle::Open, s[3..].trim().to_string())
    } else if s.starts_with("->") {
        (ArrowStyle::Arrow, s[2..].trim().to_string())
    } else {
        (ArrowStyle::Arrow, s.to_string())
    }
}

/// Extract arrow label like |text|
fn extract_arrow_label(s: &str) -> (String, Option<String>) {
    if let Some(start) = s.find('|') {
        if let Some(end) = s[start + 1..].find('|') {
            let label = s[start + 1..start + 1 + end].to_string();
            let rest = s[start + 2 + end..].trim().to_string();
            return (rest, Some(label));
        }
    }
    (s.to_string(), None)
}

/// Parse a node definition like A[Text] or B(Text) or C{Text}
fn parse_node_def(s: &str) -> (String, Option<FlowNode>) {
    let s = s.trim();

    // Try different bracket types
    for (open, close, shape) in [
        ("((", "))", NodeShape::Circle),
        ("([", "])", NodeShape::Stadium),
        ("{{", "}}", NodeShape::Hexagon),
        ("[", "]", NodeShape::Rectangle),
        ("(", ")", NodeShape::RoundedRect),
        ("{", "}", NodeShape::Diamond),
    ] {
        if let Some(start) = s.find(open) {
            let id = s[..start].trim().to_string();
            if let Some(end) = s[start + open.len()..].find(close) {
                let label = s[start + open.len()..start + open.len() + end].to_string();
                return (id.clone(), Some(FlowNode { id, label, shape }));
            }
        }
    }

    // Plain node ID without brackets
    let id = s.to_string();
    if !id.is_empty() && id.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return (
            id.clone(),
            Some(FlowNode {
                id: id.clone(),
                label: id,
                shape: NodeShape::Rectangle,
            }),
        );
    }

    (s.to_string(), None)
}

/// Generate shapes and connectors for a flowchart
pub fn generate_elements(flowchart: &Flowchart) -> DiagramElements {
    let mut shapes = Vec::new();
    let mut connectors = Vec::new();
    let node_count = flowchart.nodes.len();

    if node_count == 0 {
        return DiagramElements {
            shapes,
            connectors,
            bounds: None,
            grouped: false,
        };
    }

    // Track element positions for bounding box calculation
    let mut element_bounds: Vec<(u32, u32, u32, u32)> = Vec::new();

    // Layout parameters (in EMUs)
    let node_width = 1_400_000u32;
    let node_height = 500_000u32;
    let h_spacing = 1_800_000u32;
    let v_spacing = 900_000u32;

    let is_horizontal = matches!(
        flowchart.direction,
        FlowDirection::LeftToRight | FlowDirection::RightToLeft
    );

    // Track node positions and their shape IDs for connector anchoring
    let mut node_positions: HashMap<String, (u32, u32)> = HashMap::new();
    let mut node_shape_ids: HashMap<String, u32> = HashMap::new();
    let mut shape_id = 10u32; // Starting shape ID

    // If we have subgraphs, layout by subgraph
    if !flowchart.subgraphs.is_empty() {
        let mut subgraph_x = 500_000u32;
        let subgraph_start_y = 1_600_000u32;

        for (sg_idx, subgraph) in flowchart.subgraphs.iter().enumerate() {
            let sg_width = node_width + 400_000;
            let title_height = 250_000u32;
            let sg_height = (subgraph.nodes.len() as u32) * v_spacing + 400_000 + title_height;
            let sg_x = subgraph_x;
            let sg_y = subgraph_start_y;

            // Subgraph background shape (no text - just background)
            let sg_shape = Shape::new(ShapeType::RoundedRectangle, sg_x, sg_y, sg_width, sg_height)
                .with_fill(ShapeFill::new(get_subgraph_color(sg_idx)))
                .with_line(ShapeLine::new("757575", 1));
            shapes.push(sg_shape);
            element_bounds.push((sg_x, sg_y, sg_width, sg_height));

            // Subgraph title at top
            let title_shape = Shape::new(
                ShapeType::Rectangle,
                sg_x + 50_000,
                sg_y + 50_000,
                sg_width - 100_000,
                title_height,
            )
            .with_id(shape_id)
            .with_text(&subgraph.name);
            shapes.push(title_shape);
            shape_id += 1;

            // Layout nodes within subgraph (offset by title height)
            for (node_idx, node_id) in subgraph.nodes.iter().enumerate() {
                if let Some(node) = flowchart.nodes.iter().find(|n| &n.id == node_id) {
                    let x = sg_x + 200_000;
                    let y = sg_y + title_height + 200_000 + (node_idx as u32) * v_spacing;

                    node_positions.insert(node.id.clone(), (x, y));
                    node_shape_ids.insert(node.id.clone(), shape_id);

                    let shape = create_node_shape(node, x, y, node_width, node_height, shape_id);
                    shapes.push(shape);
                    element_bounds.push((x, y, node_width, node_height));
                    shape_id += 1;
                }
            }

            subgraph_x += sg_width + 600_000;
        }

        // Layout any nodes not in subgraphs
        let mut orphan_y = subgraph_start_y;
        for node in &flowchart.nodes {
            if !node_positions.contains_key(&node.id) {
                let x = subgraph_x;
                let y = orphan_y;

                node_positions.insert(node.id.clone(), (x, y));
                node_shape_ids.insert(node.id.clone(), shape_id);

                let shape = create_node_shape(node, x, y, node_width, node_height, shape_id);
                shapes.push(shape);
                element_bounds.push((x, y, node_width, node_height));
                shape_id += 1;

                orphan_y += v_spacing;
            }
        }
    } else {
        // Simple grid layout without subgraphs
        let start_x = 1_000_000u32;
        let start_y = 1_800_000u32;
        let cols = if is_horizontal { node_count.min(5) } else { 1 };

        for (i, node) in flowchart.nodes.iter().enumerate() {
            let col = i % cols;
            let row = i / cols;

            let (x, y) = if is_horizontal {
                (
                    start_x + (col as u32) * h_spacing,
                    start_y + (row as u32) * v_spacing,
                )
            } else {
                (
                    start_x + (col as u32) * h_spacing,
                    start_y + (i as u32) * v_spacing,
                )
            };

            node_positions.insert(node.id.clone(), (x, y));
            node_shape_ids.insert(node.id.clone(), shape_id);

            let shape = create_node_shape(node, x, y, node_width, node_height, shape_id);
            shapes.push(shape);
            element_bounds.push((x, y, node_width, node_height));
            shape_id += 1;
        }
    }

    // Create connectors for connections with shape anchoring
    for conn in &flowchart.connections {
        if let (Some(&(from_x, from_y)), Some(&(to_x, to_y))) =
            (node_positions.get(&conn.from), node_positions.get(&conn.to))
        {
            // Get shape IDs for anchoring
            let from_shape_id = node_shape_ids.get(&conn.from).copied();
            let to_shape_id = node_shape_ids.get(&conn.to).copied();

            // Determine connection sites and positions based on relative positions
            let (start_site, end_site, start_x, start_y, end_x, end_y) = if is_horizontal {
                // Horizontal flow: connect Right -> Left
                (
                    ConnectionSite::Right,
                    ConnectionSite::Left,
                    from_x + node_width,
                    from_y + node_height / 2,
                    to_x,
                    to_y + node_height / 2,
                )
            } else {
                // Vertical flow: connect Bottom -> Top
                (
                    ConnectionSite::Bottom,
                    ConnectionSite::Top,
                    from_x + node_width / 2,
                    from_y + node_height,
                    to_x + node_width / 2,
                    to_y,
                )
            };

            // Use elbow connector for better auto-routing when shapes are not aligned
            let connector_type = if (start_x as i32 - end_x as i32).abs() < 100_000
                || (start_y as i32 - end_y as i32).abs() < 100_000
            {
                ConnectorType::Straight
            } else {
                ConnectorType::Elbow
            };

            let (line_color, line_dash) = match conn.arrow_type {
                ArrowStyle::Thick => ("E65100", LineDash::Solid),
                ArrowStyle::Dotted => ("757575", LineDash::Dash),
                ArrowStyle::Open => ("1565C0", LineDash::Solid),
                ArrowStyle::Arrow => ("1565C0", LineDash::Solid),
            };

            let mut connector = Connector::new(connector_type, start_x, start_y, end_x, end_y)
                .with_line(ConnectorLine::new(line_color, 19050).with_dash(line_dash))
                .with_end_arrow(ArrowType::Triangle);

            // Anchor connector to shapes for auto-routing
            if let Some(from_id) = from_shape_id {
                connector = connector.connect_start(from_id, start_site);
            }
            if let Some(to_id) = to_shape_id {
                connector = connector.connect_end(to_id, end_site);
            }

            // Create separate label shape for better font control
            if let Some(label) = &conn.label {
                let label_width = 600_000u32;
                let label_height = 250_000u32;
                let mid_x = (start_x + end_x) / 2;
                let mid_y = (start_y + end_y) / 2;

                let label_shape = Shape::new(
                    ShapeType::Rectangle,
                    mid_x.saturating_sub(label_width / 2),
                    mid_y.saturating_sub(label_height / 2),
                    label_width,
                    label_height,
                )
                .with_id(shape_id)
                .with_fill(ShapeFill::new("FFFFFF"))
                .with_line(ShapeLine::new("757575", 1))
                .with_text(label);

                shapes.push(label_shape);
                element_bounds.push((
                    mid_x - label_width / 2,
                    mid_y - label_height / 2,
                    label_width,
                    label_height,
                ));
                shape_id += 1;
            }

            connectors.push(connector);
        }
    }

    // Calculate bounding box for the entire diagram
    let bounds = DiagramBounds::from_elements(&element_bounds);

    DiagramElements {
        shapes,
        connectors,
        bounds,
        grouped: true, // Flowcharts should be grouped
    }
}

fn get_subgraph_color(index: usize) -> &'static str {
    const COLORS: [&str; 6] = ["E3F2FD", "F3E5F5", "E8F5E9", "FFF3E0", "E0F7FA", "FCE4EC"];
    COLORS[index % COLORS.len()]
}

fn create_node_shape(
    node: &FlowNode,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    shape_id: u32,
) -> Shape {
    let shape_type = match node.shape {
        NodeShape::Rectangle => ShapeType::Rectangle,
        NodeShape::RoundedRect => ShapeType::RoundedRectangle,
        NodeShape::Stadium => ShapeType::RoundedRectangle,
        NodeShape::Diamond => ShapeType::Diamond,
        NodeShape::Circle => ShapeType::Ellipse,
        NodeShape::Hexagon => ShapeType::Hexagon,
    };

    let fill_color = match node.shape {
        NodeShape::Diamond => "FFF3E0",
        NodeShape::Circle => "E3F2FD",
        _ => "FFFFFF",
    };

    Shape::new(shape_type, x, y, width, height)
        .with_id(shape_id)
        .with_fill(ShapeFill::new(fill_color))
        .with_line(ShapeLine::new("1565C0", 2))
        .with_text(&node.label)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_flowchart_nodes() {
        let code = "flowchart LR\n    A[Start] --> B[Process] --> C[End]";
        let flowchart = parse(code);
        assert_eq!(flowchart.direction, FlowDirection::LeftToRight);
        assert!(!flowchart.nodes.is_empty());
        assert!(!flowchart.connections.is_empty());
    }

    #[test]
    fn test_parse_node_shapes() {
        let (id, node) = parse_node_def("A[Rectangle]");
        assert_eq!(id, "A");
        assert!(node.is_some());
        assert_eq!(node.unwrap().shape, NodeShape::Rectangle);

        let (id, node) = parse_node_def("B(Rounded)");
        assert_eq!(id, "B");
        assert_eq!(node.unwrap().shape, NodeShape::RoundedRect);

        let (id, node) = parse_node_def("C{Diamond}");
        assert_eq!(id, "C");
        assert_eq!(node.unwrap().shape, NodeShape::Diamond);
    }

    #[test]
    fn test_generate_flowchart_shapes() {
        let code = "flowchart LR\n    A[Start] --> B[End]";
        let flowchart = parse(code);
        let elements = generate_elements(&flowchart);
        assert!(!elements.shapes.is_empty());
    }
}
