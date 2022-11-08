use twimi::startup::ApiServer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = ApiServer::build()?;
    server.run().await?;
    Ok(())
}
