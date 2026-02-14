//! Table and TableBuilder for constructing tables

use super::row::TableRow;

/// Table definition with rows and positioning
#[derive(Clone, Debug)]
pub struct Table {
    /// Table rows
    pub rows: Vec<TableRow>,
    /// Column widths in EMU
    pub column_widths: Vec<u32>,
    /// X position in EMU
    pub x: u32,
    /// Y position in EMU
    pub y: u32,
}

impl Table {
    /// Create a new table with explicit rows, column widths, and position
    pub fn new(rows: Vec<TableRow>, column_widths: Vec<u32>, x: u32, y: u32) -> Self {
        Table { rows, column_widths, x, y }
    }

    /// Create a table from raw data (2D string array)
    pub fn from_data(data: Vec<Vec<&str>>, column_widths: Vec<u32>, x: u32, y: u32) -> Self {
        use super::cell::TableCell;
        
        let rows = data
            .into_iter()
            .map(|row_data| {
                TableRow::new(row_data.into_iter().map(TableCell::new).collect())
            })
            .collect();

        Table {
            rows,
            column_widths,
            x,
            y,
        }
    }

    /// Calculate total table width
    pub fn width(&self) -> u32 {
        self.column_widths.iter().sum()
    }

    /// Calculate total table height based on row heights
    pub fn height(&self) -> u32 {
        self.rows
            .iter()
            .map(|row| row.height.unwrap_or(400000))
            .sum()
    }

    /// Get the number of rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get the number of columns
    pub fn column_count(&self) -> usize {
        self.column_widths.len()
    }
}

/// Builder for creating tables with fluent API
#[derive(Clone, Debug)]
pub struct TableBuilder {
    column_widths: Vec<u32>,
    rows: Vec<TableRow>,
    x: u32,
    y: u32,
}

impl TableBuilder {
    /// Create a new table builder with column widths
    pub fn new(column_widths: Vec<u32>) -> Self {
        TableBuilder {
            column_widths,
            rows: Vec::new(),
            x: 0,
            y: 0,
        }
    }

    /// Add a row to the table
    pub fn add_row(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }

    /// Set table position
    pub fn position(mut self, x: u32, y: u32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Add a simple row from strings
    pub fn add_simple_row(mut self, cells: Vec<&str>) -> Self {
        let row = TableRow::new(
            cells.into_iter().map(super::cell::TableCell::new).collect(),
        );
        self.rows.push(row);
        self
    }

    /// Build the final table
    pub fn build(self) -> Table {
        Table {
            rows: self.rows,
            column_widths: self.column_widths,
            x: self.x,
            y: self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::table::cell::TableCell;

    #[test]
    fn test_table_from_data() {
        let table = Table::from_data(
            vec![vec!["A", "B"], vec!["1", "2"]],
            vec![1000000, 1000000],
            0,
            0,
        );
        assert_eq!(table.row_count(), 2);
        assert_eq!(table.column_count(), 2);
    }

    #[test]
    fn test_table_dimensions() {
        let table = Table::from_data(
            vec![vec!["A", "B", "C"]],
            vec![1000000, 1500000, 2000000],
            0,
            0,
        );
        assert_eq!(table.width(), 4500000);
    }

    #[test]
    fn test_table_builder() {
        let table = TableBuilder::new(vec![1000000, 1000000])
            .add_row(TableRow::new(vec![
                TableCell::new("Header 1"),
                TableCell::new("Header 2"),
            ]))
            .add_row(TableRow::new(vec![
                TableCell::new("Data 1"),
                TableCell::new("Data 2"),
            ]))
            .position(500000, 1000000)
            .build();

        assert_eq!(table.row_count(), 2);
        assert_eq!(table.x, 500000);
        assert_eq!(table.y, 1000000);
    }
}
