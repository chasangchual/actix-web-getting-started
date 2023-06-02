use backend::{database, web_app};
use actix_web::dev::Server;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    database::init_db().await;

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind a random port");

    web_app::run(listener)?.await
}
