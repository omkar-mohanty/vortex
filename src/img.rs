use std::str::FromStr;
use image::ImageOutputFormat;

#[derive(Clone)]
pub struct  TargetFormat {
    format: ImageOutputFormat
}

const DEFAULT_JPEG_QUALITY: u8 = 10;

impl FromStr for TargetFormat {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ImageOutputFormat::*;

        let format = match s {
            "jpeg" =>  Jpeg(DEFAULT_JPEG_QUALITY),
            "png" => Png,
            _ =>return  Err("Invalid format")
        };

        Ok(Self {
            format
        })
    }
}
