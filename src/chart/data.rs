//! Chart Data Management - Data series and values for charts


/// Data point in a series
#[derive(Clone, Debug)]
pub struct DataPoint {
    /// Index
    index: u32,
    /// Value
    value: f64,
    /// Label (optional)
    label: Option<String>,
}

impl DataPoint {
    /// Create a new data point
    pub fn new(index: u32, value: f64) -> Self {
        Self {
            index,
            value,
            label: None,
        }
    }

    /// Set label
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// Get index
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Get value
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get label
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Set value
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
}

/// Data series
#[derive(Clone, Debug)]
pub struct DataSeries {
    /// Series index
    index: u32,
    /// Series name
    name: String,
    /// Data points
    points: Vec<DataPoint>,
}

impl DataSeries {
    /// Create a new data series
    pub fn new(index: u32, name: String) -> Self {
        Self {
            index,
            name,
            points: vec![],
        }
    }

    /// Add a data point
    pub fn add_point(&mut self, point: DataPoint) {
        self.points.push(point);
    }

    /// Add a value
    pub fn add_value(&mut self, value: f64) {
        let index = self.points.len() as u32;
        self.add_point(DataPoint::new(index, value));
    }

    /// Get series index
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Get series name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get all points
    pub fn points(&self) -> &[DataPoint] {
        &self.points
    }

    /// Get point count
    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    /// Get point by index
    pub fn get_point(&self, index: usize) -> Option<&DataPoint> {
        self.points.get(index)
    }

    /// Get mutable point by index
    pub fn get_point_mut(&mut self, index: usize) -> Option<&mut DataPoint> {
        self.points.get_mut(index)
    }

    /// Generate XML for data series
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(&format!(r#"<c:ser><c:idx val="{}"/><c:order val="{}"/>"#, self.index, self.index));
        xml.push('\n');
        
        // Series name
        xml.push_str(&format!(r#"<c:tx><c:strRef><c:f>{}</c:f></c:strRef></c:tx>"#, self.name));
        xml.push('\n');
        
        // Data points
        xml.push_str(r#"<c:val><c:numRef><c:numCache>"#);
        xml.push('\n');
        
        for point in &self.points {
            xml.push_str(&format!(
                r#"<c:pt idx="{}"><c:v>{}</c:v></c:pt>"#,
                point.index, point.value
            ));
            xml.push('\n');
        }
        
        xml.push_str(r#"</c:numCache></c:numRef></c:val></c:ser>"#);
        xml
    }
}

/// Chart data
#[derive(Clone, Debug)]
pub struct ChartData {
    /// Data series
    series: Vec<DataSeries>,
    /// Category labels
    categories: Vec<String>,
}

impl ChartData {
    /// Create a new chart data
    pub fn new() -> Self {
        Self {
            series: vec![],
            categories: vec![],
        }
    }

    /// Add a data series
    pub fn add_series(&mut self, series: DataSeries) {
        self.series.push(series);
    }

    /// Create and add a new series
    pub fn create_series(&mut self, name: String) -> usize {
        let index = self.series.len() as u32;
        self.add_series(DataSeries::new(index, name));
        self.series.len() - 1
    }

    /// Add a category label
    pub fn add_category(&mut self, label: String) {
        self.categories.push(label);
    }

    /// Get all series
    pub fn series(&self) -> &[DataSeries] {
        &self.series
    }

    /// Get mutable series by index
    pub fn get_series_mut(&mut self, index: usize) -> Option<&mut DataSeries> {
        self.series.get_mut(index)
    }

    /// Get series count
    pub fn series_count(&self) -> usize {
        self.series.len()
    }

    /// Get categories
    pub fn categories(&self) -> &[String] {
        &self.categories
    }

    /// Get category count
    pub fn category_count(&self) -> usize {
        self.categories.len()
    }

    /// Generate XML for chart data
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(r#"<c:plotArea><c:barChart><c:barDir val="col"/>"#);
        xml.push('\n');

        // Add all series
        for series in &self.series {
            xml.push_str(&series.to_xml());
            xml.push('\n');
        }

        xml.push_str(r#"</c:barChart></c:plotArea>"#);
        xml
    }
}

impl Default for ChartData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_point_creation() {
        let point = DataPoint::new(0, 10.5);
        assert_eq!(point.index(), 0);
        assert_eq!(point.value(), 10.5);
        assert!(point.label().is_none());
    }

    #[test]
    fn test_data_point_with_label() {
        let point = DataPoint::new(0, 10.5).with_label("Q1".to_string());
        assert_eq!(point.label(), Some("Q1"));
    }

    #[test]
    fn test_data_point_set_value() {
        let mut point = DataPoint::new(0, 10.5);
        point.set_value(20.5);
        assert_eq!(point.value(), 20.5);
    }

    #[test]
    fn test_data_series_creation() {
        let series = DataSeries::new(0, "Sales".to_string());
        assert_eq!(series.index(), 0);
        assert_eq!(series.name(), "Sales");
        assert_eq!(series.point_count(), 0);
    }

    #[test]
    fn test_data_series_add_point() {
        let mut series = DataSeries::new(0, "Sales".to_string());
        series.add_point(DataPoint::new(0, 10.0));
        series.add_point(DataPoint::new(1, 20.0));

        assert_eq!(series.point_count(), 2);
    }

    #[test]
    fn test_data_series_add_value() {
        let mut series = DataSeries::new(0, "Sales".to_string());
        series.add_value(10.0);
        series.add_value(20.0);
        series.add_value(30.0);

        assert_eq!(series.point_count(), 3);
        assert_eq!(series.get_point(0).unwrap().value(), 10.0);
        assert_eq!(series.get_point(2).unwrap().value(), 30.0);
    }

    #[test]
    fn test_data_series_get_point() {
        let mut series = DataSeries::new(0, "Sales".to_string());
        series.add_value(10.0);

        let point = series.get_point(0);
        assert!(point.is_some());
        assert_eq!(point.unwrap().value(), 10.0);
    }

    #[test]
    fn test_data_series_get_point_mut() {
        let mut series = DataSeries::new(0, "Sales".to_string());
        series.add_value(10.0);

        if let Some(point) = series.get_point_mut(0) {
            point.set_value(15.0);
        }

        assert_eq!(series.get_point(0).unwrap().value(), 15.0);
    }

    #[test]
    fn test_chart_data_creation() {
        let data = ChartData::new();
        assert_eq!(data.series_count(), 0);
        assert_eq!(data.category_count(), 0);
    }

    #[test]
    fn test_chart_data_add_series() {
        let mut data = ChartData::new();
        let series = DataSeries::new(0, "Sales".to_string());
        data.add_series(series);

        assert_eq!(data.series_count(), 1);
    }

    #[test]
    fn test_chart_data_create_series() {
        let mut data = ChartData::new();
        let idx = data.create_series("Sales".to_string());
        data.create_series("Expenses".to_string());

        assert_eq!(idx, 0);
        assert_eq!(data.series_count(), 2);
    }

    #[test]
    fn test_chart_data_add_category() {
        let mut data = ChartData::new();
        data.add_category("Q1".to_string());
        data.add_category("Q2".to_string());
        data.add_category("Q3".to_string());

        assert_eq!(data.category_count(), 3);
    }

    #[test]
    fn test_chart_data_get_series_mut() {
        let mut data = ChartData::new();
        data.create_series("Sales".to_string());

        if let Some(series) = data.get_series_mut(0) {
            series.add_value(100.0);
            series.add_value(200.0);
        }

        assert_eq!(data.series()[0].point_count(), 2);
    }

    #[test]
    fn test_chart_data_to_xml() {
        let mut data = ChartData::new();
        let idx = data.create_series("Sales".to_string());
        
        if let Some(series) = data.get_series_mut(idx) {
            series.add_value(100.0);
            series.add_value(200.0);
        }

        let xml = data.to_xml();
        assert!(xml.contains(r#"<c:plotArea>"#));
        assert!(xml.contains(r#"<c:barChart>"#));
        assert!(xml.contains(r#"<c:ser>"#));
        assert!(xml.contains(r#"</c:plotArea>"#));
    }

    #[test]
    fn test_chart_data_default() {
        let data = ChartData::default();
        assert_eq!(data.series_count(), 0);
    }
}
