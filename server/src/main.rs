mod download;
use download::handle_download_route;

mod explorer;
use explorer::handle_explorer;

mod delete;
use delete::handle_delete_route;

mod directory_route;
use directory_route::handle_directory_creation_route;

mod file_manager;

#[tokio::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut app = tide::new();

    handle_download_route(&mut app).await?;
    app.at("/explorer/*").get(handle_explorer);
    app.at("/delete/*").get(handle_delete_route);
    app.at("/directory").get(handle_directory_creation_route);

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
