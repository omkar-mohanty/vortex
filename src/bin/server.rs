use std::io::Cursor;

use actix_web::{post, web::Bytes, App, HttpResponse, HttpServer, Responder};

use unpdf::{Img, ImageFormat};

#[post("/extract")]
async fn extract(req: Bytes) -> impl Responder {
    let reader = Cursor::new(&req[..]);
    let target_format = ImageFormat::default();
    let image = Img::new(reader, target_format).unwrap();
    let mut buf = Vec::new();
    image.write_to(&mut Cursor::new(&mut buf)).unwrap();
    HttpResponse::Ok().body(buf)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(extract))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
