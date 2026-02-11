//! Connector support for linking shapes in PPTX
//!
//! Provides connector types and XML generation for connecting shapes.

use crate::core::escape_xml;

/// Connector types available in PPTX
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ConnectorType {
    /// Straight line connector
    Straight,
    /// Elbow (bent) connector
    Elbow,
    /// Curved connector
    Curved,
}

impl ConnectorType {
    /// Get the preset geometry name for the connector
    pub fn preset_name(&self) -> &'static str {
        match self {
            ConnectorType::Straight => "straightConnector1",
            ConnectorType::Elbow => "bentConnector3",
            ConnectorType::Curved => "curvedConnector3",
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ConnectorType::Straight => "Straight Connector",
            ConnectorType::Elbow => "Elbow Connector",
            ConnectorType::Curved => "Curved Connector",
        }
    }
}

/// Arrow head types for connectors
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ArrowType {
    /// No arrow
    None,
    /// Triangle arrow
    Triangle,
    /// Stealth arrow
    Stealth,
    /// Diamond arrow
    Diamond,
    /// Oval arrow
    Oval,
    /// Open arrow
    Open,
}

impl ArrowType {
    /// Get OOXML arrow type value
    pub fn xml_value(&self) -> &'static str {
        match self {
            ArrowType::None => "none",
            ArrowType::Triangle => "triangle",
            ArrowType::Stealth => "stealth",
            ArrowType::Diamond => "diamond",
            ArrowType::Oval => "oval",
            ArrowType::Open => "arrow",
        }
    }
}

/// Arrow size
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ArrowSize {
    Small,
    Medium,
    Large,
}

impl ArrowSize {
    /// Get OOXML size value
    pub fn xml_value(&self) -> &'static str {
        match self {
            ArrowSize::Small => "sm",
            ArrowSize::Medium => "med",
            ArrowSize::Large => "lg",
        }
    }
}

/// Connection point on a shape
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ConnectionSite {
    /// Top center
    Top,
    /// Bottom center
    Bottom,
    /// Left center
    Left,
    /// Right center
    Right,
    /// Top left corner
    TopLeft,
    /// Top right corner
    TopRight,
    /// Bottom left corner
    BottomLeft,
    /// Bottom right corner
    BottomRight,
    /// Center
    Center,
}

impl ConnectionSite {
    /// Get connection site index (0-based)
    pub fn index(&self) -> u32 {
        match self {
            ConnectionSite::Top => 0,
            ConnectionSite::Right => 1,
            ConnectionSite::Bottom => 2,
            ConnectionSite::Left => 3,
            ConnectionSite::TopLeft => 4,
            ConnectionSite::TopRight => 5,
            ConnectionSite::BottomRight => 6,
            ConnectionSite::BottomLeft => 7,
            ConnectionSite::Center => 8,
        }
    }
}

/// Connector line style
#[derive(Clone, Debug)]
pub struct ConnectorLine {
    /// Line color (RGB hex)
    pub color: String,
    /// Line width in EMU
    pub width: u32,
    /// Dash style
    pub dash: LineDash,
}

impl Default for ConnectorLine {
    fn default() -> Self {
        ConnectorLine {
            color: "000000".to_string(),
            width: 12700, // 1pt
            dash: LineDash::Solid,
        }
    }
}

impl ConnectorLine {
    /// Create new connector line
    pub fn new(color: &str, width: u32) -> Self {
        ConnectorLine {
            color: color.trim_start_matches('#').to_uppercase(),
            width,
            dash: LineDash::Solid,
        }
    }

    /// Set dash style
    pub fn with_dash(mut self, dash: LineDash) -> Self {
        self.dash = dash;
        self
    }
}

/// Line dash styles
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum LineDash {
    Solid,
    Dash,
    Dot,
    DashDot,
    DashDotDot,
    LongDash,
    LongDashDot,
}

impl LineDash {
    /// Get OOXML dash value
    pub fn xml_value(&self) -> &'static str {
        match self {
            LineDash::Solid => "solid",
            LineDash::Dash => "dash",
            LineDash::Dot => "dot",
            LineDash::DashDot => "dashDot",
            LineDash::DashDotDot => "lgDashDotDot",
            LineDash::LongDash => "lgDash",
            LineDash::LongDashDot => "lgDashDot",
        }
    }
}

/// Connector definition
#[derive(Clone, Debug)]
pub struct Connector {
    /// Connector type
    pub connector_type: ConnectorType,
    /// Start X position in EMU
    pub start_x: u32,
    /// Start Y position in EMU
    pub start_y: u32,
    /// End X position in EMU
    pub end_x: u32,
    /// End Y position in EMU
    pub end_y: u32,
    /// Line style
    pub line: ConnectorLine,
    /// Start arrow
    pub start_arrow: ArrowType,
    /// End arrow
    pub end_arrow: ArrowType,
    /// Arrow size
    pub arrow_size: ArrowSize,
    /// Connected shape ID at start (optional)
    pub start_shape_id: Option<u32>,
    /// Connection site at start shape
    pub start_site: Option<ConnectionSite>,
    /// Connected shape ID at end (optional)
    pub end_shape_id: Option<u32>,
    /// Connection site at end shape
    pub end_site: Option<ConnectionSite>,
    /// Optional label text
    pub label: Option<String>,
}

impl Connector {
    /// Create a new connector
    pub fn new(
        connector_type: ConnectorType,
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
    ) -> Self {
        Connector {
            connector_type,
            start_x,
            start_y,
            end_x,
            end_y,
            line: ConnectorLine::default(),
            start_arrow: ArrowType::None,
            end_arrow: ArrowType::None,
            arrow_size: ArrowSize::Medium,
            start_shape_id: None,
            start_site: None,
            end_shape_id: None,
            end_site: None,
            label: None,
        }
    }

    /// Create a straight connector
    pub fn straight(start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> Self {
        Self::new(ConnectorType::Straight, start_x, start_y, end_x, end_y)
    }

    /// Create an elbow connector
    pub fn elbow(start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> Self {
        Self::new(ConnectorType::Elbow, start_x, start_y, end_x, end_y)
    }

    /// Create a curved connector
    pub fn curved(start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> Self {
        Self::new(ConnectorType::Curved, start_x, start_y, end_x, end_y)
    }

    /// Set line style
    pub fn with_line(mut self, line: ConnectorLine) -> Self {
        self.line = line;
        self
    }

    /// Set line color
    pub fn with_color(mut self, color: &str) -> Self {
        self.line.color = color.trim_start_matches('#').to_uppercase();
        self
    }

    /// Set line width in EMU
    pub fn with_width(mut self, width: u32) -> Self {
        self.line.width = width;
        self
    }

    /// Set start arrow
    pub fn with_start_arrow(mut self, arrow: ArrowType) -> Self {
        self.start_arrow = arrow;
        self
    }

    /// Set end arrow
    pub fn with_end_arrow(mut self, arrow: ArrowType) -> Self {
        self.end_arrow = arrow;
        self
    }

    /// Set both arrows
    pub fn with_arrows(mut self, start: ArrowType, end: ArrowType) -> Self {
        self.start_arrow = start;
        self.end_arrow = end;
        self
    }

    /// Set arrow size
    pub fn with_arrow_size(mut self, size: ArrowSize) -> Self {
        self.arrow_size = size;
        self
    }

    /// Connect to start shape
    pub fn connect_start(mut self, shape_id: u32, site: ConnectionSite) -> Self {
        self.start_shape_id = Some(shape_id);
        self.start_site = Some(site);
        self
    }

    /// Connect to end shape
    pub fn connect_end(mut self, shape_id: u32, site: ConnectionSite) -> Self {
        self.end_shape_id = Some(shape_id);
        self.end_site = Some(site);
        self
    }

    /// Add label text
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// Calculate width for XML
    fn width(&self) -> u32 {
        self.end_x.abs_diff(self.start_x)
    }

    /// Calculate height for XML
    fn height(&self) -> u32 {
        self.end_y.abs_diff(self.start_y)
    }

    /// Check if connector is flipped horizontally
    fn flip_h(&self) -> bool {
        self.end_x < self.start_x
    }

    /// Check if connector is flipped vertically
    fn flip_v(&self) -> bool {
        self.end_y < self.start_y
    }
}

/// Generate connector XML for a slide
pub fn generate_connector_xml(connector: &Connector, shape_id: usize) -> String {
    let x = connector.start_x.min(connector.end_x);
    let y = connector.start_y.min(connector.end_y);
    let cx = connector.width();
    let cy = connector.height();

    let flip_h = if connector.flip_h() { " flipH=\"1\"" } else { "" };
    let flip_v = if connector.flip_v() { " flipV=\"1\"" } else { "" };

    let mut xml = format!(
        r#"<p:cxnSp>
<p:nvCxnSpPr>
<p:cNvPr id="{}" name="Connector {}"/>
<p:cNvCxnSpPr>"#,
        shape_id, shape_id
    );

    // Add connection references if connected to shapes
    if let (Some(start_id), Some(start_site)) = (connector.start_shape_id, connector.start_site) {
        xml.push_str(&format!(
            r#"
<a:stCxn id="{}" idx="{}"/>"#,
            start_id, start_site.index()
        ));
    }

    if let (Some(end_id), Some(end_site)) = (connector.end_shape_id, connector.end_site) {
        xml.push_str(&format!(
            r#"
<a:endCxn id="{}" idx="{}"/>"#,
            end_id, end_site.index()
        ));
    }

    xml.push_str(&format!(
        r#"
</p:cNvCxnSpPr>
<p:nvPr/>
</p:nvCxnSpPr>
<p:spPr>
<a:xfrm{}{}>
<a:off x="{}" y="{}"/>
<a:ext cx="{}" cy="{}"/>
</a:xfrm>
<a:prstGeom prst="{}">
<a:avLst/>
</a:prstGeom>
<a:ln w="{}">
<a:solidFill>
<a:srgbClr val="{}"/>
</a:solidFill>
<a:prstDash val="{}"/>"#,
        flip_h, flip_v,
        x, y, cx, cy,
        connector.connector_type.preset_name(),
        connector.line.width,
        connector.line.color,
        connector.line.dash.xml_value()
    ));

    // Add arrow heads
    if connector.start_arrow != ArrowType::None {
        xml.push_str(&format!(
            r#"
<a:headEnd type="{}" w="{}" len="{}"/>"#,
            connector.start_arrow.xml_value(),
            connector.arrow_size.xml_value(),
            connector.arrow_size.xml_value()
        ));
    }

    if connector.end_arrow != ArrowType::None {
        xml.push_str(&format!(
            r#"
<a:tailEnd type="{}" w="{}" len="{}"/>"#,
            connector.end_arrow.xml_value(),
            connector.arrow_size.xml_value(),
            connector.arrow_size.xml_value()
        ));
    }

    xml.push_str(r#"
</a:ln>
</p:spPr>"#);

    // Add label if present
    if let Some(label) = &connector.label {
        xml.push_str(&format!(
            r#"
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US" sz="1000"/>
<a:t>{}</a:t>
</a:r>
</a:p>
</p:txBody>"#,
            escape_xml(label)
        ));
    }

    xml.push_str(r#"
</p:cxnSp>"#);

    xml
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connector_type_preset() {
        assert_eq!(ConnectorType::Straight.preset_name(), "straightConnector1");
        assert_eq!(ConnectorType::Elbow.preset_name(), "bentConnector3");
        assert_eq!(ConnectorType::Curved.preset_name(), "curvedConnector3");
    }

    #[test]
    fn test_arrow_type_xml() {
        assert_eq!(ArrowType::None.xml_value(), "none");
        assert_eq!(ArrowType::Triangle.xml_value(), "triangle");
        assert_eq!(ArrowType::Stealth.xml_value(), "stealth");
    }

    #[test]
    fn test_connector_builder() {
        let conn = Connector::straight(0, 0, 1000000, 500000)
            .with_color("FF0000")
            .with_end_arrow(ArrowType::Triangle);

        assert_eq!(conn.line.color, "FF0000");
        assert_eq!(conn.end_arrow, ArrowType::Triangle);
    }

    #[test]
    fn test_connector_with_connections() {
        let conn = Connector::elbow(0, 0, 1000000, 500000)
            .connect_start(1, ConnectionSite::Right)
            .connect_end(2, ConnectionSite::Left);

        assert_eq!(conn.start_shape_id, Some(1));
        assert_eq!(conn.start_site, Some(ConnectionSite::Right));
        assert_eq!(conn.end_shape_id, Some(2));
        assert_eq!(conn.end_site, Some(ConnectionSite::Left));
    }

    #[test]
    fn test_generate_connector_xml() {
        let conn = Connector::straight(0, 0, 1000000, 500000)
            .with_end_arrow(ArrowType::Triangle);

        let xml = generate_connector_xml(&conn, 1);
        assert!(xml.contains("p:cxnSp"));
        assert!(xml.contains("straightConnector1"));
        assert!(xml.contains("tailEnd"));
    }

    #[test]
    fn test_connector_with_label() {
        let conn = Connector::straight(0, 0, 1000000, 500000)
            .with_label("Connection");

        let xml = generate_connector_xml(&conn, 1);
        assert!(xml.contains("Connection"));
        assert!(xml.contains("p:txBody"));
    }

    #[test]
    fn test_line_dash_styles() {
        assert_eq!(LineDash::Solid.xml_value(), "solid");
        assert_eq!(LineDash::Dash.xml_value(), "dash");
        assert_eq!(LineDash::Dot.xml_value(), "dot");
        assert_eq!(LineDash::DashDot.xml_value(), "dashDot");
    }

    #[test]
    fn test_connection_site_index() {
        assert_eq!(ConnectionSite::Top.index(), 0);
        assert_eq!(ConnectionSite::Right.index(), 1);
        assert_eq!(ConnectionSite::Bottom.index(), 2);
        assert_eq!(ConnectionSite::Left.index(), 3);
    }
}
