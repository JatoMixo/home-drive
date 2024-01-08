pub const STORAGEMENT_DIR_NAME: &str = "drive-storagement";
use std::fs;
use serde_json::json;

#[derive(Debug, serde::Serialize)]
pub struct File {
    path: String,
}

impl File {
    pub fn new(path: &str) -> File {
        // Windows paths are different than Linux ones
        let path = path.replace("\\", "/");

        File {
            path: path,
        }
    }

    fn get_full_name(&self) -> String {
        self
            .path
            .split("/")
            .last()
            .unwrap()
            .to_string()
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Directory {
    path: String,
}

impl Directory {
    pub fn new(path: &str) -> Directory {
        let path = path.replace("\\", "/");
        let storagement_path = format!("{}/{}", STORAGEMENT_DIR_NAME, path);
        if fs::read_dir(&path).is_err() {
            fs::create_dir_all(storagement_path).expect("Error creating Directory");
        }

        Directory {
            path: path.to_string(),
        }
    }

    pub fn get_absolute_path(&self) -> String {
        format!("{}/{}", STORAGEMENT_DIR_NAME, self.path)
    }

    pub fn get_name(&self) -> String {
        let path_split = split_path(&self.path);
        let last_element_of_path = path_split.last().expect("Error parsing Directory Name");

        last_element_of_path.clone()
    }

    pub fn get_subdirectories(&self) -> Vec<Directory> {
        let directory_elements = fs::read_dir(&self.get_absolute_path()).expect("Path didn't exist");

        directory_elements
        .filter(|element| is_element_directory(element.as_ref().unwrap()))
        .collect::<Vec<Result<fs::DirEntry, std::io::Error>>>()
        .iter()
        .map(|sub_directory| {
            Directory::new(
                &get_element_path(sub_directory.as_ref().unwrap())
            )
        })
        .collect::<Vec<Directory>>()
    }

    pub fn get_files(&self) -> Vec<File> {
        let directory_elements = fs::read_dir(&self.get_absolute_path()).expect("Path didn't exist");

        directory_elements
        .filter(|element| !is_element_directory(element.as_ref().unwrap()))
        .collect::<Vec<Result<fs::DirEntry, std::io::Error>>>()
        .iter()
        .map(|sub_directory| {
            File::new(
                &get_element_path(sub_directory.as_ref().unwrap())
            )
        })
        .collect::<Vec<File>>()
    }

    pub fn build_files_and_directories_json(&self) -> serde_json::Value {
        let directories = self.get_subdirectories();
        let files = self.get_files();

        let directories_names = directories.iter().map(|directory| {
            directory.get_name()
        }).collect::<Vec<String>>();
        let files_names = files.iter().map(|file| {
            file.get_full_name()
        }).collect::<Vec<String>>();

        json!({
            "directories": directories_names,
            "files": files_names,
        })
    }
}

fn split_path(path: &str) -> Vec<String> {
    path
        .split("/")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&raw_string| {
            raw_string.to_string()
        })
        .collect::<Vec<String>>()
}

fn is_element_directory(element: &fs::DirEntry) -> bool {
    element.metadata().unwrap().is_dir()
}

fn get_element_path(element: &fs::DirEntry) -> String {
    let full_path = element.path().to_str().unwrap().to_string();
    let drive_storagement_prefix = &format!("{}/", STORAGEMENT_DIR_NAME);

    full_path.strip_prefix(drive_storagement_prefix).unwrap_or(&full_path).to_string()
}
