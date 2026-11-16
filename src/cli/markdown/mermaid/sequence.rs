//! Sequence diagram parsing and rendering

use crate::generator::{Shape, ShapeFill, ShapeLine, ShapeType};
use std::collections::HashMap;

/// Generate shapes for a sequence diagram
pub fn generate_shapes(code: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();
    let mut participant_ids: Vec<String> = Vec::new();
    let mut participant_names: HashMap<String, String> = HashMap::new();
    let mut messages: Vec<(String, String, String)> = Vec::new();

    for line in code.lines().skip(1) {
        let line = line.trim();

        // Parse participant
        if line.starts_with("participant") {
            let rest = line.strip_prefix("participant").unwrap_or("").trim();
            let (id, display_name) = if let Some((id, alias)) = rest.split_once(" as ") {
                (id.trim().to_string(), alias.trim().to_string())
            } else {
                let id = rest.split_whitespace().next().unwrap_or("").to_string();
                (id.clone(), id)
            };
            if !id.is_empty() && !participant_ids.contains(&id) {
                participant_ids.push(id.clone());
                participant_names.insert(id, display_name);
            }
        }
        // Parse message
        else if line.contains("->>") || line.contains("-->>") {
            let arrow = if line.contains("-->>") { "-->>" } else { "->>" };
            if let Some((from_part, rest)) = line.split_once(arrow) {
                if let Some((to_part, msg)) = rest.split_once(':') {
                    let from = from_part.trim().to_string();
                    let to = to_part.trim().to_string();
                    let text = msg.trim().to_string();

                    if !participant_ids.contains(&from) {
                        participant_ids.push(from.clone());
                        participant_names.insert(from.clone(), from.clone());
                    }
                    if !participant_ids.contains(&to) {
                        participant_ids.push(to.clone());
                        participant_names.insert(to.clone(), to.clone());
                    }

                    messages.push((from, to, text));
                }
            }
        }
    }

    // Layout parameters
    let start_x = 500_000u32;
    let start_y = 1_600_000u32;
    let participant_width = 1_400_000u32;
    let participant_height = 400_000u32;
    let h_spacing = 1_800_000u32;
    let lifeline_height = 3_000_000u32;
    let message_spacing = 450_000u32;

    let mut participant_x: HashMap<String, u32> = HashMap::new();

    for (i, id) in participant_ids.iter().enumerate() {
        let x = start_x + (i as u32) * h_spacing;
        participant_x.insert(id.clone(), x);

        let display_name = participant_names.get(id).unwrap_or(id);

        // Participant box at top
        let box_shape = Shape::new(
            ShapeType::Rectangle,
            x,
            start_y,
            participant_width,
            participant_height,
        )
        .with_fill(ShapeFill::new("E3F2FD"))
        .with_line(ShapeLine::new("1565C0", 2))
        .with_text(display_name);
        shapes.push(box_shape);

        // Lifeline
        let lifeline_x = x + participant_width / 2 - 10_000;
        let lifeline_y = start_y + participant_height;
        let lifeline = Shape::new(
            ShapeType::Rectangle,
            lifeline_x,
            lifeline_y,
            20_000,
            lifeline_height,
        )
        .with_fill(ShapeFill::new("757575"));
        shapes.push(lifeline);

        // Participant box at bottom
        let bottom_box = Shape::new(
            ShapeType::Rectangle,
            x,
            start_y + participant_height + lifeline_height,
            participant_width,
            participant_height,
        )
        .with_fill(ShapeFill::new("E3F2FD"))
        .with_line(ShapeLine::new("1565C0", 2))
        .with_text(display_name);
        shapes.push(bottom_box);
    }

    // Create message arrows
    let message_y_start = start_y + participant_height + 200_000;

    for (i, (from, to, text)) in messages.iter().enumerate() {
        if let (Some(&from_x), Some(&to_x)) = (participant_x.get(from), participant_x.get(to)) {
            let y = message_y_start + (i as u32) * message_spacing;
            let from_center = from_x + participant_width / 2;
            let to_center = to_x + participant_width / 2;

            let (arrow_x, arrow_width, is_left) = if from_center < to_center {
                (from_center, to_center - from_center, false)
            } else {
                (to_center, from_center - to_center, true)
            };

            let arrow_type = if is_left {
                ShapeType::LeftArrow
            } else {
                ShapeType::RightArrow
            };
            let arrow = Shape::new(arrow_type, arrow_x, y, arrow_width, 120_000)
                .with_fill(ShapeFill::new("1565C0"));
            shapes.push(arrow);

            let text_shape = Shape::new(
                ShapeType::Rectangle,
                arrow_x,
                y.saturating_sub(180_000),
                arrow_width,
                160_000,
            )
            .with_text(text);
            shapes.push(text_shape);
        }
    }

    shapes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_sequence_shapes() {
        let code = "sequenceDiagram\n    participant A as Alice\n    A->>B: Hello";
        let shapes = generate_shapes(code);
        assert!(!shapes.is_empty());
    }
}
