#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = twimi_app::server::startup::ApiServer::build()?;
    server.run().await?;
    Ok(())
}
