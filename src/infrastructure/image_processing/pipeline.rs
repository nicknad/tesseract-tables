use anyhow::Result;
use image::DynamicImage;
use crate::domain::traits::ImageProcessor;
use crate::domain::entities::ProcessingConfig;
use crate::infrastructure::image_processing::scaler::scale_to_dpi;
use crate::infrastructure::image_processing::binarization::binarize_otsu;
use crate::infrastructure::image_processing::deskew::deskew;
use crate::infrastructure::image_processing::noise_removal::remove_noise;
use tracing::info_span;

pub struct ImagePipeline;

impl ImagePipeline {
    pub fn new() -> Self {
        Self
    }
    
    // Internal helper for steps
    fn apply_steps(&self, image: DynamicImage, config: &ProcessingConfig) -> Result<DynamicImage> {
        let _span = info_span!("image_processing_pipeline").entered();

        // 1. Scaling
        let mut img = {
            let _s = info_span!("scale_to_dpi", dpi = config.target_dpi).entered();
            scale_to_dpi(image, config.target_dpi)
        };
        
        // 2. Grayscale (ensure it's Luma before morphological/binarization steps)
        img = {
            let _s = info_span!("to_grayscale").entered();
            DynamicImage::ImageLuma8(img.into_luma8())
        };
        
        // 3. Deskew
        if config.deskew {
            img = {
                let _s = info_span!("deskew").entered();
                deskew(img)
            };
        }
        
        // 4. Binarization
        img = {
            let _s = info_span!("binarization_otsu").entered();
            binarize_otsu(img)
        };
        
        // 5. Noise Removal
        if !config.skip_denoise {
            img = {
                let _s = info_span!("noise_removal").entered();
                remove_noise(img)
            };
        }
        
        Ok(img)
    }
}

impl ImageProcessor for ImagePipeline {
    fn process(&self, image: DynamicImage, config: &ProcessingConfig) -> Result<DynamicImage> {
        self.apply_steps(image, config)
    }
}
