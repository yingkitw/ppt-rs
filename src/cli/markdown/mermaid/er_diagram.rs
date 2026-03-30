//! ER diagram parsing and rendering

use super::types::DiagramElements;
use crate::generator::connectors::{
    ArrowType, ConnectionSite, Connector, ConnectorLine, ConnectorType,
};
use crate::generator::{Shape, ShapeFill, ShapeLine, ShapeType};
use std::collections::HashMap;

/// Generate shapes and connectors for an ER diagram
pub fn generate_elements(code: &str) -> DiagramElements {
    let mut shapes = Vec::new();
    let mut connectors = Vec::new();

    let mut entities: HashMap<String, Vec<String>> = HashMap::new();
    let mut relationships: Vec<(String, String, String)> = Vec::new();
    let mut current_entity = String::new();

    for line in code.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }

        if line.contains("||") || line.contains("}|") || line.contains("|{") || line.contains("o{")
        {
            // Extract relationship label after colon
            let (rel_part, label) = if let Some(colon_pos) = line.find(':') {
                let label_part = line[colon_pos + 1..].trim();
                // Remove quotes if present
                let label = label_part.trim_matches('"').trim_matches('\'').to_string();
                (&line[..colon_pos], label)
            } else {
                (line, String::new())
            };

            let parts: Vec<&str> = rel_part
                .split(|c: char| c == '|' || c == '{' || c == '}' || c == 'o' || c == '-')
                .collect();
            let parts: Vec<&str> = parts
                .into_iter()
                .filter(|s| !s.is_empty() && s.chars().any(|c| c.is_alphabetic()))
                .collect();
            if parts.len() >= 2 {
                let e1 = parts[0].trim().to_string();
                let e2 = parts[parts.len() - 1].trim().to_string();
                if !entities.contains_key(&e1) {
                    entities.insert(e1.clone(), Vec::new());
                }
                if !entities.contains_key(&e2) {
                    entities.insert(e2.clone(), Vec::new());
                }
                relationships.push((e1, e2, label));
            }
        } else if line.contains('{') {
            current_entity = line.split('{').next().unwrap_or("").trim().to_string();
            if !entities.contains_key(&current_entity) {
                entities.insert(current_entity.clone(), Vec::new());
            }
        } else if line == "}" {
            current_entity.clear();
        } else if !current_entity.is_empty() && !line.is_empty() {
            if let Some(attrs) = entities.get_mut(&current_entity) {
                attrs.push(line.to_string());
            }
        }
    }

    // Layout parameters
    let start_x = 500_000u32;
    let start_y = 1_600_000u32;
    let entity_width = 2_200_000u32;
    let header_height = 400_000u32;
    let attr_height = 280_000u32;
    let h_spacing = 2_800_000u32;
    let v_spacing = 2_500_000u32;

    let mut entity_positions: HashMap<String, (u32, u32)> = HashMap::new();
    let mut entity_shape_ids: HashMap<String, u32> = HashMap::new();
    let mut shape_id = 10u32;

    for (i, (entity_name, attrs)) in entities.iter().enumerate() {
        let x = start_x + (i as u32 % 3) * h_spacing;
        let y = start_y + (i as u32 / 3) * v_spacing;
        entity_positions.insert(entity_name.clone(), (x, y));
        entity_shape_ids.insert(entity_name.clone(), shape_id);

        let header = Shape::new(ShapeType::Rectangle, x, y, entity_width, header_height)
            .with_id(shape_id)
            .with_fill(ShapeFill::new("C2185B"))
            .with_line(ShapeLine::new("880E4F", 2))
            .with_text(entity_name);
        shapes.push(header);
        shape_id += 1;

        let attrs_text = attrs.join("\n");
        let attrs_box_height = (attrs.len().max(1) as u32) * attr_height;
        let attrs_shape = Shape::new(
            ShapeType::Rectangle,
            x,
            y + header_height,
            entity_width,
            attrs_box_height,
        )
        .with_id(shape_id)
        .with_fill(ShapeFill::new("FCE4EC"))
        .with_line(ShapeLine::new("880E4F", 1))
        .with_text(&attrs_text);
        shapes.push(attrs_shape);
        shape_id += 1;
    }

    for (e1, e2, label) in &relationships {
        if let (Some(&(x1, y1)), Some(&(x2, y2))) =
            (entity_positions.get(e1), entity_positions.get(e2))
        {
            let e1_shape_id = entity_shape_ids.get(e1).copied();
            let e2_shape_id = entity_shape_ids.get(e2).copied();

            // Smart connection site selection
            let (start_site, end_site) = if x1 < x2 {
                (ConnectionSite::Right, ConnectionSite::Left)
            } else if x1 > x2 {
                (ConnectionSite::Left, ConnectionSite::Right)
            } else if y1 < y2 {
                (ConnectionSite::Bottom, ConnectionSite::Top)
            } else {
                (ConnectionSite::Top, ConnectionSite::Bottom)
            };

            let mut connector = Connector::new(
                ConnectorType::Elbow,
                x1 + entity_width,
                y1 + header_height / 2,
                x2,
                y2 + header_height / 2,
            )
            .with_line(ConnectorLine::new("880E4F", 19050))
            .with_end_arrow(ArrowType::Diamond);

            // Anchor to shapes
            if let Some(id) = e1_shape_id {
                connector = connector.connect_start(id, start_site);
            }
            if let Some(id) = e2_shape_id {
                connector = connector.connect_end(id, end_site);
            }

            // Create label shape if label exists
            if !label.is_empty() {
                let label_width = 1_000_000u32;
                let label_height = 250_000u32;
                let mid_x = (x1 + entity_width + x2) / 2;
                let mid_y = (y1 + y2 + header_height) / 2;

                let label_shape = Shape::new(
                    ShapeType::RoundedRectangle,
                    mid_x.saturating_sub(label_width / 2),
                    mid_y.saturating_sub(label_height / 2),
                    label_width,
                    label_height,
                )
                .with_id(shape_id)
                .with_fill(ShapeFill::new("FFFFFF"))
                .with_line(ShapeLine::new("880E4F", 1))
                .with_text(label);

                shapes.push(label_shape);
                shape_id += 1;
            }

            connectors.push(connector);
        }
    }

    DiagramElements::from_shapes_and_connectors(shapes, connectors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_er_diagram() {
        // Test with entity definitions
        let code = "erDiagram\n    CUSTOMER {\n        string name\n    }\n    ORDER {\n        int id\n    }\n    CUSTOMER ||--o{ ORDER : places";
        let elements = generate_elements(code);
        assert!(!elements.shapes.is_empty());
    }

    #[test]
    fn test_er_diagram_empty() {
        // Empty ER diagram should not panic
        let code = "erDiagram";
        let elements = generate_elements(code);
        assert!(elements.shapes.is_empty());
    }
}
