//! Placeholder Shapes - Placeholder support in layouts and slides

/// Placeholder type enumeration
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlaceholderType {
    /// Title placeholder
    Title,
    /// Body/content placeholder
    Body,
    /// Center title placeholder
    CenterTitle,
    /// Subtitle placeholder
    Subtitle,
    /// Object placeholder
    Object,
    /// Chart placeholder
    Chart,
    /// Table placeholder
    Table,
    /// Clip art placeholder
    ClipArt,
    /// Diagram placeholder
    Diagram,
    /// Media placeholder
    Media,
    /// Slide number placeholder
    SlideNumber,
    /// Footer placeholder
    Footer,
    /// Header placeholder
    Header,
    /// Date placeholder
    Date,
}

impl PlaceholderType {
    /// Get placeholder type string for XML
    pub fn type_str(&self) -> &str {
        match self {
            PlaceholderType::Title => "title",
            PlaceholderType::Body => "body",
            PlaceholderType::CenterTitle => "ctrTitle",
            PlaceholderType::Subtitle => "subTitle",
            PlaceholderType::Object => "obj",
            PlaceholderType::Chart => "chart",
            PlaceholderType::Table => "tbl",
            PlaceholderType::ClipArt => "clipArt",
            PlaceholderType::Diagram => "dgm",
            PlaceholderType::Media => "media",
            PlaceholderType::SlideNumber => "sldNum",
            PlaceholderType::Footer => "ftr",
            PlaceholderType::Header => "hf",
            PlaceholderType::Date => "dt",
        }
    }
}

/// Placeholder Shape
#[derive(Clone, Debug)]
pub struct Placeholder {
    /// Placeholder type
    placeholder_type: PlaceholderType,
    /// Placeholder index
    index: Option<u32>,
    /// Placeholder ID
    id: u32,
    /// Placeholder name
    name: String,
}

impl Placeholder {
    /// Create a new placeholder
    pub fn new(placeholder_type: PlaceholderType, id: u32) -> Self {
        let name = format!("{} Placeholder", placeholder_type.type_str());
        Self {
            placeholder_type,
            index: None,
            id,
            name,
        }
    }

    /// Set placeholder index
    pub fn with_index(mut self, index: u32) -> Self {
        self.index = Some(index);
        self
    }

    /// Get placeholder type
    pub fn placeholder_type(&self) -> &PlaceholderType {
        &self.placeholder_type
    }

    /// Get placeholder index
    pub fn index(&self) -> Option<u32> {
        self.index
    }

    /// Get placeholder ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get placeholder name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Generate XML for placeholder element
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(&format!(r#"<p:ph type="{}""#, self.placeholder_type.type_str()));
        
        if let Some(idx) = self.index {
            xml.push_str(&format!(r#" idx="{}""#, idx));
        }
        
        xml.push_str("/>");
        xml
    }
}

/// Placeholder Manager - Manages placeholders in a layout
#[derive(Clone, Debug)]
pub struct PlaceholderManager {
    /// List of placeholders
    placeholders: Vec<Placeholder>,
    /// Next placeholder ID
    next_id: u32,
}

impl PlaceholderManager {
    /// Create a new placeholder manager
    pub fn new() -> Self {
        Self {
            placeholders: vec![],
            next_id: 2, // Start at 2 (1 is reserved for group shape)
        }
    }

    /// Add a placeholder
    pub fn add_placeholder(&mut self, placeholder_type: PlaceholderType) -> Placeholder {
        let id = self.next_id;
        self.next_id += 1;
        
        let placeholder = Placeholder::new(placeholder_type, id);
        self.placeholders.push(placeholder.clone());
        placeholder
    }

    /// Add a placeholder with index
    pub fn add_placeholder_with_index(&mut self, placeholder_type: PlaceholderType, index: u32) -> Placeholder {
        let id = self.next_id;
        self.next_id += 1;
        
        let placeholder = Placeholder::new(placeholder_type, id).with_index(index);
        self.placeholders.push(placeholder.clone());
        placeholder
    }

    /// Get all placeholders
    pub fn all(&self) -> &[Placeholder] {
        &self.placeholders
    }

    /// Get number of placeholders
    pub fn len(&self) -> usize {
        self.placeholders.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.placeholders.is_empty()
    }
}

impl Default for PlaceholderManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_type_str() {
        assert_eq!(PlaceholderType::Title.type_str(), "title");
        assert_eq!(PlaceholderType::Body.type_str(), "body");
        assert_eq!(PlaceholderType::CenterTitle.type_str(), "ctrTitle");
    }

    #[test]
    fn test_placeholder_creation() {
        let ph = Placeholder::new(PlaceholderType::Title, 2);
        assert_eq!(ph.id(), 2);
        assert_eq!(ph.placeholder_type(), &PlaceholderType::Title);
        assert_eq!(ph.index(), None);
    }

    #[test]
    fn test_placeholder_with_index() {
        let ph = Placeholder::new(PlaceholderType::Body, 3).with_index(1);
        assert_eq!(ph.index(), Some(1));
    }

    #[test]
    fn test_placeholder_to_xml() {
        let ph = Placeholder::new(PlaceholderType::Title, 2);
        let xml = ph.to_xml();
        assert!(xml.contains(r#"<p:ph type="title""#));
        assert!(xml.contains("/>"));
    }

    #[test]
    fn test_placeholder_to_xml_with_index() {
        let ph = Placeholder::new(PlaceholderType::Body, 3).with_index(1);
        let xml = ph.to_xml();
        assert!(xml.contains(r#"<p:ph type="body""#));
        assert!(xml.contains(r#"idx="1""#));
    }

    #[test]
    fn test_placeholder_manager_creation() {
        let manager = PlaceholderManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_placeholder_manager_add() {
        let mut manager = PlaceholderManager::new();
        let ph1 = manager.add_placeholder(PlaceholderType::Title);
        let ph2 = manager.add_placeholder(PlaceholderType::Body);
        
        assert_eq!(manager.len(), 2);
        assert_eq!(ph1.id(), 2);
        assert_eq!(ph2.id(), 3);
    }

    #[test]
    fn test_placeholder_manager_add_with_index() {
        let mut manager = PlaceholderManager::new();
        let ph = manager.add_placeholder_with_index(PlaceholderType::Body, 1);
        
        assert_eq!(ph.index(), Some(1));
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_placeholder_manager_all() {
        let mut manager = PlaceholderManager::new();
        manager.add_placeholder(PlaceholderType::Title);
        manager.add_placeholder(PlaceholderType::Body);
        
        let all = manager.all();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_placeholder_manager_default() {
        let manager = PlaceholderManager::default();
        assert!(manager.is_empty());
    }
}
