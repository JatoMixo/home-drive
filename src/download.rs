use crate::directory_reader::STORAGEMENT_DIR_NAME;

pub async fn handle_download_route(app: &mut tide::Server<()>) -> tide::Result<()> {
    const ROUTE: &str = "/download";

    app.at(ROUTE).serve_dir(STORAGEMENT_DIR_NAME)?;

    Ok(())
}
