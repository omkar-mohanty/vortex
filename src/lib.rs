mod img;

pub use img::TargetFormat;

pub type Result<T>  = std::result::Result<T, Box<dyn std::error::Error>>;
 
