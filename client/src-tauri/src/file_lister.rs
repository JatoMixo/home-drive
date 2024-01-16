#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DirectoryContent {
    directories: Vec<String>,
    files: Vec<String>,
}

impl DirectoryContent {
    pub fn new(directories: Vec<String>, files: Vec<String>) -> DirectoryContent {
        DirectoryContent {
            directories: directories,
            files: files,
        }
    }
}

#[tauri::command]
pub fn get_elements_in_path(ip: &str, path: &str) -> DirectoryContent {
    const SERVER_PORT: u16 = 8080;
    
    let request = reqwest::blocking::get(&format!("http://{}:{}/explorer/{}", ip, SERVER_PORT, path));
    match request {
        Ok(request_content) => {
            let directory_content: DirectoryContent = request_content.json::<DirectoryContent>().unwrap();

            directory_content
        },
        Err(_) => {
            DirectoryContent::new(Vec::new(), Vec::new())
        }
    }
}
