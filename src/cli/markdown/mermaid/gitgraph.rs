//! Git graph diagram rendering

use crate::generator::{Shape, ShapeType, ShapeFill, ShapeLine};

/// Generate shapes for a git graph
pub fn generate_shapes(code: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();
    let mut commits: Vec<(String, String, u32)> = Vec::new(); // (id, branch, position)
    let mut branches: Vec<String> = vec!["main".to_string()];
    let mut current_branch = "main".to_string();
    let mut commit_count = 0u32;
    
    for line in code.lines().skip(1) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        if trimmed.starts_with("commit") {
            // Parse: commit id: "message"
            let id = if trimmed.contains("id:") {
                let parts: Vec<&str> = trimmed.split("id:").collect();
                if parts.len() > 1 {
                    parts[1].trim().trim_matches('"').to_string()
                } else {
                    format!("C{}", commit_count)
                }
            } else {
                format!("C{}", commit_count)
            };
            commits.push((id, current_branch.clone(), commit_count));
            commit_count += 1;
        } else if trimmed.starts_with("branch") {
            let branch_name = trimmed.strip_prefix("branch").unwrap_or("").trim().to_string();
            if !branch_name.is_empty() && !branches.contains(&branch_name) {
                branches.push(branch_name);
            }
        } else if trimmed.starts_with("checkout") {
            let branch_name = trimmed.strip_prefix("checkout").unwrap_or("").trim().to_string();
            if !branch_name.is_empty() {
                current_branch = branch_name;
            }
        } else if trimmed.starts_with("merge") {
            // Merge creates a commit on current branch
            let merge_from = trimmed.strip_prefix("merge").unwrap_or("").trim();
            commits.push((format!("Merge {}", merge_from), current_branch.clone(), commit_count));
            commit_count += 1;
        }
    }
    
    // Layout constants
    let start_x = 1_000_000u32;
    let start_y = 2_000_000u32;
    let commit_spacing = 800_000u32;
    let branch_spacing = 600_000u32;
    let commit_size = 300_000u32;
    
    // Branch colors
    let branch_colors = ["1565C0", "2E7D32", "7B1FA2", "E65100", "C2185B"];
    
    // Draw branch lines first (as background)
    for (i, branch) in branches.iter().enumerate() {
        let y = start_y + (i as u32) * branch_spacing;
        let color = branch_colors[i % branch_colors.len()];
        
        // Branch label
        shapes.push(
            Shape::new(ShapeType::RoundedRectangle, start_x - 800_000, y - 100_000, 700_000, 200_000)
                .with_fill(ShapeFill::new(color))
                .with_text(branch)
        );
        
        // Branch line (simplified as rectangle)
        let line_length = (commit_count.max(1) as u32) * commit_spacing + 200_000;
        shapes.push(
            Shape::new(ShapeType::Rectangle, start_x, y - 25_000, line_length, 50_000)
                .with_fill(ShapeFill::new(color))
        );
    }
    
    // Draw commits
    for (id, branch, pos) in &commits {
        let branch_idx = branches.iter().position(|b| b == branch).unwrap_or(0);
        let x = start_x + (*pos) * commit_spacing;
        let y = start_y + (branch_idx as u32) * branch_spacing - commit_size / 2;
        let color = branch_colors[branch_idx % branch_colors.len()];
        
        shapes.push(
            Shape::new(ShapeType::Circle, x, y, commit_size, commit_size)
                .with_fill(ShapeFill::new("FFFFFF"))
                .with_line(ShapeLine::new(color, 25400))
                .with_text(id)
        );
    }
    
    // If no commits, add placeholder
    if commits.is_empty() {
        shapes.push(
            Shape::new(ShapeType::Rectangle, 1_000_000, 2_000_000, 7_000_000, 3_000_000)
                .with_fill(ShapeFill::new("ECEFF1"))
                .with_line(ShapeLine::new("607D8B", 25400))
                .with_text("Git Graph")
        );
    }
    
    shapes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_gitgraph() {
        let code = "gitGraph\n    commit id: \"Initial\"\n    branch feature\n    checkout feature\n    commit id: \"Feature\"\n    checkout main\n    merge feature";
        let shapes = generate_shapes(code);
        assert!(!shapes.is_empty());
    }

    #[test]
    fn test_gitgraph_empty() {
        let code = "gitGraph";
        let shapes = generate_shapes(code);
        assert!(!shapes.is_empty()); // Should have placeholder
    }
}
