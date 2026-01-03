//! SlideContent struct for complex presentations

use crate::generator::tables::Table;
use crate::generator::shapes::Shape;
use crate::generator::images::Image;
use crate::generator::connectors::Connector;
use crate::generator::media::{Video, Audio};
use crate::generator::charts::Chart;

use super::bullet::{BulletStyle, BulletPoint};
use super::layout::SlideLayout;
use super::code_block::CodeBlock;
use super::transition::TransitionType;

/// Slide content for more complex presentations
#[derive(Clone, Debug)]
pub struct SlideContent {
    pub title: String,
    pub content: Vec<String>,
    /// Rich bullet points with styles and levels
    pub bullets: Vec<BulletPoint>,
    /// Default bullet style for this slide
    pub bullet_style: BulletStyle,
    pub title_size: Option<u32>,
    pub content_size: Option<u32>,
    pub title_bold: bool,
    pub content_bold: bool,
    pub title_italic: bool,
    pub content_italic: bool,
    pub title_underline: bool,
    pub content_underline: bool,
    pub title_color: Option<String>,
    pub content_color: Option<String>,
    pub has_table: bool,
    pub has_chart: bool,
    pub has_image: bool,
    pub layout: SlideLayout,
    pub transition: TransitionType,
    pub table: Option<Table>,
    pub shapes: Vec<Shape>,
    pub images: Vec<Image>,
    /// Speaker notes for the slide
    pub notes: Option<String>,
    /// Connectors between shapes
    pub connectors: Vec<Connector>,
    /// Videos embedded in slide
    pub videos: Vec<Video>,
    /// Audio files embedded in slide
    pub audios: Vec<Audio>,
    /// Charts embedded in slide
    pub charts: Vec<Chart>,
    /// Code blocks with syntax highlighting
    pub code_blocks: Vec<CodeBlock>,
}

impl SlideContent {
    pub fn new(title: &str) -> Self {
        SlideContent {
            title: title.to_string(),
            content: Vec::new(),
            bullets: Vec::new(),
            bullet_style: BulletStyle::Bullet,
            title_size: Some(44),
            content_size: Some(28),
            title_bold: true,
            content_bold: false,
            title_italic: false,
            content_italic: false,
            title_underline: false,
            content_underline: false,
            title_color: None,
            content_color: None,
            has_table: false,
            has_chart: false,
            has_image: false,
            layout: SlideLayout::TitleAndContent,
            transition: TransitionType::None,
            table: None,
            shapes: Vec::new(),
            images: Vec::new(),
            notes: None,
            connectors: Vec::new(),
            videos: Vec::new(),
            audios: Vec::new(),
            charts: Vec::new(),
            code_blocks: Vec::new(),
        }
    }

    /// Set the slide transition
    pub fn with_transition(mut self, transition: TransitionType) -> Self {
        self.transition = transition;
        self
    }
    
    /// Add a bullet point with default style
    pub fn add_bullet(mut self, text: &str) -> Self {
        self.content.push(text.to_string());
        self.bullets.push(BulletPoint::new(text).with_style(self.bullet_style));
        self
    }
    
    /// Add a bullet point with specific style
    pub fn add_styled_bullet(mut self, text: &str, style: BulletStyle) -> Self {
        self.content.push(text.to_string());
        self.bullets.push(BulletPoint::new(text).with_style(style));
        self
    }
    
    /// Add a numbered item (shorthand for add_styled_bullet with Number)
    pub fn add_numbered(mut self, text: &str) -> Self {
        self.content.push(text.to_string());
        self.bullets.push(BulletPoint::new(text).with_style(BulletStyle::Number));
        self
    }
    
    /// Add a lettered item (shorthand for add_styled_bullet with LetterLower)
    pub fn add_lettered(mut self, text: &str) -> Self {
        self.content.push(text.to_string());
        self.bullets.push(BulletPoint::new(text).with_style(BulletStyle::LetterLower));
        self
    }
    
    /// Add a sub-bullet (indented)
    pub fn add_sub_bullet(mut self, text: &str) -> Self {
        self.content.push(format!("  {}", text));
        self.bullets.push(BulletPoint::new(text).with_level(1).with_style(self.bullet_style));
        self
    }
    
    /// Set default bullet style for this slide
    pub fn with_bullet_style(mut self, style: BulletStyle) -> Self {
        self.bullet_style = style;
        self
    }

    pub fn title_size(mut self, size: u32) -> Self {
        self.title_size = Some(size);
        self
    }

    pub fn content_size(mut self, size: u32) -> Self {
        self.content_size = Some(size);
        self
    }

    pub fn title_bold(mut self, bold: bool) -> Self {
        self.title_bold = bold;
        self
    }

    pub fn content_bold(mut self, bold: bool) -> Self {
        self.content_bold = bold;
        self
    }

    pub fn title_italic(mut self, italic: bool) -> Self {
        self.title_italic = italic;
        self
    }

    pub fn content_italic(mut self, italic: bool) -> Self {
        self.content_italic = italic;
        self
    }

    pub fn title_underline(mut self, underline: bool) -> Self {
        self.title_underline = underline;
        self
    }

    pub fn content_underline(mut self, underline: bool) -> Self {
        self.content_underline = underline;
        self
    }

    pub fn title_color(mut self, color: &str) -> Self {
        self.title_color = Some(color.trim_start_matches('#').to_uppercase());
        self
    }

    pub fn content_color(mut self, color: &str) -> Self {
        self.content_color = Some(color.trim_start_matches('#').to_uppercase());
        self
    }

    pub fn with_table(mut self) -> Self {
        self.has_table = true;
        self
    }

    pub fn with_chart(mut self) -> Self {
        self.has_chart = true;
        self
    }

    pub fn with_image(mut self) -> Self {
        self.has_image = true;
        self
    }

    pub fn layout(mut self, layout: SlideLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn table(mut self, table: Table) -> Self {
        self.table = Some(table);
        self.has_table = true;
        self
    }

    /// Add a shape to the slide
    pub fn add_shape(mut self, shape: Shape) -> Self {
        self.shapes.push(shape);
        self
    }

    /// Add multiple shapes to the slide
    pub fn with_shapes(mut self, shapes: Vec<Shape>) -> Self {
        self.shapes.extend(shapes);
        self
    }

    /// Add an image to the slide
    pub fn add_image(mut self, image: Image) -> Self {
        self.images.push(image);
        self.has_image = true;
        self
    }

    /// Add multiple images to the slide
    pub fn with_images(mut self, images: Vec<Image>) -> Self {
        self.images.extend(images);
        self.has_image = true;
        self
    }

    /// Add speaker notes to the slide
    pub fn notes(mut self, notes: &str) -> Self {
        self.notes = Some(notes.to_string());
        self
    }

    /// Check if slide has speaker notes
    pub fn has_notes(&self) -> bool {
        self.notes.is_some()
    }

    /// Add a connector to the slide
    pub fn add_connector(mut self, connector: Connector) -> Self {
        self.connectors.push(connector);
        self
    }

    /// Add multiple connectors to the slide
    pub fn with_connectors(mut self, connectors: Vec<Connector>) -> Self {
        self.connectors.extend(connectors);
        self
    }

    /// Add a video to the slide
    pub fn add_video(mut self, video: Video) -> Self {
        self.videos.push(video);
        self
    }

    /// Add multiple videos to the slide
    pub fn with_videos(mut self, videos: Vec<Video>) -> Self {
        self.videos.extend(videos);
        self
    }

    /// Add an audio file to the slide
    pub fn add_audio(mut self, audio: Audio) -> Self {
        self.audios.push(audio);
        self
    }

    /// Add multiple audio files to the slide
    pub fn with_audios(mut self, audios: Vec<Audio>) -> Self {
        self.audios.extend(audios);
        self
    }

    /// Add a chart to the slide
    pub fn add_chart(mut self, chart: Chart) -> Self {
        self.charts.push(chart);
        self.has_chart = true;
        self
    }

    /// Add multiple charts to the slide
    pub fn with_charts(mut self, charts: Vec<Chart>) -> Self {
        self.charts.extend(charts);
        self.has_chart = true;
        self
    }

    /// Check if slide has any media
    pub fn has_media(&self) -> bool {
        !self.videos.is_empty() || !self.audios.is_empty()
    }

    /// Check if slide has connectors
    pub fn has_connectors(&self) -> bool {
        !self.connectors.is_empty()
    }
}

