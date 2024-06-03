use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let settings = get_configuration().expect("Failed to read the configuration!");
    let connection = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to Database");
    let address = format!("127.0.0.1:{}", settings.application_port);
    let listner = TcpListener::bind(address)?;
    run(listner, connection)?.await
}
