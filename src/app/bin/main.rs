#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = twimi::app::startup::ApiServer::build()?;
    server.run().await?;
    Ok(())
}
