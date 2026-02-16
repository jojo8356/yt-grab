mod cli;
mod core;
mod error;
mod model;

use clap::Parser;
use console::Style;
use indicatif::MultiProgress;

use cli::args::Args;
use cli::interactive;
use core::downloader::download_all;
use core::playlist::{PlaylistFetcher, YtDlpFetcher};
use model::config::DownloadConfig;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        let err_style = Style::new().bold().red();
        eprintln!("\n  {} {e}", err_style.apply_to("Error:"));
        std::process::exit(1);
    }
}

async fn run() -> error::Result<()> {
    let args = Args::parse();
    let header = Style::new().bold().cyan();
    let success = Style::new().bold().green();
    let err_style = Style::new().bold().red();

    // Fetch playlist/video info
    println!(
        "\n  {} Fetching info from URL...",
        header.apply_to("yt-grab")
    );

    let fetcher = YtDlpFetcher;
    let (playlist_title, videos) = fetcher.fetch(&args.url)?;

    let is_single = videos.len() == 1 && playlist_title.is_none();

    // Select videos
    let selected = if is_single {
        println!(
            "  {} Single video: {}",
            header.apply_to(">>"),
            videos[0].title
        );
        videos
    } else {
        interactive::display_playlist(&playlist_title, &videos);

        let indices = if args.all {
            (1..=videos.len()).collect()
        } else {
            interactive::prompt_selection(videos.len())?
        };

        videos
            .into_iter()
            .filter(|v| indices.contains(&v.index))
            .collect()
    };

    println!(
        "  {} {} item(s) selected",
        header.apply_to(">>"),
        selected.len()
    );

    // Fetch chapters for each selected video and let user pick
    let mut selected = selected;
    for video in &mut selected {
        // For playlist items, chapters weren't fetched yet (flat-playlist mode)
        if !video.has_chapters() && playlist_title.is_some() {
            println!(
                "  {} Fetching chapters for \"{}\"...",
                header.apply_to(">>"),
                video.title
            );
            match YtDlpFetcher::fetch_chapters(&video.url) {
                Ok(chapters) if !chapters.is_empty() => {
                    video.chapters = chapters;
                }
                _ => {}
            }
        }

        if video.has_chapters() {
            interactive::prompt_chapter_selection(video)?;
        }
    }

    // Select format
    let format = match args.parse_format() {
        Some(f) => f,
        None => interactive::prompt_format()?,
    };

    println!("  {} Format: {format}\n", header.apply_to(">>"));

    let config = DownloadConfig {
        format,
        output_dir: args.output.clone(),
        concurrency: args.concurrency,
    };

    // Download
    let multi = MultiProgress::new();
    let results = download_all(&selected, &config, &multi).await;

    // Summary
    let succeeded = results.iter().filter(|r| r.success).count();
    let failed = results.iter().filter(|r| !r.success).count();

    println!();
    if failed == 0 {
        println!(
            "  {} All {succeeded} download(s) completed!",
            success.apply_to("Done!")
        );
    } else {
        println!(
            "  {} {succeeded} succeeded, {failed} failed",
            err_style.apply_to("Done.")
        );
        for r in &results {
            if !r.success {
                if let Some(err) = &r.error {
                    println!("    {} {}: {err}", err_style.apply_to("✗"), r.title);
                }
            }
        }
    }

    println!(
        "  {} {}",
        header.apply_to("Output:"),
        args.output.display()
    );
    println!();

    Ok(())
}
