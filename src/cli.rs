use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "table-ocr", about = "Batch-enhance table images and extract data via OCR")]
pub struct Args {
    /// Input folder path containing images
    #[arg(short, long)]
    pub input_dir: PathBuf,

    /// Output CSV file path
    #[arg(short, long)]
    pub output: PathBuf,

    /// Language(s) for OCR (e.g., "eng+jpn")
    #[arg(short, long, default_value = "eng")]
    pub lang: String,

    /// Number of expected columns per row in output CSV
    #[arg(short, long, default_value_t = 5)]
    pub columns: usize,

    /// X start coordinate for column OCR
    #[arg(long)]
    pub x_start: Option<u32>,

    /// X end coordinate for column OCR
    #[arg(long)]
    pub x_end: Option<u32>,

    /// Target DPI for image scaling (default: 300)
    /// Images are scaled to a width of: 3000px * (dpi/300)
    /// - 300 DPI -> 3000px width (good for most scans)
    /// - 600 DPI -> 6000px width (high-res scans)
    /// Use this to ensure consistent x-coordinates across images.
    #[arg(long, default_value = "300")]
    pub target_dpi: u32,

    /// Enable deskewing
    #[arg(long, default_value = "true")]
    pub deskew: bool,

    /// Skip heavy noise removal filters for clean digital images/screenshots
    #[arg(long, default_value_t = false)]
    pub skip_denoise: bool,
}
