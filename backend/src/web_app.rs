use std::net::TcpListener;
use actix_web::{web, get, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    println!("host {}, port {}", listener.local_addr().unwrap().ip(), listener.local_addr().unwrap().port());
    let server = HttpServer::new(|| {
            App::new().service(pong)
        })
        .listen(listener)?
        .run();

    Ok(server)
}

#[get("/ping")]
async fn pong() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

