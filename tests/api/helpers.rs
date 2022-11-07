use twimi::startup::ApiServer;

pub async fn spawn_app() -> String {
    let server =
        ApiServer::build(("localhost", 0)).expect("Failed to build server as a background task");
    let port = server.port();
    let _ = tokio::spawn(server.run());

    format!("http://localhost:{port}")
}
