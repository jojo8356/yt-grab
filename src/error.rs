use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid range: {0}")]
    InvalidRange(String),

    #[error("yt-dlp error: {0}")]
    YtDlp(String),

    #[error("No videos found in playlist")]
    EmptyPlaylist,

    #[error("Download failed for \"{title}\": {reason}")]
    DownloadFailed { title: String, reason: String },

    #[error("User cancelled")]
    Cancelled,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
