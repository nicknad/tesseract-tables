use criterion::{criterion_group, criterion_main, Criterion};
use image::DynamicImage;
use table_ocr::domain::entities::ProcessingConfig;
use table_ocr::domain::traits::ImageProcessor;
use table_ocr::infrastructure::image_processing::pipeline::ImagePipeline;
use std::path::Path;

fn benchmark_pipeline(c: &mut Criterion) {
    let pipeline = ImagePipeline::new();
    let config = ProcessingConfig {
        language: "pol".to_string(),
        target_dpi: 300,
        deskew: true,
        column_region: None,
    };

    // Try to load a real image from the files directory if it exists
    let img_path = Path::new("files/IMG_4335.png");
    let img = if img_path.exists() {
        image::open(img_path).expect("Failed to open test image")
    } else {
        // Fallback to a generated image if the file is missing
        DynamicImage::ImageRgb8(image::RgbImage::new(1000, 1000))
    };

    c.bench_function("image_pipeline_process", |b| {
        b.iter(|| {
            let _ = pipeline.process(img.clone(), &config).unwrap();
        })
    });
}

criterion_group!(benches, benchmark_pipeline);
criterion_main!(benches);
