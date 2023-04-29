use clap::Parser;
use log::LevelFilter;
use pdf::{
    file::FileOptions,
    object::{Resolve, XObject},
};
use std::{
    fs::File,
    io::BufWriter,
    ops::Deref,
    path::{Path, PathBuf},
    str::FromStr,
};
use vortex::{writer::create_writer, ImageFormat, RawImage, Result};

/// vortex is a tool to extract images from pdf files
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
    /// Optional  output image format i.e jpeg, png etc,
    #[arg(short, long)]
    target_format: Option<String>,
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

        let data = img.image_data(&file)?;

        let img_dict = img.deref().to_owned();

        let img = RawImage::new(&data, img_dict);

        let target_format = match args.target_format {
            Some(ref format) => ImageFormat::from_str(format)?,
            None => ImageFormat::default(),
        };

        let writer = get_io_writer(&out_dir, &target_format, i);

        let mut img_writer = create_writer(img, target_format);

        img_writer.write_to(writer)?;
    }

    Ok(())
}

fn get_io_writer(dir: &Path, target_format: &ImageFormat, index: usize) -> BufWriter<File> {
    let filename = format!("extracted_image_{}.{}", index, target_format);
    let filename = PathBuf::from_str(&filename).unwrap();
    let joined_path = dir.join(filename);
    let file = File::create(joined_path).unwrap();
    BufWriter::new(file)
}
