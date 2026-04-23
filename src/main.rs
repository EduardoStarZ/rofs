use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use rustls::ServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pemfile::{certs, private_key};
use rustls::server::WebPkiClientVerifier;

use ntex::web;
use ntex_files as nfs;

mod upload;

fn configurar_mtls() -> ServerConfig {
    // --- Certificado e chave do SERVIDOR ---
    let cert_file = File::open("certs/server.crt")
        .expect("Arquivo certs/server.crt não encontrado");
    let key_file = File::open("certs/server.key")
        .expect("Arquivo certs/server.key não encontrado");

    let server_certs: Vec<CertificateDer> = certs(&mut BufReader::new(cert_file))
        .map(|c| c.unwrap())
        .collect();

    let server_key: PrivateKeyDer = private_key(&mut BufReader::new(key_file))
        .unwrap()
        .expect("Invalid server private key");

    // --- CA raiz para verificar certificados dos CLIENTES (mTLS) ---
    let ca_file = File::open("certs/ca.pem")
        .expect("Arquivo certs/ca.pem não encontrado");

    let ca_certs: Vec<CertificateDer> = certs(&mut BufReader::new(ca_file))
        .map(|c| c.unwrap())
        .collect();

    let mut root_store = rustls::RootCertStore::empty();
    for ca_cert in ca_certs {
        root_store.add(ca_cert).expect("Falha ao adicionar CA ao store");
    }

    // Verifica certificado do cliente — ativa o mTLS
    let client_verifier = WebPkiClientVerifier::builder(Arc::new(root_store))
        .build()
        .expect("Falha ao construir verificador de cliente");

    ServerConfig::builder()
        .with_client_cert_verifier(client_verifier)
        .with_single_cert(server_certs, server_key)
        .expect("Configuração TLS inválida")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor mTLS iniciando em https://0.0.0.0:4000");

    let tls_config = configurar_mtls();

    web::HttpServer::new(move || {
        web::App::new()
            .route("/upload", web::post().to(upload::handler_upload))
            .service(
                nfs::Files::new("/", "./static/")
                    .show_files_listing()
                    .use_last_modified(true),
            )
    })
    .bind_rustls("0.0.0.0:4000", tls_config)?
    .run()
    .await
}