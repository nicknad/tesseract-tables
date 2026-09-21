use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use table_ocr::cli::Args;
use table_ocr::domain::entities::{ImageInfo, ProcessingConfig, ColumnRegion};
use table_ocr::domain::traits::BatchProcessor;
use table_ocr::infrastructure::image_processing::pipeline::ImagePipeline;
use table_ocr::infrastructure::image_processing::scaler::get_target_width;
use table_ocr::infrastructure::ocr::tesseract::LeptessOcrEngine;
use table_ocr::infrastructure::parallel::batch_processor::RayonBatchProcessor;
use table_ocr::csv_export::write_csv_from_ocr_texts;

use tracing::{info_span, info};
use tracing_subscriber::prelude::*;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    // Initialize tracing with Chrome trace exporter
    let (chrome_layer, _guard) = tracing_chrome::ChromeLayerBuilder::new()
        .include_args(true)
        .build();
    
    tracing_subscriber::registry()
        .with(chrome_layer)
        .with(tracing_subscriber::fmt::layer()) // Log to stdout too
        .init();

    info!("Starting app");
    let args = Args::parse();
    let target_width = get_target_width(args.target_dpi);
    info!(
        "Starting OCR batch with input: {:?}, output: {:?}, lang: {}, target_dpi: {}, target_width: {}px",
        args.input_dir, args.output, args.lang, args.target_dpi, target_width
    );

    let image_paths = get_image_paths(&args.input_dir)?;
    info!("Found {} images to process", image_paths.len());

    let images: Vec<ImageInfo> = image_paths
        .into_iter()
        .map(|path| ImageInfo { path })
        .collect();

    let column_region = if args.x_start.is_some() || args.x_end.is_some() {
        Some(ColumnRegion {
            x_start: args.x_start.unwrap_or(0),
            x_end: args.x_end.unwrap_or(10000), 
            y_start: None,
            y_end: None,
        })
    } else {
        None
    };

    let config = ProcessingConfig {
        language: args.lang,
        target_dpi: args.target_dpi,
        deskew: args.deskew,
        column_region,
        skip_denoise: args.skip_denoise,
    };

    let image_processor = Arc::new(ImagePipeline::new());
    let ocr_engine = Arc::new(LeptessOcrEngine::new());
    let batch_processor = RayonBatchProcessor::new(image_processor, ocr_engine);

    let span = info_span!("batch_processing").entered();
    let results = batch_processor.process_batch(&images, &config)?;
    drop(span);

    let mut all_text_lines = Vec::new();
    for res in results {
        all_text_lines.extend(res.text_lines.into_iter().filter(|s| !s.trim().is_empty()));
    }

    info!("Total lines collected: {}", all_text_lines.len());
    write_csv_from_ocr_texts(&args.output, &all_text_lines, args.columns)?;
    info!("CSV file written to {:?}", &args.output);

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
    let ext = path.extension().and_then(|e| e.to_str()).map(|s| s.to_ascii_lowercase());
    matches!(ext.as_deref(), Some("png" | "jpg" | "jpeg" | "bmp" | "tiff"))
}
