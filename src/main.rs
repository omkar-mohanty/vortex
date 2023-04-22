use std::path::PathBuf;

use clap::Parser;
use html::metadata::Title;
use pdf::{
    file::FileOptions,
    object::Resolve,
};

#[derive(Parser)]
struct Args {
    pdf_file: PathBuf,
    output_folder: Option<PathBuf>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args = Args::parse();

    let mut res_file_name = "res.html".into();

    if let Some(out_dir) = args.output_folder {
        res_file_name = out_dir.join("res.html");
        std::fs::create_dir(out_dir)?;
    }

    let mut html_title = Title::builder();

    let file = FileOptions::cached().open(args.pdf_file)?;

    if let Some(ref info) = file.trailer.info_dict {
        let title = info.get("Title").and_then(|p| p.to_string_lossy().ok());

        html_title.title(title.unwrap());
    }

    let res_string = html_title.build().to_string();

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

    std::fs::write(res_file_name, res_string)?;

    Ok(())
}
