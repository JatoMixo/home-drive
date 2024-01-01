mod download;
use download::handle_download_route;

mod explorer;
use explorer::handle_explorer;

mod file_manager;

mod visualizer;
use visualizer::handle_visualizer_route;

#[tokio::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut app = tide::new();

    handle_download_route(&mut app).await?;
    app.at("/content/*").get(handle_visualizer_route);
    app.at("/*").get(handle_explorer);

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
