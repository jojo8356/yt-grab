use std::path::PathBuf;

use clap::Parser;

/// Interactive YouTube playlist downloader
#[derive(Parser, Debug)]
#[command(name = "yt-grab", version, about)]
pub struct Args {
    /// YouTube URL (video or playlist)
    pub url: String,

    /// Output directory
    #[arg(short, long, default_value = ".")]
    pub output: PathBuf,

    /// Number of concurrent downloads
    #[arg(short, long, default_value_t = 3)]
    pub concurrency: usize,

    /// Format: mp3, opus, m4a, mp4, mkv (interactive if omitted)
    #[arg(short, long)]
    pub format: Option<String>,

    /// Download all items without interactive selection
    #[arg(long)]
    pub all: bool,

    /// Select specific items (e.g. "1", "1,3-5") — skips interactive selection
    #[arg(short, long)]
    pub items: Option<String>,

    /// Skip chapter selection, download full video(s)
    #[arg(long)]
    pub no_chapters: bool,
}

impl Args {
    pub fn parse_format(&self) -> Option<crate::model::format::MediaFormat> {
        use crate::model::format::{AudioCodec, MediaFormat, VideoCodec};

        self.format.as_ref().map(|f| match f.to_lowercase().as_str() {
            "mp3" => MediaFormat::Audio(AudioCodec::Mp3),
            "opus" => MediaFormat::Audio(AudioCodec::Opus),
            "m4a" => MediaFormat::Audio(AudioCodec::M4a),
            "mp4" => MediaFormat::Video(VideoCodec::Mp4),
            "mkv" => MediaFormat::Video(VideoCodec::Mkv),
            _ => MediaFormat::Audio(AudioCodec::Mp3),
        })
    }
}
