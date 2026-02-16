use std::process::Command;

use crate::error::{AppError, Result};
use crate::model::video::{Chapter, VideoInfo, YtDlpPlaylistJson, YtDlpVideoJson};

pub trait PlaylistFetcher {
    fn fetch(&self, url: &str) -> Result<(Option<String>, Vec<VideoInfo>)>;
}

pub struct YtDlpFetcher;

impl YtDlpFetcher {
    /// Fetch full video metadata (including chapters) for a single video URL.
    pub fn fetch_chapters(url: &str) -> Result<Vec<Chapter>> {
        let output = Command::new("yt-dlp")
            .args(["-J", "--no-warnings", url])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::YtDlp(stderr.trim().to_string()));
        }

        let json: YtDlpVideoJson = serde_json::from_slice(&output.stdout)?;
        Ok(json.chapters.unwrap_or_default())
    }
}

impl PlaylistFetcher for YtDlpFetcher {
    fn fetch(&self, url: &str) -> Result<(Option<String>, Vec<VideoInfo>)> {
        let output = Command::new("yt-dlp")
            .args(["--flat-playlist", "-J", "--no-warnings", url])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::YtDlp(stderr.trim().to_string()));
        }

        let json: YtDlpPlaylistJson = serde_json::from_slice(&output.stdout)?;

        // Check if this is a playlist with entries
        if let Some(entries) = json.entries {
            if entries.is_empty() {
                return Err(AppError::EmptyPlaylist);
            }

            let videos: Vec<VideoInfo> = entries
                .into_iter()
                .enumerate()
                .map(|(i, e)| {
                    let id = e.id.unwrap_or_default();
                    VideoInfo {
                        index: i + 1,
                        url: e
                            .url
                            .unwrap_or_else(|| format!("https://www.youtube.com/watch?v={id}")),
                        id: id.clone(),
                        title: e.title.unwrap_or_else(|| format!("Video {}", i + 1)),
                        duration: e.duration,
                        chapters: Vec::new(),
                        selected_sections: Vec::new(),
                    }
                })
                .collect();

            Ok((json.title, videos))
        } else {
            // Single video — chapters may be available directly
            let id = json.id.unwrap_or_default();
            let video = VideoInfo {
                index: 1,
                url: json
                    .webpage_url
                    .unwrap_or_else(|| format!("https://www.youtube.com/watch?v={id}")),
                id: id.clone(),
                title: json.full_title.unwrap_or_else(|| "Unknown".to_string()),
                duration: json.duration,
                chapters: json.chapters.unwrap_or_default(),
                selected_sections: Vec::new(),
            };
            Ok((None, vec![video]))
        }
    }
}
