use image::DynamicImage;
use image::imageops::FilterType;

/// Minimum target width for clear OCR (pixels).
const MIN_TARGET_WIDTH: u32 = 3000;

/// Baseline DPI the minimum target width is defined at.
const BASELINE_DPI: u32 = 300;

/// Scale an image up to the width implied by `target_dpi`.
///
/// 300 DPI maps to a 3000px width, 600 DPI to 6000px, and so on. Images that
/// are already at least that wide are returned unchanged.
pub fn scale_to_dpi(img: DynamicImage, target_dpi: u32) -> DynamicImage {
    let factor = (target_dpi as f32 / BASELINE_DPI as f32).max(1.0);
    let target_width = (MIN_TARGET_WIDTH as f32 * factor) as u32;

    if img.width() >= target_width {
        return img;
    }

    let scale = target_width as f32 / img.width() as f32;
    img.resize(
        target_width,
        (img.height() as f32 * scale) as u32,
        FilterType::Triangle,
    )
}
