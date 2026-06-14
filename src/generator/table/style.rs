//! Shared table styling presets and builders

use super::builder::Table;
use super::cell::TableCell;
use super::row::TableRow;
use super::TableBuilder;

/// Header background used by HTML/Markdown import tables.
pub const IMPORT_HEADER_BG: &str = "4472C4";
/// Header background used by helper table presets.
pub const HELPER_HEADER_BG: &str = "1F4E79";
/// Standard header text color.
pub const HEADER_TEXT: &str = "FFFFFF";

/// Default slide position for imported tables (EMU).
pub const DEFAULT_TABLE_X: u32 = 500_000;
pub const DEFAULT_TABLE_Y: u32 = 1_800_000;
/// Default total table width for imported tables (EMU).
pub const DEFAULT_TABLE_WIDTH: u32 = 8_000_000;

/// Styled header cell preset.
pub fn header_cell(text: &str) -> TableCell {
    TableCell::new(text)
        .bold()
        .background_color(IMPORT_HEADER_BG)
        .text_color(HEADER_TEXT)
}

/// Build a positioned table from string rows with optional header styling.
pub fn table_from_string_rows(rows: Vec<Vec<String>>, style_header: bool) -> Table {
    let col_count = rows.iter().map(|row| row.len()).max().unwrap_or(1);
    let col_width = DEFAULT_TABLE_WIDTH / col_count as u32;
    let col_widths = vec![col_width; col_count];

    let mut builder = TableBuilder::new(col_widths);

    for (row_index, row_data) in rows.iter().enumerate() {
        let cells: Vec<TableCell> = row_data
            .iter()
            .map(|text| {
                if style_header && row_index == 0 {
                    header_cell(text)
                } else {
                    TableCell::new(text)
                }
            })
            .collect();

        let mut cells = cells;
        while cells.len() < col_count {
            cells.push(TableCell::new(""));
        }

        builder = builder.add_row(TableRow::new(cells));
    }

    builder
        .position(DEFAULT_TABLE_X, DEFAULT_TABLE_Y)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_cell_preset() {
        let cell = header_cell("Name");
        assert!(cell.bold);
        assert_eq!(cell.background_color.as_deref(), Some(IMPORT_HEADER_BG));
        assert_eq!(cell.text_color.as_deref(), Some(HEADER_TEXT));
    }

    #[test]
    fn test_table_from_string_rows_styles_first_row() {
        let table = table_from_string_rows(
            vec![vec!["A".into(), "B".into()], vec!["1".into(), "2".into()]],
            true,
        );
        assert_eq!(table.rows.len(), 2);
        assert!(table.rows[0].cells[0].bold);
        assert!(!table.rows[1].cells[0].bold);
    }
}
