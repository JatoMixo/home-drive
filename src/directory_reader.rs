pub const STORAGEMENT_DIR_NAME: &str = "drive-storagement";
use std::fs::read_dir;

pub fn list_directory(file_path: &str) -> Vec<String> {

    read_dir(format!("drive-storagement/{}", file_path)).unwrap().map(|file| {

        String::from(file.unwrap()
            .path()
            .file_name().unwrap()
            .to_str().unwrap())

    }).collect::<Vec<String>>()
}