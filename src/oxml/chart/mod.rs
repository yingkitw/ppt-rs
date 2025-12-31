//! Chart XML elements for OOXML
//!
//! Provides types for parsing and generating DrawingML chart elements.

use super::xmlchemy::XmlElement;
use crate::util::format_lang_attributes;

/// Chart type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartKind {
    Bar,
    Column,
    Line,
    Pie,
    Area,
    Scatter,
    Doughnut,
    Radar,
}

impl ChartKind {
    pub fn xml_element(&self) -> &'static str {
        match self {
            ChartKind::Bar => "c:barChart",
            ChartKind::Column => "c:barChart",
            ChartKind::Line => "c:lineChart",
            ChartKind::Pie => "c:pieChart",
            ChartKind::Area => "c:areaChart",
            ChartKind::Scatter => "c:scatterChart",
            ChartKind::Doughnut => "c:doughnutChart",
            ChartKind::Radar => "c:radarChart",
        }
    }

    pub fn from_element(name: &str) -> Option<Self> {
        match name {
            "barChart" => Some(ChartKind::Bar),
            "lineChart" => Some(ChartKind::Line),
            "pieChart" => Some(ChartKind::Pie),
            "areaChart" => Some(ChartKind::Area),
            "scatterChart" => Some(ChartKind::Scatter),
            "doughnutChart" => Some(ChartKind::Doughnut),
            "radarChart" => Some(ChartKind::Radar),
            _ => None,
        }
    }
}

/// Data point value
#[derive(Debug, Clone)]
pub struct DataPoint {
    pub index: u32,
    pub value: f64,
}

impl DataPoint {
    pub fn new(index: u32, value: f64) -> Self {
        DataPoint { index, value }
    }

    pub fn to_xml(&self) -> String {
        format!(r#"<c:pt idx="{}"><c:v>{}</c:v></c:pt>"#, self.index, self.value)
    }
}

/// Category (string) value
#[derive(Debug, Clone)]
pub struct CategoryPoint {
    pub index: u32,
    pub value: String,
}

impl CategoryPoint {
    pub fn new(index: u32, value: &str) -> Self {
        CategoryPoint { index, value: value.to_string() }
    }

    pub fn to_xml(&self) -> String {
        format!(r#"<c:pt idx="{}"><c:v>{}</c:v></c:pt>"#, self.index, escape_xml(&self.value))
    }
}

/// Numeric data reference
#[derive(Debug, Clone)]
pub struct NumericData {
    pub formula: String,
    pub points: Vec<DataPoint>,
}

impl NumericData {
    pub fn new(formula: &str) -> Self {
        NumericData {
            formula: formula.to_string(),
            points: Vec::new(),
        }
    }

    pub fn add_point(mut self, index: u32, value: f64) -> Self {
        self.points.push(DataPoint::new(index, value));
        self
    }

    pub fn from_values(values: &[f64]) -> Self {
        NumericData::from_values_with_sheet(values, "Sheet1")
    }
    
    pub fn from_values_with_chart_number(values: &[f64], chart_number: usize) -> Self {
        let sheet_name = if chart_number == 1 {
            "Sheet1".to_string()
        } else {
            format!("Sheet{}", chart_number)
        };
        NumericData::from_values_with_sheet(values, &sheet_name)
    }
    
    pub fn from_values_with_sheet(values: &[f64], sheet_name: &str) -> Self {
        let formula = format!("'{}'!$B$2", sheet_name);
        let mut data = NumericData::new(&formula);
        for (i, &v) in values.iter().enumerate() {
            data.points.push(DataPoint::new(i as u32, v));
        }
        data
    }

    pub fn to_xml(&self) -> String {
        let mut xml = format!(
            r#"<c:numRef><c:f>{}</c:f><c:numCache><c:formatCode>General</c:formatCode><c:ptCount val="{}"/>"#,
            self.formula,
            self.points.len()
        );
        for pt in &self.points {
            xml.push_str(&pt.to_xml());
        }
        xml.push_str("</c:numCache></c:numRef>");
        xml
    }
}

/// String data reference
#[derive(Debug, Clone)]
pub struct StringData {
    pub formula: String,
    pub points: Vec<CategoryPoint>,
}

impl StringData {
    pub fn new(formula: &str) -> Self {
        StringData {
            formula: formula.to_string(),
            points: Vec::new(),
        }
    }

    pub fn from_categories(categories: &[&str]) -> Self {
        StringData::from_categories_with_sheet(categories, "Sheet1")
    }
    
    pub fn from_categories_with_chart_number(categories: &[&str], chart_number: usize) -> Self {
        let sheet_name = if chart_number == 1 {
            "Sheet1".to_string()
        } else {
            format!("Sheet{}", chart_number)
        };
        StringData::from_categories_with_sheet(categories, &sheet_name)
    }
    
    pub fn from_categories_with_sheet(categories: &[&str], sheet_name: &str) -> Self {
        let formula = format!("'{}'!$A$2", sheet_name);
        let mut data = StringData::new(&formula);
        for (i, &cat) in categories.iter().enumerate() {
            data.points.push(CategoryPoint::new(i as u32, cat));
        }
        data
    }

    pub fn to_xml(&self) -> String {
        let mut xml = format!(
            r#"<c:strRef><c:f>{}</c:f><c:strCache><c:ptCount val="{}"/>"#,
            self.formula,
            self.points.len()
        );
        for pt in &self.points {
            xml.push_str(&pt.to_xml());
        }
        xml.push_str("</c:strCache></c:strRef>");
        xml
    }
}

/// Chart series
#[derive(Debug, Clone)]
pub struct ChartSeries {
    pub index: u32,
    pub name: String,
    pub values: NumericData,
    pub categories: Option<StringData>,
}

impl ChartSeries {
    pub fn new(index: u32, name: &str, values: NumericData) -> Self {
        ChartSeries {
            index,
            name: name.to_string(),
            values,
            categories: None,
        }
    }

    pub fn with_categories(mut self, categories: StringData) -> Self {
        self.categories = Some(categories);
        self
    }

    pub fn parse(elem: &XmlElement) -> Option<Self> {
        ChartSeries::parse_with_chart_number(elem, 1)
    }
    
    pub fn parse_with_chart_number(elem: &XmlElement, chart_number: usize) -> Option<Self> {
        let index = elem.find("idx")
            .and_then(|e| e.attr("val"))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);

        let name = elem.find_descendant("t")
            .map(|t| t.text_content())
            .unwrap_or_default();

        // Parse values - use chart number for worksheet naming
        let sheet_name = if chart_number == 1 {
            "Sheet1".to_string()
        } else {
            format!("Sheet{}", chart_number)
        };
        let values = NumericData::new(&format!("'{}'!$B$2", sheet_name));

        Some(ChartSeries {
            index,
            name,
            values,
            categories: None,
        })
    }

    pub fn to_xml(&self) -> String {
        self.to_xml_with_sheet("Sheet1")
    }
    
    pub fn to_xml_with_chart_number(&self, chart_number: usize) -> String {
        let sheet_name = if chart_number == 1 {
            "Sheet1".to_string()
        } else {
            format!("Sheet{}", chart_number)
        };
        self.to_xml_with_sheet(&sheet_name)
    }

    pub fn to_xml_with_sheet(&self, sheet_name: &str) -> String {
        let formula = format!("'{}'!$B$1", sheet_name);
        let mut xml = format!(
            r#"<c:ser><c:idx val="{}"/><c:order val="{}"/><c:tx><c:strRef><c:f>{}</c:f><c:strCache><c:ptCount val="1"/><c:pt idx="0"><c:v>{}</c:v></c:pt></c:strCache></c:strRef></c:tx>"#,
            self.index,
            self.index,
            formula,
            escape_xml(&self.name)
        );

        if let Some(ref cats) = self.categories {
            xml.push_str("<c:cat>");
            xml.push_str(&cats.to_xml());
            xml.push_str("</c:cat>");
        }

        xml.push_str("<c:val>");
        xml.push_str(&self.values.to_xml());
        xml.push_str("</c:val>");
        xml.push_str("</c:ser>");
        xml
    }
}

/// Chart axis
#[derive(Debug, Clone)]
pub struct ChartAxis {
    pub id: u32,
    pub position: String,
    pub cross_axis_id: u32,
    pub delete: bool,
}

impl ChartAxis {
    pub fn category(id: u32, cross_id: u32) -> Self {
        ChartAxis {
            id,
            position: "b".to_string(),
            cross_axis_id: cross_id,
            delete: false,
        }
    }

    pub fn value(id: u32, cross_id: u32) -> Self {
        ChartAxis {
            id,
            position: "l".to_string(),
            cross_axis_id: cross_id,
            delete: false,
        }
    }

    pub fn to_category_xml(&self) -> String {
        format!(
            r#"<c:catAx><c:axId val="{}"/><c:scaling><c:orientation val="minMax"/></c:scaling><c:delete val="{}"/><c:axPos val="{}"/><c:majorTickMark val="out"/><c:minorTickMark val="none"/><c:tickLblPos val="nextTo"/><c:crossAx val="{}"/><c:crosses val="autoZero"/></c:catAx>"#,
            self.id,
            if self.delete { "1" } else { "0" },
            self.position,
            self.cross_axis_id
        )
    }

    pub fn to_value_xml(&self) -> String {
        format!(
            r#"<c:valAx><c:axId val="{}"/><c:scaling><c:orientation val="minMax"/></c:scaling><c:delete val="{}"/><c:axPos val="{}"/><c:majorGridlines/><c:numFmt formatCode="General" sourceLinked="1"/><c:majorTickMark val="out"/><c:minorTickMark val="none"/><c:tickLblPos val="nextTo"/><c:crossAx val="{}"/><c:crosses val="autoZero"/></c:valAx>"#,
            self.id,
            if self.delete { "1" } else { "0" },
            self.position,
            self.cross_axis_id
        )
    }
}

/// Chart legend
#[derive(Debug, Clone)]
pub struct ChartLegend {
    pub position: String,
    pub overlay: bool,
}

impl ChartLegend {
    pub fn new(position: &str) -> Self {
        ChartLegend {
            position: position.to_string(),
            overlay: false,
        }
    }

    pub fn right() -> Self {
        Self::new("r")
    }

    pub fn bottom() -> Self {
        Self::new("b")
    }

    pub fn to_xml(&self) -> String {
        format!(
            r#"<c:legend><c:legendPos val="{}"/><c:overlay val="{}"/></c:legend>"#,
            self.position,
            if self.overlay { "1" } else { "0" }
        )
    }
}

/// Chart title
#[derive(Debug, Clone)]
pub struct ChartTitle {
    pub text: String,
}

impl ChartTitle {
    pub fn new(text: &str) -> Self {
        ChartTitle { text: text.to_string() }
    }

    pub fn to_xml(&self) -> String {
        let lang_attrs = format_lang_attributes();
        format!(
            r#"<c:title><c:tx><c:rich><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr {} dirty="0"/><a:t>{}</a:t></a:r></a:p></c:rich></c:tx><c:overlay val="0"/></c:title>"#,
            lang_attrs,
            escape_xml(&self.text)
        )
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_kind() {
        assert_eq!(ChartKind::Bar.xml_element(), "c:barChart");
        assert_eq!(ChartKind::Pie.xml_element(), "c:pieChart");
    }

    #[test]
    fn test_data_point() {
        let pt = DataPoint::new(0, 42.5);
        let xml = pt.to_xml();
        assert!(xml.contains("idx=\"0\""));
        assert!(xml.contains("42.5"));
    }

    #[test]
    fn test_numeric_data() {
        let data = NumericData::from_values(&[10.0, 20.0, 30.0]);
        let xml = data.to_xml();
        assert!(xml.contains("numRef"));
        assert!(xml.contains("ptCount val=\"3\""));
    }

    #[test]
    fn test_chart_series() {
        let values = NumericData::from_values(&[100.0, 200.0]);
        let series = ChartSeries::new(0, "Sales", values);
        let xml = series.to_xml();
        
        assert!(xml.contains("<c:ser>"));
        assert!(xml.contains("Sales"));
    }

    #[test]
    fn test_chart_legend() {
        let legend = ChartLegend::right();
        let xml = legend.to_xml();
        assert!(xml.contains("legendPos val=\"r\""));
    }

    #[test]
    fn test_chart_title() {
        let title = ChartTitle::new("Revenue Report");
        let xml = title.to_xml();
        assert!(xml.contains("Revenue Report"));
    }
}
