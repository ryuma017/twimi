use reqwest::{StatusCode, Url};

use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let base_url = spawn_app().await;
    let client = reqwest::Client::new();
    let endpoint = base_url.join("/health_check").unwrap();

    let response = client
        .get(endpoint)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn health_check_db_works() {
    let base_url = spawn_app().await;
    let client = reqwest::Client::new();
    let endpoint = base_url.join("/health_check/database").unwrap();

    let response = client
        .get(endpoint)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    assert_eq!(response.content_length(), Some(0));
}
