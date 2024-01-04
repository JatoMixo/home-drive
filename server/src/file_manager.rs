pub const STORAGEMENT_DIR_NAME: &str = "drive-storagement";
use std::fs::read_dir;

pub fn list_elements_in_directory(file_path: &str) -> Vec<String> {
    read_dir(format!("{}/{}", STORAGEMENT_DIR_NAME, file_path)).unwrap().map(|file| {
        String::from(file.unwrap()
            .path()
            .file_name().unwrap()
            .to_str().unwrap())
    }).collect::<Vec<String>>()
}

pub fn read_txt_file(file_path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(format!("{}/{}", STORAGEMENT_DIR_NAME, file_path))
}
