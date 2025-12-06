//! Journey diagram (user journey) rendering

use crate::generator::{Shape, ShapeType, ShapeFill, ShapeLine};

/// Generate shapes for a journey diagram
pub fn generate_shapes(code: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();
    let mut sections: Vec<(String, Vec<(String, u8)>)> = Vec::new();
    let mut current_section = String::new();
    let mut current_items: Vec<(String, u8)> = Vec::new();
    
    for line in code.lines().skip(1) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("title") {
            continue;
        }
        
        if trimmed.starts_with("section") {
            // Save previous section
            if !current_section.is_empty() {
                sections.push((current_section.clone(), current_items.clone()));
                current_items.clear();
            }
            current_section = trimmed.strip_prefix("section").unwrap_or("").trim().to_string();
        } else if trimmed.contains(':') {
            // Parse item: "Task name: score: Actor"
            let parts: Vec<&str> = trimmed.split(':').collect();
            if parts.len() >= 2 {
                let task = parts[0].trim().to_string();
                let score = parts[1].trim().parse::<u8>().unwrap_or(3);
                current_items.push((task, score));
            }
        }
    }
    
    // Save last section
    if !current_section.is_empty() {
        sections.push((current_section, current_items));
    }
    
    // Layout constants
    let start_x = 500_000u32;
    let start_y = 1_800_000u32;
    let section_width = 2_000_000u32;
    let item_height = 400_000u32;
    let section_gap = 200_000u32;
    
    let mut x = start_x;
    
    for (section_name, items) in &sections {
        // Section header
        shapes.push(
            Shape::new(ShapeType::Rectangle, x, start_y, section_width, 350_000)
                .with_fill(ShapeFill::new("7B1FA2"))
                .with_text(section_name)
        );
        
        // Items in section
        let mut y = start_y + 400_000;
        for (task, score) in items {
            // Score determines color (1-5 scale)
            let color = match score {
                1 => "FFCDD2", // Red - bad
                2 => "FFE0B2", // Orange
                3 => "FFF9C4", // Yellow - neutral
                4 => "C8E6C9", // Light green
                5 => "A5D6A7", // Green - good
                _ => "E0E0E0",
            };
            
            shapes.push(
                Shape::new(ShapeType::RoundedRectangle, x + 50_000, y, section_width - 100_000, item_height - 50_000)
                    .with_fill(ShapeFill::new(color))
                    .with_line(ShapeLine::new("9E9E9E", 12700))
                    .with_text(&format!("{} ({})", task, score))
            );
            
            y += item_height;
        }
        
        x += section_width + section_gap;
    }
    
    // If no sections parsed, create placeholder
    if shapes.is_empty() {
        shapes.push(
            Shape::new(ShapeType::Rectangle, 1_000_000, 2_000_000, 7_000_000, 3_000_000)
                .with_fill(ShapeFill::new("F3E5F5"))
                .with_line(ShapeLine::new("7B1FA2", 25400))
                .with_text("User Journey Diagram")
        );
    }
    
    shapes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_journey() {
        let code = "journey\n    title User Journey\n    section Discovery\n      Find product: 3: User\n    section Purchase\n      Add to cart: 4: User";
        let shapes = generate_shapes(code);
        assert!(!shapes.is_empty());
    }

    #[test]
    fn test_journey_empty() {
        let code = "journey";
        let shapes = generate_shapes(code);
        assert!(!shapes.is_empty()); // Should have placeholder
    }
}
