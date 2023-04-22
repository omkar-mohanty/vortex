use std::{path::PathBuf, fs};

use clap::Parser;
use html::metadata::Title;
use pdf::{
    file::FileOptions,
    object::{Resolve, XObject},
    enc::StreamFilter,
};

#[derive(Parser)]
struct Args {
    pdf_file: PathBuf,
    output_folder: Option<PathBuf>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args = Args::parse();

    let out_dir:PathBuf = args.output_folder.unwrap_or(PathBuf::from(args.pdf_file.clone()));

    std::fs::create_dir(out_dir)?;

    let mut html_title = Title::builder();

    let file = FileOptions::cached().open(args.pdf_file)?;

    if let Some(ref info) = file.trailer.info_dict {
        let title = info.get("Title").and_then(|p| p.to_string_lossy().ok());

        html_title.title(title.unwrap());
    }

    let mut images: Vec<_> = vec![];

    for page in file.pages().take(10) {
        let page = page.unwrap();

        let resources = page.resources()?;

        images.extend(
            resources
                .xobjects
                .iter()
                .map(|(_name, &r)| file.get(r).unwrap())
                .filter(|o| matches!(**o, pdf::object::XObject::Image(_))),
        )
    }

    for (i, o) in images.iter().enumerate() {
        let img = match **o {
            XObject::Image(ref im) =>  im,
            _ => continue
        };

        let (data, filter)=  img.raw_image_data(&file)?;

        use StreamFilter::*;
        let ext = match filter {
            Some(DCTDecode(_)) => "jpeg",
            Some(JBIG2Decode) => "jbig2",
            Some(JPXDecode) => "jp2k",
            _ => continue
        };
        
        let fname = format!("extracted_image_{}.{}", i, ext);

        fs::write(fname.as_str(), data)?;

    }

    Ok(())
}
