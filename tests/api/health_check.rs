use reqwest::{StatusCode, Url};

use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let url = Url::parse(&format!("{}/health_check", address)).expect("Failed to parse url");

    let response = client
        .get(url)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn health_check_db_works() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let url = Url::parse(&format!("{}/health_check/database", address)).expect("Failed to parse url");

    let response = client
        .get(url)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    assert_eq!(response.content_length(), Some(0));
}
