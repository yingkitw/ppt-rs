//! Chart module - chart types and data management

pub mod axis;
pub mod data;
pub mod enhancements;
pub mod data_table;

pub use axis::Axis;
pub use data::{ChartData, DataSeries, DataPoint};
pub use enhancements::{Trendline, TrendlineType, ErrorBar, ErrorBarType, ErrorBarDirection, ChartEnhancementManager};
pub use data_table::DataTable;
use crate::enums::chart::ChartType;

/// Chart - represents a chart object
#[derive(Clone, Debug)]
pub struct Chart {
    /// Chart type
    chart_type: ChartType,
    /// Chart data
    data: ChartData,
    /// Data table (optional)
    data_table: Option<DataTable>,
}

impl Chart {
    /// Create a new chart
    pub fn new(chart_type: ChartType) -> Self {
        Self {
            chart_type,
            data: ChartData::new(),
            data_table: None,
        }
    }

    /// Get chart type
    pub fn chart_type(&self) -> ChartType {
        self.chart_type
    }

    /// Get chart data
    pub fn data(&self) -> &ChartData {
        &self.data
    }

    /// Get mutable chart data
    pub fn data_mut(&mut self) -> &mut ChartData {
        &mut self.data
    }

    /// Get data table
    pub fn data_table(&self) -> Option<&DataTable> {
        self.data_table.as_ref()
    }

    /// Get mutable data table
    pub fn data_table_mut(&mut self) -> &mut Option<DataTable> {
        &mut self.data_table
    }

    /// Set data table
    pub fn set_data_table(&mut self, table: DataTable) {
        self.data_table = Some(table);
    }

    /// Remove data table
    pub fn remove_data_table(&mut self) {
        self.data_table = None;
    }

    /// Check if data table is visible
    pub fn has_data_table(&self) -> bool {
        self.data_table.is_some()
    }
}

impl Default for Chart {
    fn default() -> Self {
        Self::new(ChartType::ColumnClustered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_creation() {
        use crate::enums::chart::ChartType;
        let chart = Chart::new(ChartType::ColumnClustered);
        assert_eq!(chart.data().series_count(), 0);
    }

    #[test]
    fn test_chart_data_basic() {
        use crate::enums::chart::ChartType;
        let mut chart = Chart::new(ChartType::ColumnClustered);
        let idx = chart.data_mut().create_series("Sales".to_string());
        
        if let Some(series) = chart.data_mut().get_series_mut(idx) {
            series.add_value(100.0);
            series.add_value(200.0);
        }

        assert_eq!(chart.data().series_count(), 1);
    }

    #[test]
    fn test_chart_data_table_add() {
        use crate::enums::chart::ChartType;
        let mut chart = Chart::new(ChartType::ColumnClustered);
        assert!(!chart.has_data_table());
        
        let table = DataTable::new();
        chart.set_data_table(table);
        assert!(chart.has_data_table());
    }

    #[test]
    fn test_chart_data_table_remove() {
        use crate::enums::chart::ChartType;
        let mut chart = Chart::new(ChartType::ColumnClustered);
        chart.set_data_table(DataTable::new());
        assert!(chart.has_data_table());
        
        chart.remove_data_table();
        assert!(!chart.has_data_table());
    }

    #[test]
    fn test_chart_data_table_options() {
        use crate::enums::chart::ChartType;
        let mut chart = Chart::new(ChartType::ColumnClustered);
        let mut table = DataTable::new();
        table.set_show_legend_keys(false);
        table.set_show_h_border(false);
        
        chart.set_data_table(table);
        
        if let Some(dt) = chart.data_table() {
            assert!(!dt.show_legend_keys());
            assert!(!dt.show_h_border());
            assert!(dt.show_v_border());
        }
    }
}
