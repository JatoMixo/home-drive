use crate::file_manager::File;

#[derive(serde::Deserialize)]
struct FileCreationRequest {
    name: String,
    path: String,
    content: Vec<u8>,
}

fn join_name_and_path(name: &str, path: &str) -> String {
    format!("{}/{}", path, name)
}

pub async fn handle_upload_route(mut req: tide::Request<()>) -> tide::Result<String> {
    let FileCreationRequest {name, path, content} = req.body_json().await?;

    let _ = File::create(&join_name_and_path(&name, &path), content);
    Ok(String::new())
}
