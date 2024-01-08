use crate::file_manager::{Directory, File};

pub async fn handle_delete_route(req: tide::Request<()>) -> tide::Result<String> {
    let mut path = req.url().path().to_string();
    if path.starts_with("/delete/") {
        path = path.strip_prefix("/delete/").unwrap_or(&path).to_string();
    }

    // Check if it's file. Dumb but functional way
    if path.contains(".") {
        let _ = File::new(&path).get_deleted();
        return Ok(String::new());
    }

    let _ = Directory::new(&path).get_deleted();

    Ok(String::new())
}