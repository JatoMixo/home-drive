async fn handle_download_route(app: &mut tide::Server<()>) -> tide::Result<()> {
    const ROUTE: &str = "/download";
    const DIR_NAME: &str = "drive-storagement";

    app.at(ROUTE).serve_dir(DIR_NAME)?;

    Ok(())
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut app = tide::new();

    handle_download_route(&mut app).await?;

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
