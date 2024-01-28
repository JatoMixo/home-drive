use crate::server::Server;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DirectoryContent {
    directories: Vec<String>,
    files: Vec<String>,
}

impl DirectoryContent {
    pub fn new(directories: Vec<String>, files: Vec<String>) -> DirectoryContent {
        DirectoryContent {
            directories,
            files,
        }
    }
}

#[tauri::command]
pub fn get_elements_in_path(path: &str, server: Server) -> DirectoryContent {
    let request = reqwest::blocking::get(&server.get_explorer_route_for_path(path));

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
