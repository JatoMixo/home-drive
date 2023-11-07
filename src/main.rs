mod download;
use download::handle_download_route;

#[tokio::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut app = tide::new();

    handle_download_route(&mut app).await?;

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
