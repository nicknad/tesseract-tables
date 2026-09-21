use anyhow::Result;
use tracing::info;
use rayon::prelude::*;
use std::sync::Arc;
use image::ImageReader;
use crate::domain::traits::{BatchProcessor, ImageProcessor, OcrEngine};
use crate::domain::entities::{ImageInfo, OcrResult, ProcessingConfig};

pub struct RayonBatchProcessor {
    image_processor: Arc<dyn ImageProcessor>,
    ocr_engine: Arc<dyn OcrEngine>,
}

impl RayonBatchProcessor {
    pub fn new(image_processor: Arc<dyn ImageProcessor>, ocr_engine: Arc<dyn OcrEngine>) -> Self {
        Self {
            image_processor,
            ocr_engine,
        }
    }
}

impl BatchProcessor for RayonBatchProcessor {
    fn process_batch(&self, images: &[ImageInfo], config: &ProcessingConfig) -> Result<Vec<OcrResult>> {
        images.par_iter()
            .map(|info| {
                info!("Processing Image: {}", info.path.display());
                let img = ImageReader::open(&info.path)?
                    .with_guessed_format()?
                    .decode()?;
                let enhanced = self.image_processor.process(img, config)?;
                
                if let Some(ref region) = config.column_region {
                    self.ocr_engine.recognize_region(&enhanced, &config.language, region)
                } else {
                    self.ocr_engine.recognize(&enhanced, &config.language)
                }
            })
            .collect::<Result<Vec<OcrResult>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::traits::{MockImageProcessor, MockOcrEngine};
    use crate::domain::entities::{ImageInfo, OcrResult, ProcessingConfig};
    use std::sync::Arc;
    use tempfile::NamedTempFile;
    use image::{DynamicImage, RgbImage};

    #[test]
    fn test_process_batch() {
        let mut mock_processor = MockImageProcessor::new();
        let mut mock_ocr = MockOcrEngine::new();

        // Setup expectations
        mock_processor.expect_process()
            .times(1)
            .returning(|img, _| Ok(img));
            
        mock_ocr.expect_recognize()
            .times(1)
            .returning(|_, _| Ok(OcrResult { text_lines: vec!["test".to_string()] }));

        let processor = RayonBatchProcessor::new(
            Arc::new(mock_processor),
            Arc::new(mock_ocr)
        );

        // Create a dummy image file
        let tmp_file = NamedTempFile::new().unwrap();
        let img = DynamicImage::ImageRgb8(RgbImage::new(10, 10));
        img.save(tmp_file.path()).unwrap();

        let images = vec![ImageInfo { path: tmp_file.path().to_path_buf() }];
        let config = ProcessingConfig {
            language: "eng".to_string(),
            target_dpi: 300,
            deskew: false,
            column_region: None,
            skip_denoise: false,
        };

        let results = processor.process_batch(&images, &config).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].text_lines[0], "test");
    }
}
