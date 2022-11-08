use twimi::startup::{parse_config_file, ApiServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let settings = parse_config_file().expect("Failed to parse settings");
    let server = ApiServer::build(settings)?;
    server.run().await?;

    Ok(())
}
