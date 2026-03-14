//! Simplified table creation utilities
//!
//! This module provides easy-to-use helpers for creating tables with minimal boilerplate.

use crate::generator::{TableBuilder, TableRow, TableCell};

/// Create a simple table with the specified number of rows and columns
///
/// # Example
/// ```
/// use ppt_rs::helpers::tables::simple_table;
///
/// let table = simple_table(3, 4); // 3 rows, 4 columns with equal widths
/// ```
pub fn simple_table(rows: usize, cols: usize) -> TableBuilder {
    let col_width = 9144000 / cols as u32; // Divide slide width equally
    let column_widths = vec![col_width; cols];
    let mut builder = TableBuilder::new(column_widths);
    
    // Add empty rows
    for _ in 0..rows {
        let cells = (0..cols).map(|_| TableCell::new("")).collect();
        builder = builder.add_row(TableRow::new(cells));
    }
    
    builder
}

/// Create a table with custom column widths (in inches)
///
/// # Example
/// ```
/// use ppt_rs::helpers::tables::table_with_widths;
///
/// let table = table_with_widths(&[2.0, 3.0, 2.5]); // 3 columns with specific widths
/// ```
pub fn table_with_widths(widths_inches: &[f64]) -> TableBuilder {
    let column_widths: Vec<u32> = widths_inches
        .iter()
        .map(|w| (w * 914400.0) as u32)
        .collect();
    TableBuilder::new(column_widths)
}

/// Create a table from a 2D array of data
///
/// # Example
/// ```
/// use ppt_rs::helpers::tables::table_from_data;
///
/// let data = vec![
///     vec!["Name", "Age", "City"],
///     vec!["Alice", "30", "NYC"],
///     vec!["Bob", "25", "LA"],
/// ];
/// let table = table_from_data(&data, None);
/// ```
pub fn table_from_data(data: &[Vec<&str>], column_widths: Option<Vec<f64>>) -> TableBuilder {
    if data.is_empty() {
        return simple_table(0, 0);
    }
    
    let cols = data[0].len();
    let widths = if let Some(widths) = column_widths {
        widths.iter().map(|w| (w * 914400.0) as u32).collect()
    } else {
        let col_width = 9144000 / cols as u32;
        vec![col_width; cols]
    };
    
    let mut builder = TableBuilder::new(widths);
    
    for row_data in data {
        let cells: Vec<TableCell> = row_data.iter().map(|&text| TableCell::new(text)).collect();
        builder = builder.add_row(TableRow::new(cells));
    }
    
    builder
}

/// Create a table with a header row
///
/// # Example
/// ```
/// use ppt_rs::helpers::tables::table_with_header;
///
/// let headers = vec!["Name", "Age", "City"];
/// let table = table_with_header(&headers, 5); // Header + 5 data rows
/// ```
pub fn table_with_header(headers: &[&str], data_rows: usize) -> TableBuilder {
    let cols = headers.len();
    let col_width = 9144000 / cols as u32;
    let column_widths = vec![col_width; cols];
    
    let mut builder = TableBuilder::new(column_widths);
    
    // Add header row with styling
    let header_cells: Vec<TableCell> = headers
        .iter()
        .map(|&text| {
            TableCell::new(text)
                .bold()
                .background_color("1F4E79")
                .text_color("FFFFFF")
                .align_center()
        })
        .collect();
    builder = builder.add_row(TableRow::new(header_cells));
    
    // Add empty data rows
    for _ in 0..data_rows {
        let cells = (0..cols).map(|_| TableCell::new("")).collect();
        builder = builder.add_row(TableRow::new(cells));
    }
    
    builder
}

/// Quick table builder for common patterns
pub struct QuickTable {
    builder: TableBuilder,
    cols: usize,
}

impl QuickTable {
    /// Create a new quick table with equal column widths
    pub fn new(cols: usize) -> Self {
        let col_width = 9144000 / cols as u32;
        let column_widths = vec![col_width; cols];
        Self {
            builder: TableBuilder::new(column_widths),
            cols,
        }
    }

    /// Create with custom column widths in inches
    pub fn with_widths(widths_inches: &[f64]) -> Self {
        let column_widths: Vec<u32> = widths_inches
            .iter()
            .map(|w| (w * 914400.0) as u32)
            .collect();
        Self {
            builder: TableBuilder::new(column_widths.clone()),
            cols: column_widths.len(),
        }
    }

    /// Add a header row with automatic styling
    pub fn header(mut self, headers: &[&str]) -> Self {
        let cells: Vec<TableCell> = headers
            .iter()
            .map(|&text| {
                TableCell::new(text)
                    .bold()
                    .background_color("1F4E79")
                    .text_color("FFFFFF")
                    .align_center()
            })
            .collect();
        self.builder = self.builder.add_row(TableRow::new(cells));
        self
    }

    /// Add a data row
    pub fn row(mut self, data: &[&str]) -> Self {
        let cells: Vec<TableCell> = data.iter().map(|&text| TableCell::new(text)).collect();
        self.builder = self.builder.add_row(TableRow::new(cells));
        self
    }

    /// Add a styled row with custom cell styling
    pub fn styled_row(mut self, cells: Vec<TableCell>) -> Self {
        self.builder = self.builder.add_row(TableRow::new(cells));
        self
    }

    /// Add multiple rows at once
    pub fn rows(mut self, data: &[Vec<&str>]) -> Self {
        for row_data in data {
            let cells: Vec<TableCell> = row_data.iter().map(|&text| TableCell::new(text)).collect();
            self.builder = self.builder.add_row(TableRow::new(cells));
        }
        self
    }

    /// Set the table position (in inches)
    pub fn at(mut self, x: f64, y: f64) -> Self {
        self.builder = self.builder.position((x * 914400.0) as u32, (y * 914400.0) as u32);
        self
    }

    /// Build the final table
    pub fn build(self) -> crate::generator::Table {
        self.builder.build()
    }
}

/// Helper to create a cell with common styling
pub fn cell(text: &str) -> TableCell {
    TableCell::new(text)
}

/// Helper to create a header cell
pub fn header_cell(text: &str) -> TableCell {
    TableCell::new(text)
        .bold()
        .background_color("1F4E79")
        .text_color("FFFFFF")
        .align_center()
}

/// Helper to create a highlighted cell
pub fn highlight_cell(text: &str, color: &str) -> TableCell {
    TableCell::new(text)
        .background_color(color)
        .bold()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_table() {
        let _table = simple_table(3, 4);
        // Table should be created successfully
    }

    #[test]
    fn test_table_with_widths() {
        let _table = table_with_widths(&[2.0, 3.0, 2.5]);
        // Table should be created with custom widths
    }

    #[test]
    fn test_table_from_data() {
        let data = vec![
            vec!["Name", "Age", "City"],
            vec!["Alice", "30", "NYC"],
            vec!["Bob", "25", "LA"],
        ];
        let _table = table_from_data(&data, None);
        // Table should be created from data
    }

    #[test]
    fn test_quick_table() {
        let _table = QuickTable::new(3)
            .header(&["Name", "Age", "City"])
            .row(&["Alice", "30", "NYC"])
            .row(&["Bob", "25", "LA"])
            .at(0.5, 1.5)
            .build();
        // Table should be built successfully
    }
}
