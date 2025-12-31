//! Chart data structures

use super::types::ChartType;

/// Chart data series
#[derive(Clone, Debug)]
pub struct ChartSeries {
    pub name: String,
    pub values: Vec<f64>,
    pub x_values: Option<Vec<f64>>,
    pub bubble_sizes: Option<Vec<f64>>,
}

impl ChartSeries {
    /// Create a new chart series
    pub fn new(name: &str, values: Vec<f64>) -> Self {
        ChartSeries {
            name: name.to_string(),
            values,
            x_values: None,
            bubble_sizes: None,
        }
    }

    /// Create a new XY chart series (for scatter and bubble charts)
    pub fn new_xy(name: &str, x_values: Vec<f64>, y_values: Vec<f64>) -> Self {
        ChartSeries {
            name: name.to_string(),
            values: y_values,
            x_values: Some(x_values),
            bubble_sizes: None,
        }
    }

    /// Create a new bubble chart series
    pub fn new_bubble(name: &str, x_values: Vec<f64>, y_values: Vec<f64>, bubble_sizes: Vec<f64>) -> Self {
        ChartSeries {
            name: name.to_string(),
            values: y_values,
            x_values: Some(x_values),
            bubble_sizes: Some(bubble_sizes),
        }
    }

    /// Get the number of data points
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Check if series is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// Chart definition
#[derive(Clone, Debug)]
pub struct Chart {
    pub title: String,
    pub chart_type: ChartType,
    pub categories: Vec<String>,
    pub series: Vec<ChartSeries>,
    pub x: u32,      // Position X in EMU
    pub y: u32,      // Position Y in EMU
    pub width: u32,  // Width in EMU
    pub height: u32, // Height in EMU
}

impl Chart {
    /// Create a new chart
    pub fn new(
        title: &str,
        chart_type: ChartType,
        categories: Vec<String>,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Self {
        Chart {
            title: title.to_string(),
            chart_type,
            categories,
            series: Vec::new(),
            x,
            y,
            width,
            height,
        }
    }

    /// Add a data series
    pub fn add_series(mut self, series: ChartSeries) -> Self {
        self.series.push(series);
        self
    }

    /// Get number of categories
    pub fn category_count(&self) -> usize {
        self.categories.len()
    }

    /// Get number of series
    pub fn series_count(&self) -> usize {
        self.series.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_series() {
        let series = ChartSeries::new("Sales", vec![10.0, 20.0, 30.0]);
        assert_eq!(series.name, "Sales");
        assert_eq!(series.len(), 3);
        assert!(!series.is_empty());
    }

    #[test]
    fn test_chart_add_series() {
        let chart = Chart::new("Test", ChartType::Pie, vec!["A".to_string()], 0, 0, 1000000, 1000000)
            .add_series(ChartSeries::new("Data", vec![50.0]));

        assert_eq!(chart.series_count(), 1);
    }
}
