pub const STORAGEMENT_DIR_NAME: &str = "drive-storagement";
use std::fs::read_dir;

#[derive(serde::Serialize)]
pub struct FileElement {
    pub name: String,
    pub is_directory: bool,
}

impl FileElement {
    pub fn new(name: String, is_directory: bool) -> FileElement {
        FileElement {
            name: name,
            is_directory: is_directory,
        }
    }
}

pub fn file_elements_in_path(path: String) -> Vec<FileElement> {
    string_vector_to_file_elements(list_elements_in_directory(path))
}

fn string_vector_to_file_elements(string_vector: Vec<String>) -> Vec<FileElement> {
    string_vector
        .iter()
        .map(|string| {
            let is_directory = !string.contains(".");

            FileElement::new(string.to_string(), is_directory)
        })
        .collect::<Vec<FileElement>>()
}

fn list_elements_in_directory(file_path: String) -> Vec<String> {
    read_dir(format!("{}/{}", STORAGEMENT_DIR_NAME, file_path)).unwrap().map(|file| {
        String::from(file.unwrap()
            .path()
            .file_name().unwrap()
            .to_str().unwrap())
    }).collect::<Vec<String>>()
}
