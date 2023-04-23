use crate::Result;
use image::{DynamicImage, ImageOutputFormat};
use std::{io::Cursor, str::FromStr};

#[derive(Clone)]
pub struct TargetFormat {
    format: ImageOutputFormat,
}

const DEFAULT_JPEG_QUALITY: u8 = 10;

impl FromStr for TargetFormat {
    type Err = &'static str;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use ImageOutputFormat::*;

        let format = match s {
            "jpeg" => Jpeg(DEFAULT_JPEG_QUALITY),
            "png" => Png,
            _ => return Err("Invalid format"),
        };

        Ok(Self { format })
    }
}

pub struct Image {
    dynamic: DynamicImage,
    target_format: TargetFormat,
}

impl Image {
    pub fn new(data: &[u8], target_format: TargetFormat) -> Result<Self> {
        let dynamic = image::io::Reader::new(Cursor::new(data))
            .with_guessed_format()?
            .decode()?;

        Ok(Self {
            dynamic,
            target_format,
        })
    }
}
