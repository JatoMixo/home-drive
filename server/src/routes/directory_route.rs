use crate::file_manager::Directory;

pub async fn handle_directory_creation_route(req: tide::Request<()>) -> tide::Result<String> {

    let _directory: Directory = Directory::new(&req.query::<Directory>()?.get_path());

    Ok(String::new())
}