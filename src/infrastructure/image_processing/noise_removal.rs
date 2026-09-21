use image::DynamicImage;
use imageproc::distance_transform::Norm;
use imageproc::filter::median_filter;
use imageproc::morphology::{close_mut, open_mut};

pub fn remove_noise(img: DynamicImage) -> DynamicImage {
    let gray = img.into_luma8();
    let mut filtered = median_filter(&gray, 1, 1);

    open_mut(&mut filtered, Norm::L1, 1);
    close_mut(&mut filtered, Norm::L1, 1);

    DynamicImage::ImageLuma8(filtered)
}
