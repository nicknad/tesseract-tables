//! Error module
//!
//! Centralized error type using `thiserror` for consistent error reporting.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),

    #[error("OCR error: {0}")]
    Ocr(#[from] Box<dyn std::error::Error + Send + Sync>),
    // Ocr(#[from] leptess::LepTessError),

    #[error("CSV writing error: {0}")]
    Csv(#[from] csv::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
