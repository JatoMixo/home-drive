use crate::file_manager::{Directory, File};

pub async fn handle_delete_route(req: tide::Request<()>) -> tide::Result<String> {
    let path = req.query::<Directory>()?.get_path();

    // Check if it's file. Dumb but functional way
    if path.contains(".") {
        let _ = File::open(&path).get_deleted();
        return Ok(String::new());
    }

    let _ = Directory::new(&path).get_deleted();

    Ok(String::new())
}