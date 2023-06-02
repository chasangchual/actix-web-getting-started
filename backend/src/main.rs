use actix_web::{web, get, App, HttpRequest, HttpServer, Responder, HttpResponse};

#[get("/ping")]
async fn pong(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(pong)
    })
    .bind("127.0.0.1:8030")?
    .run()
    .await
}
