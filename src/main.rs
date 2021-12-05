mod di;
mod domain;
mod logger;
mod repository;
mod router;
mod server;
mod service;

use di::{setup, Mode};
use server::host::Host;

#[tokio::main]
async fn main() {
    logger::init();

    let selected_port: u16 = std::env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse()
        .expect("Port should be numeric");

    let mode = std::env::var("FLISAN_PF_API_MODE").unwrap_or("development".to_string()).to_lowercase();
    let mode = match mode.as_str() {
        "development" => Mode::Development,
        _ => Mode::Production
    };

    setup(mode)
        .execute_server(Host::Ipv4("0.0.0.0"), selected_port)
        .await
        .expect("Server failed");
}
