use std::str::FromStr;

use reqwest::Url;

use twimi_app::server::startup::ApiServer;

pub async fn spawn_app() -> Url {
    std::env::set_var("ACTIX_CONFIG_FILE", "../config/config.toml");
    let server = ApiServer::build().expect("Failed to build server as a background task");
    let port = server.port();
    let _ = tokio::spawn(server.run());

    Url::from_str(&format!("http://localhost:{}", port)).unwrap()
}
