mod download;
use download::handle_download_route;

mod explorer;
use explorer::handle_explorer;

#[tokio::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut app = tide::new();

    handle_download_route(&mut app).await?;
    app.at("/*").get(handle_explorer);

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
