/*use crate::file_manager::file_elements_in_path;

pub async fn handle_explorer(req: tide::Request<()>) -> tide::Result<tide::Body> {
    let file_path = req.url().path().to_string();

    let file_list = file_elements_in_path(file_path);

    Ok(tide::Body::from_json(&file_list)?)
}*/
