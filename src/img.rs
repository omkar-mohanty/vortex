use crate::{err, ImgError, Result};
use image::{DynamicImage, ImageOutputFormat};
use std::{
    io::{BufRead, Seek, Write},
    ops::Deref,
    str::FromStr,
};

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

pub struct Img {
    dynamic: DynamicImage,
    target_format: ImageFormat,
}

impl Img {
    pub fn new<R: BufRead + Seek>(source: R, target_format: ImageFormat) -> Result<Self> {
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

        let _ = Img::new(reader, ImageFormat::from_str("png")?)?;

        Ok(())
    }
}
