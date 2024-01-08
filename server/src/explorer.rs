use crate::file_manager::Directory;

pub async fn handle_explorer(req: tide::Request<()>) -> tide::Result<tide::Body> {
    let mut file_path = req.url().path().to_string();
    if file_path.starts_with("/explorer/") {
        file_path = file_path.strip_prefix("/explorer/").unwrap_or(&file_path).to_string();
    }

    let directory_content = Directory::new(&file_path).build_files_and_directories_json();

    Ok(tide::Body::from_json(&directory_content)?)
}
