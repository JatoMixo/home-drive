use tide::{StatusCode, Response};

use crate::file_manager::File;

pub async fn handle_download_route(req: tide::Request<()>) -> tide::Result<Response> {

    const ROUTE_NAME: &str = "/download/";

    let path: String = req
        .url()
        .path()
        .strip_prefix(ROUTE_NAME)
        .unwrap()
        .to_string();

    let file = File::open(&path);

    let response = Response::builder(StatusCode::Ok)
        .body(file.get_content())
        .header("content-type", "application/octet-stream")
        .build();

    Ok(response)
}
