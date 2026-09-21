use image::DynamicImage;
use imageproc::distance_transform::Norm;
use imageproc::filter::median_filter;
use imageproc::morphology::{close, open};

pub fn remove_noise(img: DynamicImage) -> DynamicImage {
    // 1. Ensure we have a Luma8 image for imageproc operations
    let gray = img.into_luma8();

    // 2. Apply Median Filter to remove salt-and-pepper noise
    // Radius 1 means a 3x3 window
    let filtered = median_filter(&gray, 1, 1);

    // 3. Morphological Opening (Erosion then Dilation)
    // This removes small white noise (bright spots) from the background
    let opened = open(&filtered, Norm::L1, 1);

    // 4. Morphological Closing (Dilation then Erosion)
    // This fills small black holes in the white foreground (characters)
    let closed = close(&opened, Norm::L1, 1);

    DynamicImage::ImageLuma8(closed)
}
