use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::error::{AppError, Result};
use crate::model::config::DownloadConfig;
use crate::model::format::{AudioCodec, MediaFormat, VideoCodec};
use crate::model::video::VideoInfo;

pub struct DownloadResult {
    pub title: String,
    pub success: bool,
    pub error: Option<String>,
}

pub trait Downloader {
    fn download(
        &self,
        video: &VideoInfo,
        config: &DownloadConfig,
        progress_cb: Box<dyn Fn(&str) + Send>,
    ) -> impl std::future::Future<Output = Result<DownloadResult>> + Send;
}

pub struct YtDlpDownloader;

impl YtDlpDownloader {
    fn build_args(video: &VideoInfo, config: &DownloadConfig) -> Vec<String> {
        let mut args = vec![
            "--no-warnings".to_string(),
            "--newline".to_string(),
            "--progress".to_string(),
            "-o".to_string(),
        ];

        // Use section_title in filename when downloading specific sections
        let template = if video.has_selected_sections() {
            "%(title)s - %(section_title)s.%(ext)s"
        } else {
            "%(title)s.%(ext)s"
        };
        let output_template = config.output_dir.join(template).to_string_lossy().to_string();
        args.push(output_template);

        // Add --download-sections for each selected chapter
        for (start, end) in &video.selected_sections {
            let start_m = *start as u64 / 60;
            let start_s = *start as u64 % 60;
            let end_m = *end as u64 / 60;
            let end_s = *end as u64 % 60;
            args.push("--download-sections".to_string());
            args.push(format!("*{start_m:02}:{start_s:02}-{end_m:02}:{end_s:02}"));
        }

        match config.format {
            MediaFormat::Audio(codec) => {
                args.push("-x".to_string());
                args.push("--audio-format".to_string());
                args.push(match codec {
                    AudioCodec::Mp3 => "mp3".to_string(),
                    AudioCodec::Opus => "opus".to_string(),
                    AudioCodec::M4a => "m4a".to_string(),
                });
                args.push("--audio-quality".to_string());
                args.push("0".to_string());
            }
            MediaFormat::Video(codec) => {
                args.push("--merge-output-format".to_string());
                args.push(match codec {
                    VideoCodec::Mp4 => "mp4".to_string(),
                    VideoCodec::Mkv => "mkv".to_string(),
                });
            }
        }

        args.push(video.url.clone());
        args
    }
}

impl Downloader for YtDlpDownloader {
    async fn download(
        &self,
        video: &VideoInfo,
        config: &DownloadConfig,
        progress_cb: Box<dyn Fn(&str) + Send>,
    ) -> Result<DownloadResult> {
        let args = Self::build_args(video, config);

        // Ensure output directory exists
        let dir = &config.output_dir;
        if !dir.exists() {
            tokio::fs::create_dir_all(dir).await?;
        }

        let mut child = Command::new("yt-dlp")
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // Read stdout and stderr concurrently
        let stderr_handle = {
            let stderr = child.stderr.take();
            tokio::spawn(async move {
                let mut err_lines = Vec::new();
                if let Some(stderr) = stderr {
                    let reader = BufReader::new(stderr);
                    let mut lines = reader.lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        err_lines.push(line);
                    }
                }
                err_lines
            })
        };

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                progress_cb(&line);
            }
        }

        let status = child.wait().await?;
        let stderr_lines = stderr_handle.await.unwrap_or_default();

        if status.success() {
            Ok(DownloadResult {
                title: video.title.clone(),
                success: true,
                error: None,
            })
        } else {
            // Extract the most useful error line from stderr
            let reason = stderr_lines
                .iter()
                .rfind(|l| l.starts_with("ERROR:"))
                .cloned()
                .unwrap_or_else(|| "yt-dlp exited with non-zero status".to_string());
            Err(AppError::DownloadFailed {
                title: video.title.clone(),
                reason,
            })
        }
    }
}

/// Extract a percentage from yt-dlp progress output lines.
/// Lines look like: "[download]  45.2% of 5.23MiB at 1.2MiB/s ETA 00:03"
pub fn parse_progress_percent(line: &str) -> Option<f64> {
    if !line.contains("[download]") {
        return None;
    }
    // Find the percentage pattern
    let line = line.trim();
    for word in line.split_whitespace() {
        if word.ends_with('%') {
            if let Ok(pct) = word.trim_end_matches('%').parse::<f64>() {
                return Some(pct);
            }
        }
    }
    None
}

pub async fn download_all(
    videos: &[VideoInfo],
    config: &DownloadConfig,
    multi_progress: &indicatif::MultiProgress,
) -> Vec<DownloadResult> {
    use indicatif::{ProgressBar, ProgressStyle};
    use tokio::sync::Semaphore;

    let semaphore = std::sync::Arc::new(Semaphore::new(config.concurrency));
    let config = std::sync::Arc::new(config.clone());
    let mut handles = Vec::new();

    let style = ProgressStyle::with_template(
        "  {prefix:.cyan} [{bar:30.green/dim}] {msg}",
    )
    .unwrap()
    .progress_chars("━╸─");

    for video in videos {
        let permit = semaphore.clone();
        let config = config.clone();
        let video = video.clone();
        let pb = multi_progress.add(ProgressBar::new(100));
        pb.set_style(style.clone());
        let title_short = if video.title.len() > 40 {
            format!("{}...", &video.title[..37])
        } else {
            video.title.clone()
        };
        pb.set_prefix(title_short);
        pb.set_message("waiting...");

        let handle = tokio::spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            pb.set_message("downloading");

            let pb_clone = pb.clone();
            let downloader = YtDlpDownloader;
            let result = downloader
                .download(
                    &video,
                    &config,
                    Box::new(move |line: &str| {
                        if let Some(pct) = parse_progress_percent(line) {
                            pb_clone.set_position(pct as u64);
                        }
                    }),
                )
                .await;

            match result {
                Ok(r) => {
                    pb.set_position(100);
                    pb.finish_with_message("done ✓");
                    r
                }
                Err(e) => {
                    pb.finish_with_message("FAILED ✗");
                    DownloadResult {
                        title: video.title.clone(),
                        success: false,
                        error: Some(e.to_string()),
                    }
                }
            }
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(r) => results.push(r),
            Err(e) => results.push(DownloadResult {
                title: "unknown".to_string(),
                success: false,
                error: Some(e.to_string()),
            }),
        }
    }
    results
}
