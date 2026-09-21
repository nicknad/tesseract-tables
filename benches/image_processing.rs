use criterion::{Criterion, criterion_group, criterion_main};
use image::DynamicImage;
use std::path::Path;
use table_ocr::domain::entities::ProcessingConfig;
use table_ocr::infrastructure::image_processing::pipeline::ImagePipeline;

fn benchmark_pipeline(c: &mut Criterion) {
    let pipeline = ImagePipeline::new();
    let config = ProcessingConfig {
        language: "pol".to_string(),
        target_dpi: 300,
        column_region: None,
        skip_denoise: false,
    };

    let img_path = Path::new("files/IMG_4335.png");
    let img = if img_path.exists() {
        image::open(img_path).expect("Failed to open test image")
    } else {
        DynamicImage::ImageRgb8(image::RgbImage::new(1000, 1000))
    };

    c.bench_function("image_pipeline_process", |b| {
        b.iter(|| {
            let _ = pipeline.process(img.clone(), &config);
        })
    });
}

criterion_group!(benches, benchmark_pipeline);
criterion_main!(benches);
