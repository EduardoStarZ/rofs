use ntex::web;
use ntex_files as nfs;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    println!("Starting process at localhost:8000.");

    let server = web::HttpServer::new( move || {
        web::App::new().service(
            nfs::Files::new("/", "./static/")
                .show_files_listing()
                .use_last_modified(true),
        )
    });

    server.bind(("0.0.0.0", 8000))?.run().await
}
