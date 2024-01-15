#[derive(serde::Deserialize, Debug)]
pub struct DirectoryContent {
    directories: Vec<String>,
    files: Vec<String>,
}

#[derive(Debug)]
pub enum RequestError {
    PathNotFound,
} 

#[tauri::command]
pub async fn get_elements_in_path(ip: &str, path: &str) -> Result<DirectoryContent, RequestError> {
    const SERVER_PORT: u16 = 8080;
    
    let request = reqwest::get(&format!("http://{}:{}/explorer/{}", ip, SERVER_PORT, path)).await;
    match request {
        Ok(request_content) => {
            let directory_content: DirectoryContent = request_content.json::<DirectoryContent>().await.unwrap();

            Ok(directory_content)
        },
        Err(_) => {
            Err(RequestError::PathNotFound)
        }
    }
}