use backend::{database, web_app};
use actix_web::dev::Server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    database::init_db().await;
    web_app::run()?.await
}
