use twimi::startup::ApiServer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // TODO: toml とかの config ファイルから設定を読み込むようにしたい
    let server = ApiServer::build(("127.0.0.1", 8080))?;
    server.run().await?;

    Ok(())
}
