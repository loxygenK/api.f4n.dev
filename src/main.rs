mod domain;
mod logger;
mod server;
mod service;

use server::execute_server;

use crate::server::Host;

#[tokio::main]
async fn main() {
    logger::init();

    execute_server(Host::Localhost, 8000)
        .await
        .expect("Server failed");
}
