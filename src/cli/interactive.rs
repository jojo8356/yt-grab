use console::Style;
use dialoguer::{Input, Select};

use crate::core::range_parser::parse_ranges;
use crate::error::{AppError, Result};
use crate::model::format::MediaFormat;
use crate::model::video::VideoInfo;

pub fn display_playlist(title: &Option<String>, videos: &[VideoInfo]) {
    let header = Style::new().bold().cyan();
    let dim = Style::new().dim();

    if let Some(t) = title {
        println!("\n  {} {}", header.apply_to("Playlist:"), t);
    }
    println!(
        "  {} {} items\n",
        header.apply_to("Found:"),
        videos.len()
    );

    for v in videos {
        println!(
            "  {}{:>3}{} {} {}{}{}",
            Style::new().yellow().apply_to("["),
            Style::new().yellow().bold().apply_to(v.index),
            Style::new().yellow().apply_to("]"),
            v.title,
            dim.apply_to("("),
            dim.apply_to(v.duration_display()),
            dim.apply_to(")"),
        );
    }
    println!();
}

pub fn prompt_selection(total: usize) -> Result<Vec<usize>> {
    let prompt_style = Style::new().bold().green();
    println!(
        "  {} Enter selection (e.g. 1,3-5 or 'all'):",
        prompt_style.apply_to(">>")
    );

    let input: String = Input::new()
        .with_prompt("  ")
        .interact_text()
        .map_err(|_| AppError::Cancelled)?;

    parse_ranges(&input, total)
}

/// Display chapters for a video and prompt the user to select which ones to download.
/// Returns the selected (start_time, end_time) pairs, or empty vec for "full video".
pub fn prompt_chapter_selection(video: &mut VideoInfo) -> Result<()> {
    let header = Style::new().bold().cyan();
    let dim = Style::new().dim();
    let prompt_style = Style::new().bold().green();

    println!(
        "\n  {} \"{}\" has {} chapters:",
        header.apply_to(">>"),
        video.title,
        video.chapters.len()
    );

    for (i, ch) in video.chapters.iter().enumerate() {
        println!(
            "  {}{:>3}{} {} {}{}{}",
            Style::new().magenta().apply_to("["),
            Style::new().magenta().bold().apply_to(i + 1),
            Style::new().magenta().apply_to("]"),
            ch.title,
            dim.apply_to("("),
            dim.apply_to(ch.time_display()),
            dim.apply_to(")"),
        );
    }

    println!(
        "\n  {} Select chapters (e.g. 1,3-5, 'all' for all chapters, or press Enter for full video):",
        prompt_style.apply_to(">>")
    );

    let input: String = Input::new()
        .with_prompt("  ")
        .allow_empty(true)
        .interact_text()
        .map_err(|_| AppError::Cancelled)?;

    let input = input.trim();

    if input.is_empty() {
        // Full video, no sections
        return Ok(());
    }

    let indices = parse_ranges(input, video.chapters.len())?;

    video.selected_sections = indices
        .iter()
        .map(|&i| {
            let ch = &video.chapters[i - 1];
            (ch.start_time, ch.end_time)
        })
        .collect();

    Ok(())
}

pub fn prompt_format() -> Result<MediaFormat> {
    let options = MediaFormat::all_options();
    let labels: Vec<&str> = options.iter().map(|(l, _)| *l).collect();

    let selection = Select::new()
        .with_prompt("  Select format")
        .items(&labels)
        .default(0)
        .interact()
        .map_err(|_| AppError::Cancelled)?;

    Ok(options[selection].1)
}
