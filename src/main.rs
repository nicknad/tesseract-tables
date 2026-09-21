use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

use table_ocr::cli::Args;
use table_ocr::csv_export::write_csv_from_ocr_texts;
use table_ocr::domain::entities::{ColumnRegion, ProcessingConfig};
use table_ocr::infrastructure::image_processing::pipeline::ImagePipeline;
use table_ocr::infrastructure::ocr::tesseract::LeptessOcrEngine;
use table_ocr::infrastructure::parallel::batch_processor::RayonBatchProcessor;

fn main() -> Result<()> {
    let args = Args::parse();
    eprintln!(
        "Starting OCR batch with input: {:?}, output: {:?}, lang: {}, target_dpi: {}",
        args.input_dir, args.output, args.lang, args.target_dpi
    );

    let image_paths = get_image_paths(&args.input_dir)?;
    eprintln!("Found {} images to process", image_paths.len());

    let column_region = if args.x_start.is_some() || args.x_end.is_some() {
        Some(ColumnRegion {
            x_start: args.x_start.unwrap_or(0),
            x_end: args.x_end.unwrap_or(10000),
        })
    } else {
        None
    };

    let config = ProcessingConfig {
        language: args.lang,
        target_dpi: args.target_dpi,
        column_region,
        skip_denoise: args.skip_denoise,
    };

    let batch_processor = RayonBatchProcessor::new(ImagePipeline::new(), LeptessOcrEngine::new());
    let results = batch_processor.process_batch(&image_paths, &config)?;

    let all_text_lines: Vec<String> = results.into_iter().flat_map(|r| r.text_lines).collect();

    eprintln!("Total lines collected: {}", all_text_lines.len());
    write_csv_from_ocr_texts(&args.output, &all_text_lines, args.columns)?;
    eprintln!("CSV file written to {:?}", &args.output);

    Ok(())
}

fn get_image_paths(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if is_image(&path) {
            paths.push(path);
        }
    }
    Ok(paths)
}

fn is_image(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase());
    matches!(
        ext.as_deref(),
        Some("png" | "jpg" | "jpeg" | "bmp" | "tiff")
    )
}
