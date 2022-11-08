use std::str::FromStr;

use actix_settings::Settings;
use reqwest::Url;

use twimi::startup::{parse_config_file, ApiServer};

pub async fn spawn_app() -> Url {
    let mut settings = parse_config_file().unwrap();
    let server = ApiServer::build(settings).expect("Failed to build server as a background task");
    let port = server.port();
    let _ = tokio::spawn(server.run());

    Url::from_str(&format!("http://localhost:{}", port)).unwrap()
}
