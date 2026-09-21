//! CSV export module
//!
//! Converts raw OCR text lines into a structured CSV file.

use std::fs::File;
use std::path::Path;
use anyhow::Result;
use csv::Writer;

/// Writes OCR-extracted lines to a CSV file with fixed column layout.
pub fn write_csv_from_ocr_texts<P: AsRef<Path>>(
    path: P,
    lines: &[String],
    columns: usize,
) -> Result<()> {
    let mut writer = Writer::from_writer(File::create(path)?);

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let cells = line
            .split_whitespace()
            .take(columns);

        let trimmed: Vec<&str> = cells.into_iter()
            .take_while(|&s| !s.is_empty() && s != "\"\"")
            .collect();

        writer.write_record(trimmed)?;
    }

    writer.flush()?;
    Ok(())
}
