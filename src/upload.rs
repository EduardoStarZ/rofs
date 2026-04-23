use ntex::web::{self, HttpResponse};
use ntex_multipart::Multipart;
use futures_util::StreamExt;
use std::io::Write;

pub async fn handler_upload(
    mut payload: Multipart,  // HttpRequest removido — não era usado
) -> Result<HttpResponse, web::Error> {

    let mut file_received = false;

    // Itera sobre cada parte do formulário multipart
    while let Some(item) = payload.next().await {
        // map_err converte o erro do multipart para web::Error
        let mut field = item.map_err(|e| {
            web::error::ErrorBadRequest(format!("Erro no multipart: {}", e))
        })?;

        // Pega o Content-Disposition para saber o nome do campo
        let content_disposition = field
            .headers()
            .get("content-disposition")  // minúsculo — headers HTTP são case-insensitive mas ntex normaliza
            .and_then(|cd| cd.to_str().ok())
            .unwrap_or("")
            .to_string();

        // Só processa o campo chamado "arquivo" (ou "file")
        if content_disposition.contains("name=\"arquivo\"")
            || content_disposition.contains("name=\"file\"")
        {
            file_received = true;
            let mut content = Vec::new();

            // Lê os chunks (pedaços) do arquivo
            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|e| {
                    web::error::ErrorInternalServerError(
                        format!("Erro ao ler chunk: {}", e)
                    )
                })?;
                content.extend_from_slice(&data);
            }

            // Cria o diretório de uploads se não existir
            std::fs::create_dir_all("uploads")
                .map_err(|e| web::error::ErrorInternalServerError(e))?;

            // Nome único baseado em timestamp
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let file_path = format!("uploads/file_{}.bin", timestamp);

            // Salva o arquivo
            let mut file = std::fs::File::create(&file_path)
                .map_err(|e| web::error::ErrorInternalServerError(e))?;

            file.write_all(&content)
                .map_err(|e| web::error::ErrorInternalServerError(e))?;

            println!("Arquivo salvo em: {}", file_path);
        }
    }

    if file_received {
        Ok(HttpResponse::Ok().body("Upload realizado com sucesso!"))
    } else {
        Ok(HttpResponse::BadRequest().body("Nenhum arquivo encontrado na requisição"))
    }
}