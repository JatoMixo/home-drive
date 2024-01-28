use serde_json::json;
use crate::server::Server;

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

impl FileUpload {
    pub fn build_json(&self) -> serde_json::Value {
        json!({
            "name": self.name,
            "path": self.path_to_upload,
            "content": self.content,
        })
    }
}

#[tauri::command]
pub fn upload_file(file_upload: FileUpload, server: Server) -> Result<(), UploadError> {
    let upload_client = reqwest::blocking::Client::new();

    let server_response = upload_client.post(server.get_upload_route())
        .json(&file_upload.build_json())
        .send();
    
    match server_response {
        Ok(_) => Ok(()),
        Err(_) => Err(UploadError::ServerNotFound),
    }
}
