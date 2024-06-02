use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let settings = get_configuration().expect("Failed to read the configuration!");
    let address = format!("127.0.0.1:{}", settings.application_port);
    let listner = TcpListener::bind(address)?;
    run(listner)?.await
}
