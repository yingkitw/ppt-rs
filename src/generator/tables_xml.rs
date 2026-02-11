//! Table XML generation for PPTX presentations
//!
//! Generates proper PPTX XML for tables with cells, rows, and formatting

use crate::generator::tables::{Table, TableRow, TableCell};

/// Generate table XML for a slide
pub fn generate_table_xml(table: &Table, shape_id: usize) -> String {
    let x = table.x;
    let y = table.y;
    let width = table.width();
    let height = table.height();
    let mut xml = format!(
        r#"<p:graphicFrame>
<p:nvGraphicFramePr>
<p:cNvPr id="{shape_id}" name="Table {shape_id}"/>
<p:cNvGraphicFramePr/>
<p:nvPr/>
</p:nvGraphicFramePr>
<p:xfrm>
<a:off x="{x}" y="{y}"/>
<a:ext cx="{width}" cy="{height}"/>
</p:xfrm>
<a:graphic>
<a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/table">
<a:tbl>
<a:tblPr firstRow="1" bandHVals="1"/>
<a:tblGrid>"#
    );

    // Add column widths
    for width in &table.column_widths {
        xml.push_str(&format!(r#"<a:gridCol w="{width}"/>"#));
    }

    xml.push_str("</a:tblGrid>");

    // Add rows
    for row in &table.rows {
        xml.push_str(&generate_row_xml(row));
    }

    xml.push_str(
        r#"</a:tbl>
</a:graphicData>
</a:graphic>
</p:graphicFrame>"#
    );

    xml
}

/// Generate row XML
fn generate_row_xml(row: &TableRow) -> String {
    let height = row.height.unwrap_or(400000);
    
    let mut xml = format!(r#"<a:tr h="{height}">"#);

    for cell in &row.cells {
        xml.push_str(&generate_cell_xml(cell));
    }

    xml.push_str("</a:tr>");
    xml
}

/// Generate cell XML with formatting
/// Based on reference PPTX structure: txBody comes BEFORE tcPr
fn generate_cell_xml(cell: &TableCell) -> String {
    let mut xml = String::from("<a:tc");
    
    // Add merge attributes
    if cell.col_span > 1 {
        xml.push_str(&format!(r#" gridSpan="{}""#, cell.col_span));
    }
    if cell.row_span > 1 {
        xml.push_str(&format!(r#" rowSpan="{}""#, cell.row_span));
    }
    if cell.h_merge {
        xml.push_str(r#" hMerge="1""#);
    }
    if cell.v_merge {
        xml.push_str(r#" vMerge="1""#);
    }
    
    xml.push_str(">");

    // Merged-over cells (hMerge/vMerge) must have empty text body
    if cell.h_merge || cell.v_merge {
        xml.push_str(r#"<a:txBody><a:bodyPr/><a:lstStyle/><a:p/></a:txBody><a:tcPr/>"#);
        xml.push_str("</a:tc>");
        return xml;
    }

    // === TEXT BODY (must come first!) ===
    xml.push_str(r#"<a:txBody><a:bodyPr/><a:lstStyle/><a:p>"#);
    
    // Text run with simple properties (like reference PPTX)
    xml.push_str("<a:r>");
    
    // Run properties - keep it simple like the reference
    xml.push_str(r#"<a:rPr lang="en-US" dirty="0""#);
    
    // Add optional formatting attributes
    if cell.bold {
        xml.push_str(r#" b="1""#);
    }
    if cell.italic {
        xml.push_str(r#" i="1""#);
    }
    if cell.underline {
        xml.push_str(r#" u="sng""#);
    }
    if let Some(size) = cell.font_size {
        xml.push_str(&format!(r#" sz="{}""#, size * 100));
    }
    
    // Check if we need child elements
    let has_color = cell.text_color.is_some();
    let has_font = cell.font_family.is_some();
    
    if has_color || has_font {
        xml.push_str(">");
        if let Some(ref color) = cell.text_color {
            xml.push_str(&format!(r#"<a:solidFill><a:srgbClr val="{color}"/></a:solidFill>"#));
        }
        if let Some(ref font) = cell.font_family {
            xml.push_str(&format!(r#"<a:latin typeface="{font}"/>"#));
        }
        xml.push_str("</a:rPr>");
    } else {
        xml.push_str("/>");
    }
    
    // Text content
    let text = escape_xml(&cell.text);
    xml.push_str(&format!(r#"<a:t>{text}</a:t>"#));
    
    xml.push_str("</a:r></a:p></a:txBody>");

    // === CELL PROPERTIES (comes after txBody) ===
    if cell.background_color.is_some() {
        let color = cell.background_color.as_ref().unwrap();
        xml.push_str(&format!(
            r#"<a:tcPr><a:solidFill><a:srgbClr val="{color}"/></a:solidFill></a:tcPr>"#
        ));
    } else {
        xml.push_str("<a:tcPr/>");
    }

    xml.push_str("</a:tc>");
    xml
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }

    #[test]
    fn test_generate_cell_with_bold() {
        let cell = TableCell::new("Bold").bold();
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains(r#"b="1""#));
    }

    #[test]
    fn test_generate_cell_with_background_color() {
        let cell = TableCell::new("Colored").background_color("FF0000");
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains("FF0000"));
    }

    #[test]
    fn test_generate_cell_with_italic() {
        let cell = TableCell::new("Italic").italic();
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains(r#"i="1""#));
    }

    #[test]
    fn test_generate_cell_with_underline() {
        let cell = TableCell::new("Underline").underline();
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains(r#"u="sng""#));
    }

    #[test]
    fn test_generate_cell_with_text_color() {
        let cell = TableCell::new("Red Text").text_color("FF0000");
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains("FF0000"));
        assert!(xml.contains("srgbClr"));
    }

    #[test]
    fn test_generate_cell_with_font_size() {
        let cell = TableCell::new("Large").font_size(24);
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains("sz=\"2400\""));
    }

    #[test]
    fn test_generate_cell_with_font_family() {
        let cell = TableCell::new("Arial").font_family("Arial");
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains("typeface=\"Arial\""));
        assert!(xml.contains("latin"));
    }

    #[test]
    fn test_generate_cell_with_all_formatting() {
        let cell = TableCell::new("Styled")
            .bold()
            .italic()
            .underline()
            .text_color("0000FF")
            .background_color("FFFF00")
            .font_size(18)
            .font_family("Calibri");
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains(r#"b="1""#));
        assert!(xml.contains(r#"i="1""#));
        assert!(xml.contains(r#"u="sng""#));
        assert!(xml.contains("0000FF")); // text color
        assert!(xml.contains("FFFF00")); // background color
        assert!(xml.contains("sz=\"1800\""));
        assert!(xml.contains("typeface=\"Calibri\""));
    }

    #[test]
    fn test_escape_xml_in_cell() {
        let cell = TableCell::new("Test & <Data>");
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains("&amp;"));
        assert!(xml.contains("&lt;"));
        assert!(xml.contains("&gt;"));
    }

    #[test]
    fn test_generate_cell_with_multiline() {
        let cell = TableCell::new("Line 1\nLine 2\nLine 3");
        let xml = generate_cell_xml(&cell);
        // Text content should be preserved (newlines escaped or kept)
        assert!(xml.contains("Line 1"));
        // Structure should be valid
        assert!(xml.contains("<a:txBody>"));
        assert!(xml.contains("</a:txBody>"));
    }

    #[test]
    fn test_txbody_before_tcpr() {
        // Verify txBody comes before tcPr (critical for PowerPoint)
        let cell = TableCell::new("Test").background_color("FF0000");
        let xml = generate_cell_xml(&cell);
        let txbody_pos = xml.find("<a:txBody>").unwrap();
        let tcpr_pos = xml.find("<a:tcPr>").unwrap();
        assert!(txbody_pos < tcpr_pos, "txBody must come before tcPr");
    }
}
