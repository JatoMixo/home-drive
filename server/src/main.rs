mod routes;
use routes::download::handle_download_route;
use routes::explorer::handle_explorer;
use routes::delete::handle_delete_route;
use routes::directory_route::handle_directory_creation_route;
use routes::upload::handle_upload_route;

mod file_manager;

#[tokio::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut app = tide::new();

    handle_download_route(&mut app).await?;
    app.at("/explorer/*").get(handle_explorer);
    app.at("/delete").get(handle_delete_route);
    app.at("/directory").get(handle_directory_creation_route);
    app.at("/upload").post(handle_upload_route);

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
