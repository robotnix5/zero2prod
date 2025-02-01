//! main.rs

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
    println!("listener: {:?}", listener);
    run(listener)?.await
}
