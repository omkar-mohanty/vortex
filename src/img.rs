use super::Extract;
use crate::{err, ImgError};
use image::ImageOutputFormat;
use pdf::object::ImageDict;
use std::{ops::Deref, str::FromStr};

#[derive(Clone)]
pub enum ImageFormat {
    Jpeg(u8),
    Png,
    Jp2k,
}

impl Default for ImageFormat {
    fn default() -> Self {
        ImageFormat::Jpeg(DEFAULT_JPEG_QUALITY)
    }
}

const DEFAULT_JPEG_QUALITY: u8 = 10;

impl FromStr for ImageFormat {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use ImageFormat::*;
        Ok(match s {
            "jpeg" => Jpeg(DEFAULT_JPEG_QUALITY),
            "png" => Png,
            "jp2k" => Jp2k,
            _ => return err!("Invalid format"),
        })
    }
}

impl From<ImageOutputFormat> for ImageFormat {
    fn from(value: ImageOutputFormat) -> Self {
        use ImageOutputFormat::*;
        match value {
            Png => ImageFormat::Png,
            Jpeg(q) => ImageFormat::Jpeg(q),
            _ => ImageFormat::Jpeg(DEFAULT_JPEG_QUALITY),
        }
    }
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ImageFormat::*;

        match self {
            Png => f.write_str("png"),
            Jpeg(_) => f.write_str("jpeg"),
            Jp2k => f.write_str("jp2k"),
        }
    }
}

impl From<ImageFormat> for ImageOutputFormat {
    fn from(value: ImageFormat) -> Self {
        use ImageOutputFormat::*;
        match value {
            ImageFormat::Jpeg(q) => Jpeg(q),
            ImageFormat::Png => Png,
            _ => Jpeg(DEFAULT_JPEG_QUALITY),
        }
    }
}

pub struct RawImage {
    data: Vec<u8>,
    pub image_dict: ImageDict,
}

impl Extract for RawImage {}

impl RawImage {
    pub fn new(source: &[u8], image_dict: ImageDict) -> Self {
        Self {
            data: source.to_vec(),
            image_dict,
        }
    }
}

impl Deref for RawImage {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}
