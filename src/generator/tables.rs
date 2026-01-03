//! Table creation support for PPTX generation

/// Horizontal text alignment
#[derive(Clone, Debug, Default, PartialEq)]
pub enum CellAlign {
    Left,
    #[default]
    Center,
    Right,
    Justify,
}

impl CellAlign {
    /// Get the OOXML alignment value
    pub fn as_str(&self) -> &'static str {
        match self {
            CellAlign::Left => "l",
            CellAlign::Center => "ctr",
            CellAlign::Right => "r",
            CellAlign::Justify => "just",
        }
    }
}

/// Vertical text alignment
#[derive(Clone, Debug, Default, PartialEq)]
pub enum CellVAlign {
    Top,
    #[default]
    Middle,
    Bottom,
}

impl CellVAlign {
    /// Get the OOXML anchor value
    pub fn as_str(&self) -> &'static str {
        match self {
            CellVAlign::Top => "t",
            CellVAlign::Middle => "ctr",
            CellVAlign::Bottom => "b",
        }
    }
}

/// Table cell content
#[derive(Clone, Debug)]
pub struct TableCell {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub text_color: Option<String>,      // RGB hex color for text
    pub background_color: Option<String>, // RGB hex color for background
    pub font_size: Option<u32>,          // Font size in points
    pub font_family: Option<String>,     // Font family name
    pub align: CellAlign,                // Horizontal alignment
    pub valign: CellVAlign,              // Vertical alignment
    pub wrap_text: bool,                 // Text wrapping
    pub row_span: u32,
    pub col_span: u32,
    pub v_merge: bool,
    pub h_merge: bool,
}

impl TableCell {
    /// Create a new table cell
    pub fn new(text: &str) -> Self {
        TableCell {
            text: text.to_string(),
            bold: false,
            italic: false,
            underline: false,
            text_color: None,
            background_color: None,
            font_size: None,
            font_family: None,
            align: CellAlign::Center,
            valign: CellVAlign::Middle,
            wrap_text: true,
            row_span: 1,
            col_span: 1,
            v_merge: false,
            h_merge: false,
        }
    }

    /// Set cell text as bold
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set cell text as italic
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Set cell text as underlined
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }


    /// Set cell text color (RGB hex format, e.g., "FF0000" or "#FF0000")
    pub fn text_color(mut self, color: &str) -> Self {
        self.text_color = Some(color.trim_start_matches('#').to_uppercase());
        self
    }

    /// Set cell background color (RGB hex format, e.g., "FF0000" or "#FF0000")
    pub fn background_color(mut self, color: &str) -> Self {
        self.background_color = Some(color.trim_start_matches('#').to_uppercase());
        self
    }

    /// Set font size in points
    pub fn font_size(mut self, size: u32) -> Self {
        self.font_size = Some(size);
        self
    }

    /// Set font family name
    pub fn font_family(mut self, family: &str) -> Self {
        self.font_family = Some(family.to_string());
        self
    }

    /// Set horizontal text alignment
    pub fn align(mut self, align: CellAlign) -> Self {
        self.align = align;
        self
    }

    /// Set horizontal text alignment to left
    pub fn align_left(mut self) -> Self {
        self.align = CellAlign::Left;
        self
    }

    /// Set horizontal text alignment to right
    pub fn align_right(mut self) -> Self {
        self.align = CellAlign::Right;
        self
    }

    /// Set horizontal text alignment to center
    pub fn align_center(mut self) -> Self {
        self.align = CellAlign::Center;
        self
    }

    /// Set vertical text alignment
    pub fn valign(mut self, valign: CellVAlign) -> Self {
        self.valign = valign;
        self
    }

    /// Set vertical text alignment to top
    pub fn valign_top(mut self) -> Self {
        self.valign = CellVAlign::Top;
        self
    }

    /// Set vertical text alignment to bottom
    pub fn valign_bottom(mut self) -> Self {
        self.valign = CellVAlign::Bottom;
        self
    }

    /// Enable or disable text wrapping
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap_text = wrap;
        self
    }

    /// Set row span (number of rows to merge down)
    pub fn with_row_span(mut self, span: u32) -> Self {
        self.row_span = span;
        self
    }

    /// Set column span (number of columns to merge right)
    pub fn with_col_span(mut self, span: u32) -> Self {
        self.col_span = span;
        self
    }

    /// Set vertical merge flag (for cells covered by row span)
    pub fn with_v_merge(mut self) -> Self {
        self.v_merge = true;
        self
    }

    /// Set horizontal merge flag (for cells covered by col span)
    pub fn with_h_merge(mut self) -> Self {
        self.h_merge = true;
        self
    }
}

/// Table row
#[derive(Clone, Debug)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
    pub height: Option<u32>, // in EMU
}

impl TableRow {
    /// Create a new table row
    pub fn new(cells: Vec<TableCell>) -> Self {
        TableRow {
            cells,
            height: None,
        }
    }

    /// Set row height
    pub fn with_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }
}

/// Table definition
#[derive(Clone, Debug)]
pub struct Table {
    pub rows: Vec<TableRow>,
    pub column_widths: Vec<u32>, // in EMU
    pub x: u32,                  // Position X in EMU
    pub y: u32,                  // Position Y in EMU
}

impl Table {
    /// Create a new table
    pub fn new(rows: Vec<TableRow>, column_widths: Vec<u32>, x: u32, y: u32) -> Self {
        Table {
            rows,
            column_widths,
            x,
            y,
        }
    }

    /// Get number of columns
    pub fn column_count(&self) -> usize {
        self.column_widths.len()
    }

    /// Get number of rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get total table width (sum of column widths)
    pub fn width(&self) -> u32 {
        self.column_widths.iter().sum()
    }

    /// Get total table height (sum of row heights)
    pub fn height(&self) -> u32 {
        self.rows
            .iter()
            .map(|r| r.height.unwrap_or(400000))
            .sum()
    }

    /// Create a simple table from 2D data
    pub fn from_data(data: Vec<Vec<&str>>, column_widths: Vec<u32>, x: u32, y: u32) -> Self {
        let rows = data
            .into_iter()
            .map(|row| {
                let cells = row
                    .into_iter()
                    .map(|cell| TableCell::new(cell))
                    .collect();
                TableRow::new(cells)
            })
            .collect();

        Table {
            rows,
            column_widths,
            x,
            y,
        }
    }
}

/// Table builder for fluent API
pub struct TableBuilder {
    rows: Vec<TableRow>,
    column_widths: Vec<u32>,
    x: u32,
    y: u32,
}

impl TableBuilder {
    /// Create a new table builder
    pub fn new(column_widths: Vec<u32>) -> Self {
        TableBuilder {
            rows: Vec::new(),
            column_widths,
            x: 0,
            y: 0,
        }
    }

    /// Set table position
    pub fn position(mut self, x: u32, y: u32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Add a row
    pub fn add_row(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }

    /// Add a simple row from strings
    pub fn add_simple_row(mut self, cells: Vec<&str>) -> Self {
        let row = TableRow::new(
            cells
                .into_iter()
                .map(|c| TableCell::new(c))
                .collect(),
        );
        self.rows.push(row);
        self
    }

    /// Build the table
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

    #[test]
    fn test_table_cell_builder() {
        let cell = TableCell::new("Header")
            .bold()
            .italic()
            .underline()
            .text_color("FF0000")
            .background_color("0000FF")
            .font_size(24)
            .font_family("Arial");
        assert_eq!(cell.text, "Header");
        assert!(cell.bold);
        assert!(cell.italic);
        assert!(cell.underline);
        assert_eq!(cell.text_color, Some("FF0000".to_string()));
        assert_eq!(cell.background_color, Some("0000FF".to_string()));
        assert_eq!(cell.font_size, Some(24));
        assert_eq!(cell.font_family, Some("Arial".to_string()));
    }

    #[test]
    fn test_table_row() {
        let cells = vec![TableCell::new("A"), TableCell::new("B")];
        let row = TableRow::new(cells).with_height(500000);
        assert_eq!(row.cells.len(), 2);
        assert_eq!(row.height, Some(500000));
    }

    #[test]
    fn test_table_from_data() {
        let data = vec![
            vec!["Name", "Age"],
            vec!["Alice", "30"],
            vec!["Bob", "25"],
        ];
        let table = Table::from_data(data, vec![1000000, 1000000], 0, 0);
        assert_eq!(table.row_count(), 3);
        assert_eq!(table.column_count(), 2);
    }

    #[test]
    fn test_table_builder() {
        let table = TableBuilder::new(vec![1000000, 1000000])
            .position(100000, 200000)
            .add_simple_row(vec!["Header 1", "Header 2"])
            .add_simple_row(vec!["Cell 1", "Cell 2"])
            .build();

        assert_eq!(table.row_count(), 2);
        assert_eq!(table.column_count(), 2);
        assert_eq!(table.x, 100000);
        assert_eq!(table.y, 200000);
    }
}
