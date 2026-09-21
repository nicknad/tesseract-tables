use anyhow::Result;
use image::DynamicImage;
use leptess::{LepTess, Variable};
use crate::domain::traits::OcrEngine;
use crate::domain::entities::{OcrResult, ColumnRegion};
use std::io::Cursor;
use image::ImageFormat;
use tracing::info_span;

use std::cell::RefCell;

thread_local! {
    static TESS_ENGINE: RefCell<Option<(String, LepTess)>> = RefCell::new(None);
}

pub struct LeptessOcrEngine;

impl LeptessOcrEngine {
    pub fn new() -> Self {
        Self
    }

    fn with_engine<F, R>(&self, lang: &str, f: F) -> Result<R>
    where
        F: FnOnce(&mut LepTess) -> Result<R>,
    {
        TESS_ENGINE.with(|cell| {
            let mut cache = cell.borrow_mut();
            if cache.is_none() || cache.as_ref().unwrap().0 != lang {
                let _s = info_span!("tesseract_init").entered();
                let mut lt = LepTess::new(None, lang)?;
                lt.set_variable(Variable::TesseditPagesegMode, "6")?;
                *cache = Some((lang.to_string(), lt));
            }
            let (_, lt) = cache.as_mut().unwrap();
            f(lt)
        })
    }
}

impl OcrEngine for LeptessOcrEngine {
    fn recognize(&self, image: &DynamicImage, lang: &str) -> Result<OcrResult> {
        let _span = info_span!("ocr_recognize", lang = lang).entered();
        
        let bmp_bytes = {
            let _s = info_span!("image_to_bmp_bytes").entered();
            let mut bytes = Vec::new();
            image.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Bmp)?;
            bytes
        };
        
        self.with_engine(lang, |lt| {
            {
                let _s = info_span!("tesseract_set_image").entered();
                lt.set_image_from_mem(&bmp_bytes)?;
            }
            
            let text = {
                let _s = info_span!("tesseract_get_text").entered();
                lt.get_utf8_text()?
            };
            
            Ok(OcrResult {
                text_lines: text.lines().map(|s| s.to_string()).collect(),
            })
        })
    }

    fn recognize_region(&self, image: &DynamicImage, lang: &str, region: &ColumnRegion) -> Result<OcrResult> {
        let _span = info_span!("ocr_recognize_region", lang = lang).entered();
        
        let bmp_bytes = {
            let _s = info_span!("image_to_bmp_bytes").entered();
            let mut bytes = Vec::new();
            image.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Bmp)?;
            bytes
        };

        self.with_engine(lang, |lt| {
            {
                let _s = info_span!("tesseract_set_image").entered();
                lt.set_image_from_mem(&bmp_bytes)?;
            }
            
            let width = region.x_end - region.x_start;
            let height = region.y_end.unwrap_or(image.height()) - region.y_start.unwrap_or(0);
            
            lt.set_rectangle(
                region.x_start as i32, 
                region.y_start.unwrap_or(0) as i32, 
                width as i32, 
                height as i32
            );
            
            let text = {
                let _s = info_span!("tesseract_get_text").entered();
                lt.get_utf8_text()?
            };

            Ok(OcrResult {
                text_lines: text.lines().map(|s| s.to_string()).collect(),
            })
        })
    }
}
