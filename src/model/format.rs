use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum AudioCodec {
    Mp3,
    Opus,
    M4a,
}

#[derive(Debug, Clone, Copy)]
pub enum VideoCodec {
    Mp4,
    Mkv,
}

#[derive(Debug, Clone, Copy)]
pub enum MediaFormat {
    Audio(AudioCodec),
    Video(VideoCodec),
}

impl fmt::Display for AudioCodec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mp3 => write!(f, "mp3"),
            Self::Opus => write!(f, "opus"),
            Self::M4a => write!(f, "m4a"),
        }
    }
}

impl fmt::Display for VideoCodec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mp4 => write!(f, "mp4"),
            Self::Mkv => write!(f, "mkv"),
        }
    }
}

impl fmt::Display for MediaFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Audio(c) => write!(f, "audio/{c}"),
            Self::Video(c) => write!(f, "video/{c}"),
        }
    }
}

impl MediaFormat {
    pub fn all_options() -> &'static [(&'static str, MediaFormat)] {
        &[
            ("Audio - MP3", MediaFormat::Audio(AudioCodec::Mp3)),
            ("Audio - Opus", MediaFormat::Audio(AudioCodec::Opus)),
            ("Audio - M4A", MediaFormat::Audio(AudioCodec::M4a)),
            ("Video - MP4", MediaFormat::Video(VideoCodec::Mp4)),
            ("Video - MKV", MediaFormat::Video(VideoCodec::Mkv)),
        ]
    }
}
