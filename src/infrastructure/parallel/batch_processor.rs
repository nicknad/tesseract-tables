use anyhow::Result;
use image::ImageReader;
use rayon::prelude::*;
use std::path::PathBuf;

use crate::domain::entities::{OcrResult, ProcessingConfig};
use crate::infrastructure::image_processing::pipeline::ImagePipeline;
use crate::infrastructure::ocr::tesseract::LeptessOcrEngine;

pub struct RayonBatchProcessor {
    image_processor: ImagePipeline,
    ocr_engine: LeptessOcrEngine,
}

impl RayonBatchProcessor {
    pub fn new(image_processor: ImagePipeline, ocr_engine: LeptessOcrEngine) -> Self {
        Self {
            image_processor,
            ocr_engine,
        }
    }

    pub fn process_batch(
        &self,
        images: &[PathBuf],
        config: &ProcessingConfig,
    ) -> Result<Vec<OcrResult>> {
        images
            .par_iter()
            .map(|path| {
                eprintln!("Processing image: {}", path.display());
                let img = ImageReader::open(path)?.with_guessed_format()?.decode()?;
                let enhanced = self.image_processor.process(img, config);
                self.ocr_engine.recognize(
                    &enhanced,
                    &config.language,
                    config.column_region.as_ref(),
                )
            })
            .collect()
    }
}
