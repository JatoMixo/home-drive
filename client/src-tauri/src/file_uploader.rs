use serde_json::json;

#[derive(serde::Deserialize, serde::Serialize)]
pub enum UploadError {
    ServerNotFound,
}

#[derive(serde::Deserialize)]
pub struct FileUpload {
    name: String,
    path_to_upload: String,
    content: Vec<u8>,
}

#[derive(serde::Deserialize)]
pub struct Server {
    ip: String,
    port: u16,
}

#[tauri::command]
pub fn upload_file(file_upload: FileUpload, server: Server) -> Result<(), UploadError> {
    let upload_client = reqwest::blocking::Client::new();
    let server_response = upload_client.post(&format!("http://{}:{}/upload", server.ip, server.port))
        .json(&json!({
            "name": file_upload.name,
            "path": file_upload.path_to_upload,
            "content": file_upload.content,
        }))
        .send();
    
    match server_response {
        Ok(_) => Ok(()),
        Err(_) => Err(UploadError::ServerNotFound),
    }
}