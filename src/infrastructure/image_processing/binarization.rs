use image::{DynamicImage, Luma};
use imageproc::contrast::otsu_level;

pub fn binarize_otsu(img: DynamicImage) -> DynamicImage {
    let mut gray = img.into_luma8();
    let p = otsu_level(&gray);
    for pixel in gray.pixels_mut() {
        if pixel[0] > p {
            *pixel = Luma([255]);
        } else {
            *pixel = Luma([0]);
        }
    }
    DynamicImage::ImageLuma8(gray)
}
