use crate::RawImage;
use crate::Result;

use pdf::any::AnySync;
use pdf::backend::Backend;
use pdf::file::Cache;
use pdf::file::File;
use pdf::file::FileOptions;
use pdf::object::PageRc;
use pdf::object::{Resolve, XObject};
use pdf::PdfError;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

pub enum Method<'a> {
    File(PathBuf),
    Bytes(&'a [u8]),
}

pub fn get_pages<T, K, Y>(file: &File<T, K, Y>) -> Result<Vec<PageRc>>
where
    T: Backend,
    K: Cache<std::result::Result<AnySync, Arc<PdfError>>>,
    Y: Cache<std::result::Result<Arc<[u8]>, Arc<PdfError>>>,
{
    Ok(file
        .pages()
        .map(|page| page.unwrap())
        .collect::<Vec<PageRc>>())
}

pub fn get_raw_images<T, K, Y>(page: PageRc, file: &File<T, K, Y>) -> Result<Vec<RawImage>>
where
    T: Backend,
    K: Cache<std::result::Result<AnySync, Arc<PdfError>>>,
    Y: Cache<std::result::Result<Arc<[u8]>, Arc<PdfError>>>,
{
    let mut images = vec![];

    let resources = page.resources()?;

    images.extend(
        resources
            .xobjects
            .iter()
            .map(|(_name, &r)| file.get(r).unwrap())
            .filter(|o| matches!(**o, pdf::object::XObject::Image(_))),
    );

    log::debug!("main : total images {}", images.len());

    let mut raw_images = vec![];

    for o in images.iter() {
        let img = match **o {
            XObject::Image(ref im) => im,
            _ => continue,
        };

        let data = img.image_data(file)?;

        let img_dict = img.deref().to_owned();

        let img = RawImage::new(&data, img_dict);

        raw_images.push(img)
    }
    Ok(raw_images)
}

pub fn extract_images(method: Method) -> Result<Vec<RawImage>> {
    let (pages, file) = match method {
        Method::File(path) => {
            let file = FileOptions::cached().open(path)?;

            (get_pages(&file)?, file)
        }
        Method::Bytes(_bytes) => {
            todo!("Parse files in network")
        }
    };

    let mut images: Vec<RawImage> = vec![];

    for page in pages {
        images.extend(get_raw_images(page, &file)?);
    }

    Ok(images)
}
