pub async fn handle_download_route(app: &mut tide::Server<()>) -> tide::Result<()> {
    const ROUTE: &str = "/download";
    const DIR_NAME: &str = "drive-storagement";

    app.at(ROUTE).serve_dir(DIR_NAME)?;

    Ok(())
}
