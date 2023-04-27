use std::{
    fs::File,
    io::{BufWriter, Seek, Write},
    net::TcpStream,
    path::{Path, PathBuf},
};

pub trait Writer: Write + Seek {}

pub trait WriterFactory {
    fn create_writer(self) -> Box<dyn Writer>;
}

pub struct FileWriterFactory<W: Write> {
    inner: W,
}

impl<W: Write + Seek> Writer for BufWriter<W> {}

impl<W: Write + Seek + 'static> WriterFactory for FileWriterFactory<W> {
    fn create_writer(self) -> Box<dyn Writer> {
        Box::new(BufWriter::new(self.inner))
    }
}

pub enum WriteMethod {
    File(PathBuf),
    Network(TcpStream),
}

pub fn create_writer(inner: WriteMethod) -> impl WriterFactory {
    match inner {
        WriteMethod::File(path) => {
            let inner = if Path::new(&path).exists() {
                File::open(path).unwrap()
            } else {
                File::create(path).unwrap()
            };

            FileWriterFactory { inner }
        }
        WriteMethod::Network(_) => todo!("Implement network writer"),
    }
}
