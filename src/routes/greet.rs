use actix_web::Responder;

pub async fn greet() -> impl Responder {
    "Welcome to my first Http server for newsletter mailing list".to_string()
}
