mod img;
pub mod writer;

use std::{error::Error, fmt::Display};

pub use img::{ImageFormat, RawImage};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[macro_export]
macro_rules! err {
    ($msg:expr) => {
        Err(Box::new(ImgError { msg: $msg }))
    };
}

#[derive(Debug)]
pub struct ImgError<'a> {
    msg: &'a str,
}

impl<'a> Display for ImgError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg)
    }
}

impl<'a> Error for ImgError<'a> {}
