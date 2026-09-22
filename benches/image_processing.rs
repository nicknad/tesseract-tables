use criterion::{Criterion, criterion_group, criterion_main};
use image::DynamicImage;
use image::imageops::FilterType;
use std::path::Path;
use table_ocr::domain::entities::ProcessingConfig;
use table_ocr::infrastructure::image_processing::pipeline::ImagePipeline;

fn test_image() -> DynamicImage {
    let img_path = Path::new("files/IMG_4335.png");
    if img_path.exists() {
        image::open(img_path).expect("Failed to open test image")
    } else {
        DynamicImage::ImageRgb8(image::RgbImage::new(1284, 2778))
    }
}

fn benchmark_pipeline(c: &mut Criterion) {
    let pipeline = ImagePipeline;
    let img = test_image();

    for (name, skip_denoise) in [("pipeline_full", false), ("pipeline_skip_denoise", true)] {
        let config = ProcessingConfig {
            language: "eng".to_string(),
            target_dpi: 300,
            column_region: None,
            skip_denoise,
        };

        c.bench_function(name, |b| {
            b.iter(|| {
                let _ = pipeline.process(img.clone(), &config);
            })
        });
    }
}

fn benchmark_resizing(c: &mut Criterion) {
    let img = test_image();
    let target_width = 3000;
    let target_height = (img.height() as f32 * target_width as f32 / img.width() as f32) as u32;

    for (name, filter) in [
        ("resize_triangle", FilterType::Triangle),
        ("resize_catmullrom", FilterType::CatmullRom),
        ("resize_lanczos3", FilterType::Lanczos3),
    ] {
        c.bench_function(name, |b| {
            b.iter(|| img.resize(target_width, target_height, filter))
        });
    }
}

criterion_group!(benches, benchmark_pipeline, benchmark_resizing);
criterion_main!(benches);
