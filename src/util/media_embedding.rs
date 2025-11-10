//! Media Embedding Support
//!
//! This module provides comprehensive media embedding support including:
//! - Video embedding (MP4, WebM, AVI, MOV, etc.)
//! - Audio embedding (MP3, WAV, M4A, OGG, etc.)
//! - Media metadata and properties
//! - Media playback configuration
//! - Media relationship management

use std::path::Path;

/// Media types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    /// MP4 video
    Mp4,
    /// WebM video
    WebM,
    /// AVI video
    Avi,
    /// MOV video
    Mov,
    /// MKV video
    Mkv,
    /// FLV video
    Flv,
    /// WMV video
    Wmv,
    /// MP3 audio
    Mp3,
    /// WAV audio
    Wav,
    /// M4A audio
    M4a,
    /// OGG audio
    Ogg,
    /// FLAC audio
    Flac,
    /// AAC audio
    Aac,
    /// WMA audio
    Wma,
}

impl MediaType {
    /// Get the MIME type
    pub fn mime_type(&self) -> &str {
        match self {
            MediaType::Mp4 => "video/mp4",
            MediaType::WebM => "video/webm",
            MediaType::Avi => "video/x-msvideo",
            MediaType::Mov => "video/quicktime",
            MediaType::Mkv => "video/x-matroska",
            MediaType::Flv => "video/x-flv",
            MediaType::Wmv => "video/x-ms-wmv",
            MediaType::Mp3 => "audio/mpeg",
            MediaType::Wav => "audio/wav",
            MediaType::M4a => "audio/mp4",
            MediaType::Ogg => "audio/ogg",
            MediaType::Flac => "audio/flac",
            MediaType::Aac => "audio/aac",
            MediaType::Wma => "audio/x-ms-wma",
        }
    }

    /// Get the file extension
    pub fn extension(&self) -> &str {
        match self {
            MediaType::Mp4 => "mp4",
            MediaType::WebM => "webm",
            MediaType::Avi => "avi",
            MediaType::Mov => "mov",
            MediaType::Mkv => "mkv",
            MediaType::Flv => "flv",
            MediaType::Wmv => "wmv",
            MediaType::Mp3 => "mp3",
            MediaType::Wav => "wav",
            MediaType::M4a => "m4a",
            MediaType::Ogg => "ogg",
            MediaType::Flac => "flac",
            MediaType::Aac => "aac",
            MediaType::Wma => "wma",
        }
    }

    /// Detect media type from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "mp4" => Some(MediaType::Mp4),
            "webm" => Some(MediaType::WebM),
            "avi" => Some(MediaType::Avi),
            "mov" => Some(MediaType::Mov),
            "mkv" => Some(MediaType::Mkv),
            "flv" => Some(MediaType::Flv),
            "wmv" => Some(MediaType::Wmv),
            "mp3" => Some(MediaType::Mp3),
            "wav" => Some(MediaType::Wav),
            "m4a" => Some(MediaType::M4a),
            "ogg" => Some(MediaType::Ogg),
            "flac" => Some(MediaType::Flac),
            "aac" => Some(MediaType::Aac),
            "wma" => Some(MediaType::Wma),
            _ => None,
        }
    }

    /// Check if media type is video
    pub fn is_video(&self) -> bool {
        matches!(
            self,
            MediaType::Mp4
                | MediaType::WebM
                | MediaType::Avi
                | MediaType::Mov
                | MediaType::Mkv
                | MediaType::Flv
                | MediaType::Wmv
        )
    }

    /// Check if media type is audio
    pub fn is_audio(&self) -> bool {
        matches!(
            self,
            MediaType::Mp3
                | MediaType::Wav
                | MediaType::M4a
                | MediaType::Ogg
                | MediaType::Flac
                | MediaType::Aac
                | MediaType::Wma
        )
    }
}

/// Media playback settings
#[derive(Debug, Clone)]
pub struct MediaPlayback {
    /// Auto-play on slide show
    auto_play: bool,
    /// Loop playback
    loop_playback: bool,
    /// Mute audio
    mute: bool,
    /// Volume level (0.0-1.0)
    volume: f32,
    /// Show playback controls
    show_controls: bool,
    /// Full screen on play
    full_screen: bool,
    /// Hide while not playing
    hide_while_not_playing: bool,
}

impl Default for MediaPlayback {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaPlayback {
    /// Create new media playback settings
    pub fn new() -> Self {
        Self {
            auto_play: false,
            loop_playback: false,
            mute: false,
            volume: 1.0,
            show_controls: true,
            full_screen: false,
            hide_while_not_playing: false,
        }
    }

    /// Set auto-play
    pub fn set_auto_play(mut self, auto_play: bool) -> Self {
        self.auto_play = auto_play;
        self
    }

    /// Get auto-play
    pub fn auto_play(&self) -> bool {
        self.auto_play
    }

    /// Set loop playback
    pub fn set_loop_playback(mut self, loop_playback: bool) -> Self {
        self.loop_playback = loop_playback;
        self
    }

    /// Get loop playback
    pub fn loop_playback(&self) -> bool {
        self.loop_playback
    }

    /// Set mute
    pub fn set_mute(mut self, mute: bool) -> Self {
        self.mute = mute;
        self
    }

    /// Get mute
    pub fn mute(&self) -> bool {
        self.mute
    }

    /// Set volume (0.0-1.0)
    pub fn set_volume(mut self, volume: f32) -> Self {
        self.volume = volume.max(0.0).min(1.0);
        self
    }

    /// Get volume
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Set show controls
    pub fn set_show_controls(mut self, show_controls: bool) -> Self {
        self.show_controls = show_controls;
        self
    }

    /// Get show controls
    pub fn show_controls(&self) -> bool {
        self.show_controls
    }

    /// Set full screen
    pub fn set_full_screen(mut self, full_screen: bool) -> Self {
        self.full_screen = full_screen;
        self
    }

    /// Get full screen
    pub fn full_screen(&self) -> bool {
        self.full_screen
    }

    /// Set hide while not playing
    pub fn set_hide_while_not_playing(mut self, hide: bool) -> Self {
        self.hide_while_not_playing = hide;
        self
    }

    /// Get hide while not playing
    pub fn hide_while_not_playing(&self) -> bool {
        self.hide_while_not_playing
    }

    /// Generate XML for playback settings
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<p:mediaPlayback");
        if self.auto_play {
            xml.push_str(" autoPlay=\"true\"");
        }
        if self.loop_playback {
            xml.push_str(" loop=\"true\"");
        }
        if self.mute {
            xml.push_str(" mute=\"true\"");
        }
        if self.show_controls {
            xml.push_str(" showControls=\"true\"");
        }
        if self.full_screen {
            xml.push_str(" fullScreen=\"true\"");
        }
        if self.hide_while_not_playing {
            xml.push_str(" hideWhileNotPlaying=\"true\"");
        }
        xml.push_str(&format!(" volume=\"{}\"", (self.volume * 100000.0) as i32));
        xml.push_str("/>");
        xml
    }
}

/// Embedded media object
#[derive(Debug, Clone)]
pub struct EmbeddedMedia {
    /// Media type
    media_type: MediaType,
    /// Media file path
    file_path: String,
    /// Media file name
    file_name: String,
    /// Media size in bytes
    file_size: u64,
    /// Media duration in milliseconds
    duration_ms: u64,
    /// Media width in pixels
    width: u32,
    /// Media height in pixels
    height: u32,
    /// Playback settings
    playback: MediaPlayback,
    /// Relationship ID
    rel_id: Option<String>,
}

impl EmbeddedMedia {
    /// Create new embedded media
    pub fn new(media_type: MediaType, file_path: impl Into<String>) -> Self {
        let file_path = file_path.into();
        let file_name = Path::new(&file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("media")
            .to_string();

        Self {
            media_type,
            file_path,
            file_name,
            file_size: 0,
            duration_ms: 0,
            width: 0,
            height: 0,
            playback: MediaPlayback::new(),
            rel_id: None,
        }
    }

    /// Get media type
    pub fn media_type(&self) -> MediaType {
        self.media_type
    }

    /// Get file path
    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    /// Get file name
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Set file size
    pub fn set_file_size(mut self, size: u64) -> Self {
        self.file_size = size;
        self
    }

    /// Get file size
    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    /// Set duration
    pub fn set_duration_ms(mut self, duration: u64) -> Self {
        self.duration_ms = duration;
        self
    }

    /// Get duration
    pub fn duration_ms(&self) -> u64 {
        self.duration_ms
    }

    /// Set dimensions
    pub fn set_dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Get width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get playback settings
    pub fn playback(&self) -> &MediaPlayback {
        &self.playback
    }

    /// Set playback settings
    pub fn set_playback(mut self, playback: MediaPlayback) -> Self {
        self.playback = playback;
        self
    }

    /// Set relationship ID
    pub fn set_rel_id(mut self, rel_id: impl Into<String>) -> Self {
        self.rel_id = Some(rel_id.into());
        self
    }

    /// Get relationship ID
    pub fn rel_id(&self) -> Option<&str> {
        self.rel_id.as_deref()
    }

    /// Generate XML for embedded media
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<p:media");
        if let Some(rel_id) = &self.rel_id {
            xml.push_str(&format!(" r:embed=\"{}\"", rel_id));
        }
        xml.push_str(&format!(" type=\"{}\"", if self.media_type.is_video() {
            "video"
        } else {
            "audio"
        }));
        if self.width > 0 && self.height > 0 {
            xml.push_str(&format!(" width=\"{}\" height=\"{}\"", self.width, self.height));
        }
        if self.duration_ms > 0 {
            xml.push_str(&format!(" duration=\"{}\"", self.duration_ms));
        }
        xml.push_str(">");
        xml.push_str(&self.playback.to_xml());
        xml.push_str("</p:media>");
        xml
    }
}

/// Media manager for handling multiple media objects
#[derive(Debug, Clone)]
pub struct MediaManager {
    /// Embedded media objects
    media_objects: Vec<EmbeddedMedia>,
}

impl Default for MediaManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaManager {
    /// Create new media manager
    pub fn new() -> Self {
        Self {
            media_objects: Vec::new(),
        }
    }

    /// Add embedded media
    pub fn add_media(mut self, media: EmbeddedMedia) -> Self {
        self.media_objects.push(media);
        self
    }

    /// Get media by index
    pub fn get_media(&self, index: usize) -> Option<&EmbeddedMedia> {
        self.media_objects.get(index)
    }

    /// Get mutable media by index
    pub fn get_media_mut(&mut self, index: usize) -> Option<&mut EmbeddedMedia> {
        self.media_objects.get_mut(index)
    }

    /// Get all media objects
    pub fn media_objects(&self) -> &[EmbeddedMedia] {
        &self.media_objects
    }

    /// Get media count
    pub fn media_count(&self) -> usize {
        self.media_objects.len()
    }

    /// Remove media by index
    pub fn remove_media(&mut self, index: usize) -> Option<EmbeddedMedia> {
        if index < self.media_objects.len() {
            Some(self.media_objects.remove(index))
        } else {
            None
        }
    }

    /// Clear all media
    pub fn clear_media(&mut self) {
        self.media_objects.clear();
    }

    /// Generate XML for all media
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        for media in &self.media_objects {
            xml.push_str(&media.to_xml());
        }
        xml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_type_mp4() {
        let media_type = MediaType::Mp4;
        assert_eq!(media_type.mime_type(), "video/mp4");
        assert_eq!(media_type.extension(), "mp4");
        assert!(media_type.is_video());
        assert!(!media_type.is_audio());
    }

    #[test]
    fn test_media_type_mp3() {
        let media_type = MediaType::Mp3;
        assert_eq!(media_type.mime_type(), "audio/mpeg");
        assert_eq!(media_type.extension(), "mp3");
        assert!(!media_type.is_video());
        assert!(media_type.is_audio());
    }

    #[test]
    fn test_media_type_from_extension() {
        assert_eq!(MediaType::from_extension("mp4"), Some(MediaType::Mp4));
        assert_eq!(MediaType::from_extension("mp3"), Some(MediaType::Mp3));
        assert_eq!(MediaType::from_extension("webm"), Some(MediaType::WebM));
        assert_eq!(MediaType::from_extension("unknown"), None);
    }

    #[test]
    fn test_media_playback_new() {
        let playback = MediaPlayback::new();
        assert!(!playback.auto_play());
        assert!(!playback.loop_playback());
        assert!(!playback.mute());
        assert_eq!(playback.volume(), 1.0);
        assert!(playback.show_controls());
    }

    #[test]
    fn test_media_playback_set_auto_play() {
        let playback = MediaPlayback::new().set_auto_play(true);
        assert!(playback.auto_play());
    }

    #[test]
    fn test_media_playback_set_volume() {
        let playback = MediaPlayback::new().set_volume(0.5);
        assert_eq!(playback.volume(), 0.5);
    }

    #[test]
    fn test_media_playback_to_xml() {
        let playback = MediaPlayback::new().set_auto_play(true);
        let xml = playback.to_xml();
        assert!(xml.contains("<p:mediaPlayback"));
        assert!(xml.contains("autoPlay=\"true\""));
        assert!(xml.contains("/>"));
    }

    #[test]
    fn test_embedded_media_new() {
        let media = EmbeddedMedia::new(MediaType::Mp4, "/path/to/video.mp4");
        assert_eq!(media.media_type(), MediaType::Mp4);
        assert_eq!(media.file_path(), "/path/to/video.mp4");
        assert_eq!(media.file_name(), "video.mp4");
    }

    #[test]
    fn test_embedded_media_set_file_size() {
        let media = EmbeddedMedia::new(MediaType::Mp3, "audio.mp3")
            .set_file_size(5_000_000);
        assert_eq!(media.file_size(), 5_000_000);
    }

    #[test]
    fn test_embedded_media_set_duration() {
        let media = EmbeddedMedia::new(MediaType::Mp4, "video.mp4")
            .set_duration_ms(180_000);
        assert_eq!(media.duration_ms(), 180_000);
    }

    #[test]
    fn test_embedded_media_set_dimensions() {
        let media = EmbeddedMedia::new(MediaType::Mp4, "video.mp4")
            .set_dimensions(1920, 1080);
        assert_eq!(media.width(), 1920);
        assert_eq!(media.height(), 1080);
    }

    #[test]
    fn test_embedded_media_set_rel_id() {
        let media = EmbeddedMedia::new(MediaType::Mp4, "video.mp4")
            .set_rel_id("rId5");
        assert_eq!(media.rel_id(), Some("rId5"));
    }

    #[test]
    fn test_embedded_media_to_xml() {
        let media = EmbeddedMedia::new(MediaType::Mp4, "video.mp4")
            .set_rel_id("rId5")
            .set_dimensions(1920, 1080);
        let xml = media.to_xml();
        assert!(xml.contains("<p:media"));
        assert!(xml.contains("r:embed=\"rId5\""));
        assert!(xml.contains("type=\"video\""));
        assert!(xml.contains("width=\"1920\""));
        assert!(xml.contains("height=\"1080\""));
        assert!(xml.contains("</p:media>"));
    }

    #[test]
    fn test_media_manager_new() {
        let manager = MediaManager::new();
        assert_eq!(manager.media_count(), 0);
    }

    #[test]
    fn test_media_manager_add_media() {
        let media = EmbeddedMedia::new(MediaType::Mp4, "video.mp4");
        let manager = MediaManager::new().add_media(media);
        assert_eq!(manager.media_count(), 1);
    }

    #[test]
    fn test_media_manager_get_media() {
        let media = EmbeddedMedia::new(MediaType::Mp3, "audio.mp3");
        let manager = MediaManager::new().add_media(media);
        assert!(manager.get_media(0).is_some());
        assert_eq!(manager.get_media(0).unwrap().media_type(), MediaType::Mp3);
    }

    #[test]
    fn test_media_manager_remove_media() {
        let media = EmbeddedMedia::new(MediaType::Mp4, "video.mp4");
        let mut manager = MediaManager::new().add_media(media);
        assert_eq!(manager.media_count(), 1);
        manager.remove_media(0);
        assert_eq!(manager.media_count(), 0);
    }

    #[test]
    fn test_media_manager_to_xml() {
        let media = EmbeddedMedia::new(MediaType::Mp4, "video.mp4")
            .set_rel_id("rId5");
        let manager = MediaManager::new().add_media(media);
        let xml = manager.to_xml();
        assert!(xml.contains("<p:media"));
        assert!(xml.contains("r:embed=\"rId5\""));
    }
}
