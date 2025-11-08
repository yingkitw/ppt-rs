//! Chart Enhancements - Trendlines and error bars for charts

/// Trendline type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrendlineType {
    /// Linear trendline
    Linear,
    /// Exponential trendline
    Exponential,
    /// Logarithmic trendline
    Logarithmic,
    /// Power trendline
    Power,
    /// Polynomial trendline
    Polynomial,
    /// Moving average trendline
    MovingAverage,
}

impl TrendlineType {
    /// Get trendline type string
    pub fn type_str(&self) -> &str {
        match self {
            TrendlineType::Linear => "linear",
            TrendlineType::Exponential => "exp",
            TrendlineType::Logarithmic => "log",
            TrendlineType::Power => "power",
            TrendlineType::Polynomial => "poly",
            TrendlineType::MovingAverage => "movingAvg",
        }
    }
}

/// Trendline
#[derive(Clone, Debug)]
pub struct Trendline {
    /// Trendline type
    trendline_type: TrendlineType,
    /// Name
    name: Option<String>,
    /// Show equation
    show_equation: bool,
    /// Show R-squared value
    show_r_squared: bool,
    /// Polynomial order (for polynomial trendlines)
    polynomial_order: u32,
    /// Period (for moving average)
    period: u32,
}

impl Trendline {
    /// Create a new trendline
    pub fn new(trendline_type: TrendlineType) -> Self {
        Self {
            trendline_type,
            name: None,
            show_equation: false,
            show_r_squared: false,
            polynomial_order: 2,
            period: 2,
        }
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set show equation
    pub fn set_show_equation(&mut self, show: bool) {
        self.show_equation = show;
    }

    /// Get show equation
    pub fn show_equation(&self) -> bool {
        self.show_equation
    }

    /// Set show R-squared
    pub fn set_show_r_squared(&mut self, show: bool) {
        self.show_r_squared = show;
    }

    /// Get show R-squared
    pub fn show_r_squared(&self) -> bool {
        self.show_r_squared
    }

    /// Set polynomial order
    pub fn set_polynomial_order(&mut self, order: u32) {
        self.polynomial_order = order;
    }

    /// Get polynomial order
    pub fn polynomial_order(&self) -> u32 {
        self.polynomial_order
    }

    /// Set period
    pub fn set_period(&mut self, period: u32) {
        self.period = period;
    }

    /// Get period
    pub fn period(&self) -> u32 {
        self.period
    }

    /// Generate XML for trendline
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(&format!(r#"<c:trendline><c:trendlineType val="{}"/>"#, self.trendline_type.type_str()));

        if let Some(name) = &self.name {
            xml.push_str(&format!(r#"<c:name>{}</c:name>"#, name));
        }

        if self.show_equation {
            xml.push_str(r#"<c:dispEq val="1"/>"#);
        }

        if self.show_r_squared {
            xml.push_str(r#"<c:dispRSqr val="1"/>"#);
        }

        xml.push_str(r#"</c:trendline>"#);
        xml
    }
}

/// Error bar type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorBarType {
    /// Standard deviation
    StandardDeviation,
    /// Standard error
    StandardError,
    /// Percentage
    Percentage,
    /// Fixed value
    Fixed,
}

impl ErrorBarType {
    /// Get error bar type string
    pub fn type_str(&self) -> &str {
        match self {
            ErrorBarType::StandardDeviation => "stdDev",
            ErrorBarType::StandardError => "stdErr",
            ErrorBarType::Percentage => "percentage",
            ErrorBarType::Fixed => "fixed",
        }
    }
}

/// Error bar direction
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorBarDirection {
    /// Plus only
    Plus,
    /// Minus only
    Minus,
    /// Both plus and minus
    Both,
}

impl ErrorBarDirection {
    /// Get direction string
    pub fn dir_str(&self) -> &str {
        match self {
            ErrorBarDirection::Plus => "plus",
            ErrorBarDirection::Minus => "minus",
            ErrorBarDirection::Both => "both",
        }
    }
}

/// Error bar
#[derive(Clone, Debug)]
pub struct ErrorBar {
    /// Error bar type
    error_bar_type: ErrorBarType,
    /// Direction
    direction: ErrorBarDirection,
    /// Value (for fixed type)
    value: f64,
    /// Plus value (for custom)
    plus_value: Option<f64>,
    /// Minus value (for custom)
    minus_value: Option<f64>,
}

impl ErrorBar {
    /// Create a new error bar
    pub fn new(error_bar_type: ErrorBarType, direction: ErrorBarDirection) -> Self {
        Self {
            error_bar_type,
            direction,
            value: 0.0,
            plus_value: None,
            minus_value: None,
        }
    }

    /// Set value
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    /// Get value
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Set plus value
    pub fn set_plus_value(&mut self, value: f64) {
        self.plus_value = Some(value);
    }

    /// Get plus value
    pub fn plus_value(&self) -> Option<f64> {
        self.plus_value
    }

    /// Set minus value
    pub fn set_minus_value(&mut self, value: f64) {
        self.minus_value = Some(value);
    }

    /// Get minus value
    pub fn minus_value(&self) -> Option<f64> {
        self.minus_value
    }

    /// Generate XML for error bar
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(&format!(
            r#"<c:errBars><c:errBarType val="{}"/><c:errDir val="{}"/>"#,
            self.error_bar_type.type_str(),
            self.direction.dir_str()
        ));

        if let Some(plus) = self.plus_value {
            xml.push_str(&format!(r#"<c:plus val="{}"/>"#, plus));
        }

        if let Some(minus) = self.minus_value {
            xml.push_str(&format!(r#"<c:minus val="{}"/>"#, minus));
        }

        xml.push_str(r#"</c:errBars>"#);
        xml
    }
}

/// Chart enhancement manager
#[derive(Clone, Debug)]
pub struct ChartEnhancementManager {
    /// Trendlines
    trendlines: Vec<Trendline>,
    /// Error bars
    error_bars: Vec<ErrorBar>,
}

impl ChartEnhancementManager {
    /// Create a new chart enhancement manager
    pub fn new() -> Self {
        Self {
            trendlines: vec![],
            error_bars: vec![],
        }
    }

    /// Add a trendline
    pub fn add_trendline(&mut self, trendline: Trendline) -> usize {
        self.trendlines.push(trendline);
        self.trendlines.len() - 1
    }

    /// Create and add a trendline
    pub fn create_trendline(&mut self, trendline_type: TrendlineType) -> usize {
        self.add_trendline(Trendline::new(trendline_type))
    }

    /// Get trendline by index
    pub fn get_trendline(&self, index: usize) -> Option<&Trendline> {
        self.trendlines.get(index)
    }

    /// Get mutable trendline by index
    pub fn get_trendline_mut(&mut self, index: usize) -> Option<&mut Trendline> {
        self.trendlines.get_mut(index)
    }

    /// Get all trendlines
    pub fn trendlines(&self) -> &[Trendline] {
        &self.trendlines
    }

    /// Add an error bar
    pub fn add_error_bar(&mut self, error_bar: ErrorBar) -> usize {
        self.error_bars.push(error_bar);
        self.error_bars.len() - 1
    }

    /// Create and add an error bar
    pub fn create_error_bar(&mut self, error_bar_type: ErrorBarType, direction: ErrorBarDirection) -> usize {
        self.add_error_bar(ErrorBar::new(error_bar_type, direction))
    }

    /// Get error bar by index
    pub fn get_error_bar(&self, index: usize) -> Option<&ErrorBar> {
        self.error_bars.get(index)
    }

    /// Get mutable error bar by index
    pub fn get_error_bar_mut(&mut self, index: usize) -> Option<&mut ErrorBar> {
        self.error_bars.get_mut(index)
    }

    /// Get all error bars
    pub fn error_bars(&self) -> &[ErrorBar] {
        &self.error_bars
    }

    /// Get trendline count
    pub fn trendline_count(&self) -> usize {
        self.trendlines.len()
    }

    /// Get error bar count
    pub fn error_bar_count(&self) -> usize {
        self.error_bars.len()
    }
}

impl Default for ChartEnhancementManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trendline_type_str() {
        assert_eq!(TrendlineType::Linear.type_str(), "linear");
        assert_eq!(TrendlineType::Exponential.type_str(), "exp");
        assert_eq!(TrendlineType::Polynomial.type_str(), "poly");
    }

    #[test]
    fn test_trendline_creation() {
        let trendline = Trendline::new(TrendlineType::Linear);
        assert!(!trendline.show_equation());
        assert!(!trendline.show_r_squared());
    }

    #[test]
    fn test_trendline_properties() {
        let mut trendline = Trendline::new(TrendlineType::Polynomial);
        trendline.set_name("Trend".to_string());
        trendline.set_show_equation(true);
        trendline.set_show_r_squared(true);
        trendline.set_polynomial_order(3);

        assert_eq!(trendline.name(), Some("Trend"));
        assert!(trendline.show_equation());
        assert!(trendline.show_r_squared());
        assert_eq!(trendline.polynomial_order(), 3);
    }

    #[test]
    fn test_trendline_to_xml() {
        let trendline = Trendline::new(TrendlineType::Linear);
        let xml = trendline.to_xml();
        assert!(xml.contains(r#"<c:trendline>"#));
        assert!(xml.contains(r#"val="linear""#));
        assert!(xml.contains(r#"</c:trendline>"#));
    }

    #[test]
    fn test_error_bar_type_str() {
        assert_eq!(ErrorBarType::StandardDeviation.type_str(), "stdDev");
        assert_eq!(ErrorBarType::Percentage.type_str(), "percentage");
        assert_eq!(ErrorBarType::Fixed.type_str(), "fixed");
    }

    #[test]
    fn test_error_bar_direction_str() {
        assert_eq!(ErrorBarDirection::Plus.dir_str(), "plus");
        assert_eq!(ErrorBarDirection::Minus.dir_str(), "minus");
        assert_eq!(ErrorBarDirection::Both.dir_str(), "both");
    }

    #[test]
    fn test_error_bar_creation() {
        let error_bar = ErrorBar::new(ErrorBarType::StandardDeviation, ErrorBarDirection::Both);
        assert_eq!(error_bar.value(), 0.0);
        assert!(error_bar.plus_value().is_none());
    }

    #[test]
    fn test_error_bar_properties() {
        let mut error_bar = ErrorBar::new(ErrorBarType::Fixed, ErrorBarDirection::Plus);
        error_bar.set_value(5.0);
        error_bar.set_plus_value(10.0);

        assert_eq!(error_bar.value(), 5.0);
        assert_eq!(error_bar.plus_value(), Some(10.0));
    }

    #[test]
    fn test_error_bar_to_xml() {
        let error_bar = ErrorBar::new(ErrorBarType::StandardDeviation, ErrorBarDirection::Both);
        let xml = error_bar.to_xml();
        assert!(xml.contains(r#"<c:errBars>"#));
        assert!(xml.contains(r#"val="stdDev""#));
        assert!(xml.contains(r#"val="both""#));
        assert!(xml.contains(r#"</c:errBars>"#));
    }

    #[test]
    fn test_chart_enhancement_manager_creation() {
        let manager = ChartEnhancementManager::new();
        assert_eq!(manager.trendline_count(), 0);
        assert_eq!(manager.error_bar_count(), 0);
    }

    #[test]
    fn test_chart_enhancement_manager_add_trendline() {
        let mut manager = ChartEnhancementManager::new();
        manager.create_trendline(TrendlineType::Linear);
        manager.create_trendline(TrendlineType::Polynomial);

        assert_eq!(manager.trendline_count(), 2);
    }

    #[test]
    fn test_chart_enhancement_manager_add_error_bar() {
        let mut manager = ChartEnhancementManager::new();
        manager.create_error_bar(ErrorBarType::StandardDeviation, ErrorBarDirection::Both);
        manager.create_error_bar(ErrorBarType::Percentage, ErrorBarDirection::Plus);

        assert_eq!(manager.error_bar_count(), 2);
    }

    #[test]
    fn test_chart_enhancement_manager_get_trendline() {
        let mut manager = ChartEnhancementManager::new();
        manager.create_trendline(TrendlineType::Linear);

        let trendline = manager.get_trendline(0);
        assert!(trendline.is_some());
    }

    #[test]
    fn test_chart_enhancement_manager_get_error_bar() {
        let mut manager = ChartEnhancementManager::new();
        manager.create_error_bar(ErrorBarType::Fixed, ErrorBarDirection::Both);

        let error_bar = manager.get_error_bar(0);
        assert!(error_bar.is_some());
    }

    #[test]
    fn test_chart_enhancement_manager_get_mut() {
        let mut manager = ChartEnhancementManager::new();
        manager.create_trendline(TrendlineType::Linear);

        if let Some(trendline) = manager.get_trendline_mut(0) {
            trendline.set_show_equation(true);
        }

        assert!(manager.get_trendline(0).unwrap().show_equation());
    }

    #[test]
    fn test_chart_enhancement_manager_default() {
        let manager = ChartEnhancementManager::default();
        assert_eq!(manager.trendline_count(), 0);
    }
}
