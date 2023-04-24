use crate::{Result, ImgError, err};
use image::{DynamicImage, ImageOutputFormat};
use std::{
    io::{BufRead, Seek, Write, SeekFrom},
    str::FromStr, net::TcpStream, fs::File, ops::Deref,
};

#[derive(Clone)]
pub struct TargetFormat {
    format: ImageOutputFormat,
}

const DEFAULT_JPEG_QUALITY: u8 = 10;

impl FromStr for TargetFormat {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use ImageOutputFormat::*;

        let format = match s {
            "jpeg" => Jpeg(DEFAULT_JPEG_QUALITY),
            "png" => Png,
            _ => return err!("Invalid format"),
        };

        Ok(Self { format })
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

    pub fn write_to<W: Write + Seek>(self, writer:&mut W) -> Result<()> {
        self.dynamic.write_to(writer, self.target_format.format)?;
        Ok(())
    }
}

impl Deref for Img {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.dynamic.as_bytes()
    }
}

pub trait ImgWriter: Write + Seek {
    
}

pub struct FileWriter {
    file: File,
    image: Img
}

impl Write for FileWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}

impl Seek for FileWriter {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        todo!("Implement seek for file writer")
    }
}

impl ImgWriter for FileWriter {
}

pub struct TcpWriter {
    stream : TcpStream,
    image: Img
}

impl Write for TcpWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
    
}

impl Seek for TcpWriter {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        todo!("Implement seek from for bytes in raw image")
    }
    
}

impl ImgWriter for TcpWriter {
    
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::Result;
    use super::*;

    #[test]
    fn test_file() -> Result<()> {
        let file = std::fs::File::open("./resources/sample_png.png")?;

        let reader = BufReader::new(file);

        let _ =  Img::new(reader, TargetFormat::from_str("png")?)?;

        Ok(())
    }
}
