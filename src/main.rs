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

    setup(Mode::Development)
        .execute_server(Host::Localhost, 8000)
        .await
        .expect("Server failed");
}
