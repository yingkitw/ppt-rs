//! Table-related objects

use crate::text::TextFrame;

/// Table - a DrawingML table object
pub struct Table {
    rows: Vec<TableRow>,
    columns: Vec<TableColumn>,
    first_row: bool,
    first_col: bool,
    last_row: bool,
    last_col: bool,
    horz_banding: bool,
    vert_banding: bool,
}

impl Table {
    /// Create a new table with specified dimensions
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut table_rows = Vec::new();
        let mut table_cols = Vec::new();
        
        for i in 0..rows {
            table_rows.push(TableRow::new(i, cols));
        }
        
        for i in 0..cols {
            table_cols.push(TableColumn::new(i));
        }
        
        Self {
            rows: table_rows,
            columns: table_cols,
            first_row: false,
            first_col: false,
            last_row: false,
            last_col: false,
            horz_banding: false,
            vert_banding: false,
        }
    }

    /// Get a cell by row and column index (0-based)
    pub fn cell(&self, row_idx: usize, col_idx: usize) -> Option<&TableCell> {
        self.rows.get(row_idx)?.cells().get(col_idx)
    }

    /// Get a mutable cell by row and column index
    pub fn cell_mut(&mut self, row_idx: usize, col_idx: usize) -> Option<&mut TableCell> {
        self.rows.get_mut(row_idx)?.cells_mut().get_mut(col_idx)
    }

    /// Get rows
    pub fn rows(&self) -> &[TableRow] {
        &self.rows
    }

    /// Get mutable rows
    pub fn rows_mut(&mut self) -> &mut [TableRow] {
        &mut self.rows
    }

    /// Get columns
    pub fn columns(&self) -> &[TableColumn] {
        &self.columns
    }

    /// Get mutable columns
    pub fn columns_mut(&mut self) -> &mut [TableColumn] {
        &mut self.columns
    }

    /// Get number of rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get number of columns
    pub fn col_count(&self) -> usize {
        self.columns.len()
    }

    /// Check if first row has distinct formatting
    pub fn first_row(&self) -> bool {
        self.first_row
    }

    /// Set first row distinct formatting
    pub fn set_first_row(&mut self, value: bool) {
        self.first_row = value;
    }

    /// Check if first column has distinct formatting
    pub fn first_col(&self) -> bool {
        self.first_col
    }

    /// Set first column distinct formatting
    pub fn set_first_col(&mut self, value: bool) {
        self.first_col = value;
    }

    /// Check if last row has distinct formatting
    pub fn last_row(&self) -> bool {
        self.last_row
    }

    /// Set last row distinct formatting
    pub fn set_last_row(&mut self, value: bool) {
        self.last_row = value;
    }

    /// Check if last column has distinct formatting
    pub fn last_col(&self) -> bool {
        self.last_col
    }

    /// Set last column distinct formatting
    pub fn set_last_col(&mut self, value: bool) {
        self.last_col = value;
    }

    /// Check if horizontal banding is enabled
    pub fn horz_banding(&self) -> bool {
        self.horz_banding
    }

    /// Set horizontal banding
    pub fn set_horz_banding(&mut self, value: bool) {
        self.horz_banding = value;
    }

    /// Check if vertical banding is enabled
    pub fn vert_banding(&self) -> bool {
        self.vert_banding
    }

    /// Set vertical banding
    pub fn set_vert_banding(&mut self, value: bool) {
        self.vert_banding = value;
    }
}

/// Table row
pub struct TableRow {
    index: usize,
    cells: Vec<TableCell>,
    height: u32, // in EMU
}

impl TableRow {
    /// Create a new table row
    pub fn new(index: usize, col_count: usize) -> Self {
        let mut cells = Vec::new();
        for i in 0..col_count {
            cells.push(TableCell::new(index, i));
        }
        Self {
            index,
            cells,
            height: 457200, // Default 0.5 inch
        }
    }

    /// Get cells in this row
    pub fn cells(&self) -> &[TableCell] {
        &self.cells
    }

    /// Get mutable cells in this row
    pub fn cells_mut(&mut self) -> &mut [TableCell] {
        &mut self.cells
    }

    /// Get row height in EMU
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set row height in EMU
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    /// Get row index
    pub fn index(&self) -> usize {
        self.index
    }
}

/// Table column
pub struct TableColumn {
    index: usize,
    width: u32, // in EMU
}

impl TableColumn {
    /// Create a new table column
    pub fn new(index: usize) -> Self {
        Self {
            index,
            width: 1828800, // Default 2 inches
        }
    }

    /// Get column width in EMU
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Set column width in EMU
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    /// Get column index
    pub fn index(&self) -> usize {
        self.index
    }
}

/// Table cell
pub struct TableCell {
    row_idx: usize,
    col_idx: usize,
    text_frame: TextFrame,
    merge_span: Option<(usize, usize)>, // (row_span, col_span)
}

impl TableCell {
    /// Create a new table cell
    pub fn new(row_idx: usize, col_idx: usize) -> Self {
        Self {
            row_idx,
            col_idx,
            text_frame: TextFrame::new(),
            merge_span: None,
        }
    }

    /// Get the text frame
    pub fn text_frame(&self) -> &TextFrame {
        &self.text_frame
    }

    /// Get mutable text frame
    pub fn text_frame_mut(&mut self) -> &mut TextFrame {
        &mut self.text_frame
    }

    /// Get row index
    pub fn row_idx(&self) -> usize {
        self.row_idx
    }

    /// Get column index
    pub fn col_idx(&self) -> usize {
        self.col_idx
    }

    /// Check if this cell is merged
    pub fn is_merged(&self) -> bool {
        self.merge_span.is_some()
    }

    /// Get merge span (row_span, col_span)
    pub fn merge_span(&self) -> Option<(usize, usize)> {
        self.merge_span
    }

    /// Set merge span
    pub fn set_merge_span(&mut self, row_span: usize, col_span: usize) {
        self.merge_span = Some((row_span, col_span));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_new() {
        let table = Table::new(3, 4);
        assert_eq!(table.row_count(), 3);
        assert_eq!(table.col_count(), 4);
        assert!(!table.first_row());
        assert!(!table.first_col());
    }

    #[test]
    fn test_table_cell() {
        let table = Table::new(2, 2);
        let cell = table.cell(0, 0).unwrap();
        assert_eq!(cell.row_idx(), 0);
        assert_eq!(cell.col_idx(), 0);
    }

    #[test]
    fn test_table_row_column() {
        let table = Table::new(2, 3);
        let row = &table.rows()[0];
        let col = &table.columns()[0];
        
        assert_eq!(row.index(), 0);
        assert_eq!(col.index(), 0);
        assert_eq!(row.height(), 457200);
        assert_eq!(col.width(), 1828800);
    }

    #[test]
    fn test_table_formatting() {
        let mut table = Table::new(2, 2);
        table.set_first_row(true);
        table.set_first_col(true);
        table.set_last_row(true);
        table.set_last_col(true);
        table.set_horz_banding(true);
        table.set_vert_banding(true);
        
        assert!(table.first_row());
        assert!(table.first_col());
        assert!(table.last_row());
        assert!(table.last_col());
        assert!(table.horz_banding());
        assert!(table.vert_banding());
    }

    #[test]
    fn test_table_cell_merge() {
        let mut table = Table::new(2, 2);
        let cell = table.cell_mut(0, 0).unwrap();
        cell.set_merge_span(2, 2);
        assert!(cell.is_merged());
        assert_eq!(cell.merge_span(), Some((2, 2)));
    }
}
