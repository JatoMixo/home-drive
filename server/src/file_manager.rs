pub const STORAGEMENT_DIR_NAME: &str = "drive-storagement";
use std::fs;
use std::io::Write;
use serde_json::json;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct File {
    path: String,
}

impl File {
    pub fn create(path: &str, content: Vec<u8>) -> File {
        let absolute_path = &convert_local_path_to_absolute(path);

        let mut file = fs::File::create(absolute_path).unwrap();
        file.write_all(content.as_slice()).unwrap();

        File::open(path)
    }

    pub fn open(path: &str) -> File {
        // Windows paths are different than Linux ones
        let path = path.replace("\\", "/");

        File {
            path,
        }
    }

    fn get_name(&self) -> String {
        self
            .path
            .split("/")
            .last()
            .unwrap()
            .to_string()
    }

    pub fn get_absolute_path(&self) -> String {
        convert_local_path_to_absolute(&self.path)
    }

    pub fn get_deleted(self) -> Result<(), std::io::Error> {
        fs::remove_file(self.get_absolute_path())
    }

    pub fn get_content(&self) -> Vec<u8> {
        std::fs::read(self.get_absolute_path()).unwrap()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Directory {
    path: String,
}

impl Directory {
    pub fn new(path: &str) -> Directory {
        let path = path.replace("\\", "/");
        let storagement_path = convert_local_path_to_absolute(&path);

        if fs::read_dir(&path).is_err() {
            fs::create_dir_all(storagement_path).expect("Error creating Directory");
        }

        Directory {
            path: path.to_string(),
        }
    }

    pub fn get_absolute_path(&self) -> String {
        convert_local_path_to_absolute(&self.path)
    }

    pub fn get_name(&self) -> String {
        let path_split = split_path(&self.path);
        let last_element_of_path = path_split.last().expect("Error parsing Directory Name");

        last_element_of_path.clone()
    }

    fn get_directory_elements_filtered_by(&self, condition: fn(&fs::DirEntry) -> bool) -> Vec<Result<fs::DirEntry, std::io::Error>> {
        let directory_elements = fs::read_dir(&self.get_absolute_path()).expect("Path didn't exist");

        directory_elements
            .filter(|element| condition(element.as_ref().unwrap()))
            .collect::<Vec<Result<fs::DirEntry, std::io::Error>>>()
    }

    pub fn get_subdirectories(&self) -> Vec<Directory> {
        self.get_directory_elements_filtered_by(|element| is_element_directory(element))
            .iter()
            .map(|sub_directory| {
                Directory::new(
                    &get_element_path(sub_directory.as_ref().unwrap())
                )
            })
            .collect::<Vec<Directory>>()
    }

    pub fn get_files(&self) -> Vec<File> {
        self.get_directory_elements_filtered_by(|element| !is_element_directory(element))
            .iter()
            .map(|file| {
                File::open(
                    &get_element_path(file.as_ref().unwrap())
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
            file.get_name()
        }).collect::<Vec<String>>();

        json!({
            "directories": directories_names,
            "files": files_names,
        })
    }

    pub fn get_deleted(self) -> Result<(), std::io::Error> {
        fs::remove_dir_all(self.get_absolute_path())
    }

    pub fn get_path(&self) -> String {
        self.path.to_string()
    }
}

impl Default for Directory {
    fn default() -> Directory {
        Directory::new("")
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
    let absolute_path = element.path().to_str().unwrap().to_string();
    let drive_storagement_prefix = &format!("{}/", STORAGEMENT_DIR_NAME);

    absolute_path.strip_prefix(drive_storagement_prefix).unwrap_or(&absolute_path).to_string()
}

fn convert_local_path_to_absolute(local_path: &str) -> String {
    format!("{}/{}", STORAGEMENT_DIR_NAME, local_path)
}
