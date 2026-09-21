use anyhow::Result;
use image::{DynamicImage, ImageFormat};
use leptess::{LepTess, Variable};
use std::cell::RefCell;
use std::io::Cursor;

use crate::domain::entities::{ColumnRegion, OcrResult};

thread_local! {
    static TESS_ENGINE: RefCell<Option<(String, LepTess)>> = RefCell::new(None);
}

pub struct LeptessOcrEngine;

impl LeptessOcrEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn recognize(
        &self,
        image: &DynamicImage,
        lang: &str,
        region: Option<&ColumnRegion>,
    ) -> Result<OcrResult> {
        let mut bmp_bytes = Vec::new();
        image.write_to(&mut Cursor::new(&mut bmp_bytes), ImageFormat::Bmp)?;

        TESS_ENGINE.with(|cell| {
            let mut cache = cell.borrow_mut();
            if cache.is_none() || cache.as_ref().unwrap().0 != lang {
                let mut lt = LepTess::new(None, lang)?;
                lt.set_variable(Variable::TesseditPagesegMode, "6")?;
                *cache = Some((lang.to_string(), lt));
            }

            let (_, lt) = cache.as_mut().unwrap();
            lt.set_image_from_mem(&bmp_bytes)?;

            if let Some(region) = region {
                lt.set_rectangle(
                    region.x_start as i32,
                    0,
                    region.x_end.saturating_sub(region.x_start) as i32,
                    image.height() as i32,
                );
            }

            let text = lt.get_utf8_text()?;
            Ok(OcrResult {
                text_lines: text.lines().map(|s| s.to_string()).collect(),
            })
        })
    }
}
