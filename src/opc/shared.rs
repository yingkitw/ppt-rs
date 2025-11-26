//! Shared OPC utilities

/// Represents a relationship between parts
#[derive(Debug, Clone)]
pub struct Relationship {
    pub rel_id: String,
    pub rel_type: String,
    pub target_ref: String,
    pub target_mode: Option<String>,
}

impl Relationship {
    /// Create a new Relationship
    pub fn new(rel_id: String, rel_type: String, target_ref: String) -> Self {
        Relationship {
            rel_id,
            rel_type,
            target_ref,
            target_mode: None,
        }
    }

    /// Set the target mode (e.g., "External")
    pub fn with_target_mode(mut self, mode: String) -> Self {
        self.target_mode = Some(mode);
        self
    }
}
