//! Media format support for SVG, GIF, and YouTube
//!
//! Extends media support with SVG, animated GIF, and YouTube embed capabilities.
//! Based on PptxGenJS media enhancements.

/// Media format types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaFormat {
    /// Standard image formats (PNG, JPG, BMP)
    Image,
    /// Scalable Vector Graphics
    SVG,
    /// Animated GIF
    AnimatedGIF,
    /// YouTube video embed
    YouTube,
    /// Video file (MP4, WebM, etc.)
    Video,
    /// Audio file (MP3, WAV, etc.)
    Audio,
}

impl MediaFormat {
    /// Get MIME type for format
    pub fn mime_type(&self) -> &'static str {
        match self {
            MediaFormat::Image => "image/png",
            MediaFormat::SVG => "image/svg+xml",
            MediaFormat::AnimatedGIF => "image/gif",
            MediaFormat::YouTube => "application/x-youtube",
            MediaFormat::Video => "video/mp4",
            MediaFormat::Audio => "audio/mpeg",
        }
    }

    /// Get file extension for format
    pub fn extension(&self) -> &'static str {
        match self {
            MediaFormat::Image => "png",
            MediaFormat::SVG => "svg",
            MediaFormat::AnimatedGIF => "gif",
            MediaFormat::YouTube => "youtube",
            MediaFormat::Video => "mp4",
            MediaFormat::Audio => "mp3",
        }
    }

    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "svg" => Some(MediaFormat::SVG),
            "gif" => Some(MediaFormat::AnimatedGIF),
            "youtube" | "yt" => Some(MediaFormat::YouTube),
            "mp4" | "webm" | "mov" => Some(MediaFormat::Video),
            "mp3" | "wav" | "aac" => Some(MediaFormat::Audio),
            "png" | "jpg" | "jpeg" | "bmp" => Some(MediaFormat::Image),
            _ => None,
        }
    }

    /// Detect format from MIME type
    pub fn from_mime_type(mime: &str) -> Option<Self> {
        match mime.to_lowercase().as_str() {
            "image/svg+xml" => Some(MediaFormat::SVG),
            "image/gif" => Some(MediaFormat::AnimatedGIF),
            "application/x-youtube" => Some(MediaFormat::YouTube),
            mime if mime.starts_with("video/") => Some(MediaFormat::Video),
            mime if mime.starts_with("audio/") => Some(MediaFormat::Audio),
            mime if mime.starts_with("image/") => Some(MediaFormat::Image),
            _ => None,
        }
    }

    /// Check if format is supported in PowerPoint
    pub fn is_supported(&self) -> bool {
        match self {
            MediaFormat::Image => true,
            MediaFormat::SVG => true,
            MediaFormat::AnimatedGIF => true,
            MediaFormat::YouTube => true,
            MediaFormat::Video => true,
            MediaFormat::Audio => true,
        }
    }

    /// Check if format is vector-based
    pub fn is_vector(&self) -> bool {
        matches!(self, MediaFormat::SVG)
    }

    /// Check if format is animated
    pub fn is_animated(&self) -> bool {
        matches!(self, MediaFormat::AnimatedGIF)
    }

    /// Check if format is embedded (not file-based)
    pub fn is_embedded(&self) -> bool {
        matches!(self, MediaFormat::YouTube)
    }
}

/// SVG image configuration
#[derive(Debug, Clone)]
pub struct SVGConfig {
    /// SVG data or path
    pub data: String,
    /// Preserve aspect ratio
    pub preserve_aspect_ratio: bool,
    /// Allow external resources
    pub allow_external: bool,
}

impl SVGConfig {
    /// Create new SVG configuration
    pub fn new(data: String) -> Self {
        Self {
            data,
            preserve_aspect_ratio: true,
            allow_external: false,
        }
    }

    /// Set preserve aspect ratio
    pub fn set_preserve_aspect_ratio(&mut self, preserve: bool) {
        self.preserve_aspect_ratio = preserve;
    }

    /// Set allow external resources
    pub fn set_allow_external(&mut self, allow: bool) {
        self.allow_external = allow;
    }
}

/// Animated GIF configuration
#[derive(Debug, Clone)]
pub struct GIFConfig {
    /// GIF data or path
    pub data: String,
    /// Auto-play on slide show
    pub auto_play: bool,
    /// Loop animation
    pub loop_animation: bool,
    /// Play on click
    pub play_on_click: bool,
}

impl GIFConfig {
    /// Create new GIF configuration
    pub fn new(data: String) -> Self {
        Self {
            data,
            auto_play: true,
            loop_animation: true,
            play_on_click: false,
        }
    }

    /// Set auto-play
    pub fn set_auto_play(&mut self, auto_play: bool) {
        self.auto_play = auto_play;
    }

    /// Set loop animation
    pub fn set_loop_animation(&mut self, loop_anim: bool) {
        self.loop_animation = loop_anim;
    }

    /// Set play on click
    pub fn set_play_on_click(&mut self, play_on_click: bool) {
        self.play_on_click = play_on_click;
    }
}

/// YouTube embed configuration
#[derive(Debug, Clone)]
pub struct YouTubeConfig {
    /// YouTube video ID
    pub video_id: String,
    /// Auto-play on slide show
    pub auto_play: bool,
    /// Show controls
    pub show_controls: bool,
    /// Loop video
    pub loop_video: bool,
    /// Start time in seconds
    pub start_time: Option<u32>,
    /// End time in seconds
    pub end_time: Option<u32>,
}

impl YouTubeConfig {
    /// Create new YouTube configuration
    pub fn new(video_id: String) -> Self {
        Self {
            video_id,
            auto_play: false,
            show_controls: true,
            loop_video: false,
            start_time: None,
            end_time: None,
        }
    }

    /// Set auto-play
    pub fn set_auto_play(&mut self, auto_play: bool) {
        self.auto_play = auto_play;
    }

    /// Set show controls
    pub fn set_show_controls(&mut self, show: bool) {
        self.show_controls = show;
    }

    /// Set loop video
    pub fn set_loop_video(&mut self, loop_video: bool) {
        self.loop_video = loop_video;
    }

    /// Set start time
    pub fn set_start_time(&mut self, seconds: u32) {
        self.start_time = Some(seconds);
    }

    /// Set end time
    pub fn set_end_time(&mut self, seconds: u32) {
        self.end_time = Some(seconds);
    }

    /// Get YouTube embed URL
    pub fn get_embed_url(&self) -> String {
        let mut url = format!("https://www.youtube.com/embed/{}", self.video_id);
        let mut params = Vec::new();

        if self.auto_play {
            params.push("autoplay=1".to_string());
        }

        if !self.show_controls {
            params.push("controls=0".to_string());
        }

        if self.loop_video {
            params.push("loop=1".to_string());
        }

        if let Some(start) = self.start_time {
            params.push(format!("start={}", start));
        }

        if let Some(end) = self.end_time {
            params.push(format!("end={}", end));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        url
    }

    /// Validate YouTube video ID
    pub fn validate_video_id(video_id: &str) -> bool {
        // YouTube video IDs are 11 characters long and contain alphanumeric, dash, and underscore
        video_id.len() == 11 && video_id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_format_mime_type() {
        assert_eq!(MediaFormat::SVG.mime_type(), "image/svg+xml");
        assert_eq!(MediaFormat::AnimatedGIF.mime_type(), "image/gif");
        assert_eq!(MediaFormat::YouTube.mime_type(), "application/x-youtube");
    }

    #[test]
    fn test_media_format_extension() {
        assert_eq!(MediaFormat::SVG.extension(), "svg");
        assert_eq!(MediaFormat::AnimatedGIF.extension(), "gif");
        assert_eq!(MediaFormat::YouTube.extension(), "youtube");
    }

    #[test]
    fn test_media_format_from_extension() {
        assert_eq!(MediaFormat::from_extension("svg"), Some(MediaFormat::SVG));
        assert_eq!(MediaFormat::from_extension("gif"), Some(MediaFormat::AnimatedGIF));
        assert_eq!(MediaFormat::from_extension("youtube"), Some(MediaFormat::YouTube));
        assert_eq!(MediaFormat::from_extension("mp4"), Some(MediaFormat::Video));
        assert_eq!(MediaFormat::from_extension("mp3"), Some(MediaFormat::Audio));
        assert_eq!(MediaFormat::from_extension("png"), Some(MediaFormat::Image));
    }

    #[test]
    fn test_media_format_from_mime_type() {
        assert_eq!(MediaFormat::from_mime_type("image/svg+xml"), Some(MediaFormat::SVG));
        assert_eq!(MediaFormat::from_mime_type("image/gif"), Some(MediaFormat::AnimatedGIF));
        assert_eq!(MediaFormat::from_mime_type("application/x-youtube"), Some(MediaFormat::YouTube));
        assert_eq!(MediaFormat::from_mime_type("video/mp4"), Some(MediaFormat::Video));
        assert_eq!(MediaFormat::from_mime_type("audio/mpeg"), Some(MediaFormat::Audio));
    }

    #[test]
    fn test_media_format_is_supported() {
        assert!(MediaFormat::SVG.is_supported());
        assert!(MediaFormat::AnimatedGIF.is_supported());
        assert!(MediaFormat::YouTube.is_supported());
    }

    #[test]
    fn test_media_format_is_vector() {
        assert!(MediaFormat::SVG.is_vector());
        assert!(!MediaFormat::AnimatedGIF.is_vector());
        assert!(!MediaFormat::Image.is_vector());
    }

    #[test]
    fn test_media_format_is_animated() {
        assert!(MediaFormat::AnimatedGIF.is_animated());
        assert!(!MediaFormat::SVG.is_animated());
        assert!(!MediaFormat::Image.is_animated());
    }

    #[test]
    fn test_media_format_is_embedded() {
        assert!(MediaFormat::YouTube.is_embedded());
        assert!(!MediaFormat::SVG.is_embedded());
        assert!(!MediaFormat::AnimatedGIF.is_embedded());
    }

    #[test]
    fn test_svg_config_new() {
        let config = SVGConfig::new("<svg></svg>".to_string());
        assert_eq!(config.data, "<svg></svg>");
        assert!(config.preserve_aspect_ratio);
        assert!(!config.allow_external);
    }

    #[test]
    fn test_svg_config_set_properties() {
        let mut config = SVGConfig::new("<svg></svg>".to_string());
        config.set_preserve_aspect_ratio(false);
        config.set_allow_external(true);
        assert!(!config.preserve_aspect_ratio);
        assert!(config.allow_external);
    }

    #[test]
    fn test_gif_config_new() {
        let config = GIFConfig::new("image.gif".to_string());
        assert_eq!(config.data, "image.gif");
        assert!(config.auto_play);
        assert!(config.loop_animation);
        assert!(!config.play_on_click);
    }

    #[test]
    fn test_gif_config_set_properties() {
        let mut config = GIFConfig::new("image.gif".to_string());
        config.set_auto_play(false);
        config.set_loop_animation(false);
        config.set_play_on_click(true);
        assert!(!config.auto_play);
        assert!(!config.loop_animation);
        assert!(config.play_on_click);
    }

    #[test]
    fn test_youtube_config_new() {
        let config = YouTubeConfig::new("dQw4w9WgXcQ".to_string());
        assert_eq!(config.video_id, "dQw4w9WgXcQ");
        assert!(!config.auto_play);
        assert!(config.show_controls);
        assert!(!config.loop_video);
    }

    #[test]
    fn test_youtube_config_set_properties() {
        let mut config = YouTubeConfig::new("dQw4w9WgXcQ".to_string());
        config.set_auto_play(true);
        config.set_show_controls(false);
        config.set_loop_video(true);
        config.set_start_time(10);
        config.set_end_time(120);

        assert!(config.auto_play);
        assert!(!config.show_controls);
        assert!(config.loop_video);
        assert_eq!(config.start_time, Some(10));
        assert_eq!(config.end_time, Some(120));
    }

    #[test]
    fn test_youtube_config_get_embed_url() {
        let mut config = YouTubeConfig::new("dQw4w9WgXcQ".to_string());
        let url = config.get_embed_url();
        assert!(url.contains("dQw4w9WgXcQ"));

        config.set_auto_play(true);
        config.set_start_time(10);
        let url = config.get_embed_url();
        assert!(url.contains("autoplay=1"));
        assert!(url.contains("start=10"));
    }

    #[test]
    fn test_youtube_config_validate_video_id() {
        assert!(YouTubeConfig::validate_video_id("dQw4w9WgXcQ"));
        assert!(!YouTubeConfig::validate_video_id("short"));
        assert!(!YouTubeConfig::validate_video_id("toolongvideoidentifier"));
    }
}
