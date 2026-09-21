use image::DynamicImage;
use imageproc::contrast::otsu_level;

pub fn binarize_otsu(img: DynamicImage) -> DynamicImage {
    let mut gray = img.into_luma8();
    let threshold = otsu_level(&gray);

    gray.pixels_mut().for_each(|pixel| {
        pixel[0] = if pixel[0] > threshold { 255 } else { 0 };
    });

    DynamicImage::ImageLuma8(gray)
}
