use clap::{Parser, Subcommand};
use log::LevelFilter;
use pdf::{
    enc::StreamFilter,
    file::FileOptions,
    object::{Resolve, XObject},
};
use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr, io::Cursor,
};
use unpdf::{Result, TargetFormat, Img};

/// unpdf is a tool to extract images from pdf files
#[derive(Parser)]
struct Args {
    /// Pdf file to extract images from
    pdf_file: PathBuf,
    /// Folder to store extracted images
    #[arg(short, long, value_name = "OUTPUT FOLDER")]
    output_folder: Option<PathBuf>,
    /// Logging level
    log_level: Option<LevelFilter>,
    /// Log file
    log_file: Option<PathBuf>,
    /// Optional target format,
    target_format: Option<TargetFormat>,
}

#[derive(Subcommand)]
enum Command {
    Log {
        log_level: Option<LevelFilter>,
        log_file: Option<PathBuf>,
    },
}

fn init_log(args: &Args) -> env_logger::Builder {
    let mut builder = env_logger::builder();

    if !cfg!(debug_assertions) {
        let log_file =
            std::fs::File::create(args.log_file.clone().unwrap_or("log.txt".into())).unwrap();
        builder.target(env_logger::Target::Pipe(Box::new(log_file)));
    }

    builder.filter_level(LevelFilter::Debug);

    builder
}

fn main() -> Result<()> {
    let args = Args::parse();

    init_log(&args).init();

    let out_dir: PathBuf = args.output_folder.unwrap_or(PathBuf::from_str("output")?);

    if !Path::new(&out_dir).exists() {
        std::fs::create_dir(out_dir.clone())?;
    }

    let file = FileOptions::cached().open(args.pdf_file)?;

    let mut images: Vec<_> = vec![];

    for page in file.pages() {
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

    log::debug!("main : total images {}", images.len());

    for (i, o) in images.iter().enumerate() {
        let img = match **o {
            XObject::Image(ref im) => im,
            _ => continue,
        };

        let (data, filter) = img.raw_image_data(&file)?;

        use StreamFilter::*;

        let ext = match filter {
            Some(DCTDecode(_)) => "jpeg",
            Some(JBIG2Decode) => "jbig2",
            Some(JPXDecode) => "jp2k",
            _ => {
                log::debug!("main : unsupported image format");
                continue;
            }
        };

        let source = Cursor::new(data);

        let target_format = args.target_format.unwrap_or(TargetFormat::from_str("jpeg"));
        
        let img = Img::new(source, target_format);

        log::debug!("Detected format : {ext}");

        let fname = format!("extracted_image_{}.{}", i, ext);

        let dir_file = out_dir.join(fname.clone());

        fs::write(dir_file, data)?;

        log::debug!("main :  wrote output file");
    }

    Ok(())
}
