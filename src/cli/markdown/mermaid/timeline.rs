//! Timeline diagram parsing and rendering

use crate::generator::{Shape, ShapeFill, ShapeLine, ShapeType};

/// Generate shapes for a timeline
pub fn generate_shapes(code: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();

    let mut title = String::new();
    let mut events: Vec<(String, Vec<String>)> = Vec::new();
    let mut current_date = String::new();
    let mut current_items: Vec<String> = Vec::new();

    for line in code.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }

        if line.starts_with("title") {
            title = line.strip_prefix("title").unwrap_or("").trim().to_string();
        } else if line.contains(':') {
            if !current_date.is_empty() {
                events.push((current_date.clone(), current_items.clone()));
                current_items.clear();
            }
            let (date, item) = line.split_once(':').unwrap();
            current_date = date.trim().to_string();
            if !item.trim().is_empty() {
                current_items.push(item.trim().to_string());
            }
        } else if !current_date.is_empty() {
            current_items.push(line.to_string());
        }
    }

    if !current_date.is_empty() {
        events.push((current_date, current_items));
    }

    // Layout parameters
    let start_x = 500_000u32;
    let start_y = 1_600_000u32;
    let timeline_y = 2_500_000u32;
    let event_width = 1_400_000u32;
    let event_spacing = 1_600_000u32;
    let date_height = 300_000u32;
    let item_height = 250_000u32;

    if !title.is_empty() {
        let title_shape = Shape::new(ShapeType::Rectangle, start_x, start_y, 7_500_000, 400_000)
            .with_text(&title);
        shapes.push(title_shape);
    }

    // Timeline line
    let line_width = (events.len() as u32) * event_spacing + 500_000;
    let timeline_line = Shape::new(
        ShapeType::Rectangle,
        start_x,
        timeline_y,
        line_width,
        30_000,
    )
    .with_fill(ShapeFill::new("5D4037"));
    shapes.push(timeline_line);

    // Events
    let colors = ["EFEBE9", "D7CCC8", "BCAAA4", "A1887F"];

    for (i, (date, items)) in events.iter().enumerate() {
        let x = start_x + (i as u32) * event_spacing;
        let color = colors[i % colors.len()];

        // Date marker
        let marker = Shape::new(
            ShapeType::Ellipse,
            x + event_width / 2 - 75_000,
            timeline_y - 60_000,
            150_000,
            150_000,
        )
        .with_fill(ShapeFill::new("5D4037"));
        shapes.push(marker);

        // Date label
        let date_shape = Shape::new(
            ShapeType::Rectangle,
            x,
            timeline_y - date_height - 100_000,
            event_width,
            date_height,
        )
        .with_fill(ShapeFill::new("5D4037"))
        .with_text(date);
        shapes.push(date_shape);

        // Event items
        let items_text = items.join("\n");
        let items_height = (items.len().max(1) as u32) * item_height;
        let items_shape = Shape::new(
            ShapeType::RoundedRectangle,
            x,
            timeline_y + 150_000,
            event_width,
            items_height,
        )
        .with_fill(ShapeFill::new(color))
        .with_line(ShapeLine::new("5D4037", 1))
        .with_text(&items_text);
        shapes.push(items_shape);
    }

    shapes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_timeline_shapes() {
        let code = "timeline\n    title History\n    2020 : Event A\n    2021 : Event B";
        let shapes = generate_shapes(code);
        assert!(!shapes.is_empty());
    }
}
