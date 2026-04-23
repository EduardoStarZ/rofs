// Teste 2
#[tokio::test]
async fn test_upload_with_valid_certificate() {
    let cert = std::fs::read("certs/client.crt").unwrap();
    let key  = std::fs::read("certs/client.key").unwrap();
    let ca   = std::fs::read("certs/ca.pem").unwrap();

    // from_pem recebe cert + key CONCATENADOS em um único &[u8]
    let mut pem = cert.clone();
    pem.extend_from_slice(&key);
    let identity = reqwest::Identity::from_pem(&pem).unwrap();

    let ca_cert = reqwest::Certificate::from_pem(&ca).unwrap();

    let client = reqwest::Client::builder()
        .add_root_certificate(ca_cert)
        .identity(identity)
        .build()
        .unwrap();

    let file = reqwest::multipart::Part::bytes(
        "Test upload with valid certificate".as_bytes().to_vec()
    )
    .file_name("test_upload.txt")
    .mime_str("text/plain")
    .unwrap();

    let form = reqwest::multipart::Form::new().part("file", file);

    let response: reqwest::Response = client
        .post("https://localhost:4000/upload")
        .multipart(form)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

// Teste 3
#[tokio::test]
async fn test_upload_without_file() {
    let cert = std::fs::read("certs/client.crt").unwrap();
    let key  = std::fs::read("certs/client.key").unwrap();
    let ca   = std::fs::read("certs/ca.pem").unwrap();

    // Mesma coisa — concatena cert + key
    let mut pem = cert.clone();
    pem.extend_from_slice(&key);
    let identity = reqwest::Identity::from_pem(&pem).unwrap();

    let ca_cert = reqwest::Certificate::from_pem(&ca).unwrap();

    let client = reqwest::Client::builder()
        .add_root_certificate(ca_cert)
        .identity(identity)
        .build()
        .unwrap();

    let response: reqwest::Response = client
        .post("https://localhost:4000/upload")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 400);
}