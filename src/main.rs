#![recursion_limit = "256"]

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use ntex::{http, web::{self, middleware}};
use ntex_files as nfs;
use rofs::pages;
use env_logger::Env;

static IP : &str = "0.0.0.0";
static HTTPS_PORT : u16 = 4000;
static FULL_HTTPS_ADDR : (&str, u16) = (IP, HTTPS_PORT);

#[ntex::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder.set_private_key_file("certs/key.pem", SslFiletype::PEM).unwrap();

    builder.set_certificate_chain_file("certs/cert.pem").unwrap();

    let server = web::HttpServer::new( async || {
        web::App::new()
            .middleware(middleware::Logger::default())
            .middleware(middleware::Logger::new("%a %{User-Agent}i"))
            .state(web::types::FormConfig::default().limit(100000000))
            .service(
            nfs::Files::new("/static", "./static/")
                .show_files_listing()
                .use_last_modified(true),
            )
            .service(index)
            .service(pages::upload)
            .service(pages::receiver)
    });

    return server
        .bind_openssl(FULL_HTTPS_ADDR, builder)?
        .run()
        .await;
}

#[web::get("/")]
async fn index() -> web::HttpResponse {
    return web::HttpResponse::PermanentRedirect().header(http::header::LOCATION, "/static").finish();
}
