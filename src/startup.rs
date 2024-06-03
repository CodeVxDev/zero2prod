use crate::routes::{greet, health_check, subscribe};
use actix_web::{dev::Server, web, web::Data, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listner: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listner)?
    .run();

    Ok(server)
}
