use crate::file_manager::read_txt_file;

pub async fn handle_visualizer_route(req: tide::Request<()>) -> tide::Result<String> {
    let mut path = req.url().path().to_string();
    path = path.strip_prefix("/content/").unwrap().to_string();

    Ok(read_txt_file(&path).unwrap())
}