use tide::Request;

#[tokio::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut app = tide::new();

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
