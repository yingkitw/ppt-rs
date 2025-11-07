//! GraphicFrame shape - container for charts, tables, and other graphic objects

use crate::chart::Chart;
use crate::shapes::base::{BaseShape, Shape};
use crate::table::Table;
use crate::enums::shapes::ShapeType;

/// GraphicFrame - container shape for table, chart, smart art, and media objects
pub struct GraphicFrame {
    base: BaseShape,
    content_type: GraphicFrameContentType,
    chart: Option<Chart>,
    table: Option<Table>,
}

/// Type of content in a GraphicFrame
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicFrameContentType {
    Chart,
    Table,
    OleObject,
    SmartArt,
    Unknown,
}

impl GraphicFrame {
    /// Create a new GraphicFrame for a chart
    pub fn with_chart(id: u32, name: String, chart: Chart) -> Self {
        Self {
            base: BaseShape::new(id, name),
            content_type: GraphicFrameContentType::Chart,
            chart: Some(chart),
            table: None,
        }
    }

    /// Create a new GraphicFrame for a table
    pub fn with_table(id: u32, name: String, table: Table) -> Self {
        Self {
            base: BaseShape::new(id, name),
            content_type: GraphicFrameContentType::Table,
            chart: None,
            table: Some(table),
        }
    }

    /// Create a new empty GraphicFrame
    pub fn new(id: u32, name: String) -> Self {
        Self {
            base: BaseShape::new(id, name),
            content_type: GraphicFrameContentType::Unknown,
            chart: None,
            table: None,
        }
    }

    /// Check if this graphic frame contains a chart
    pub fn has_chart(&self) -> bool {
        matches!(self.content_type, GraphicFrameContentType::Chart) && self.chart.is_some()
    }

    /// Get the chart (if available)
    pub fn chart(&self) -> Option<&Chart> {
        self.chart.as_ref()
    }

    /// Get mutable chart (if available)
    pub fn chart_mut(&mut self) -> Option<&mut Chart> {
        self.chart.as_mut()
    }

    /// Check if this graphic frame contains a table
    pub fn has_table(&self) -> bool {
        matches!(self.content_type, GraphicFrameContentType::Table) && self.table.is_some()
    }

    /// Get the table (if available)
    pub fn table(&self) -> Option<&Table> {
        self.table.as_ref()
    }

    /// Get mutable table (if available)
    pub fn table_mut(&mut self) -> Option<&mut Table> {
        self.table.as_mut()
    }

    /// Get the content type
    pub fn content_type(&self) -> GraphicFrameContentType {
        self.content_type
    }

    /// Set the chart
    pub fn set_chart(&mut self, chart: Chart) {
        self.content_type = GraphicFrameContentType::Chart;
        self.chart = Some(chart);
        self.table = None;
    }

    /// Set the table
    pub fn set_table(&mut self, table: Table) {
        self.content_type = GraphicFrameContentType::Table;
        self.table = Some(table);
        self.chart = None;
    }
}

impl Shape for GraphicFrame {
    fn id(&self) -> u32 {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn set_name(&mut self, name: String) {
        self.base.set_name(name);
    }

    fn left(&self) -> i64 {
        self.base.left()
    }

    fn set_left(&mut self, left: i64) {
        self.base.set_left(left);
    }

    fn top(&self) -> i64 {
        self.base.top()
    }

    fn set_top(&mut self, top: i64) {
        self.base.set_top(top);
    }

    fn width(&self) -> u32 {
        self.base.width()
    }

    fn set_width(&mut self, width: u32) {
        self.base.set_width(width);
    }

    fn height(&self) -> u32 {
        self.base.height()
    }

    fn set_height(&mut self, height: u32) {
        self.base.set_height(height);
    }

    fn has_text_frame(&self) -> bool {
        false // GraphicFrame doesn't have its own text frame
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::Chart;
    use crate::enums::chart::ChartType;
    use crate::table::Table;

    #[test]
    fn test_graphic_frame_with_chart() {
        let chart = Chart::new(ChartType::ColumnClustered);
        let gf = GraphicFrame::with_chart(1, "Chart1".to_string(), chart);
        assert!(gf.has_chart());
        assert!(!gf.has_table());
        assert!(gf.chart().is_some());
    }

    #[test]
    fn test_graphic_frame_with_table() {
        let table = Table::new(3, 4);
        let gf = GraphicFrame::with_table(1, "Table1".to_string(), table);
        assert!(!gf.has_chart());
        assert!(gf.has_table());
        assert!(gf.table().is_some());
    }

    #[test]
    fn test_graphic_frame_set_chart() {
        let mut gf = GraphicFrame::new(1, "Frame1".to_string());
        let chart = Chart::new(ChartType::Pie);
        gf.set_chart(chart);
        assert!(gf.has_chart());
        assert!(!gf.has_table());
    }

    #[test]
    fn test_graphic_frame_set_table() {
        let mut gf = GraphicFrame::new(1, "Frame1".to_string());
        let table = Table::new(2, 2);
        gf.set_table(table);
        assert!(!gf.has_chart());
        assert!(gf.has_table());
    }
}

