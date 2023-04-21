use std::path::PathBuf;

use pdf::file::FileOptions;
use clap::Parser;
use html::metadata::Title;

#[derive(Parser)]
struct Args {
    pdf_file : PathBuf
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args = Args::parse();

    let mut html_title = Title::builder();

    let file = FileOptions::cached().open(args.pdf_file)?;

    if let Some(ref info) = file.trailer.info_dict {
        let title = info.get("Title").and_then(|p| p.to_string_lossy().ok());

        html_title.title(title.unwrap());

    }

    let res_string = html_title.build().to_string(); 

    std::fs::write("res.html", res_string)?;

    Ok(())
}
