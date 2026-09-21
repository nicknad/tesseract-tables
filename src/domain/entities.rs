use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct OcrResult {
    pub text_lines: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ColumnRegion {
    pub x_start: u32,
    pub x_end: u32,
    pub y_start: Option<u32>,
    pub y_end: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct ProcessingConfig {
    pub language: String,
    pub target_dpi: u32,
    pub deskew: bool,
    pub column_region: Option<ColumnRegion>,
    pub skip_denoise: bool,
}
