use crate::directory_reader::list_directory;

pub async fn handle_explorer(req: tide::Request<()>) -> tide::Result<tide::Body> {
    let file_path = req.url().path();

    let file_list = list_directory(file_path);

    Ok(tide::Body::from_json(&file_list)?)
}
