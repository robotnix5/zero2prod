//! main.rs

use zero2prod::configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = configuration::get_configuration().expect("failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = std::net::TcpListener::bind(address)?;
    run(listener)?.await
}
