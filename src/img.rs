use crate::{err, ImgError, Result};
use image::{DynamicImage, ImageOutputFormat};
use std::{
    io::{BufRead, Seek, Write},
    ops::Deref,
    str::FromStr,
};

#[derive(Clone)]
pub struct TargetFormat {
    format: String,
}

impl Default for TargetFormat {
    fn default() -> Self {
        Self {
            format: String::from_str("jpeg").unwrap(),
        }
    }
}

const DEFAULT_JPEG_QUALITY: u8 = 10;

impl FromStr for TargetFormat {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let format = match s {
            "jpeg" => s,
            "png" => s,
            _ => return err!("Invalid format"),
        };

        Ok(Self {
            format: format.to_string(),
        })
    }
}

impl From<ImageOutputFormat> for TargetFormat {
    fn from(value: ImageOutputFormat) -> Self {
        use ImageOutputFormat::*;
        let format = match value {
            Png => "png",
            Jpeg(_) => "jpeg",
            _ => "jpeg",
        };

        Self {
            format: format.to_string(),
        }
    }
}

impl From<TargetFormat> for ImageOutputFormat {
    fn from(value: TargetFormat) -> Self {
        use ImageOutputFormat::*;
        match value.format.deref() {
            "jpeg" => Jpeg(DEFAULT_JPEG_QUALITY),
            "png" => Png,
            _ => Jpeg(DEFAULT_JPEG_QUALITY),
        }
    }
}

pub struct Img {
    dynamic: DynamicImage,
    target_format: TargetFormat,
}

impl Img {
    pub fn new<R: BufRead + Seek>(source: R, target_format: TargetFormat) -> Result<Self> {
        let dynamic = image::io::Reader::new(source)
            .with_guessed_format()?
            .decode()?;

        Ok(Self {
            dynamic,
            target_format,
        })
    }

    pub fn write_to<W: Write + Seek>(self, writer: &mut W) -> Result<()> {
        let format: ImageOutputFormat = self.target_format.into();
        self.dynamic.write_to(writer, format)?;
        Ok(())
    }
}

impl Deref for Img {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.dynamic.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;
    use crate::Result;

    #[test]
    fn test_file() -> Result<()> {
        let file = std::fs::File::open("./resources/sample_png.png")?;

        let reader = BufReader::new(file);

        let _ = Img::new(reader, TargetFormat::from_str("png")?)?;

        Ok(())
    }
}
