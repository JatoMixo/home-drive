mod download;
use download::handle_download_route;
use std::fs::read_dir;

async fn explorer(req: tide::Request<()>) -> tide::Result<tide::Body> {
    let file_path = req.url().path();
    let file_list = read_dir(format!("drive-storagement/{}", file_path)).unwrap().map(|file| {
        String::from(file.unwrap().path().file_name().unwrap().to_str().unwrap())
    }).collect::<Vec<String>>();

    Ok(tide::Body::from_json(&file_list)?)
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut app = tide::new();

    handle_download_route(&mut app).await?;
    app.at("/*").get(explorer);

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
