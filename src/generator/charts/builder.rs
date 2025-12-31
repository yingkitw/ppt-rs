//! Chart builder for fluent API

use super::types::ChartType;
use super::data::{Chart, ChartSeries};

/// Chart builder for fluent API
pub struct ChartBuilder {
    title: String,
    chart_type: ChartType,
    categories: Vec<String>,
    series: Vec<ChartSeries>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl ChartBuilder {
    /// Create a new chart builder
    pub fn new(title: &str, chart_type: ChartType) -> Self {
        ChartBuilder {
            title: title.to_string(),
            chart_type,
            categories: Vec::new(),
            series: Vec::new(),
            x: 2675890,      // Default x position (based on WPS reference)
            y: 1725930,      // Default y position (based on WPS reference)
            width: 6839585,  // Default width (based on WPS reference)
            height: 3959860, // Default height (based on WPS reference)
        }
    }

    /// Set chart position
    pub fn position(mut self, x: u32, y: u32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set chart size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Add categories
    pub fn categories(mut self, categories: Vec<&str>) -> Self {
        self.categories = categories.into_iter().map(|c| c.to_string()).collect();
        self
    }

    /// Add a data series
    pub fn add_series(mut self, series: ChartSeries) -> Self {
        self.series.push(series);
        self
    }

    /// Build the chart
    pub fn build(self) -> Chart {
        Chart {
            title: self.title,
            chart_type: self.chart_type,
            categories: self.categories,
            series: self.series,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_builder() {
        let chart = ChartBuilder::new("Revenue", ChartType::Bar)
            .categories(vec!["Q1", "Q2", "Q3"])
            .add_series(ChartSeries::new("2023", vec![100.0, 150.0, 200.0]))
            .add_series(ChartSeries::new("2024", vec![120.0, 180.0, 220.0]))
            .position(100000, 200000)
            .size(4000000, 3000000)
            .build();

        assert_eq!(chart.title, "Revenue");
        assert_eq!(chart.chart_type, ChartType::Bar);
        assert_eq!(chart.category_count(), 3);
        assert_eq!(chart.series_count(), 2);
        assert_eq!(chart.x, 100000);
        assert_eq!(chart.y, 200000);
    }
}
