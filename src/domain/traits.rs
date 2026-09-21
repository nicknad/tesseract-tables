use anyhow::Result;
use image::DynamicImage;
use crate::domain::entities::{OcrResult, ProcessingConfig, ImageInfo, ColumnRegion};

#[cfg_attr(test, mockall::automock)]
pub trait ImageProcessor: Send + Sync {
    fn process(&self, image: DynamicImage, config: &ProcessingConfig) -> Result<DynamicImage>;
}

#[cfg_attr(test, mockall::automock)]
pub trait OcrEngine: Send + Sync {
    fn recognize(&self, image: &DynamicImage, lang: &str) -> Result<OcrResult>;
    fn recognize_region(&self, image: &DynamicImage, lang: &str, region: &ColumnRegion) -> Result<OcrResult>;
}

#[cfg_attr(test, mockall::automock)]
pub trait BatchProcessor: Send + Sync {
    fn process_batch(&self, images: &[ImageInfo], config: &ProcessingConfig) -> Result<Vec<OcrResult>>;
}
