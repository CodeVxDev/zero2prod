use actix_web::Responder;

pub async fn greet() -> impl Responder {
    format!("Welcome to my first Http server for newsletter mailing list")
}
