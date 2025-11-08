//! SmartArt Graphics - SmartArt diagram support

/// SmartArt layout type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SmartArtLayoutType {
    /// Process layout
    Process,
    /// Hierarchy layout
    Hierarchy,
    /// Cycle layout
    Cycle,
    /// Relationship layout
    Relationship,
    /// Matrix layout
    Matrix,
    /// Pyramid layout
    Pyramid,
    /// List layout
    List,
    /// Picture layout
    Picture,
}

impl SmartArtLayoutType {
    /// Get layout ID
    pub fn layout_id(&self) -> &str {
        match self {
            SmartArtLayoutType::Process => "http://schemas.openxmlformats.org/drawingml/2006/smartart/process",
            SmartArtLayoutType::Hierarchy => "http://schemas.openxmlformats.org/drawingml/2006/smartart/hierarchy",
            SmartArtLayoutType::Cycle => "http://schemas.openxmlformats.org/drawingml/2006/smartart/cycle",
            SmartArtLayoutType::Relationship => "http://schemas.openxmlformats.org/drawingml/2006/smartart/relationship",
            SmartArtLayoutType::Matrix => "http://schemas.openxmlformats.org/drawingml/2006/smartart/matrix",
            SmartArtLayoutType::Pyramid => "http://schemas.openxmlformats.org/drawingml/2006/smartart/pyramid",
            SmartArtLayoutType::List => "http://schemas.openxmlformats.org/drawingml/2006/smartart/list",
            SmartArtLayoutType::Picture => "http://schemas.openxmlformats.org/drawingml/2006/smartart/picture",
        }
    }

    /// Get layout name
    pub fn name(&self) -> &str {
        match self {
            SmartArtLayoutType::Process => "Process",
            SmartArtLayoutType::Hierarchy => "Hierarchy",
            SmartArtLayoutType::Cycle => "Cycle",
            SmartArtLayoutType::Relationship => "Relationship",
            SmartArtLayoutType::Matrix => "Matrix",
            SmartArtLayoutType::Pyramid => "Pyramid",
            SmartArtLayoutType::List => "List",
            SmartArtLayoutType::Picture => "Picture",
        }
    }
}

/// SmartArt data point
#[derive(Clone, Debug)]
pub struct SmartArtDataPoint {
    /// Index
    index: u32,
    /// Text content
    text: String,
    /// Level (for hierarchical layouts)
    level: u32,
}

impl SmartArtDataPoint {
    /// Create a new data point
    pub fn new(index: u32, text: String) -> Self {
        Self {
            index,
            text,
            level: 0,
        }
    }

    /// Set level
    pub fn with_level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    /// Get index
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Get text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get level
    pub fn level(&self) -> u32 {
        self.level
    }
}

/// SmartArt graphic
#[derive(Clone, Debug)]
pub struct SmartArt {
    /// SmartArt ID
    id: u32,
    /// Layout type
    layout_type: SmartArtLayoutType,
    /// Data points
    data_points: Vec<SmartArtDataPoint>,
    /// Name
    name: String,
}

impl SmartArt {
    /// Create a new SmartArt
    pub fn new(id: u32, layout_type: SmartArtLayoutType, name: String) -> Self {
        Self {
            id,
            layout_type,
            data_points: vec![],
            name,
        }
    }

    /// Add a data point
    pub fn add_data_point(&mut self, point: SmartArtDataPoint) {
        self.data_points.push(point);
    }

    /// Add text data point
    pub fn add_text(&mut self, text: String) {
        let index = self.data_points.len() as u32;
        self.add_data_point(SmartArtDataPoint::new(index, text));
    }

    /// Add text with level
    pub fn add_text_with_level(&mut self, text: String, level: u32) {
        let index = self.data_points.len() as u32;
        self.add_data_point(SmartArtDataPoint::new(index, text).with_level(level));
    }

    /// Get SmartArt ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get layout type
    pub fn layout_type(&self) -> &SmartArtLayoutType {
        &self.layout_type
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get all data points
    pub fn data_points(&self) -> &[SmartArtDataPoint] {
        &self.data_points
    }

    /// Get data point count
    pub fn data_point_count(&self) -> usize {
        self.data_points.len()
    }

    /// Generate XML for SmartArt
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(&format!(
            r#"<p:graphicFrame id="{}" name="{}"><p:nvGraphicFramePr><p:cNvPr id="{}" name="{}"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>"#,
            self.id, self.name, self.id, self.name
        ));
        xml.push('\n');

        // Add data points
        xml.push_str(r#"<p:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/></p:xfrm>"#);
        xml.push('\n');

        xml.push_str(&format!(
            r#"<a:graphic><a:graphicData uri="{}"><dgm:relIds r:dm="rId1" r:lo="rId2" r:qs="rId3"/></a:graphicData></a:graphic>"#,
            self.layout_type.layout_id()
        ));
        xml.push('\n');

        xml.push_str(r#"</p:graphicFrame>"#);
        xml
    }
}

/// SmartArt Manager
#[derive(Clone, Debug)]
pub struct SmartArtManager {
    /// SmartArt graphics
    smartarts: Vec<SmartArt>,
}

impl SmartArtManager {
    /// Create a new SmartArt manager
    pub fn new() -> Self {
        Self {
            smartarts: vec![],
        }
    }

    /// Add a SmartArt
    pub fn add_smartart(&mut self, smartart: SmartArt) -> usize {
        self.smartarts.push(smartart);
        self.smartarts.len() - 1
    }

    /// Create and add a new SmartArt
    pub fn create_smartart(&mut self, layout_type: SmartArtLayoutType, name: String) -> usize {
        let id = self.smartarts.len() as u32;
        self.add_smartart(SmartArt::new(id, layout_type, name))
    }

    /// Get SmartArt by index
    pub fn get(&self, index: usize) -> Option<&SmartArt> {
        self.smartarts.get(index)
    }

    /// Get mutable SmartArt by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut SmartArt> {
        self.smartarts.get_mut(index)
    }

    /// Get all SmartArt
    pub fn all(&self) -> &[SmartArt] {
        &self.smartarts
    }

    /// Get number of SmartArt
    pub fn len(&self) -> usize {
        self.smartarts.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.smartarts.is_empty()
    }
}

impl Default for SmartArtManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smartart_layout_type_id() {
        assert!(SmartArtLayoutType::Process.layout_id().contains("process"));
        assert!(SmartArtLayoutType::Hierarchy.layout_id().contains("hierarchy"));
    }

    #[test]
    fn test_smartart_layout_type_name() {
        assert_eq!(SmartArtLayoutType::Process.name(), "Process");
        assert_eq!(SmartArtLayoutType::Hierarchy.name(), "Hierarchy");
    }

    #[test]
    fn test_smartart_data_point_creation() {
        let point = SmartArtDataPoint::new(0, "Item 1".to_string());
        assert_eq!(point.index(), 0);
        assert_eq!(point.text(), "Item 1");
        assert_eq!(point.level(), 0);
    }

    #[test]
    fn test_smartart_data_point_with_level() {
        let point = SmartArtDataPoint::new(0, "Item 1".to_string()).with_level(1);
        assert_eq!(point.level(), 1);
    }

    #[test]
    fn test_smartart_creation() {
        let smartart = SmartArt::new(1, SmartArtLayoutType::Process, "Diagram".to_string());
        assert_eq!(smartart.id(), 1);
        assert_eq!(smartart.name(), "Diagram");
        assert_eq!(smartart.data_point_count(), 0);
    }

    #[test]
    fn test_smartart_add_text() {
        let mut smartart = SmartArt::new(1, SmartArtLayoutType::Process, "Diagram".to_string());
        smartart.add_text("Step 1".to_string());
        smartart.add_text("Step 2".to_string());

        assert_eq!(smartart.data_point_count(), 2);
    }

    #[test]
    fn test_smartart_add_text_with_level() {
        let mut smartart = SmartArt::new(1, SmartArtLayoutType::Hierarchy, "Org Chart".to_string());
        smartart.add_text_with_level("CEO".to_string(), 0);
        smartart.add_text_with_level("Manager".to_string(), 1);

        assert_eq!(smartart.data_point_count(), 2);
        assert_eq!(smartart.data_points()[1].level(), 1);
    }

    #[test]
    fn test_smartart_to_xml() {
        let smartart = SmartArt::new(1, SmartArtLayoutType::Process, "Diagram".to_string());
        let xml = smartart.to_xml();
        assert!(xml.contains(r#"<p:graphicFrame"#));
        assert!(xml.contains(r#"name="Diagram""#));
        assert!(xml.contains(r#"<a:graphicData"#));
    }

    #[test]
    fn test_smartart_manager_creation() {
        let manager = SmartArtManager::new();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_smartart_manager_add() {
        let mut manager = SmartArtManager::new();
        let smartart = SmartArt::new(1, SmartArtLayoutType::Process, "Diagram".to_string());
        manager.add_smartart(smartart);

        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_smartart_manager_create() {
        let mut manager = SmartArtManager::new();
        manager.create_smartart(SmartArtLayoutType::Process, "Diagram".to_string());
        manager.create_smartart(SmartArtLayoutType::Hierarchy, "Org Chart".to_string());

        assert_eq!(manager.len(), 2);
    }

    #[test]
    fn test_smartart_manager_get() {
        let mut manager = SmartArtManager::new();
        manager.create_smartart(SmartArtLayoutType::Process, "Diagram".to_string());

        let smartart = manager.get(0);
        assert!(smartart.is_some());
        assert_eq!(smartart.unwrap().name(), "Diagram");
    }

    #[test]
    fn test_smartart_manager_get_mut() {
        let mut manager = SmartArtManager::new();
        manager.create_smartart(SmartArtLayoutType::Process, "Diagram".to_string());

        if let Some(smartart) = manager.get_mut(0) {
            smartart.add_text("Step 1".to_string());
        }

        assert_eq!(manager.get(0).unwrap().data_point_count(), 1);
    }

    #[test]
    fn test_smartart_manager_default() {
        let manager = SmartArtManager::default();
        assert!(manager.is_empty());
    }
}
