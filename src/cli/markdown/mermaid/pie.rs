//! Pie chart diagram parsing and rendering

use crate::generator::{Shape, ShapeFill, ShapeLine, ShapeType};

/// Parse pie chart data
pub fn parse(code: &str) -> Vec<(String, f64)> {
    let mut slices = Vec::new();

    for line in code.lines().skip(1) {
        let line = line.trim();
        if line.contains(':') && !line.starts_with("title") {
            if let Some((label, value)) = line.split_once(':') {
                let label = label.trim().trim_matches('"').to_string();
                if let Ok(val) = value.trim().parse::<f64>() {
                    slices.push((label, val));
                }
            }
        }
    }

    slices
}

/// Generate shapes for a pie chart
pub fn generate_shapes(slices: &[(String, f64)]) -> Vec<Shape> {
    let mut shapes = Vec::new();

    if slices.is_empty() {
        return shapes;
    }

    let colors = [
        "4472C4", "ED7D31", "A5A5A5", "FFC000", "5B9BD5", "70AD47", "9E480E", "997300",
    ];
    let center_x = 2_500_000u32;
    let center_y = 3_000_000u32;
    let radius = 1_500_000u32;

    // Create a circle for the pie
    let pie_circle = Shape::new(
        ShapeType::Ellipse,
        center_x - radius,
        center_y - radius,
        radius * 2,
        radius * 2,
    )
    .with_fill(ShapeFill::new(colors[0]))
    .with_line(ShapeLine::new("FFFFFF", 2));
    shapes.push(pie_circle);

    // Create legend
    let legend_x = 5_000_000u32;
    let legend_y = 2_000_000u32;
    let legend_height = 350_000u32;

    let total: f64 = slices.iter().map(|(_, v)| v).sum();

    for (i, (label, value)) in slices.iter().enumerate() {
        let color = colors[i % colors.len()];
        let percentage = if total > 0.0 {
            value / total * 100.0
        } else {
            0.0
        };

        // Color box
        let box_shape = Shape::new(
            ShapeType::Rectangle,
            legend_x,
            legend_y + (i as u32) * legend_height,
            200_000,
            200_000,
        )
        .with_fill(ShapeFill::new(color));
        shapes.push(box_shape);

        // Label
        let label_text = format!("{} ({:.1}%)", label, percentage);
        let label_shape = Shape::new(
            ShapeType::Rectangle,
            legend_x + 300_000,
            legend_y + (i as u32) * legend_height,
            2_500_000,
            200_000,
        )
        .with_text(&label_text);
        shapes.push(label_shape);
    }

    shapes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pie() {
        let code = "pie\n    \"Dogs\" : 45\n    \"Cats\" : 30";
        let slices = parse(code);
        assert_eq!(slices.len(), 2);
        assert_eq!(slices[0].0, "Dogs");
        assert_eq!(slices[0].1, 45.0);
    }

    #[test]
    fn test_generate_pie_shapes() {
        let slices = vec![("A".to_string(), 50.0), ("B".to_string(), 50.0)];
        let shapes = generate_shapes(&slices);
        assert!(!shapes.is_empty());
    }
}
