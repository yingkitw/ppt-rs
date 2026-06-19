//! XML generation for tables in PPTX format

use super::builder::Table;
use super::format::generate_cell_xml;
use super::row::TableRow;
use crate::core::XmlWriter;

/// Generate table XML for a slide
pub fn generate_table_xml(table: &Table, shape_id: usize) -> String {
    let x = table.x.to_string();
    let y = table.y.to_string();
    let width = table.width().to_string();
    let height = table.height().to_string();
    let shape_id_str = shape_id.to_string();
    let table_name = format!("Table {shape_id}");

    let mut writer = XmlWriter::with_capacity(4096);
    writer.start_element(
        "p:graphicFrame",
        &[],
    );
    writer.raw("<p:nvGraphicFramePr>");
    writer.empty_element(
        "p:cNvPr",
        &[("id", &shape_id_str), ("name", &table_name)],
    );
    writer.raw("<p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>");
    writer.raw("<p:xfrm>");
    writer.empty_element("a:off", &[("x", &x), ("y", &y)]);
    writer.empty_element("a:ext", &[("cx", &width), ("cy", &height)]);
    writer.raw("</p:xfrm><a:graphic><a:graphicData uri=\"http://schemas.openxmlformats.org/drawingml/2006/table\"><a:tbl><a:tblPr firstRow=\"1\" bandRow=\"1\"/><a:tblGrid>");

    for (col_idx, col_width) in table.column_widths.iter().enumerate() {
        let col_id = 20_000 + col_idx;
        writer.raw(&format!(
            r#"<a:gridCol w="{col_width}"><a:extLst><a:ext uri="{{9D8B030D-6E8A-4147-A177-3AD203B41FA5}}"><a16:colId xmlns:a16="http://schemas.microsoft.com/office/drawing/2014/main" val="{col_id}"/></a:ext></a:extLst></a:gridCol>"#
        ));
    }

    writer.raw("</a:tblGrid>");

    for (row_idx, row) in table.rows.iter().enumerate() {
        writer.raw(&generate_row_xml(row, row_idx));
    }

    writer.raw("</a:tbl></a:graphicData></a:graphic>");
    writer.end_element("p:graphicFrame");
    writer.finish()
}

/// Generate row XML
fn generate_row_xml(row: &TableRow, row_idx: usize) -> String {
    let height = row.height.unwrap_or(400000).to_string();
    let row_id = 10_000 + row_idx;
    let mut writer = XmlWriter::with_capacity(512);
    writer.start_element("a:tr", &[("h", &height)]);

    for cell in &row.cells {
        writer.raw(&generate_cell_xml(cell));
    }

    writer.raw(&format!(
        r#"<a:extLst><a:ext uri="{{0D108BD9-81ED-4DB2-BD59-A6C34878D82A}}"><a16:rowId xmlns:a16="http://schemas.microsoft.com/office/drawing/2014/main" val="{row_id}"/></a:ext></a:extLst>"#
    ));
    writer.end_element("a:tr");
    writer.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::table::TableCell;

    #[test]
    fn test_generate_simple_table_xml() {
        let table = Table::from_data(
            vec![vec!["A", "B"], vec!["1", "2"]],
            vec![1000000, 1000000],
            0,
            0,
        );

        let xml = generate_table_xml(&table, 1);
        assert!(xml.contains("a:tbl"));
        assert!(xml.contains("a:tr"));
        assert!(xml.contains("a:tc"));
        assert!(xml.contains("a16:colId"));
        assert!(xml.contains("a16:rowId"));
        assert!(xml.contains("<a:t>A</a:t>"));
        assert!(xml.contains("<a:t>B</a:t>"));
    }

    #[test]
    fn test_generate_cell_with_bold() {
        let cell = TableCell::new("Bold").bold();
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains(r#"b="1""#));
    }

    #[test]
    fn test_generate_cell_with_text_color() {
        let cell = TableCell::new("Red").text_color("FF0000");
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains("FF0000"));
    }

    #[test]
    fn test_generate_cell_with_background_color() {
        let cell = TableCell::new("Blue BG").background_color("0000FF");
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains("0000FF"));
    }

    #[test]
    fn test_escape_xml() {
        let cell = TableCell::new("Test & <Data>");
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains("&amp;"));
        assert!(xml.contains("&lt;"));
        assert!(xml.contains("&gt;"));
    }

    #[test]
    fn test_txbody_before_tcpr() {
        let cell = TableCell::new("Test").background_color("FF0000");
        let xml = generate_cell_xml(&cell);
        let txbody_pos = xml.find("<a:txBody>").unwrap();
        let tcpr_pos = xml.find("<a:tcPr").unwrap();
        assert!(txbody_pos < tcpr_pos, "txBody must come before tcPr");
    }

    #[test]
    fn test_font_included_when_specified() {
        let cell = TableCell::new("Test").font_family("Arial");
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains(r#"<a:latin typeface="Arial"/>"#));
    }
}
