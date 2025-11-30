//! Chart type definitions

/// Chart types supported
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum ChartType {
    /// Vertical bar chart
    Bar,
    /// Horizontal bar chart
    BarHorizontal,
    /// Stacked bar chart
    BarStacked,
    /// 100% stacked bar chart
    BarStacked100,
    /// Line chart
    Line,
    /// Line chart with markers
    LineMarkers,
    /// Stacked line chart
    LineStacked,
    /// Pie chart
    Pie,
    /// Doughnut chart
    Doughnut,
    /// Area chart
    Area,
    /// Stacked area chart
    AreaStacked,
    /// 100% stacked area chart
    AreaStacked100,
    /// Scatter chart (XY)
    Scatter,
    /// Scatter with lines
    ScatterLines,
    /// Scatter with smooth lines
    ScatterSmooth,
    /// Bubble chart
    Bubble,
    /// Radar chart
    Radar,
    /// Filled radar chart
    RadarFilled,
    /// Stock chart (High-Low-Close)
    StockHLC,
    /// Stock chart (Open-High-Low-Close)
    StockOHLC,
    /// Combo chart (bar + line)
    Combo,
}

impl ChartType {
    /// Get string representation
    pub fn as_str(&self) -> &str {
        match self {
            ChartType::Bar => "bar",
            ChartType::BarHorizontal => "barHorizontal",
            ChartType::BarStacked => "barStacked",
            ChartType::BarStacked100 => "barStacked100",
            ChartType::Line => "line",
            ChartType::LineMarkers => "lineMarkers",
            ChartType::LineStacked => "lineStacked",
            ChartType::Pie => "pie",
            ChartType::Doughnut => "doughnut",
            ChartType::Area => "area",
            ChartType::AreaStacked => "areaStacked",
            ChartType::AreaStacked100 => "areaStacked100",
            ChartType::Scatter => "scatter",
            ChartType::ScatterLines => "scatterLines",
            ChartType::ScatterSmooth => "scatterSmooth",
            ChartType::Bubble => "bubble",
            ChartType::Radar => "radar",
            ChartType::RadarFilled => "radarFilled",
            ChartType::StockHLC => "stockHLC",
            ChartType::StockOHLC => "stockOHLC",
            ChartType::Combo => "combo",
        }
    }

    /// Get OOXML chart element name
    pub fn xml_element(&self) -> &str {
        match self {
            ChartType::Bar | ChartType::BarStacked | ChartType::BarStacked100 => "c:barChart",
            ChartType::BarHorizontal => "c:barChart",
            ChartType::Line | ChartType::LineMarkers | ChartType::LineStacked => "c:lineChart",
            ChartType::Pie => "c:pieChart",
            ChartType::Doughnut => "c:doughnutChart",
            ChartType::Area | ChartType::AreaStacked | ChartType::AreaStacked100 => "c:areaChart",
            ChartType::Scatter | ChartType::ScatterLines | ChartType::ScatterSmooth => "c:scatterChart",
            ChartType::Bubble => "c:bubbleChart",
            ChartType::Radar | ChartType::RadarFilled => "c:radarChart",
            ChartType::StockHLC | ChartType::StockOHLC => "c:stockChart",
            ChartType::Combo => "c:barChart", // Primary chart type for combo
        }
    }

    /// Get bar direction for bar charts
    pub fn bar_direction(&self) -> Option<&str> {
        match self {
            ChartType::Bar | ChartType::BarStacked | ChartType::BarStacked100 => Some("col"),
            ChartType::BarHorizontal => Some("bar"),
            _ => None,
        }
    }

    /// Get grouping type for charts
    pub fn grouping(&self) -> Option<&str> {
        match self {
            ChartType::Bar | ChartType::BarHorizontal => Some("clustered"),
            ChartType::BarStacked | ChartType::LineStacked | ChartType::AreaStacked => Some("stacked"),
            ChartType::BarStacked100 | ChartType::AreaStacked100 => Some("percentStacked"),
            ChartType::Line | ChartType::LineMarkers | ChartType::Area => Some("standard"),
            _ => None,
        }
    }

    /// Check if chart type uses markers
    pub fn has_markers(&self) -> bool {
        matches!(self, ChartType::LineMarkers | ChartType::Scatter | ChartType::ScatterLines)
    }

    /// Check if chart type uses smooth lines
    pub fn is_smooth(&self) -> bool {
        matches!(self, ChartType::ScatterSmooth)
    }

    /// Get scatter style for scatter charts
    pub fn scatter_style(&self) -> Option<&str> {
        match self {
            ChartType::Scatter => Some("marker"),
            ChartType::ScatterLines => Some("lineMarker"),
            ChartType::ScatterSmooth => Some("smoothMarker"),
            _ => None,
        }
    }

    /// Get radar style for radar charts
    pub fn radar_style(&self) -> Option<&str> {
        match self {
            ChartType::Radar => Some("marker"),
            ChartType::RadarFilled => Some("filled"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_type_str() {
        assert_eq!(ChartType::Bar.as_str(), "bar");
        assert_eq!(ChartType::Line.as_str(), "line");
        assert_eq!(ChartType::Pie.as_str(), "pie");
        assert_eq!(ChartType::Doughnut.as_str(), "doughnut");
        assert_eq!(ChartType::Area.as_str(), "area");
        assert_eq!(ChartType::Scatter.as_str(), "scatter");
        assert_eq!(ChartType::Bubble.as_str(), "bubble");
        assert_eq!(ChartType::Radar.as_str(), "radar");
    }

    #[test]
    fn test_chart_type_xml_element() {
        assert_eq!(ChartType::Bar.xml_element(), "c:barChart");
        assert_eq!(ChartType::Line.xml_element(), "c:lineChart");
        assert_eq!(ChartType::Pie.xml_element(), "c:pieChart");
        assert_eq!(ChartType::Doughnut.xml_element(), "c:doughnutChart");
        assert_eq!(ChartType::Area.xml_element(), "c:areaChart");
        assert_eq!(ChartType::Scatter.xml_element(), "c:scatterChart");
        assert_eq!(ChartType::Bubble.xml_element(), "c:bubbleChart");
        assert_eq!(ChartType::Radar.xml_element(), "c:radarChart");
    }

    #[test]
    fn test_bar_direction() {
        assert_eq!(ChartType::Bar.bar_direction(), Some("col"));
        assert_eq!(ChartType::BarHorizontal.bar_direction(), Some("bar"));
        assert_eq!(ChartType::Line.bar_direction(), None);
    }

    #[test]
    fn test_grouping() {
        assert_eq!(ChartType::Bar.grouping(), Some("clustered"));
        assert_eq!(ChartType::BarStacked.grouping(), Some("stacked"));
        assert_eq!(ChartType::BarStacked100.grouping(), Some("percentStacked"));
    }

    #[test]
    fn test_scatter_style() {
        assert_eq!(ChartType::Scatter.scatter_style(), Some("marker"));
        assert_eq!(ChartType::ScatterLines.scatter_style(), Some("lineMarker"));
        assert_eq!(ChartType::ScatterSmooth.scatter_style(), Some("smoothMarker"));
        assert_eq!(ChartType::Bar.scatter_style(), None);
    }

    #[test]
    fn test_radar_style() {
        assert_eq!(ChartType::Radar.radar_style(), Some("marker"));
        assert_eq!(ChartType::RadarFilled.radar_style(), Some("filled"));
        assert_eq!(ChartType::Line.radar_style(), None);
    }

    #[test]
    fn test_has_markers() {
        assert!(ChartType::LineMarkers.has_markers());
        assert!(ChartType::Scatter.has_markers());
        assert!(!ChartType::Line.has_markers());
        assert!(!ChartType::Pie.has_markers());
    }

    #[test]
    fn test_is_smooth() {
        assert!(ChartType::ScatterSmooth.is_smooth());
        assert!(!ChartType::Scatter.is_smooth());
        assert!(!ChartType::Line.is_smooth());
    }
}
