use askama::Template;
use ntex::{http, web};
use ntex_multipart::Multipart;
use std::{fs, io::Write};
use futures_util::StreamExt as _;

#[derive(Template)]
#[template(path = "upload.html")]
pub struct UploadTemplate {}

#[web::get("/upload")]
pub async fn upload() -> web::HttpResponse {
    let template : UploadTemplate = UploadTemplate {};

    let result : String = match template.render() {
        Ok(value) => value,
        Err(_) => String::from("<h1>500 Internal Server Error</h1>")
    };

    return web::HttpResponse::Ok().body(result);
}

#[web::post("/upload")]
pub async fn receiver(request: web::HttpRequest, web_payload : web::types::Payload) -> web::HttpResponse {
    let mut payload = Multipart::new(request.headers(), web_payload);

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();

        let content_disposition = field.content_disposition();

        let raw_filename = content_disposition.unwrap().get_filename();

        let proc_filename : String = format!("static/{}", &raw_filename.unwrap());

        if let Some(_) = raw_filename {
            let mut bytes : Vec<u8> = Vec::new();

            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                bytes.extend_from_slice(&data);
            }

            let mut file : fs::File = fs::File::create_new(proc_filename).unwrap();

            file.write(&bytes).unwrap();
        } else {
            let mut text = Vec::new();

            while let Some(chunk) = field.next().await {
                text.extend_from_slice(&chunk.unwrap());
            }
        }
    }

    return web::HttpResponse::SeeOther().header(http::header::LOCATION, "/static").finish();
}
