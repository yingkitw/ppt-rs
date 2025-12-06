//! Quadrant chart rendering

use crate::generator::{Shape, ShapeType, ShapeFill, ShapeLine};

/// Generate shapes for a quadrant chart
pub fn generate_shapes(code: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();
    let mut points: Vec<(String, f32, f32)> = Vec::new();
    let mut x_axis = ("Low", "High");
    let mut y_axis = ("Low", "High");
    let mut quadrant_labels = ["", "", "", ""];
    
    for line in code.lines().skip(1) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("title") {
            continue;
        }
        
        if trimmed.starts_with("x-axis") {
            // Parse: x-axis Low Effort --> High Effort
            let parts: Vec<&str> = trimmed.split("-->").collect();
            if parts.len() == 2 {
                x_axis.0 = parts[0].strip_prefix("x-axis").unwrap_or("").trim();
                x_axis.1 = parts[1].trim();
            }
        } else if trimmed.starts_with("y-axis") {
            let parts: Vec<&str> = trimmed.split("-->").collect();
            if parts.len() == 2 {
                y_axis.0 = parts[0].strip_prefix("y-axis").unwrap_or("").trim();
                y_axis.1 = parts[1].trim();
            }
        } else if trimmed.starts_with("quadrant-") {
            // Parse quadrant labels
            if let Some(rest) = trimmed.strip_prefix("quadrant-1") {
                quadrant_labels[0] = rest.trim();
            } else if let Some(rest) = trimmed.strip_prefix("quadrant-2") {
                quadrant_labels[1] = rest.trim();
            } else if let Some(rest) = trimmed.strip_prefix("quadrant-3") {
                quadrant_labels[2] = rest.trim();
            } else if let Some(rest) = trimmed.strip_prefix("quadrant-4") {
                quadrant_labels[3] = rest.trim();
            }
        } else if trimmed.contains('[') && trimmed.contains(']') {
            // Parse point: "Label: [x, y]"
            if let Some(bracket_start) = trimmed.find('[') {
                let label = trimmed[..bracket_start].trim_end_matches(':').trim();
                let coords = &trimmed[bracket_start..];
                if let Some(bracket_end) = coords.find(']') {
                    let coord_str = &coords[1..bracket_end];
                    let parts: Vec<&str> = coord_str.split(',').collect();
                    if parts.len() == 2 {
                        let x = parts[0].trim().parse::<f32>().unwrap_or(0.5);
                        let y = parts[1].trim().parse::<f32>().unwrap_or(0.5);
                        points.push((label.to_string(), x, y));
                    }
                }
            }
        }
    }
    
    // Layout constants
    let chart_x = 1_000_000u32;
    let chart_y = 1_800_000u32;
    let chart_width = 6_000_000u32;
    let chart_height = 4_000_000u32;
    
    // Draw quadrant background (4 rectangles)
    let half_w = chart_width / 2;
    let half_h = chart_height / 2;
    
    // Quadrant colors (top-left, top-right, bottom-left, bottom-right)
    let colors = ["C8E6C9", "BBDEFB", "FFECB3", "FFCDD2"];
    
    // Q1: top-right (high x, high y)
    shapes.push(
        Shape::new(ShapeType::Rectangle, chart_x + half_w, chart_y, half_w, half_h)
            .with_fill(ShapeFill::new(colors[0]))
            .with_line(ShapeLine::new("9E9E9E", 12700))
    );
    // Q2: top-left (low x, high y)
    shapes.push(
        Shape::new(ShapeType::Rectangle, chart_x, chart_y, half_w, half_h)
            .with_fill(ShapeFill::new(colors[1]))
            .with_line(ShapeLine::new("9E9E9E", 12700))
    );
    // Q3: bottom-left (low x, low y)
    shapes.push(
        Shape::new(ShapeType::Rectangle, chart_x, chart_y + half_h, half_w, half_h)
            .with_fill(ShapeFill::new(colors[2]))
            .with_line(ShapeLine::new("9E9E9E", 12700))
    );
    // Q4: bottom-right (high x, low y)
    shapes.push(
        Shape::new(ShapeType::Rectangle, chart_x + half_w, chart_y + half_h, half_w, half_h)
            .with_fill(ShapeFill::new(colors[3]))
            .with_line(ShapeLine::new("9E9E9E", 12700))
    );
    
    // Draw points
    let point_size = 300_000u32;
    for (label, x, y) in &points {
        // Convert 0-1 coordinates to chart position
        // Note: y is inverted (0 at bottom, 1 at top)
        let px = chart_x + ((*x * chart_width as f32) as u32).saturating_sub(point_size / 2);
        let py = chart_y + (((1.0 - *y) * chart_height as f32) as u32).saturating_sub(point_size / 2);
        
        shapes.push(
            Shape::new(ShapeType::Circle, px, py, point_size, point_size)
                .with_fill(ShapeFill::new("1565C0"))
                .with_text(label)
        );
    }
    
    // If no points, add placeholder text
    if points.is_empty() {
        shapes.push(
            Shape::new(ShapeType::Rectangle, chart_x + half_w - 1_500_000, chart_y + half_h - 200_000, 3_000_000, 400_000)
                .with_fill(ShapeFill::new("FFFFFF"))
                .with_text("Quadrant Chart")
        );
    }
    
    shapes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_quadrant() {
        let code = "quadrantChart\n    x-axis Low --> High\n    y-axis Low --> High\n    Point A: [0.3, 0.7]";
        let shapes = generate_shapes(code);
        assert!(shapes.len() >= 4); // At least 4 quadrants
    }

    #[test]
    fn test_quadrant_empty() {
        let code = "quadrantChart";
        let shapes = generate_shapes(code);
        assert!(!shapes.is_empty());
    }
}
