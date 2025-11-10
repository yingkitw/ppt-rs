//! Table module - table types and styling

pub mod style;
pub mod formatting;
pub mod style_manager;

pub use style::{TableStyle, TableStyleType, CellStyle, TableStyleManager};
pub use formatting::{
    CellBorder, CellShading, CellFormat, BorderStyle, CellAlignment, VerticalAlignment,
};
pub use style_manager::{TableStylePreset, TableStyle as AdvancedTableStyle, TableStyleManager as AdvancedTableStyleManager};

/// Table cell - represents a cell in a table
#[derive(Clone, Debug)]
pub struct TableCell {
    /// Cell text content
    text: String,
    /// Cell width in EMUs
    width: Option<u32>,
    /// Cell height in EMUs
    height: Option<u32>,
}

impl TableCell {
    /// Create a new table cell
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            width: None,
            height: None,
        }
    }

    /// Get cell text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set cell text
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    /// Set cell width in EMUs
    pub fn set_width(&mut self, width: u32) {
        self.width = Some(width);
    }

    /// Get cell width
    pub fn width(&self) -> Option<u32> {
        self.width
    }

    /// Set cell height in EMUs
    pub fn set_height(&mut self, height: u32) {
        self.height = Some(height);
    }

    /// Get cell height
    pub fn height(&self) -> Option<u32> {
        self.height
    }
}

impl Default for TableCell {
    fn default() -> Self {
        Self::new("")
    }
}

/// Table row - represents a row in a table
#[derive(Clone, Debug)]
pub struct TableRow {
    /// Row cells
    cells: Vec<TableCell>,
    /// Row height in EMUs
    height: Option<u32>,
}

impl TableRow {
    /// Create a new table row with specified number of columns
    pub fn new(columns: u32) -> Self {
        let cells = (0..columns)
            .map(|_| TableCell::new(""))
            .collect();
        Self {
            cells,
            height: None,
        }
    }

    /// Get number of cells
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Check if row is empty
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Get cell by index
    pub fn get(&self, index: usize) -> Option<&TableCell> {
        self.cells.get(index)
    }

    /// Get mutable cell by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut TableCell> {
        self.cells.get_mut(index)
    }

    /// Set cell text
    pub fn set_cell_text(&mut self, index: usize, text: impl Into<String>) -> bool {
        if let Some(cell) = self.cells.get_mut(index) {
            cell.set_text(text);
            true
        } else {
            false
        }
    }

    /// Set row height
    pub fn set_height(&mut self, height: u32) {
        self.height = Some(height);
    }

    /// Get row height
    pub fn height(&self) -> Option<u32> {
        self.height
    }
}

/// Table - represents a table in a presentation
#[derive(Clone, Debug)]
pub struct Table {
    /// Table rows
    rows: Vec<TableRow>,
    /// Table columns
    columns: u32,
}

impl Table {
    /// Create a new table
    pub fn new(rows: u32, columns: u32) -> Self {
        let table_rows = (0..rows)
            .map(|_| TableRow::new(columns))
            .collect();
        Self {
            rows: table_rows,
            columns,
        }
    }

    /// Get number of rows
    pub fn rows(&self) -> u32 {
        self.rows.len() as u32
    }

    /// Get number of columns
    pub fn columns(&self) -> u32 {
        self.columns
    }

    /// Get row by index
    pub fn get_row(&self, index: usize) -> Option<&TableRow> {
        self.rows.get(index)
    }

    /// Get mutable row by index
    pub fn get_row_mut(&mut self, index: usize) -> Option<&mut TableRow> {
        self.rows.get_mut(index)
    }

    /// Set cell text
    pub fn set_cell_text(&mut self, row: usize, col: usize, text: impl Into<String>) -> bool {
        if let Some(table_row) = self.rows.get_mut(row) {
            table_row.set_cell_text(col, text)
        } else {
            false
        }
    }

    /// Get cell text
    pub fn get_cell_text(&self, row: usize, col: usize) -> Option<&str> {
        self.rows
            .get(row)
            .and_then(|r| r.get(col))
            .map(|c| c.text())
    }

    /// Generate XML for table
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<a:tbl>\n");
        xml.push_str("  <a:tblPr/>\n");
        
        // Add table grid
        xml.push_str("  <a:tblGrid>\n");
        for _ in 0..self.columns {
            xml.push_str("    <a:gridCol/>\n");
        }
        xml.push_str("  </a:tblGrid>\n");
        
        // Add rows
        for row in &self.rows {
            xml.push_str("  <a:tr>\n");
            for cell in &row.cells {
                xml.push_str("    <a:tc>\n");
                xml.push_str("      <a:txBody>\n");
                xml.push_str("        <a:bodyPr/>\n");
                xml.push_str("        <a:lstStyle/>\n");
                xml.push_str("        <a:p>\n");
                xml.push_str(&format!("          <a:r><a:rPr lang=\"en-US\" dirty=\"0\"/><a:t>{}</a:t></a:r>\n", escape_xml(cell.text())));
                xml.push_str("        </a:p>\n");
                xml.push_str("      </a:txBody>\n");
                xml.push_str("      <a:tcPr/>\n");
                xml.push_str("    </a:tc>\n");
            }
            xml.push_str("  </a:tr>\n");
        }
        
        xml.push_str("</a:tbl>");
        xml
    }
}

/// Escape XML special characters
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
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
    fn test_table_cell_creation() {
        let cell = TableCell::new("Test");
        assert_eq!(cell.text(), "Test");
        assert_eq!(cell.width(), None);
        assert_eq!(cell.height(), None);
    }

    #[test]
    fn test_table_cell_dimensions() {
        let mut cell = TableCell::new("Cell");
        cell.set_width(914400);
        cell.set_height(457200);
        
        assert_eq!(cell.width(), Some(914400));
        assert_eq!(cell.height(), Some(457200));
    }

    #[test]
    fn test_table_row_creation() {
        let row = TableRow::new(5);
        assert_eq!(row.len(), 5);
        assert!(!row.is_empty());
    }

    #[test]
    fn test_table_row_cell_access() {
        let mut row = TableRow::new(3);
        assert!(row.set_cell_text(0, "Header 1"));
        assert!(row.set_cell_text(1, "Header 2"));
        assert!(row.set_cell_text(2, "Header 3"));
        
        assert_eq!(row.get(0).map(|c| c.text()), Some("Header 1"));
        assert_eq!(row.get(1).map(|c| c.text()), Some("Header 2"));
        assert_eq!(row.get(2).map(|c| c.text()), Some("Header 3"));
    }

    #[test]
    fn test_table_set_cell_text() {
        let mut table = Table::new(2, 3);
        assert!(table.set_cell_text(0, 0, "A1"));
        assert!(table.set_cell_text(0, 1, "A2"));
        assert!(table.set_cell_text(1, 0, "B1"));
        
        assert_eq!(table.get_cell_text(0, 0), Some("A1"));
        assert_eq!(table.get_cell_text(0, 1), Some("A2"));
        assert_eq!(table.get_cell_text(1, 0), Some("B1"));
    }

    #[test]
    fn test_table_get_row() {
        let table = Table::new(3, 2);
        assert!(table.get_row(0).is_some());
        assert!(table.get_row(1).is_some());
        assert!(table.get_row(2).is_some());
        assert!(table.get_row(3).is_none());
    }

    #[test]
    fn test_table_to_xml() {
        let mut table = Table::new(2, 2);
        table.set_cell_text(0, 0, "Header 1");
        table.set_cell_text(0, 1, "Header 2");
        table.set_cell_text(1, 0, "Data 1");
        table.set_cell_text(1, 1, "Data 2");
        
        let xml = table.to_xml();
        assert!(xml.contains("<a:tbl>"));
        assert!(xml.contains("</a:tbl>"));
        assert!(xml.contains("Header 1"));
        assert!(xml.contains("Data 2"));
    }

    #[test]
    fn test_table_xml_escaping() {
        let mut table = Table::new(1, 1);
        table.set_cell_text(0, 0, "Test & <special> \"chars\"");
        
        let xml = table.to_xml();
        assert!(xml.contains("&amp;"));
        assert!(xml.contains("&lt;"));
        assert!(xml.contains("&gt;"));
        assert!(xml.contains("&quot;"));
    }

    #[test]
    fn test_table_style_basic() {
        let mut manager = TableStyleManager::new();
        manager.create_style(TableStyleType::Light, "Light".to_string());
        
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_table_cell_default() {
        let cell = TableCell::default();
        assert_eq!(cell.text(), "");
        assert_eq!(cell.width(), None);
    }
}
