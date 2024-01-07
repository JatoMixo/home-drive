pub const STORAGEMENT_DIR_NAME: &str = "drive-storagement";
use std::fs;

#[derive(Debug)]
pub struct Directory {
    path: String,
}

impl Directory {
    pub fn new(path: &str) -> Directory {
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
            Directory {
                path: get_element_name(sub_directory.as_ref().unwrap()),
            }
        })
        .collect::<Vec<Directory>>()
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

fn get_element_name(element: &fs::DirEntry) -> String {
    let full_path = element.path().to_str().unwrap().to_string();
    let drive_storagement_prefix = &format!("{}/", STORAGEMENT_DIR_NAME);

    full_path.strip_prefix(drive_storagement_prefix).unwrap_or(&full_path).to_string()
}
