use std::fs::read_dir;

pub async fn handle_explorer(req: tide::Request<()>) -> tide::Result<tide::Body> {
    let file_path = req.url().path();

    let file_list = read_dir(format!("drive-storagement/{}", file_path)).unwrap().map(|file| {

        String::from(file.unwrap()
            .path()
            .file_name().unwrap()
            .to_str().unwrap())

    }).collect::<Vec<String>>();

    Ok(tide::Body::from_json(&file_list)?)
}
