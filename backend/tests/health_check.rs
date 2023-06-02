use std::net::TcpListener;

#[tokio::test]

async fn health_check_works() {
    // launch the server as a background task
    let listner = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind a random port");
    let port = listner.local_addr().unwrap().port();
    let ip = listner.local_addr().unwrap().ip();

    spawn_app(listner);

    let client = reqwest::Client::new();
    let url = format!("http://{}:{}/ping", ip, port);
    let response = client.get(url)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(4), response.content_length())

}

/**
 * spawn the server as a background service with tokio::spawn()
 */
fn spawn_app(listener:TcpListener) {
    let server = backend::web_app::run(listener).expect("Failed to spawn our app");
    let _ = tokio::spawn(server);
}