//! Table XML elements for OOXML
//!
//! Provides types for parsing and generating DrawingML table elements.

use super::xmlchemy::XmlElement;
use crate::util::format_lang_attributes;

/// Table cell properties
#[derive(Debug, Clone, Default)]
pub struct TableCellProperties {
    pub margin_left: Option<u32>,
    pub margin_right: Option<u32>,
    pub margin_top: Option<u32>,
    pub margin_bottom: Option<u32>,
    pub anchor: Option<String>,
}

impl TableCellProperties {
    pub fn parse(elem: &XmlElement) -> Self {
        TableCellProperties {
            margin_left: elem.attr("marL").and_then(|v| v.parse().ok()),
            margin_right: elem.attr("marR").and_then(|v| v.parse().ok()),
            margin_top: elem.attr("marT").and_then(|v| v.parse().ok()),
            margin_bottom: elem.attr("marB").and_then(|v| v.parse().ok()),
            anchor: elem.attr("anchor").map(|s| s.to_string()),
        }
    }

    pub fn to_xml(&self) -> String {
        let mut attrs = Vec::new();
        
        if let Some(l) = self.margin_left {
            attrs.push(format!(r#"marL="{l}""#));
        }
        if let Some(r) = self.margin_right {
            attrs.push(format!(r#"marR="{r}""#));
        }
        if let Some(t) = self.margin_top {
            attrs.push(format!(r#"marT="{t}""#));
        }
        if let Some(b) = self.margin_bottom {
            attrs.push(format!(r#"marB="{b}""#));
        }
        if let Some(ref anchor) = self.anchor {
            attrs.push(format!(r#"anchor="{anchor}""#));
        }

        if attrs.is_empty() {
            "<a:tcPr/>".to_string()
        } else {
            format!("<a:tcPr {}/>", attrs.join(" "))
        }
    }
}

/// Table cell (a:tc)
#[derive(Debug, Clone)]
pub struct TableCell {
    pub text: String,
    pub row_span: u32,
    pub col_span: u32,
    pub properties: TableCellProperties,
}

impl TableCell {
    pub fn new(text: &str) -> Self {
        TableCell {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            properties: TableCellProperties::default(),
        }
    }

    pub fn parse(elem: &XmlElement) -> Self {
        let text = elem.find_descendant("t")
            .map(|t| t.text_content())
            .unwrap_or_default();

        let row_span = elem.attr("rowSpan").and_then(|v| v.parse().ok()).unwrap_or(1);
        let col_span = elem.attr("gridSpan").and_then(|v| v.parse().ok()).unwrap_or(1);

        let properties = elem.find("tcPr")
            .map(|tc| TableCellProperties::parse(tc))
            .unwrap_or_default();

        TableCell { text, row_span, col_span, properties }
    }

    pub fn to_xml(&self) -> String {
        let mut attrs = Vec::new();
        if self.row_span > 1 {
            let row_span = self.row_span;
            attrs.push(format!(r#"rowSpan="{row_span}""#));
        }
        if self.col_span > 1 {
            let col_span = self.col_span;
            attrs.push(format!(r#"gridSpan="{col_span}""#));
        }

        let attr_str = if attrs.is_empty() { String::new() } else { format!(" {}", attrs.join(" ")) };
        let lang_attrs = format_lang_attributes();

        format!(
            r#"<a:tc{}><a:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr {} dirty="0"/><a:t>{}</a:t></a:r></a:p></a:txBody>{}</a:tc>"#,
            attr_str,
            lang_attrs,
            escape_xml(&self.text),
            self.properties.to_xml()
        )
    }
}

/// Table row (a:tr)
#[derive(Debug, Clone)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
    pub height: u32,
}

impl TableRow {
    pub fn new() -> Self {
        TableRow {
            cells: Vec::new(),
            height: 370840, // Default row height
        }
    }

    pub fn parse(elem: &XmlElement) -> Self {
        let height = elem.attr("h").and_then(|v| v.parse().ok()).unwrap_or(370840);
        let cells = elem.find_all("tc")
            .into_iter()
            .map(|tc| TableCell::parse(tc))
            .collect();

        TableRow { cells, height }
    }

    pub fn to_xml(&self) -> String {
        let height = self.height;
        let mut xml = format!(r#"<a:tr h="{height}">"#);
        for cell in &self.cells {
            xml.push_str(&cell.to_xml());
        }
        xml.push_str("</a:tr>");
        xml
    }

    pub fn add_cell(mut self, cell: TableCell) -> Self {
        self.cells.push(cell);
        self
    }
}

impl Default for TableRow {
    fn default() -> Self {
        Self::new()
    }
}

/// Table grid column (a:gridCol)
#[derive(Debug, Clone)]
pub struct GridColumn {
    pub width: u32,
}

impl GridColumn {
    pub fn new(width: u32) -> Self {
        GridColumn { width }
    }

    pub fn to_xml(&self) -> String {
        let width = self.width;
        format!(r#"<a:gridCol w="{width}"/>"#)
    }
}

/// Table (a:tbl)
#[derive(Debug, Clone)]
pub struct Table {
    pub rows: Vec<TableRow>,
    pub grid_columns: Vec<GridColumn>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            rows: Vec::new(),
            grid_columns: Vec::new(),
        }
    }

    pub fn parse(elem: &XmlElement) -> Self {
        let mut table = Table::new();

        // Parse grid columns
        if let Some(tbl_grid) = elem.find("tblGrid") {
            table.grid_columns = tbl_grid.find_all("gridCol")
                .into_iter()
                .map(|gc| {
                    let width = gc.attr("w").and_then(|v| v.parse().ok()).unwrap_or(914400);
                    GridColumn::new(width)
                })
                .collect();
        }

        // Parse rows
        table.rows = elem.find_all("tr")
            .into_iter()
            .map(|tr| TableRow::parse(tr))
            .collect();

        table
    }

    pub fn to_xml(&self) -> String {
        let mut xml = String::from("<a:tbl>");
        
        // Table properties
        xml.push_str("<a:tblPr firstRow=\"1\" bandRow=\"1\"/>");
        
        // Grid
        xml.push_str("<a:tblGrid>");
        for col in &self.grid_columns {
            xml.push_str(&col.to_xml());
        }
        xml.push_str("</a:tblGrid>");
        
        // Rows
        for row in &self.rows {
            xml.push_str(&row.to_xml());
        }
        
        xml.push_str("</a:tbl>");
        xml
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn col_count(&self) -> usize {
        self.grid_columns.len()
    }

    pub fn add_row(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }

    pub fn add_column(mut self, width: u32) -> Self {
        self.grid_columns.push(GridColumn::new(width));
        self
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_cell_to_xml() {
        let cell = TableCell::new("Hello");
        let xml = cell.to_xml();
        
        assert!(xml.contains("<a:tc>"));
        assert!(xml.contains("Hello"));
    }

    #[test]
    fn test_table_row_to_xml() {
        let row = TableRow::new()
            .add_cell(TableCell::new("A"))
            .add_cell(TableCell::new("B"));
        
        let xml = row.to_xml();
        assert!(xml.contains("<a:tr"));
        assert!(xml.contains("A"));
        assert!(xml.contains("B"));
    }

    #[test]
    fn test_table_to_xml() {
        let table = Table::new()
            .add_column(914400)
            .add_column(914400)
            .add_row(TableRow::new()
                .add_cell(TableCell::new("A1"))
                .add_cell(TableCell::new("B1")));
        
        let xml = table.to_xml();
        assert!(xml.contains("<a:tbl>"));
        assert!(xml.contains("<a:tblGrid>"));
        assert!(xml.contains("A1"));
    }
}
