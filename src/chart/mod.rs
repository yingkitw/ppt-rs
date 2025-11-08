//! Chart module - chart types and data management

pub mod axis;
pub mod data;
pub mod enhancements;

pub use axis::Axis;
pub use data::{ChartData, DataSeries, DataPoint};
pub use enhancements::{Trendline, TrendlineType, ErrorBar, ErrorBarType, ErrorBarDirection, ChartEnhancementManager};
use crate::enums::chart::ChartType;

/// Chart - represents a chart object
#[derive(Clone, Debug)]
pub struct Chart {
    /// Chart type
    chart_type: ChartType,
    /// Chart data
    data: ChartData,
}

impl Chart {
    /// Create a new chart
    pub fn new(chart_type: ChartType) -> Self {
        Self {
            chart_type,
            data: ChartData::new(),
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
}
