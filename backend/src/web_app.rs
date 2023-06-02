use actix_web::{web, get, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new().service(pong)
        })
        .bind("127.0.0.1:8030")?
        .run();

    Ok(server)
}

#[get("/ping")]
async fn pong() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

