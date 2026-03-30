//! Class diagram parsing and rendering

use super::types::DiagramElements;
use crate::generator::connectors::{
    ArrowType, ConnectionSite, Connector, ConnectorLine, ConnectorType,
};
use crate::generator::{Shape, ShapeFill, ShapeLine, ShapeType};
use std::collections::HashMap;

/// Generate shapes and connectors for a class diagram
pub fn generate_elements(code: &str) -> DiagramElements {
    let mut shapes = Vec::new();
    let mut connectors = Vec::new();

    let mut classes: Vec<(String, Vec<String>, Vec<String>)> = Vec::new();
    let mut current_class = String::new();
    let mut current_attrs: Vec<String> = Vec::new();
    let mut current_methods: Vec<String> = Vec::new();
    let mut in_class = false;
    let mut relationships: Vec<(String, String, String)> = Vec::new();

    for line in code.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }

        if line.starts_with("class ") && line.contains('{') {
            current_class = line
                .strip_prefix("class ")
                .unwrap_or("")
                .split('{')
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            in_class = true;
            current_attrs.clear();
            current_methods.clear();
        } else if line == "}" && in_class {
            classes.push((
                current_class.clone(),
                current_attrs.clone(),
                current_methods.clone(),
            ));
            in_class = false;
        } else if in_class {
            if line.contains('(') {
                current_methods.push(line.to_string());
            } else if !line.is_empty() {
                current_attrs.push(line.to_string());
            }
        } else if line.contains("<|--") || line.contains("-->") || line.contains("--") {
            let rel_type = if line.contains("<|--") {
                "extends"
            } else if line.contains("-->") {
                "uses"
            } else {
                "associates"
            };

            let parts: Vec<&str> = line
                .split(|c| c == '<' || c == '|' || c == '-' || c == '>')
                .collect();
            let parts: Vec<&str> = parts.into_iter().filter(|s| !s.is_empty()).collect();
            if parts.len() >= 2 {
                relationships.push((
                    parts[0].trim().to_string(),
                    parts[parts.len() - 1].trim().to_string(),
                    rel_type.to_string(),
                ));
            }
        }
    }

    // Layout parameters
    let start_x = 500_000u32;
    let start_y = 1_600_000u32;
    let class_width = 2_000_000u32;
    let h_spacing = 2_500_000u32;
    let header_height = 350_000u32;
    let member_height = 250_000u32;

    let mut class_positions: HashMap<String, (u32, u32)> = HashMap::new();
    let mut class_shape_ids: HashMap<String, u32> = HashMap::new();
    let mut shape_id = 10u32;

    for (i, (class_name, attrs, methods)) in classes.iter().enumerate() {
        let x = start_x + (i as u32 % 3) * h_spacing;
        let y = start_y + (i as u32 / 3) * 2_000_000;
        class_positions.insert(class_name.clone(), (x, y));
        class_shape_ids.insert(class_name.clone(), shape_id);

        // Class header (this is the shape connectors attach to)
        let header = Shape::new(ShapeType::Rectangle, x, y, class_width, header_height)
            .with_id(shape_id)
            .with_fill(ShapeFill::new("4472C4"))
            .with_line(ShapeLine::new("2F5496", 2))
            .with_text(class_name);
        shapes.push(header);
        shape_id += 1;

        // Attributes section
        let attrs_text = if attrs.is_empty() {
            String::new()
        } else {
            attrs.join("\n")
        };
        let attrs_height = (attrs.len().max(1) as u32) * member_height;
        let attrs_shape = Shape::new(
            ShapeType::Rectangle,
            x,
            y + header_height,
            class_width,
            attrs_height,
        )
        .with_id(shape_id)
        .with_fill(ShapeFill::new("D6DCE5"))
        .with_line(ShapeLine::new("2F5496", 1))
        .with_text(&attrs_text);
        shapes.push(attrs_shape);
        shape_id += 1;

        // Methods section
        let methods_text = if methods.is_empty() {
            String::new()
        } else {
            methods.join("\n")
        };
        let methods_height = (methods.len().max(1) as u32) * member_height;
        let methods_shape = Shape::new(
            ShapeType::Rectangle,
            x,
            y + header_height + attrs_height,
            class_width,
            methods_height,
        )
        .with_id(shape_id)
        .with_fill(ShapeFill::new("FFFFFF"))
        .with_line(ShapeLine::new("2F5496", 1))
        .with_text(&methods_text);
        shapes.push(methods_shape);
        shape_id += 1;
    }

    // Create connectors with shape anchoring
    for (from, to, _rel_type) in &relationships {
        if let (Some(&(from_x, from_y)), Some(&(to_x, to_y))) =
            (class_positions.get(from), class_positions.get(to))
        {
            let from_shape_id = class_shape_ids.get(from).copied();
            let to_shape_id = class_shape_ids.get(to).copied();

            // Determine connection sites based on relative positions
            let (start_site, end_site) = if from_y < to_y {
                (ConnectionSite::Bottom, ConnectionSite::Top)
            } else if from_y > to_y {
                (ConnectionSite::Top, ConnectionSite::Bottom)
            } else if from_x < to_x {
                (ConnectionSite::Right, ConnectionSite::Left)
            } else {
                (ConnectionSite::Left, ConnectionSite::Right)
            };

            let mut connector = Connector::new(
                ConnectorType::Elbow,
                from_x + class_width / 2,
                from_y,
                to_x + class_width / 2,
                to_y + 500_000,
            )
            .with_line(ConnectorLine::new("2F5496", 19050))
            .with_end_arrow(ArrowType::Triangle);

            // Anchor to shapes for auto-routing
            if let Some(id) = from_shape_id {
                connector = connector.connect_start(id, start_site);
            }
            if let Some(id) = to_shape_id {
                connector = connector.connect_end(id, end_site);
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
    fn test_generate_class_diagram() {
        let code = "classDiagram\n    class Animal {\n        +name: String\n        +eat()\n    }";
        let elements = generate_elements(code);
        assert!(!elements.shapes.is_empty());
    }
}
