#[derive(serde::Deserialize)]
struct FileElement {
    pub name: String,
    pub is_directory: bool,
}

#[tauri::command]
pub async fn get_elements_in_path(ip: String, path: String) -> Vec<FileElement> {
    const SERVER_PORT: u16 = 8080;
    let elements = reqwest::get(&format!("http://{}:{}/{}", ip, SERVER_PORT, path)).await.unwrap().json::<Vec<FileElement>>().await.unwrap();

    elements
}