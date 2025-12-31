//! Table part
//!
//! Represents table data embedded in slides with advanced formatting.
//!
//! # Features
//! - Cell merging (row span, column span)
//! - Text formatting (bold, italic, underline, strikethrough)
//! - Cell alignment (horizontal and vertical)
//! - Borders (all sides, individual sides)
//! - Background colors and gradients
//! - Font customization (size, color, family)
//! - Table styles

use super::base::{Part, PartType, ContentType};
use crate::exc::PptxError;
use crate::core::escape_xml;
use crate::util::format_lang_attributes;

/// Horizontal alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HorizontalAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

impl HorizontalAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            HorizontalAlign::Left => "l",
            HorizontalAlign::Center => "ctr",
            HorizontalAlign::Right => "r",
            HorizontalAlign::Justify => "just",
        }
    }
}

/// Vertical alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerticalAlign {
    Top,
    #[default]
    Middle,
    Bottom,
}

impl VerticalAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            VerticalAlign::Top => "t",
            VerticalAlign::Middle => "ctr",
            VerticalAlign::Bottom => "b",
        }
    }
}

/// Border style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
    Double,
    None,
}

impl BorderStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            BorderStyle::Solid => "solid",
            BorderStyle::Dashed => "dash",
            BorderStyle::Dotted => "dot",
            BorderStyle::Double => "dbl",
            BorderStyle::None => "none",
        }
    }
}

/// Cell border
#[derive(Debug, Clone, Default)]
pub struct CellBorder {
    pub width: i32,        // in EMU (12700 = 1pt)
    pub color: String,
    pub style: BorderStyle,
}

impl CellBorder {
    pub fn new(width_pt: f32, color: impl Into<String>) -> Self {
        CellBorder {
            width: (width_pt * 12700.0) as i32,
            color: color.into(),
            style: BorderStyle::Solid,
        }
    }

    pub fn style(mut self, style: BorderStyle) -> Self {
        self.style = style;
        self
    }

    pub fn to_xml(&self, tag: &str) -> String {
        if self.style == BorderStyle::None {
            return format!("<a:{}/>\n", tag);
        }
        format!(
            r#"<a:{} w="{}" cap="flat" cmpd="sng" algn="ctr">
              <a:solidFill><a:srgbClr val="{}"/></a:solidFill>
              <a:prstDash val="{}"/>
            </a:{}>"#,
            tag,
            self.width,
            self.color.trim_start_matches('#'),
            self.style.as_str(),
            tag
        )
    }
}

/// Cell borders (all four sides)
#[derive(Debug, Clone, Default)]
pub struct CellBorders {
    pub left: Option<CellBorder>,
    pub right: Option<CellBorder>,
    pub top: Option<CellBorder>,
    pub bottom: Option<CellBorder>,
}

impl CellBorders {
    pub fn all(border: CellBorder) -> Self {
        CellBorders {
            left: Some(border.clone()),
            right: Some(border.clone()),
            top: Some(border.clone()),
            bottom: Some(border),
        }
    }

    pub fn none() -> Self {
        let no_border = CellBorder { width: 0, color: String::new(), style: BorderStyle::None };
        CellBorders {
            left: Some(no_border.clone()),
            right: Some(no_border.clone()),
            top: Some(no_border.clone()),
            bottom: Some(no_border),
        }
    }

    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        if let Some(ref b) = self.left { xml.push_str(&b.to_xml("lnL")); }
        if let Some(ref b) = self.right { xml.push_str(&b.to_xml("lnR")); }
        if let Some(ref b) = self.top { xml.push_str(&b.to_xml("lnT")); }
        if let Some(ref b) = self.bottom { xml.push_str(&b.to_xml("lnB")); }
        xml
    }
}

/// Cell margins
#[derive(Debug, Clone)]
pub struct CellMargins {
    pub left: i32,   // in EMU
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

impl Default for CellMargins {
    fn default() -> Self {
        CellMargins {
            left: 91440,   // 0.1 inch
            right: 91440,
            top: 45720,    // 0.05 inch
            bottom: 45720,
        }
    }
}

impl CellMargins {
    pub fn uniform(margin: i32) -> Self {
        CellMargins { left: margin, right: margin, top: margin, bottom: margin }
    }
}

/// Table cell with advanced formatting
#[derive(Debug, Clone)]
pub struct TableCellPart {
    pub text: String,
    pub row_span: u32,
    pub col_span: u32,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub font_size: Option<u32>,
    pub font_family: Option<String>,
    pub h_align: HorizontalAlign,
    pub v_align: VerticalAlign,
    pub borders: Option<CellBorders>,
    pub margins: Option<CellMargins>,
    pub is_merged: bool,  // For cells that are part of a merge (not the anchor)
}

impl TableCellPart {
    /// Create a new table cell
    pub fn new(text: impl Into<String>) -> Self {
        TableCellPart {
            text: text.into(),
            row_span: 1,
            col_span: 1,
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            background_color: None,
            text_color: None,
            font_size: None,
            font_family: None,
            h_align: HorizontalAlign::default(),
            v_align: VerticalAlign::default(),
            borders: None,
            margins: None,
            is_merged: false,
        }
    }

    /// Create a merged placeholder cell (for cells covered by a span)
    pub fn merged() -> Self {
        let mut cell = Self::new("");
        cell.is_merged = true;
        cell
    }

    /// Set bold
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set italic
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Set underline
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Set strikethrough
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    /// Set background color
    pub fn background(mut self, color: impl Into<String>) -> Self {
        self.background_color = Some(color.into());
        self
    }

    /// Set text color
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.text_color = Some(color.into());
        self
    }

    /// Set font size (in points)
    pub fn font_size(mut self, size: u32) -> Self {
        self.font_size = Some(size);
        self
    }

    /// Set font family
    pub fn font(mut self, family: impl Into<String>) -> Self {
        self.font_family = Some(family.into());
        self
    }

    /// Set horizontal alignment
    pub fn align(mut self, align: HorizontalAlign) -> Self {
        self.h_align = align;
        self
    }

    /// Set vertical alignment
    pub fn valign(mut self, align: VerticalAlign) -> Self {
        self.v_align = align;
        self
    }

    /// Center text (horizontal and vertical)
    pub fn center(mut self) -> Self {
        self.h_align = HorizontalAlign::Center;
        self.v_align = VerticalAlign::Middle;
        self
    }

    /// Set row span
    pub fn row_span(mut self, span: u32) -> Self {
        self.row_span = span;
        self
    }

    /// Set column span
    pub fn col_span(mut self, span: u32) -> Self {
        self.col_span = span;
        self
    }

    /// Set all borders
    pub fn borders(mut self, borders: CellBorders) -> Self {
        self.borders = Some(borders);
        self
    }

    /// Set uniform border on all sides
    pub fn border(mut self, width_pt: f32, color: impl Into<String>) -> Self {
        self.borders = Some(CellBorders::all(CellBorder::new(width_pt, color)));
        self
    }

    /// Set cell margins
    pub fn margins(mut self, margins: CellMargins) -> Self {
        self.margins = Some(margins);
        self
    }

    /// Generate XML for this cell
    pub fn to_xml(&self) -> String {
        // Handle merged cells (placeholders)
        if self.is_merged {
            return r#"<a:tc hMerge="1"><a:txBody><a:bodyPr/><a:lstStyle/><a:p/></a:txBody><a:tcPr/></a:tc>"#.to_string();
        }

        let mut attrs = String::new();
        if self.row_span > 1 {
            attrs.push_str(&format!(r#" rowSpan="{}""#, self.row_span));
        }
        if self.col_span > 1 {
            attrs.push_str(&format!(r#" gridSpan="{}""#, self.col_span));
        }

        // Background fill
        let bg_xml = self.background_color.as_ref()
            .map(|c| format!(r#"<a:solidFill><a:srgbClr val="{}"/></a:solidFill>"#, c.trim_start_matches('#')))
            .unwrap_or_default();

        // Text run properties
        let mut rpr_attrs = String::new();
        if self.bold { rpr_attrs.push_str(r#" b="1""#); }
        if self.italic { rpr_attrs.push_str(r#" i="1""#); }
        if self.underline { rpr_attrs.push_str(r#" u="sng""#); }
        if self.strikethrough { rpr_attrs.push_str(r#" strike="sngStrike""#); }
        if let Some(size) = self.font_size {
            rpr_attrs.push_str(&format!(r#" sz="{}""#, size * 100));
        }

        // Text color
        let color_xml = self.text_color.as_ref()
            .map(|c| format!(r#"<a:solidFill><a:srgbClr val="{}"/></a:solidFill>"#, c.trim_start_matches('#')))
            .unwrap_or_default();

        // Font family
        let font_xml = self.font_family.as_ref()
            .map(|f| format!(r#"<a:latin typeface="{}"/>"#, f))
            .unwrap_or_default();

        // Paragraph alignment
        let p_align = format!(r#" algn="{}""#, self.h_align.as_str());

        // Cell properties
        let mut tcpr_attrs = format!(r#" anchor="{}""#, self.v_align.as_str());
        if let Some(ref m) = self.margins {
            tcpr_attrs.push_str(&format!(r#" marL="{}" marR="{}" marT="{}" marB="{}""#, 
                m.left, m.right, m.top, m.bottom));
        }

        // Borders
        let borders_xml = self.borders.as_ref()
            .map(|b| b.to_xml())
            .unwrap_or_default();

        let lang_attrs = format_lang_attributes();
        format!(
            r#"<a:tc{}>
          <a:txBody>
            <a:bodyPr/>
            <a:lstStyle/>
            <a:p{}>
              <a:r>
                <a:rPr {} dirty="0"{}>{}{}</a:rPr>
                <a:t>{}</a:t>
              </a:r>
            </a:p>
          </a:txBody>
          <a:tcPr{}>{}{}</a:tcPr>
        </a:tc>"#,
            attrs,
            p_align,
            lang_attrs,
            rpr_attrs,
            color_xml,
            font_xml,
            escape_xml(&self.text),
            tcpr_attrs,
            borders_xml,
            bg_xml
        )
    }
}

/// Table row
#[derive(Debug, Clone)]
pub struct TableRowPart {
    pub cells: Vec<TableCellPart>,
    pub height: Option<i64>, // in EMU
}

impl TableRowPart {
    /// Create a new table row
    pub fn new(cells: Vec<TableCellPart>) -> Self {
        TableRowPart {
            cells,
            height: None,
        }
    }

    /// Set row height in EMU
    pub fn height(mut self, height: i64) -> Self {
        self.height = Some(height);
        self
    }

    /// Generate XML for this row
    pub fn to_xml(&self) -> String {
        let height_attr = self.height
            .map(|h| format!(r#" h="{}""#, h))
            .unwrap_or_default();

        let cells_xml: String = self.cells.iter()
            .map(|c| c.to_xml())
            .collect::<Vec<_>>()
            .join("\n        ");

        format!(
            r#"<a:tr{}>
        {}
      </a:tr>"#,
            height_attr,
            cells_xml
        )
    }
}

/// Table part for embedding in slides
#[derive(Debug, Clone)]
pub struct TablePart {
    pub rows: Vec<TableRowPart>,
    pub col_widths: Vec<i64>, // in EMU
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
}

impl TablePart {
    /// Create a new table part
    pub fn new() -> Self {
        TablePart {
            rows: vec![],
            col_widths: vec![],
            x: 914400,      // 1 inch
            y: 1828800,     // 2 inches
            width: 7315200, // 8 inches
            height: 1828800, // 2 inches
        }
    }

    /// Add a row
    pub fn add_row(mut self, row: TableRowPart) -> Self {
        // Auto-calculate column widths if not set
        if self.col_widths.is_empty() && !row.cells.is_empty() {
            let col_count = row.cells.len();
            let col_width = self.width / col_count as i64;
            self.col_widths = vec![col_width; col_count];
        }
        self.rows.push(row);
        self
    }

    /// Set position
    pub fn position(mut self, x: i64, y: i64) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set size
    pub fn size(mut self, width: i64, height: i64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set column widths
    pub fn col_widths(mut self, widths: Vec<i64>) -> Self {
        self.col_widths = widths;
        self
    }

    /// Generate table XML for embedding in a slide
    pub fn to_slide_xml(&self, shape_id: usize) -> String {
        let grid_cols: String = self.col_widths.iter()
            .map(|w| format!(r#"<a:gridCol w="{}"/>"#, w))
            .collect::<Vec<_>>()
            .join("\n        ");

        let rows_xml: String = self.rows.iter()
            .map(|r| r.to_xml())
            .collect::<Vec<_>>()
            .join("\n      ");

        format!(
            r#"<p:graphicFrame>
  <p:nvGraphicFramePr>
    <p:cNvPr id="{}" name="Table {}"/>
    <p:cNvGraphicFramePr><a:graphicFrameLocks noGrp="1"/></p:cNvGraphicFramePr>
    <p:nvPr/>
  </p:nvGraphicFramePr>
  <p:xfrm>
    <a:off x="{}" y="{}"/>
    <a:ext cx="{}" cy="{}"/>
  </p:xfrm>
  <a:graphic>
    <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/table">
      <a:tbl>
        <a:tblPr firstRow="1" bandRow="1">
          <a:tableStyleId>{{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}}</a:tableStyleId>
        </a:tblPr>
        <a:tblGrid>
        {}
        </a:tblGrid>
      {}
      </a:tbl>
    </a:graphicData>
  </a:graphic>
</p:graphicFrame>"#,
            shape_id,
            shape_id,
            self.x,
            self.y,
            self.width,
            self.height,
            grid_cols,
            rows_xml
        )
    }
}

impl Default for TablePart {
    fn default() -> Self {
        Self::new()
    }
}

impl Part for TablePart {
    fn path(&self) -> &str {
        "" // Tables are embedded in slides, not separate parts
    }

    fn part_type(&self) -> PartType {
        PartType::Slide // Tables are part of slides
    }

    fn content_type(&self) -> ContentType {
        ContentType::Xml
    }

    fn to_xml(&self) -> Result<String, PptxError> {
        Ok(self.to_slide_xml(2))
    }

    fn from_xml(_xml: &str) -> Result<Self, PptxError> {
        Ok(TablePart::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_cell_new() {
        let cell = TableCellPart::new("Test");
        assert_eq!(cell.text, "Test");
        assert!(!cell.bold);
    }

    #[test]
    fn test_table_cell_formatting() {
        let cell = TableCellPart::new("Bold")
            .bold()
            .color("FF0000")
            .font_size(14);
        assert!(cell.bold);
        assert_eq!(cell.text_color, Some("FF0000".to_string()));
        assert_eq!(cell.font_size, Some(14));
    }

    #[test]
    fn test_table_cell_span() {
        let cell = TableCellPart::new("Merged")
            .row_span(2)
            .col_span(3);
        assert_eq!(cell.row_span, 2);
        assert_eq!(cell.col_span, 3);
    }

    #[test]
    fn test_table_row_new() {
        let row = TableRowPart::new(vec![
            TableCellPart::new("A"),
            TableCellPart::new("B"),
        ]);
        assert_eq!(row.cells.len(), 2);
    }

    #[test]
    fn test_table_part_new() {
        let table = TablePart::new()
            .add_row(TableRowPart::new(vec![
                TableCellPart::new("Header 1"),
                TableCellPart::new("Header 2"),
            ]))
            .add_row(TableRowPart::new(vec![
                TableCellPart::new("Data 1"),
                TableCellPart::new("Data 2"),
            ]));
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.col_widths.len(), 2);
    }

    #[test]
    fn test_table_to_xml() {
        let table = TablePart::new()
            .add_row(TableRowPart::new(vec![
                TableCellPart::new("Test"),
            ]));
        let xml = table.to_slide_xml(5);
        assert!(xml.contains("p:graphicFrame"));
        assert!(xml.contains("a:tbl"));
        assert!(xml.contains("Test"));
    }
}
