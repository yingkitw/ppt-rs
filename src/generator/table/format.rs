//! Shared table cell formatting and XML generation

use super::cell::{CellAlign, CellVAlign, TableCell};
use crate::core::escape_xml;
use crate::generator::slide_content::table_merge::CellMergeState;
use crate::generator::text::{color_to_xml, TextFormat};

impl TableCell {
    /// Convert cell formatting to the shared text format model.
    pub fn to_text_format(&self) -> TextFormat {
        let mut format = TextFormat::new();
        if self.bold {
            format = format.bold();
        }
        if self.italic {
            format = format.italic();
        }
        if self.underline {
            format = format.underline();
        }
        if let Some(ref color) = self.text_color {
            format = format.color(color);
        }
        if let Some(size) = self.font_size {
            format = format.font_size(size);
        }
        if let Some(ref family) = self.font_family {
            format = format.font_family(family);
        }
        format
    }
}

/// Merge attributes for `<a:tc>` from cell state.
pub fn merge_attrs_from_cell(cell: &TableCell) -> String {
    if cell.h_merge {
        return CellMergeState::HMerge.to_xml_attrs();
    }
    if cell.v_merge {
        return CellMergeState::VMerge.to_xml_attrs();
    }

    let col_span = cell.grid_span.unwrap_or(1) as usize;
    let row_span = cell.row_span.unwrap_or(1) as usize;
    CellMergeState::Anchor { row_span, col_span }.to_xml_attrs()
}

/// Generate `<a:rPr>` for a table cell text run.
pub fn run_properties_xml(cell: &TableCell) -> String {
    let format = cell.to_text_format();
    let attrs = format.to_xml_attrs();
    let color_xml = cell
        .text_color
        .as_ref()
        .map(|color| color_to_xml(color))
        .unwrap_or_default();
    let font_xml = cell
        .font_family
        .as_ref()
        .map(|font| format!(r#"<a:latin typeface="{}"/>"#, escape_xml(font)))
        .unwrap_or_default();

    if color_xml.is_empty() && font_xml.is_empty() {
        format!(r#"<a:rPr lang="en-US" dirty="0"{attrs}/>"#)
    } else {
        format!(
            r#"<a:rPr lang="en-US" dirty="0"{attrs}>{color_xml}{font_xml}</a:rPr>"#
        )
    }
}

/// Paragraph alignment attributes for `<a:p>`.
pub fn paragraph_props_xml(cell: &TableCell) -> String {
    if cell.align == CellAlign::Center {
        String::new()
    } else {
        format!(r#" algn="{}""#, cell.align.as_str())
    }
}

/// Body properties for `<a:bodyPr>` inside table cells.
pub fn body_props_xml(cell: &TableCell) -> String {
    if cell.wrap_text {
        r#"<a:bodyPr wrap="square"/>"#.to_string()
    } else {
        r#"<a:bodyPr wrap="none"/>"#.to_string()
    }
}

/// Cell properties (`<a:tcPr>`) including background and vertical alignment.
pub fn tc_properties_xml(cell: &TableCell) -> String {
    let anchor = if cell.valign == CellVAlign::Middle {
        String::new()
    } else {
        format!(r#" anchor="{}""#, cell.valign.as_str())
    };

    if let Some(ref background) = cell.background_color {
        format!(
            r#"<a:tcPr{anchor}>{}</a:tcPr>"#,
            color_to_xml(background)
        )
    } else if anchor.is_empty() {
        "<a:tcPr/>".to_string()
    } else {
        format!(r#"<a:tcPr{anchor}/>"#)
    }
}

/// Generate cell XML with formatting.
/// Based on reference PPTX structure: `txBody` comes before `tcPr`.
pub fn generate_cell_xml(cell: &TableCell) -> String {
    let merge_attrs = merge_attrs_from_cell(cell);

    if cell.h_merge || cell.v_merge {
        return format!(
            r#"<a:tc{merge_attrs}><a:txBody><a:bodyPr/><a:lstStyle/><a:p/></a:txBody><a:tcPr/></a:tc>"#
        );
    }

    let body_pr = body_props_xml(cell);
    let paragraph_props = paragraph_props_xml(cell);
    let run_props = run_properties_xml(cell);
    let text = escape_xml(&cell.text);
    let tc_pr = tc_properties_xml(cell);

    format!(
        r#"<a:tc{merge_attrs}><a:txBody>{body_pr}<a:lstStyle/><a:p{paragraph_props}><a:r>{run_props}<a:t>{text}</a:t></a:r></a:p></a:txBody>{tc_pr}</a:tc>"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_properties_bold() {
        let cell = TableCell::new("Bold").bold();
        let xml = run_properties_xml(&cell);
        assert!(xml.contains(r#"b="1""#));
    }

    #[test]
    fn test_run_properties_uses_color_helper() {
        let cell = TableCell::new("Red").text_color("FF0000");
        let xml = run_properties_xml(&cell);
        assert!(xml.contains("FF0000"));
        assert!(xml.contains("solidFill"));
    }

    #[test]
    fn test_tc_properties_background_and_valign() {
        let cell = TableCell::new("Top").background_color("0000FF").valign_top();
        let xml = tc_properties_xml(&cell);
        assert!(xml.contains("0000FF"));
        assert!(xml.contains(r#"anchor="t""#));
    }

    #[test]
    fn test_generate_cell_alignment() {
        let cell = TableCell::new("Left").align_left();
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains(r#"algn="l""#));
    }

    #[test]
    fn test_generate_cell_wrap_disabled() {
        let cell = TableCell::new("No wrap").wrap(false);
        let xml = generate_cell_xml(&cell);
        assert!(xml.contains(r#"wrap="none""#));
    }

    #[test]
    fn test_merge_attrs_uses_cell_merge_state() {
        let cell = TableCell::new("").h_merge();
        assert!(merge_attrs_from_cell(&cell).contains("hMerge"));
    }
}
