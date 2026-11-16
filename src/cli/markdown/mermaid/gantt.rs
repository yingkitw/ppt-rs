//! Gantt chart diagram parsing and rendering

use crate::generator::{Shape, ShapeFill, ShapeType};

/// Generate shapes for a Gantt chart
pub fn generate_shapes(code: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();
    let mut title = String::new();
    let mut sections: Vec<(String, Vec<(String, u32)>)> = Vec::new();
    let mut current_section = String::new();
    let mut current_tasks: Vec<(String, u32)> = Vec::new();

    for line in code.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }

        if line.starts_with("title") {
            title = line.strip_prefix("title").unwrap_or("").trim().to_string();
        } else if line.starts_with("section") {
            if !current_section.is_empty() {
                sections.push((current_section.clone(), current_tasks.clone()));
                current_tasks.clear();
            }
            current_section = line
                .strip_prefix("section")
                .unwrap_or("")
                .trim()
                .to_string();
        } else if line.contains(':')
            && !line.starts_with("dateFormat")
            && !line.starts_with("axisFormat")
        {
            if let Some((task_name, _rest)) = line.split_once(':') {
                let duration = 3u32;
                current_tasks.push((task_name.trim().to_string(), duration));
            }
        }
    }

    if !current_section.is_empty() {
        sections.push((current_section, current_tasks));
    }

    // Layout parameters
    let start_x = 500_000u32;
    let start_y = 1_600_000u32;
    let section_height = 300_000u32;
    let task_height = 250_000u32;
    let task_spacing = 280_000u32;
    let bar_width_per_unit = 600_000u32;
    let label_width = 2_000_000u32;

    if !title.is_empty() {
        let title_shape = Shape::new(ShapeType::Rectangle, start_x, start_y, 7_000_000, 400_000)
            .with_text(&title);
        shapes.push(title_shape);
    }

    let mut y = start_y + 500_000;
    let colors = ["4472C4", "ED7D31", "70AD47", "FFC000", "5B9BD5"];

    for (section_idx, (section_name, tasks)) in sections.iter().enumerate() {
        let section_shape = Shape::new(ShapeType::Rectangle, start_x, y, 7_000_000, section_height)
            .with_fill(ShapeFill::new("E0E0E0"))
            .with_text(section_name);
        shapes.push(section_shape);
        y += section_height + 50_000;

        for (task_idx, (task_name, duration)) in tasks.iter().enumerate() {
            let label_shape =
                Shape::new(ShapeType::Rectangle, start_x, y, label_width, task_height)
                    .with_text(task_name);
            shapes.push(label_shape);

            let bar_x = start_x + label_width + 100_000 + (task_idx as u32) * 200_000;
            let bar_width = duration * bar_width_per_unit;
            let color = colors[(section_idx + task_idx) % colors.len()];

            let bar_shape = Shape::new(
                ShapeType::RoundedRectangle,
                bar_x,
                y,
                bar_width,
                task_height,
            )
            .with_fill(ShapeFill::new(color));
            shapes.push(bar_shape);

            y += task_spacing;
        }
    }

    shapes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_gantt_shapes() {
        let code =
            "gantt\n    title Project\n    section Phase 1\n    Task A : a1, 2024-01-01, 30d";
        let shapes = generate_shapes(code);
        assert!(!shapes.is_empty());
    }
}
