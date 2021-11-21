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

    let mode = std::env::var("FLISAN_PF_API_MODE").unwrap_or("development".to_string()).to_lowercase();
    let mode = match mode.as_str() {
        "development" => Mode::Development,
        _ => Mode::Production
    };

    setup(mode)
        .execute_server(Host::Localhost, 8000)
        .await
        .expect("Server failed");
}
