use std::path::PathBuf;

use super::format::MediaFormat;

#[derive(Debug, Clone)]
pub struct DownloadConfig {
    pub format: MediaFormat,
    pub output_dir: PathBuf,
    pub concurrency: usize,
}
