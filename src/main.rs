use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use ntex::web;
use ntex_files as nfs;
use rofs::middleware::router;
use rofs::pages;

static IP : &str = "0.0.0.0";
static HTTPS_PORT : u16 = 4000;
static FULL_HTTPS_ADDR : (&str, u16) = (IP, HTTPS_PORT);

#[ntex::main]
async fn main() -> std::io::Result<()> {
    println!("Starting process at {IP}:{HTTPS_PORT} for HTTPS.");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder.set_private_key_file("certs/key.pem", SslFiletype::PEM).unwrap();

    builder.set_certificate_chain_file("certs/cert.pem").unwrap();

    let server = web::HttpServer::new( move || {
        web::App::new()
            .wrap(router::Https)
            .service(
            nfs::Files::new("/static", "./static/")
                .show_files_listing()
                .use_last_modified(true),
            )
            .service(pages::upload)
    });

    return server
        .bind_openssl(FULL_HTTPS_ADDR, builder)?
        .run()
        .await;
}
