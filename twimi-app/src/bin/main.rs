#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = twimi_app::startup::ApiServer::build()?;
    server.run().await?;
    Ok(())
}
