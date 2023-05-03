pub mod io;
use crate::{ImageFormat, RawImage, Result};
use image::ImageOutputFormat;
use image::{ImageBuffer, RgbImage};
use std::io::{Seek, Write};

fn get_image_dimensions(img: &RawImage) -> (u32, u32) {
    (img.image_dict.width, img.image_dict.height)
}

pub trait ImageWriter<R: Write + Seek> {
    fn write_to(&mut self, w: R) -> Result<()>;
}

struct PngWriter<'a> {
    image:&'a RawImage,
}

impl<'a> PngWriter<'a> {
    pub fn new(image: &'a RawImage) -> Self {
        PngWriter { image }
    }
}

impl<R: Write + Seek> ImageWriter<R> for PngWriter<'_> {
    fn write_to(&mut self, mut w: R) -> Result<()> {
        let mut img: RgbImage =
            ImageBuffer::new(self.image.image_dict.width, self.image.image_dict.height);
        img.copy_from_slice(&self.image);
        img.write_to(&mut w, ImageOutputFormat::Png)?;
        Ok(())
    }
}

struct JpegWriter<'a> {
    image: &'a RawImage,
    quality: u8,
}

impl<'a> JpegWriter<'a> {
    pub fn new(image:&'a RawImage, quality: u8) -> Self {
        JpegWriter { image, quality }
    }
}

impl<R: Write + Seek> ImageWriter<R> for JpegWriter<'_> {
    fn write_to(&mut self, mut w: R) -> Result<()> {
        let (width, height) = get_image_dimensions(&self.image);
        let mut img: RgbImage = ImageBuffer::new(width, height);
        log::info!(
            "image dimensions W : {width} H : {height} Total pixels : {} Raw Image Size {}",
            width * height,
            img.len()
        );
        img.copy_from_slice(&self.image);
        img.write_to(&mut w, ImageOutputFormat::Jpeg(self.quality))?;
        Ok(())
    }
}

pub fn create_img_writer<'a,R: Write + Seek>(
    image: &'a RawImage,
    format: ImageFormat,
) -> Box<dyn ImageWriter<R> + 'a> {
    use ImageFormat::*;
    match format {
        Jpeg(qual) => Box::new(JpegWriter::new(image, qual)),
        Png => Box::new(PngWriter::new(image)),
        _ => todo!("Implement other writers"),
    }
}
