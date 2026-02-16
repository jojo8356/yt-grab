# yt-grab

Fast, interactive YouTube playlist downloader built in Rust. Pick videos, select chapters, choose your format — or script it all from the command line.

## Features

- **Playlist & single video** support
- **Chapter-aware** — browse and download specific chapters from long videos
- **Multiple formats** — MP3, Opus, M4A, MP4, MKV
- **Concurrent downloads** with real-time progress bars
- **Fully scriptable** — every interactive prompt has a CLI flag equivalent

## Prerequisites

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) must be installed and available in `PATH`
- [ffmpeg](https://ffmpeg.org/) (required by yt-dlp for audio extraction and format conversion)

## Installation

### From releases

Download the latest binary for your platform from the [Releases](https://github.com/jojo8356/yt-grab/releases) page.

### From source

```bash
cargo install --path .
```

## Usage

```
yt-grab [OPTIONS] <URL>
```

### Interactive mode

Just pass a URL — yt-grab will walk you through the rest:

```bash
yt-grab "https://www.youtube.com/playlist?list=PLxxxxxxx"
```

### Non-interactive mode

Use CLI flags to skip all prompts:

```bash
# Download the first video of a playlist as MP3
yt-grab --items 1 --format mp3 --no-chapters "https://www.youtube.com/playlist?list=PLxxxxxxx"

# Download all videos as MP4
yt-grab --all --format mp4 --no-chapters -o ~/Videos "https://www.youtube.com/playlist?list=PLxxxxxxx"

# Download a single video — no playlist prompt needed
yt-grab --format opus --no-chapters "https://www.youtube.com/watch?v=xxxxxxxxx"
```

### Options

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--output <DIR>` | `-o` | Output directory | `.` |
| `--format <FMT>` | `-f` | Format: `mp3`, `opus`, `m4a`, `mp4`, `mkv` | interactive |
| `--items <RANGE>` | `-i` | Select items (e.g. `1`, `1,3-5`) | interactive |
| `--all` | | Download all playlist items | `false` |
| `--no-chapters` | | Skip chapter selection, download full video | `false` |
| `--concurrency <N>` | `-c` | Parallel downloads | `3` |

### Range syntax

The `--items` flag (and interactive prompts) accept:

- Single index: `3`
- Comma-separated: `1,3,5`
- Ranges: `2-5`
- Mixed: `1,3-5,8`
- Everything: `all`

## Examples

```bash
# Interactive — pick videos and chapters from a playlist
yt-grab "https://www.youtube.com/playlist?list=PLJLf67YAhhod0OMooE_QOYcU_D63d26_p"

# Grab just audio from video #2 of a playlist
yt-grab -i 2 -f mp3 --no-chapters "https://www.youtube.com/playlist?list=PLJLf67YAhhod0OMooE_QOYcU_D63d26_p"

# Batch download an entire playlist as MKV into a folder
yt-grab --all -f mkv --no-chapters -o ./courses "https://www.youtube.com/playlist?list=PLJLf67YAhhod0OMooE_QOYcU_D63d26_p"
```

## Building

```bash
git clone https://github.com/jojo8356/yt-grab.git
cd yt-grab
cargo build --release
```

The binary will be at `target/release/yt-grab`.

## License

MIT
