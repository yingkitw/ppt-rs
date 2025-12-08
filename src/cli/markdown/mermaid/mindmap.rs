//! Mindmap diagram parsing and rendering

use crate::generator::{Shape, ShapeType};
use super::types::{create_labeled_shape, LabelPosition};

/// Generate shapes for a mindmap
pub fn generate_shapes(code: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();
    
    let mut root = String::new();
    let mut level1: Vec<String> = Vec::new();
    let mut level2: Vec<(usize, String)> = Vec::new();
    
    for line in code.lines().skip(1) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("%%") {
            continue;
        }
        
        let spaces = line.len() - line.trim_start().len();
        let text = trimmed.trim_start_matches(|c| c == '-' || c == '+' || c == '*')
            .trim()
            .trim_matches(|c| c == '(' || c == ')' || c == '[' || c == ']')
            .to_string();
        
        if text.is_empty() { continue; }
        
        if spaces == 0 || (root.is_empty() && spaces <= 4) {
            if root.is_empty() {
                root = text;
            }
        } else if spaces <= 8 {
            level1.push(text);
        } else {
            let parent_idx = level1.len().saturating_sub(1);
            level2.push((parent_idx, text));
        }
    }
    
    // Layout parameters
    let center_x = 4_000_000u32;
    let center_y = 3_000_000u32;
    let root_width = 2_000_000u32;
    let root_height = 600_000u32;
    let node_width = 1_500_000u32;
    let node_height = 400_000u32;
    let radius1 = 2_000_000u32;
    let radius2 = 3_200_000u32;
    
    // Root node using helper
    shapes.extend(create_labeled_shape(
        ShapeType::Ellipse,
        center_x - root_width/2, center_y - root_height/2,
        root_width, root_height,
        Some("3949AB"), Some("1A237E"),
        &root, LabelPosition::Inside
    ));
    
    // Level 1 nodes (arranged in circle)
    let level1_colors = ["4472C4", "ED7D31", "70AD47", "FFC000", "5B9BD5", "9E480E"];
    let angle_step = if level1.is_empty() { 0.0 } else { 2.0 * std::f64::consts::PI / level1.len() as f64 };
    
    for (i, text) in level1.iter().enumerate() {
        let angle = (i as f64) * angle_step - std::f64::consts::PI / 2.0;
        let x = center_x + (radius1 as f64 * angle.cos()) as u32 - node_width / 2;
        let y = center_y + (radius1 as f64 * angle.sin()) as u32 - node_height / 2;
        
        let color = level1_colors[i % level1_colors.len()];
        shapes.extend(create_labeled_shape(
            ShapeType::RoundedRectangle, x, y, node_width, node_height,
            Some(color), None, text, LabelPosition::Inside
        ));
    }
    
    // Level 2 nodes
    for (parent_idx, text) in &level2 {
        if *parent_idx < level1.len() {
            let parent_angle = (*parent_idx as f64) * angle_step - std::f64::consts::PI / 2.0;
            let x = center_x + (radius2 as f64 * parent_angle.cos()) as u32 - node_width / 2;
            let y = center_y + (radius2 as f64 * parent_angle.sin()) as u32 - node_height / 2;
            
            shapes.extend(create_labeled_shape(
                ShapeType::RoundedRectangle, x, y, node_width, node_height,
                Some("E8EAF6"), Some("3949AB"), text, LabelPosition::Inside
            ));
        }
    }
    
    shapes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_mindmap_shapes() {
        let code = "mindmap\n    root((Central))\n        Branch1\n        Branch2";
        let shapes = generate_shapes(code);
        assert!(!shapes.is_empty());
    }
}
