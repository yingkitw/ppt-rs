//! Table module - table types and styling

pub mod style;

pub use style::{TableStyle, TableStyleType, CellStyle, TableStyleManager};

/// Table - represents a table in a presentation
#[derive(Clone, Debug)]
pub struct Table {
    /// Table rows
    rows: u32,
    /// Table columns
    columns: u32,
}

impl Table {
    /// Create a new table
    pub fn new(rows: u32, columns: u32) -> Self {
        Self { rows, columns }
    }

    /// Get number of rows
    pub fn rows(&self) -> u32 {
        self.rows
    }

    /// Get number of columns
    pub fn columns(&self) -> u32 {
        self.columns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let table = Table::new(3, 4);
        assert_eq!(table.rows(), 3);
        assert_eq!(table.columns(), 4);
    }

    #[test]
    fn test_table_style_basic() {
        let mut manager = TableStyleManager::new();
        manager.create_style(TableStyleType::Light, "Light".to_string());
        
        assert_eq!(manager.len(), 1);
    }
}
