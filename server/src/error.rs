pub struct PathNotFound {
    path: String,
}

impl PathNotFound {
    pub fn new(path: String) -> PathNotFound {
        PathNotFound {
            path: path,
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}