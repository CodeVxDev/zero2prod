use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpServer};
use crate::routes::{greet, health_check, subscribe};

pub fn run(listner: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listner)?
    .run();

    Ok(server)
}
