//! Media embedding support for PPTX (video and audio)
//!
//! Provides types and XML generation for embedding videos and audio files.

use crate::core::escape_xml;

/// Video format types
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum VideoFormat {
    Mp4,
    Wmv,
    Avi,
    Mov,
    Mkv,
    Webm,
    M4v,
}

impl VideoFormat {
    /// Get MIME type
    pub fn mime_type(&self) -> &'static str {
        match self {
            VideoFormat::Mp4 => "video/mp4",
            VideoFormat::Wmv => "video/x-ms-wmv",
            VideoFormat::Avi => "video/x-msvideo",
            VideoFormat::Mov => "video/quicktime",
            VideoFormat::Mkv => "video/x-matroska",
            VideoFormat::Webm => "video/webm",
            VideoFormat::M4v => "video/x-m4v",
        }
    }

    /// Get file extension
    pub fn extension(&self) -> &'static str {
        match self {
            VideoFormat::Mp4 => "mp4",
            VideoFormat::Wmv => "wmv",
            VideoFormat::Avi => "avi",
            VideoFormat::Mov => "mov",
            VideoFormat::Mkv => "mkv",
            VideoFormat::Webm => "webm",
            VideoFormat::M4v => "m4v",
        }
    }

    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "mp4" => Some(VideoFormat::Mp4),
            "wmv" => Some(VideoFormat::Wmv),
            "avi" => Some(VideoFormat::Avi),
            "mov" => Some(VideoFormat::Mov),
            "mkv" => Some(VideoFormat::Mkv),
            "webm" => Some(VideoFormat::Webm),
            "m4v" => Some(VideoFormat::M4v),
            _ => None,
        }
    }
}

/// Audio format types
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Mp3,
    Wav,
    Wma,
    M4a,
    Ogg,
    Flac,
    Aac,
}

impl AudioFormat {
    /// Get MIME type
    pub fn mime_type(&self) -> &'static str {
        match self {
            AudioFormat::Mp3 => "audio/mpeg",
            AudioFormat::Wav => "audio/wav",
            AudioFormat::Wma => "audio/x-ms-wma",
            AudioFormat::M4a => "audio/mp4",
            AudioFormat::Ogg => "audio/ogg",
            AudioFormat::Flac => "audio/flac",
            AudioFormat::Aac => "audio/aac",
        }
    }

    /// Get file extension
    pub fn extension(&self) -> &'static str {
        match self {
            AudioFormat::Mp3 => "mp3",
            AudioFormat::Wav => "wav",
            AudioFormat::Wma => "wma",
            AudioFormat::M4a => "m4a",
            AudioFormat::Ogg => "ogg",
            AudioFormat::Flac => "flac",
            AudioFormat::Aac => "aac",
        }
    }

    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "mp3" => Some(AudioFormat::Mp3),
            "wav" => Some(AudioFormat::Wav),
            "wma" => Some(AudioFormat::Wma),
            "m4a" => Some(AudioFormat::M4a),
            "ogg" => Some(AudioFormat::Ogg),
            "flac" => Some(AudioFormat::Flac),
            "aac" => Some(AudioFormat::Aac),
            _ => None,
        }
    }
}

/// Video playback options
#[derive(Clone, Debug)]
pub struct VideoOptions {
    /// Auto-play when slide is shown
    pub auto_play: bool,
    /// Loop playback
    pub loop_playback: bool,
    /// Hide when not playing
    pub hide_when_stopped: bool,
    /// Mute audio
    pub muted: bool,
    /// Start time in milliseconds
    pub start_time: Option<u32>,
    /// End time in milliseconds
    pub end_time: Option<u32>,
    /// Volume (0-100)
    pub volume: u32,
}

impl Default for VideoOptions {
    fn default() -> Self {
        VideoOptions {
            auto_play: false,
            loop_playback: false,
            hide_when_stopped: false,
            muted: false,
            start_time: None,
            end_time: None,
            volume: 100,
        }
    }
}

impl VideoOptions {
    /// Create with auto-play enabled
    pub fn auto_play() -> Self {
        VideoOptions {
            auto_play: true,
            ..Default::default()
        }
    }

    /// Set loop playback
    pub fn with_loop(mut self, loop_playback: bool) -> Self {
        self.loop_playback = loop_playback;
        self
    }

    /// Set muted
    pub fn with_muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    /// Set volume (0-100)
    pub fn with_volume(mut self, volume: u32) -> Self {
        self.volume = volume.min(100);
        self
    }

    /// Set start time in milliseconds
    pub fn with_start_time(mut self, ms: u32) -> Self {
        self.start_time = Some(ms);
        self
    }

    /// Set end time in milliseconds
    pub fn with_end_time(mut self, ms: u32) -> Self {
        self.end_time = Some(ms);
        self
    }
}

/// Audio playback options
#[derive(Clone, Debug)]
pub struct AudioOptions {
    /// Auto-play when slide is shown
    pub auto_play: bool,
    /// Loop playback
    pub loop_playback: bool,
    /// Hide icon during playback
    pub hide_during_show: bool,
    /// Play across slides
    pub play_across_slides: bool,
    /// Volume (0-100)
    pub volume: u32,
}

impl Default for AudioOptions {
    fn default() -> Self {
        AudioOptions {
            auto_play: false,
            loop_playback: false,
            hide_during_show: false,
            play_across_slides: false,
            volume: 100,
        }
    }
}

impl AudioOptions {
    /// Create with auto-play enabled
    pub fn auto_play() -> Self {
        AudioOptions {
            auto_play: true,
            ..Default::default()
        }
    }

    /// Set loop playback
    pub fn with_loop(mut self, loop_playback: bool) -> Self {
        self.loop_playback = loop_playback;
        self
    }

    /// Set play across slides
    pub fn with_play_across_slides(mut self, play: bool) -> Self {
        self.play_across_slides = play;
        self
    }

    /// Set volume (0-100)
    pub fn with_volume(mut self, volume: u32) -> Self {
        self.volume = volume.min(100);
        self
    }
}

/// Video element
#[derive(Clone, Debug)]
pub struct Video {
    /// Video file path or URL
    pub source: String,
    /// Video format
    pub format: VideoFormat,
    /// Position X in EMU
    pub x: u32,
    /// Position Y in EMU
    pub y: u32,
    /// Width in EMU
    pub width: u32,
    /// Height in EMU
    pub height: u32,
    /// Playback options
    pub options: VideoOptions,
    /// Poster image (thumbnail)
    pub poster: Option<String>,
    /// Alt text
    pub alt_text: Option<String>,
}

impl Video {
    /// Create a new video element
    pub fn new(source: &str, format: VideoFormat, x: u32, y: u32, width: u32, height: u32) -> Self {
        Video {
            source: source.to_string(),
            format,
            x,
            y,
            width,
            height,
            options: VideoOptions::default(),
            poster: None,
            alt_text: None,
        }
    }

    /// Create from file path (auto-detect format)
    pub fn from_file(path: &str, x: u32, y: u32, width: u32, height: u32) -> Option<Self> {
        let ext = path.rsplit('.').next()?;
        let format = VideoFormat::from_extension(ext)?;
        Some(Self::new(path, format, x, y, width, height))
    }

    /// Set playback options
    pub fn with_options(mut self, options: VideoOptions) -> Self {
        self.options = options;
        self
    }

    /// Set poster image
    pub fn with_poster(mut self, poster: &str) -> Self {
        self.poster = Some(poster.to_string());
        self
    }

    /// Set alt text
    pub fn with_alt_text(mut self, alt: &str) -> Self {
        self.alt_text = Some(alt.to_string());
        self
    }
}

/// Audio element
#[derive(Clone, Debug)]
pub struct Audio {
    /// Audio file path or URL
    pub source: String,
    /// Audio format
    pub format: AudioFormat,
    /// Position X in EMU (for icon)
    pub x: u32,
    /// Position Y in EMU (for icon)
    pub y: u32,
    /// Icon width in EMU
    pub width: u32,
    /// Icon height in EMU
    pub height: u32,
    /// Playback options
    pub options: AudioOptions,
    /// Alt text
    pub alt_text: Option<String>,
}

impl Audio {
    /// Create a new audio element
    pub fn new(source: &str, format: AudioFormat, x: u32, y: u32, width: u32, height: u32) -> Self {
        Audio {
            source: source.to_string(),
            format,
            x,
            y,
            width,
            height,
            options: AudioOptions::default(),
            alt_text: None,
        }
    }

    /// Create from file path (auto-detect format)
    pub fn from_file(path: &str, x: u32, y: u32, width: u32, height: u32) -> Option<Self> {
        let ext = path.rsplit('.').next()?;
        let format = AudioFormat::from_extension(ext)?;
        Some(Self::new(path, format, x, y, width, height))
    }

    /// Set playback options
    pub fn with_options(mut self, options: AudioOptions) -> Self {
        self.options = options;
        self
    }

    /// Set alt text
    pub fn with_alt_text(mut self, alt: &str) -> Self {
        self.alt_text = Some(alt.to_string());
        self
    }
}

/// Generate video XML for slide
pub fn generate_video_xml(video: &Video, shape_id: usize, video_r_id: &str, _image_r_id: &str) -> String {
    let alt_text = video.alt_text.as_deref().unwrap_or("Video");

    format!(
        r#"<p:pic>
<p:nvPicPr>
<p:cNvPr id="{}" name="Video {}" descr="{}">
<a:hlinkClick r:id="" action="ppaction://media"/>
</p:cNvPr>
<p:cNvPicPr>
<a:picLocks noChangeAspect="1"/>
</p:cNvPicPr>
<p:nvPr>
<a:videoFile r:link="{}"/>
<p:extLst>
<p:ext uri="{{DAA4B4D4-6D71-4841-9C94-3DE7FCFB9230}}">
<p14:media xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main" r:embed="{}"/>
</p:ext>
</p:extLst>
</p:nvPr>
</p:nvPicPr>
<p:blipFill>
<a:blip r:embed="{}"/>
<a:stretch>
<a:fillRect/>
</a:stretch>
</p:blipFill>
<p:spPr>
<a:xfrm>
<a:off x="{}" y="{}"/>
<a:ext cx="{}" cy="{}"/>
</a:xfrm>
<a:prstGeom prst="rect">
<a:avLst/>
</a:prstGeom>
</p:spPr>
</p:pic>"#,
        shape_id, shape_id, escape_xml(alt_text),
        video_r_id, video_r_id, video_r_id,
        video.x, video.y, video.width, video.height
    )
}

/// Generate audio XML for slide
pub fn generate_audio_xml(audio: &Audio, shape_id: usize, audio_r_id: &str) -> String {
    let alt_text = audio.alt_text.as_deref().unwrap_or("Audio");

    format!(
        r#"<p:pic>
<p:nvPicPr>
<p:cNvPr id="{}" name="Audio {}" descr="{}">
<a:hlinkClick r:id="" action="ppaction://media"/>
</p:cNvPr>
<p:cNvPicPr>
<a:picLocks noChangeAspect="1"/>
</p:cNvPicPr>
<p:nvPr>
<a:audioFile r:link="{}"/>
<p:extLst>
<p:ext uri="{{DAA4B4D4-6D71-4841-9C94-3DE7FCFB9230}}">
<p14:media xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main" r:embed="{}"/>
</p:ext>
</p:extLst>
</p:nvPr>
</p:nvPicPr>
<p:blipFill>
<a:blip r:embed="{}"/>
<a:stretch>
<a:fillRect/>
</a:stretch>
</p:blipFill>
<p:spPr>
<a:xfrm>
<a:off x="{}" y="{}"/>
<a:ext cx="{}" cy="{}"/>
</a:xfrm>
<a:prstGeom prst="rect">
<a:avLst/>
</a:prstGeom>
</p:spPr>
</p:pic>"#,
        shape_id, shape_id, escape_xml(alt_text),
        audio_r_id, audio_r_id, audio_r_id,
        audio.x, audio.y, audio.width, audio.height
    )
}

/// Generate content type for video
pub fn video_content_type(format: VideoFormat) -> String {
    format!(
        r#"<Default Extension="{}" ContentType="{}"/>"#,
        format.extension(),
        format.mime_type()
    )
}

/// Generate content type for audio
pub fn audio_content_type(format: AudioFormat) -> String {
    format!(
        r#"<Default Extension="{}" ContentType="{}"/>"#,
        format.extension(),
        format.mime_type()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_format_mime() {
        assert_eq!(VideoFormat::Mp4.mime_type(), "video/mp4");
        assert_eq!(VideoFormat::Wmv.mime_type(), "video/x-ms-wmv");
    }

    #[test]
    fn test_video_format_extension() {
        assert_eq!(VideoFormat::Mp4.extension(), "mp4");
        assert_eq!(VideoFormat::from_extension("mp4"), Some(VideoFormat::Mp4));
    }

    #[test]
    fn test_audio_format_mime() {
        assert_eq!(AudioFormat::Mp3.mime_type(), "audio/mpeg");
        assert_eq!(AudioFormat::Wav.mime_type(), "audio/wav");
    }

    #[test]
    fn test_audio_format_extension() {
        assert_eq!(AudioFormat::Mp3.extension(), "mp3");
        assert_eq!(AudioFormat::from_extension("mp3"), Some(AudioFormat::Mp3));
    }

    #[test]
    fn test_video_options() {
        let opts = VideoOptions::auto_play()
            .with_loop(true)
            .with_volume(80);
        assert!(opts.auto_play);
        assert!(opts.loop_playback);
        assert_eq!(opts.volume, 80);
    }

    #[test]
    fn test_audio_options() {
        let opts = AudioOptions::auto_play()
            .with_play_across_slides(true);
        assert!(opts.auto_play);
        assert!(opts.play_across_slides);
    }

    #[test]
    fn test_video_from_file() {
        let video = Video::from_file("test.mp4", 0, 0, 1000000, 750000);
        assert!(video.is_some());
        let video = video.unwrap();
        assert_eq!(video.format, VideoFormat::Mp4);
    }

    #[test]
    fn test_audio_from_file() {
        let audio = Audio::from_file("test.mp3", 0, 0, 500000, 500000);
        assert!(audio.is_some());
        let audio = audio.unwrap();
        assert_eq!(audio.format, AudioFormat::Mp3);
    }

    #[test]
    fn test_video_builder() {
        let video = Video::new("video.mp4", VideoFormat::Mp4, 0, 0, 1000000, 750000)
            .with_options(VideoOptions::auto_play())
            .with_alt_text("My Video");
        assert!(video.options.auto_play);
        assert_eq!(video.alt_text, Some("My Video".to_string()));
    }

    #[test]
    fn test_generate_video_xml() {
        let video = Video::new("video.mp4", VideoFormat::Mp4, 0, 0, 1000000, 750000);
        let xml = generate_video_xml(&video, 1, "rId1", "rId2");
        assert!(xml.contains("p:pic"));
        assert!(xml.contains("videoFile"));
    }

    #[test]
    fn test_generate_audio_xml() {
        let audio = Audio::new("audio.mp3", AudioFormat::Mp3, 0, 0, 500000, 500000);
        let xml = generate_audio_xml(&audio, 1, "rId1");
        assert!(xml.contains("p:pic"));
        assert!(xml.contains("audioFile"));
    }
}
