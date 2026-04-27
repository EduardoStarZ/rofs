use askama::Template;
use ntex::web;

#[derive(Template)]
#[template(path = "upload.html")]
pub struct UploadTemplate {}

#[web::get("/upload")]
pub async fn upload(request : web::HttpRequest) -> web::HttpResponse {
    let template : UploadTemplate = UploadTemplate {};

    let result : String = match template.render() {
        Ok(value) => value,
        Err(_) => String::from("<h1>502 Internal Server Error</h1>")
    };

    return web::HttpResponse::Ok().body(result);
}
