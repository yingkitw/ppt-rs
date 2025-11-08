//! Enhanced shape content management with layout integration

use crate::error::Result;

/// Shape content type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeContentType {
    /// Text content
    Text,
    /// Picture content
    Picture,
    /// Chart content
    Chart,
    /// Table content
    Table,
    /// SmartArt content
    SmartArt,
    /// OLE object
    OleObject,
    /// Media (video/audio)
    Media,
    /// Placeholder
    Placeholder,
}

/// Shape placeholder type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceholderType {
    /// Title placeholder
    Title,
    /// Body/content placeholder
    Body,
    /// Centered title
    CenteredTitle,
    /// Subtitle
    Subtitle,
    /// Date/time
    DateTime,
    /// Slide number
    SlideNumber,
    /// Footer
    Footer,
    /// Header
    Header,
    /// Object
    Object,
    /// Chart
    Chart,
    /// Table
    Table,
    /// Clip art
    ClipArt,
    /// Diagram
    Diagram,
    /// Media
    Media,
    /// Slide image
    SlideImage,
}

impl PlaceholderType {
    /// Get placeholder type name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Body => "body",
            Self::CenteredTitle => "ctrTitle",
            Self::Subtitle => "subTitle",
            Self::DateTime => "dt",
            Self::SlideNumber => "sldNum",
            Self::Footer => "ftr",
            Self::Header => "hf",
            Self::Object => "obj",
            Self::Chart => "chart",
            Self::Table => "tbl",
            Self::ClipArt => "clipArt",
            Self::Diagram => "dgm",
            Self::Media => "media",
            Self::SlideImage => "sldImg",
        }
    }

    /// Parse from name
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "title" => Some(Self::Title),
            "body" => Some(Self::Body),
            "ctrTitle" => Some(Self::CenteredTitle),
            "subTitle" => Some(Self::Subtitle),
            "dt" => Some(Self::DateTime),
            "sldNum" => Some(Self::SlideNumber),
            "ftr" => Some(Self::Footer),
            "hf" => Some(Self::Header),
            "obj" => Some(Self::Object),
            "chart" => Some(Self::Chart),
            "tbl" => Some(Self::Table),
            "clipArt" => Some(Self::ClipArt),
            "dgm" => Some(Self::Diagram),
            "media" => Some(Self::Media),
            "sldImg" => Some(Self::SlideImage),
            _ => None,
        }
    }
}

/// Shape content descriptor
#[derive(Debug, Clone)]
pub struct ShapeContent {
    /// Content type
    pub content_type: ShapeContentType,
    /// Placeholder type (if applicable)
    pub placeholder: Option<PlaceholderType>,
    /// Content data
    pub data: Vec<u8>,
    /// Metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl ShapeContent {
    /// Create new shape content
    pub fn new(content_type: ShapeContentType) -> Self {
        Self {
            content_type,
            placeholder: None,
            data: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create placeholder content
    pub fn placeholder(placeholder_type: PlaceholderType) -> Self {
        Self {
            content_type: ShapeContentType::Placeholder,
            placeholder: Some(placeholder_type),
            data: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set content data
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }

    /// Check if content is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get content size
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

/// Shape content loader for loading from layouts
pub struct ShapeContentLoader;

impl ShapeContentLoader {
    /// Load default content for placeholder type
    pub fn load_default(placeholder_type: PlaceholderType) -> Result<ShapeContent> {
        let mut content = ShapeContent::placeholder(placeholder_type);
        
        // Add default metadata based on placeholder type
        match placeholder_type {
            PlaceholderType::Title => {
                content.add_metadata("default_text".to_string(), "Click to add title".to_string());
            }
            PlaceholderType::Body => {
                content.add_metadata("default_text".to_string(), "Click to add text".to_string());
            }
            PlaceholderType::Subtitle => {
                content.add_metadata("default_text".to_string(), "Click to add subtitle".to_string());
            }
            _ => {}
        }
        
        Ok(content)
    }

    /// Load content from layout
    pub fn load_from_layout(layout_name: &str, placeholder_type: PlaceholderType) -> Result<ShapeContent> {
        let mut content = Self::load_default(placeholder_type)?;
        content.add_metadata("layout".to_string(), layout_name.to_string());
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_type_name() {
        assert_eq!(PlaceholderType::Title.name(), "title");
        assert_eq!(PlaceholderType::Body.name(), "body");
        assert_eq!(PlaceholderType::CenteredTitle.name(), "ctrTitle");
    }

    #[test]
    fn test_placeholder_type_from_name() {
        assert_eq!(PlaceholderType::from_name("title"), Some(PlaceholderType::Title));
        assert_eq!(PlaceholderType::from_name("body"), Some(PlaceholderType::Body));
        assert_eq!(PlaceholderType::from_name("invalid"), None);
    }

    #[test]
    fn test_shape_content_creation() {
        let content = ShapeContent::new(ShapeContentType::Text);
        assert_eq!(content.content_type, ShapeContentType::Text);
        assert!(content.is_empty());
    }

    #[test]
    fn test_shape_content_placeholder() {
        let content = ShapeContent::placeholder(PlaceholderType::Title);
        assert_eq!(content.content_type, ShapeContentType::Placeholder);
        assert_eq!(content.placeholder, Some(PlaceholderType::Title));
    }

    #[test]
    fn test_shape_content_metadata() {
        let mut content = ShapeContent::new(ShapeContentType::Text);
        content.add_metadata("key".to_string(), "value".to_string());
        assert_eq!(content.get_metadata("key"), Some("value"));
    }

    #[test]
    fn test_shape_content_data() {
        let mut content = ShapeContent::new(ShapeContentType::Text);
        let data = vec![1, 2, 3, 4, 5];
        content.set_data(data.clone());
        assert_eq!(content.size(), 5);
        assert!(!content.is_empty());
    }

    #[test]
    fn test_shape_content_loader_default() {
        let content = ShapeContentLoader::load_default(PlaceholderType::Title).unwrap();
        assert_eq!(content.content_type, ShapeContentType::Placeholder);
        assert!(content.get_metadata("default_text").is_some());
    }

    #[test]
    fn test_shape_content_loader_from_layout() {
        let content = ShapeContentLoader::load_from_layout("blank", PlaceholderType::Body).unwrap();
        assert_eq!(content.get_metadata("layout"), Some("blank"));
    }
}
