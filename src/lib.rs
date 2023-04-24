mod img;

pub use img::{TargetFormat, Img, ImgWriter, FileWriter};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
