use crate::file_manager::list_elements_in_directory;

pub async fn handle_explorer(req: tide::Request<()>) -> tide::Result<tide::Body> {
    let file_path = req.url().path();

    let file_list = list_elements_in_directory(file_path);

    Ok(tide::Body::from_json(&file_list)?)
}
