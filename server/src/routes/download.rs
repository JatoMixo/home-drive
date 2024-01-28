use tide::{StatusCode, Response};
use crate::file_manager::File;

pub async fn handle_download_route(req: tide::Request<()>) -> tide::Result<Response> {

    const ROUTE_NAME: &str = "/download/";
    let path: &str = req
        .url()
        .path()
        .strip_prefix(ROUTE_NAME)
        .unwrap();

    let file = File::open(path);

    const DOWNLOAD_FILE_HTTP_HEADER: &str = "application/octet-stream";
    let response = Response::builder(StatusCode::Ok)
        .body(file.get_content())
        .header("content-type", DOWNLOAD_FILE_HTTP_HEADER)
        .build();

    Ok(response)
}
