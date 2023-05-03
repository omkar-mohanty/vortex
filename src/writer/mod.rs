pub mod io;
use crate::{ImageFormat, RawImage, Result};
use image::{ImageBuffer, RgbImage};
use std::io::{Seek, Write};

fn get_image_dimensions(img: &RawImage) -> (u32, u32) {
    (img.image_dict.width, img.image_dict.height)
}

pub trait OutputWriter<R: Write + Seek> {
    fn write_to(&mut self, w: R) -> Result<()>;
}

struct ImageWriter<'a> {
    image: &'a RawImage,
    img_format: ImageFormat,
}

impl<'a> ImageWriter<'a> {
    pub fn new(image: &'a RawImage, img_format: ImageFormat) -> Self {
        ImageWriter { image, img_format }
    }
}

impl<R: Write + Seek> OutputWriter<R> for ImageWriter<'_> {
    fn write_to(&mut self, mut w: R) -> Result<()> {
        let (width, height) = get_image_dimensions(&self.image);

        let mut img: RgbImage =
            ImageBuffer::new(self.image.image_dict.width, self.image.image_dict.height);
        log::info!(
            "image dimensions W : {width} H : {height} Total pixels : {} Raw Image Size {}",
            width * height,
            img.len()
        );
        img.copy_from_slice(&self.image);
        img.write_to(&mut w, self.img_format.clone())?;
        Ok(())
    }
}

pub fn create_output_writer<'a, R: Write + Seek>(
    image: &'a RawImage,
    format: ImageFormat,
) -> Box<dyn OutputWriter<R> + 'a> {
    Box::new(ImageWriter::new(image, format))
}
