//! Chart part
//!
//! Represents a chart embedded in the presentation.

use super::base::{Part, PartType, ContentType};
use crate::exc::PptxError;
use crate::generator::charts::{Chart, generate_chart_xml_with_number};

/// Chart part (ppt/charts/chartN.xml)
#[derive(Debug, Clone)]
pub struct ChartPart {
    path: String,
    chart_number: usize,
    chart: Option<Chart>,
    xml_content: Option<String>,
}

impl ChartPart {
    /// Create a new chart part
    pub fn new(chart_number: usize) -> Self {
        ChartPart {
            path: format!("ppt/charts/chart{}.xml", chart_number),
            chart_number,
            chart: None,
            xml_content: None,
        }
    }

    /// Create from Chart
    pub fn from_chart(chart_number: usize, chart: Chart) -> Self {
        ChartPart {
            path: format!("ppt/charts/chart{}.xml", chart_number),
            chart_number,
            chart: Some(chart),
            xml_content: None,
        }
    }

    /// Get chart number
    pub fn chart_number(&self) -> usize {
        self.chart_number
    }

    /// Get chart if available
    pub fn chart(&self) -> Option<&Chart> {
        self.chart.as_ref()
    }

    /// Set chart
    pub fn set_chart(&mut self, chart: Chart) {
        self.chart = Some(chart);
        self.xml_content = None;
    }

    /// Get relative path for relationships
    pub fn rel_target(&self) -> String {
        format!("../charts/chart{}.xml", self.chart_number)
    }
}

impl Part for ChartPart {
    fn path(&self) -> &str {
        &self.path
    }

    fn part_type(&self) -> PartType {
        PartType::Chart
    }

    fn content_type(&self) -> ContentType {
        ContentType::Chart
    }

    fn to_xml(&self) -> Result<String, PptxError> {
        if let Some(ref xml) = self.xml_content {
            return Ok(xml.clone());
        }

        if let Some(ref chart) = self.chart {
            return Ok(generate_chart_xml_with_number(chart, self.chart_number, self.chart_number));
        }

        // Return minimal chart XML
        Err(PptxError::InvalidOperation("No chart data available".to_string()))
    }

    fn from_xml(xml: &str) -> Result<Self, PptxError> {
        // Basic parsing - store XML for now
        Ok(ChartPart {
            path: "ppt/charts/chart1.xml".to_string(),
            chart_number: 1,
            chart: None,
            xml_content: Some(xml.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::charts::{ChartBuilder, ChartType, ChartSeries};

    #[test]
    fn test_chart_part_new() {
        let part = ChartPart::new(1);
        assert_eq!(part.chart_number(), 1);
        assert_eq!(part.path(), "ppt/charts/chart1.xml");
    }

    #[test]
    fn test_chart_part_from_chart() {
        let chart = ChartBuilder::new("Test", ChartType::Bar)
            .categories(vec!["A", "B"])
            .add_series(ChartSeries::new("Data", vec![1.0, 2.0]))
            .build();
        
        let part = ChartPart::from_chart(2, chart);
        assert_eq!(part.chart_number(), 2);
        assert!(part.chart().is_some());
    }

    #[test]
    fn test_chart_part_to_xml() {
        let chart = ChartBuilder::new("Sales", ChartType::Bar)
            .categories(vec!["Q1", "Q2"])
            .add_series(ChartSeries::new("2024", vec![100.0, 150.0]))
            .build();
        
        let part = ChartPart::from_chart(1, chart);
        let xml = part.to_xml().unwrap();
        
        assert!(xml.contains("c:chart"));
    }

    #[test]
    fn test_chart_rel_target() {
        let part = ChartPart::new(3);
        assert_eq!(part.rel_target(), "../charts/chart3.xml");
    }
}
