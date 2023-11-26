mod download;
use download::handle_download_route;

async fn test(req: tide::Request<()>) -> tide::Result<String> {
    let file_path = req.url().path();

    Ok(file_path.to_string())
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut app = tide::new();

    handle_download_route(&mut app).await?;
    app.at("/*").get(test);

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
