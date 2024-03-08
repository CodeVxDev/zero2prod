use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    let port = listner.local_addr().unwrap().port();
    println!("{port}");
    run(listner)?.await
}
