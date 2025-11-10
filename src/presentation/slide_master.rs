//! Slide master access
//!
//! Provides access to the slide master which controls overall presentation design.


/// Slide master information
#[derive(Debug, Clone)]
pub struct SlideMasterInfo {
    name: String,
    master_id: usize,
}

impl SlideMasterInfo {
    /// Create a new slide master info
    pub fn new(name: String, master_id: usize) -> Self {
        Self { name, master_id }
    }

    /// Get the master name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the master ID
    pub fn master_id(&self) -> usize {
        self.master_id
    }
}

/// Slide master
#[derive(Debug, Clone)]
pub struct SlideMaster {
    info: SlideMasterInfo,
}

impl SlideMaster {
    /// Create a new slide master
    pub fn new(name: String, master_id: usize) -> Self {
        Self {
            info: SlideMasterInfo::new(name, master_id),
        }
    }

    /// Get the master name
    pub fn name(&self) -> &str {
        self.info.name()
    }

    /// Get the master ID
    pub fn master_id(&self) -> usize {
        self.info.master_id()
    }

    /// Get the master info
    pub fn info(&self) -> &SlideMasterInfo {
        &self.info
    }
}

/// Slide masters collection
#[derive(Debug, Clone)]
pub struct SlideMasters {
    masters: Vec<SlideMaster>,
}

impl SlideMasters {
    /// Create a new slide masters collection with default master
    pub fn new() -> Self {
        let mut masters = Vec::new();
        // Add default master
        masters.push(SlideMaster::new("Office Theme".to_string(), 1));
        Self { masters }
    }

    /// Get a master by index
    pub fn get(&self, idx: usize) -> Option<&SlideMaster> {
        self.masters.get(idx)
    }

    /// Get a master by name
    pub fn get_by_name(&self, name: &str) -> Option<&SlideMaster> {
        self.masters.iter().find(|m| m.name() == name)
    }

    /// Get all masters
    pub fn all(&self) -> &[SlideMaster] {
        &self.masters
    }

    /// Get master count
    pub fn len(&self) -> usize {
        self.masters.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.masters.is_empty()
    }

    /// Add a custom master
    pub fn add(&mut self, master: SlideMaster) {
        self.masters.push(master);
    }

    /// Get the first (default) master
    pub fn first(&self) -> Option<&SlideMaster> {
        self.masters.first()
    }
}

impl Default for SlideMasters {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_master_info() {
        let info = SlideMasterInfo::new("Test".to_string(), 1);
        assert_eq!(info.name(), "Test");
        assert_eq!(info.master_id(), 1);
    }

    #[test]
    fn test_slide_master() {
        let master = SlideMaster::new("Office".to_string(), 1);
        assert_eq!(master.name(), "Office");
        assert_eq!(master.master_id(), 1);
    }

    #[test]
    fn test_slide_masters_collection() {
        let masters = SlideMasters::new();
        assert_eq!(masters.len(), 1);
        assert!(masters.get(0).is_some());
        assert!(masters.first().is_some());
    }

    #[test]
    fn test_slide_masters_get_by_name() {
        let masters = SlideMasters::new();
        assert!(masters.get_by_name("Office Theme").is_some());
        assert!(masters.get_by_name("NonExistent").is_none());
    }

    #[test]
    fn test_slide_masters_add() {
        let mut masters = SlideMasters::new();
        let custom = SlideMaster::new("Custom".to_string(), 2);
        masters.add(custom);
        assert_eq!(masters.len(), 2);
    }
}
