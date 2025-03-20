use std::path::PathBuf;
use axum::extract::Path;
use axum::response::IntoResponse;
use tokio::fs::read;
use crate::utils::DATA_STORAGE_PATH;

pub async fn get_image(Path(filename): Path<String>) -> Result<impl IntoResponse, String> {
    let file_path = PathBuf::from(format!("{}{}", DATA_STORAGE_PATH.as_str(), filename));

    match read(&file_path).await {
        Ok(contents) => {
            let mime_type = mime_guess::from_path(&file_path)
                .first_or_octet_stream()
                .to_string();
            Ok(([(axum::http::header::CONTENT_TYPE, mime_type)], contents))
        }
        Err(_) => Err(format!("Failed to read {}", file_path.display())),
    }
}