//! Chart type definitions

/// Chart types supported
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
}

impl ChartType {
    /// Get string representation
    pub fn as_str(&self) -> &str {
        match self {
            ChartType::Bar => "bar",
            ChartType::Line => "line",
            ChartType::Pie => "pie",
        }
    }

    /// Get OOXML chart element name
    pub fn xml_element(&self) -> &str {
        match self {
            ChartType::Bar => "c:barChart",
            ChartType::Line => "c:lineChart",
            ChartType::Pie => "c:pieChart",
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
    }

    #[test]
    fn test_chart_type_xml_element() {
        assert_eq!(ChartType::Bar.xml_element(), "c:barChart");
        assert_eq!(ChartType::Line.xml_element(), "c:lineChart");
        assert_eq!(ChartType::Pie.xml_element(), "c:pieChart");
    }
}
