//! Chart-related objects

pub mod axis;

use crate::enums::chart::ChartType;
use crate::text::TextFrame;

pub use axis::{Axis, CategoryAxis, ValueAxis, DateAxis};

/// Chart - represents a chart in a presentation
pub struct Chart {
    chart_type: ChartType,
    title: Option<ChartTitle>,
    has_legend: bool,
    series: Vec<ChartSeries>,
    chart_style: Option<u32>, // 1-48
    category_axis: Option<CategoryAxis>,
    value_axis: Option<ValueAxis>,
}

impl Chart {
    /// Create a new chart
    pub fn new(chart_type: ChartType) -> Self {
        Self {
            chart_type,
            title: None,
            has_legend: false,
            series: Vec::new(),
            chart_style: None,
            category_axis: None,
            value_axis: None,
        }
    }

    /// Get the chart type
    pub fn chart_type(&self) -> ChartType {
        self.chart_type
    }

    /// Set the chart type
    pub fn set_chart_type(&mut self, chart_type: ChartType) {
        self.chart_type = chart_type;
    }

    /// Get the chart title
    pub fn title(&self) -> Option<&ChartTitle> {
        self.title.as_ref()
    }

    /// Get mutable chart title
    pub fn title_mut(&mut self) -> Option<&mut ChartTitle> {
        self.title.as_mut()
    }

    /// Set chart title
    pub fn set_title(&mut self, title: ChartTitle) {
        self.title = Some(title);
    }

    /// Check if chart has a title
    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    /// Set whether chart has a title
    pub fn set_has_title(&mut self, has_title: bool) {
        if !has_title {
            self.title = None;
        } else if self.title.is_none() {
            self.title = Some(ChartTitle::new());
        }
    }

    /// Check if chart has a legend
    pub fn has_legend(&self) -> bool {
        self.has_legend
    }

    /// Set whether chart has a legend
    pub fn set_has_legend(&mut self, has_legend: bool) {
        self.has_legend = has_legend;
    }

    /// Get chart style (1-48)
    pub fn chart_style(&self) -> Option<u32> {
        self.chart_style
    }

    /// Set chart style (1-48)
    pub fn set_chart_style(&mut self, style: Option<u32>) {
        if let Some(s) = style {
            if s >= 1 && s <= 48 {
                self.chart_style = Some(s);
            }
        } else {
            self.chart_style = None;
        }
    }

    /// Get series collection
    pub fn series(&self) -> &[ChartSeries] {
        &self.series
    }

    /// Get mutable series collection
    pub fn series_mut(&mut self) -> &mut [ChartSeries] {
        &mut self.series
    }

    /// Add a series to the chart
    pub fn add_series(&mut self, series: ChartSeries) {
        self.series.push(series);
    }

    /// Get number of series
    pub fn series_count(&self) -> usize {
        self.series.len()
    }

    /// Get category axis (if available)
    pub fn category_axis(&self) -> Option<&CategoryAxis> {
        self.category_axis.as_ref()
    }

    /// Get mutable category axis
    pub fn category_axis_mut(&mut self) -> &mut CategoryAxis {
        self.category_axis.get_or_insert_with(CategoryAxis::new)
    }

    /// Set category axis
    pub fn set_category_axis(&mut self, axis: CategoryAxis) {
        self.category_axis = Some(axis);
    }

    /// Get value axis (if available)
    pub fn value_axis(&self) -> Option<&ValueAxis> {
        self.value_axis.as_ref()
    }

    /// Get mutable value axis
    pub fn value_axis_mut(&mut self) -> &mut ValueAxis {
        self.value_axis.get_or_insert_with(ValueAxis::new)
    }

    /// Set value axis
    pub fn set_value_axis(&mut self, axis: ValueAxis) {
        self.value_axis = Some(axis);
    }
}

/// Chart title
pub struct ChartTitle {
    text_frame: TextFrame,
}

impl ChartTitle {
    /// Create a new chart title
    pub fn new() -> Self {
        Self {
            text_frame: TextFrame::new(),
        }
    }

    /// Get the text frame
    pub fn text_frame(&self) -> &TextFrame {
        &self.text_frame
    }

    /// Get mutable text frame
    pub fn text_frame_mut(&mut self) -> &mut TextFrame {
        &mut self.text_frame
    }

    /// Set the title text
    pub fn set_text(&mut self, text: &str) {
        self.text_frame.set_text(text);
    }

    /// Get the title text
    pub fn text(&self) -> &str {
        self.text_frame.text()
    }
}

/// Chart series - represents a data series in a chart
pub struct ChartSeries {
    name: String,
    values: Vec<f64>,
    categories: Vec<String>,
    index: usize,
}

impl ChartSeries {
    /// Create a new chart series
    pub fn new(name: String) -> Self {
        Self {
            name,
            values: Vec::new(),
            categories: Vec::new(),
            index: 0,
        }
    }

    /// Create a new chart series with values
    pub fn with_values(name: String, values: Vec<f64>) -> Self {
        Self {
            name,
            values,
            categories: Vec::new(),
            index: 0,
        }
    }

    /// Create a new chart series with values and categories
    pub fn with_data(name: String, values: Vec<f64>, categories: Vec<String>) -> Self {
        Self {
            name,
            values,
            categories,
            index: 0,
        }
    }

    /// Get the series name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the series name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get the values
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// Get mutable values
    pub fn values_mut(&mut self) -> &mut [f64] {
        &mut self.values
    }

    /// Set values
    pub fn set_values(&mut self, values: Vec<f64>) {
        self.values = values;
    }

    /// Get categories
    pub fn categories(&self) -> &[String] {
        &self.categories
    }

    /// Get mutable categories
    pub fn categories_mut(&mut self) -> &mut [String] {
        &mut self.categories
    }

    /// Set categories
    pub fn set_categories(&mut self, categories: Vec<String>) {
        self.categories = categories;
    }

    /// Get the series index
    pub fn index(&self) -> usize {
        self.index
    }

    /// Set the series index
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

/// Chart legend
pub struct ChartLegend {
    position: LegendPosition,
    visible: bool,
}

/// Legend position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegendPosition {
    Bottom,
    Top,
    Left,
    Right,
    TopRight,
}

impl ChartLegend {
    /// Create a new chart legend
    pub fn new() -> Self {
        Self {
            position: LegendPosition::Right,
            visible: true,
        }
    }

    /// Get the legend position
    pub fn position(&self) -> LegendPosition {
        self.position
    }

    /// Set the legend position
    pub fn set_position(&mut self, position: LegendPosition) {
        self.position = position;
    }

    /// Check if legend is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set legend visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::chart::ChartType;

    #[test]
    fn test_chart_new() {
        let chart = Chart::new(ChartType::ColumnClustered);
        assert_eq!(chart.chart_type(), ChartType::ColumnClustered);
        assert!(!chart.has_title());
        assert!(!chart.has_legend());
        assert_eq!(chart.series_count(), 0);
    }

    #[test]
    fn test_chart_title() {
        let mut chart = Chart::new(ChartType::Pie);
        chart.set_has_title(true);
        assert!(chart.has_title());
        
        if let Some(title) = chart.title_mut() {
            title.set_text("My Chart");
            assert_eq!(title.text(), "My Chart");
        }
    }

    #[test]
    fn test_chart_series() {
        let mut chart = Chart::new(ChartType::Line);
        let series = ChartSeries::with_values("Series1".to_string(), vec![1.0, 2.0, 3.0]);
        chart.add_series(series);
        
        assert_eq!(chart.series_count(), 1);
        assert_eq!(chart.series()[0].name(), "Series1");
        assert_eq!(chart.series()[0].values(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_chart_legend() {
        let legend = ChartLegend::new();
        assert!(legend.is_visible());
        assert_eq!(legend.position(), LegendPosition::Right);
    }

    #[test]
    fn test_chart_style() {
        let mut chart = Chart::new(ChartType::BarClustered);
        chart.set_chart_style(Some(5));
        assert_eq!(chart.chart_style(), Some(5));
        
        chart.set_chart_style(None);
        assert_eq!(chart.chart_style(), None);
    }

    #[test]
    fn test_chart_axes() {
        let mut chart = Chart::new(ChartType::ColumnClustered);
        
        // Test category axis
        let cat_axis = chart.category_axis_mut();
        cat_axis.set_has_title(true);
        assert!(cat_axis.has_title());
        
        // Test value axis
        let val_axis = chart.value_axis_mut();
        val_axis.set_major_unit(Some(10.0));
        val_axis.set_minimum_scale(Some(0.0));
        val_axis.set_maximum_scale(Some(100.0));
        assert_eq!(val_axis.major_unit(), Some(10.0));
        assert_eq!(val_axis.minimum_scale(), Some(0.0));
        assert_eq!(val_axis.maximum_scale(), Some(100.0));
    }
}
