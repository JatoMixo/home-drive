pub const STORAGEMENT_DIR_NAME: &str = "drive-storagement";
use std::fs;

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

    pub fn get_name(&self) -> Option<String> {
        let path_split = split_path(&self.path);
        let last_element_of_path = path_split.last()?;

        Some(last_element_of_path.clone())
    }

    /*pub fn get_subdirectories(&self) -> Vec<Directory> {
        let directory_elements = read_dir(self.path);
    }*/
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
