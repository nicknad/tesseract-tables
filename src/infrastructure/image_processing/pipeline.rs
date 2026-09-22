use image::DynamicImage;

use crate::domain::entities::ProcessingConfig;
use crate::infrastructure::image_processing::binarization::binarize_otsu;
use crate::infrastructure::image_processing::noise_removal::remove_noise;
use crate::infrastructure::image_processing::scaler::scale_to_dpi;

pub struct ImagePipeline;

impl ImagePipeline {
    pub fn process(&self, image: DynamicImage, config: &ProcessingConfig) -> DynamicImage {
        let img = scale_to_dpi(image, config.target_dpi);
        let img = binarize_otsu(img);

        if config.skip_denoise {
            img
        } else {
            remove_noise(img)
        }
    }
}
