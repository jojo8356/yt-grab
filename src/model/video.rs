use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Chapter {
    pub title: String,
    pub start_time: f64,
    pub end_time: f64,
}

impl Chapter {
    pub fn time_display(&self) -> String {
        format!("{} - {}", format_time(self.start_time), format_time(self.end_time))
    }
}

fn format_time(secs: f64) -> String {
    let total = secs as u64;
    let m = total / 60;
    let s = total % 60;
    format!("{m}:{s:02}")
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VideoInfo {
    pub index: usize,
    pub id: String,
    pub title: String,
    pub duration: Option<f64>,
    pub url: String,
    pub chapters: Vec<Chapter>,
    pub selected_sections: Vec<(f64, f64)>,
}

#[derive(Debug, Deserialize)]
pub struct YtDlpVideoJson {
    pub chapters: Option<Vec<Chapter>>,
}

#[derive(Debug, Deserialize)]
pub struct YtDlpPlaylistJson {
    pub title: Option<String>,
    pub entries: Option<Vec<YtDlpEntry>>,
    // Single video fields
    pub id: Option<String>,
    pub webpage_url: Option<String>,
    #[serde(rename = "fulltitle")]
    pub full_title: Option<String>,
    pub duration: Option<f64>,
    pub chapters: Option<Vec<Chapter>>,
}

#[derive(Debug, Deserialize)]
pub struct YtDlpEntry {
    pub id: Option<String>,
    pub title: Option<String>,
    pub duration: Option<f64>,
    pub url: Option<String>,
}

impl VideoInfo {
    pub fn duration_display(&self) -> String {
        match self.duration {
            Some(secs) => format_time(secs),
            None => "??:??".to_string(),
        }
    }

    pub fn has_chapters(&self) -> bool {
        !self.chapters.is_empty()
    }

    pub fn has_selected_sections(&self) -> bool {
        !self.selected_sections.is_empty()
    }
}
