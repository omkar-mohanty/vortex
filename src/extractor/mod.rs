use std::{ops::Range, path::PathBuf};

use crate::{Extract, RawImage};

pub enum ExtractType {
    Image,
    Page {
        range: Range<usize>,
        exclusion: Option<Vec<usize>>,
    },
    Text,
}

pub trait Extractor<T: Extract> {
    fn extract(&self) -> T;
    fn extract_all(&self) -> Vec<T>;
}

struct ImageExtractor {
    file_path: PathBuf,
}

impl Extractor<RawImage> for ImageExtractor {
    fn extract(&self) -> RawImage {
        todo!("Implement Image Extractor")
    }

    fn extract_all(&self) -> Vec<RawImage> {
        todo!("Implement extract all");
    }
}

impl ImageExtractor {
    pub fn new(file_path: PathBuf) -> Self {
        ImageExtractor { file_path }
    }
}

pub fn create_extractor<T: Extract>(
    file_path: PathBuf,
    extract_type: ExtractType,
) -> Box<dyn Extractor<T>>
where
    ImageExtractor: Extractor<T>,
{
    use ExtractType::*;
    match extract_type {
        Image => {
            let extractor = ImageExtractor::new(file_path);

            Box::new(extractor)
        }
        _ => todo!("Implement all extractor"),
    }
}
