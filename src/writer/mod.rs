use image::ImageOutputFormat;

use crate::{ImageFormat, RawImage, Result};
use std::{
    io::{Cursor, Seek, Write},
    ops::Deref,
};

pub trait ImageWriter<R: Write + Seek> {
    fn write_to(&mut self, w: R) -> Result<()>;
}

struct PngWriter {
    image: RawImage,
}

impl PngWriter {
    pub fn new(image: RawImage) -> Self {
        PngWriter { image }
    }
}

impl<R: Write + Seek> ImageWriter<R> for PngWriter {
    fn write_to(&mut self, w: R) -> Result<()> {
        let data = self.image.deref();
        let mut encoder = png::Encoder::new(w, 2, 1);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(
            // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(data)?;
        Ok(())
    }
}

struct JpegWriter {
    image: RawImage,
    quality: u8,
}

impl JpegWriter {
    pub fn new(image: RawImage, quality: u8) -> Self {
        JpegWriter { image, quality }
    }
}

impl<R: Write + Seek> ImageWriter<R> for JpegWriter {
    fn write_to(&mut self, w: R) -> Result<()> {
        let data = self.image.deref();
        let img = image::io::Reader::new(Cursor::new(data))
            .with_guessed_format()?
            .decode()?;
        let mut w = w;
        img.write_to(&mut w, ImageOutputFormat::Jpeg(self.quality))?;
        Ok(())
    }
}

pub fn create_writer<R: Write + Seek>(
    image: RawImage,
    format: ImageFormat,
) -> Box<dyn ImageWriter<R>> {
    use ImageFormat::*;
    match format {
        Jpeg(qual) => Box::new(JpegWriter::new(image, qual)),
        Png => Box::new(PngWriter::new(image)),
        _ => todo!("Implement other writers")
    }
}
