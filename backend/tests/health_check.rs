#[tokio::test]

async fn health_check_works() {
    // launch the server as a background task
    spawn_app();

    let client = reqwest::Client::new();

    let response = client.get("http://127.0.0.1:8030/ping")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(4), response.content_length())

}

/**
 * spawn the server as a background service with tokio::spawn()
 */
fn spawn_app() {
    let server = backend::web_app::run().expect("Failed to spawn our app");
    let _ = tokio::spawn(server);
}