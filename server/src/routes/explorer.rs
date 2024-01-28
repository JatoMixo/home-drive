use crate::file_manager::Directory;

pub async fn handle_explorer(req: tide::Request<()>) -> tide::Result<tide::Body> {

    const EXPLORER_ROUTE_NAME: &str = "/explorer/";
    let file_path = req
        .url()
        .path()
        .strip_prefix(EXPLORER_ROUTE_NAME)
        .unwrap();

    let directory_content = Directory::new(file_path).build_files_and_directories_json();

    Ok(tide::Body::from_json(&directory_content)?)
}
