use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use ntex::web;
use ntex_files as nfs;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    println!("Starting process at localhost:4000.");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let server = web::HttpServer::new( move || {
        web::App::new().service(
            nfs::Files::new("/", "./static/")
                .show_files_listing()
                .use_last_modified(true),
        )
    });

    server.bind_openssl(("0.0.0.0", 4000), builder)?.run().await
}
