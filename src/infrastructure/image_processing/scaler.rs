use image::DynamicImage;
use image::imageops::FilterType;

/// Minimum target width for clear OCR (pixels)
/// At this resolution, text is crisp and x-coordinates are easy to configure.
const MIN_TARGET_WIDTH: u32 = 3000;

/// Standard DPI for OCR (baseline)
const BASELINE_DPI: u32 = 300;

/// Scale image for optimal OCR quality.
///
/// Ensures images have sufficient pixel width for:
/// - Clear text recognition
/// - Precise x-coordinate configuration for column regions
///
/// Images smaller than the target width are scaled up proportionally.
/// Larger images are left at their original size (already good for OCR).
pub fn scale_to_dpi(img: DynamicImage, target_dpi: u32) -> DynamicImage {
    let current_width = img.width();

    // Calculate target width based on DPI:
    // - 300 DPI -> 3000px (baseline)
    // - 600 DPI -> 6000px (high-res scan)
    // - 150 DPI -> 1500px (low-res, will be scaled up)
    let target_width = if target_dpi >= BASELINE_DPI {
        // Scale target width proportionally to DPI
        let factor = target_dpi as f32 / BASELINE_DPI as f32;
        (MIN_TARGET_WIDTH as f32 * factor) as u32
    } else {
        // For low DPI settings, still enforce minimum
        MIN_TARGET_WIDTH.max(current_width)
    };

    if current_width < target_width {
        // Scale up to target width, maintaining aspect ratio
        let scale_factor = target_width as f32 / current_width as f32;
        let new_w = target_width;
        let new_h = (img.height() as f32 * scale_factor) as u32;
        img.resize(new_w, new_h, FilterType::Lanczos3)
    } else {
        // Image is already at or above target size
        img
    }
}

/// Get the effective target width for a given DPI setting.
/// Useful for calculating x-coordinate bounds before processing.
pub fn get_target_width(target_dpi: u32) -> u32 {
    if target_dpi >= BASELINE_DPI {
        let factor = target_dpi as f32 / BASELINE_DPI as f32;
        (MIN_TARGET_WIDTH as f32 * factor) as u32
    } else {
        MIN_TARGET_WIDTH
    }
}
